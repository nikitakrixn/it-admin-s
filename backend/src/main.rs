mod config;
mod integrations;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

use poem::{
    EndpointExt, Route, Server,
    listener::TcpListener,
    middleware::{Cors, Tracing},
};
use poem_openapi::OpenApiService;
use std::sync::Arc;
use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration
    let config = config::Config::from_env().expect("Failed to load configuration");
    config.validate().expect("Invalid configuration");

    // Initialize tracing with log level from config
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{},it_admin_backend=debug", config.app.log_level).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!(
        "Starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    tracing::info!("Environment: {}", config.app.env);
    tracing::info!("Log level: {}", config.app.log_level);

    // Log optional integrations status
    if let Some(ad_config) = &config.ad
        && ad_config.enabled
    {
        tracing::info!("Active Directory integration enabled: {}", ad_config.server);
    }

    if let Some(mikrotik_config) = &config.mikrotik
        && mikrotik_config.enabled
    {
        tracing::info!(
            "MikroTik integration enabled: {}:{}",
            mikrotik_config.host,
            mikrotik_config.port
        );
    }

    // Run migrations using sync pool
    tracing::info!("Running database migrations...");
    {
        use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

        let sync_pool = config::database::create_sync_pool(&config.database)
            .expect("Failed to create sync pool for migrations");

        let mut conn = sync_pool
            .get()
            .expect("Failed to get DB connection for migrations");
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");
        tracing::info!("Database migrations completed successfully");
    }

    // Initialize async database connection pool for main application
    let db_pool = config::database::create_async_pool(&config.database)
        .expect("Failed to create async database pool");

    // Create services (wrap AuthService in Arc for sharing)
    let auth_service = std::sync::Arc::new(services::auth_service::AuthService::new(
        db_pool.clone(),
        config.jwt.secret.clone(),
        config.jwt.expiration,
    ));

    let activity_log = services::activity_log_service::ActivityLogService::new(db_pool.clone());

    // На чистой базе создаём первого администратора из переменных окружения.
    // Пароль проверяется по общей политике, поэтому должен быть не короче 12 символов.
    let users_count = auth_service
        .count_users()
        .await
        .expect("Failed to count users");

    if users_count == 0 {
        match &config.bootstrap_admin {
            Some(admin) => {
                auth_service
                    .register_user(
                        admin.email.clone(),
                        admin.password.clone(),
                        Some("admin".to_string()),
                    )
                    .await
                    .expect("Failed to create bootstrap administrator");
                tracing::info!("Создан начальный администратор: {}", admin.email);
            }
            None if config.app.env == "production" => {
                panic!(
                    "В базе нет пользователей. Задайте BOOTSTRAP_ADMIN_EMAIL и BOOTSTRAP_ADMIN_PASSWORD для создания администратора"
                );
            }
            None => {
                tracing::warn!(
                    "В базе нет пользователей. Задайте BOOTSTRAP_ADMIN_EMAIL и BOOTSTRAP_ADMIN_PASSWORD, чтобы создать администратора"
                );
            }
        }
    }

    // Initialize MikroTik client if enabled
    let mikrotik_client = if let Some(ref mikrotik_config) = config.mikrotik {
        if mikrotik_config.enabled {
            match integrations::mikrotik::MikrotikClient::new(mikrotik_config) {
                Ok(client) => {
                    tracing::info!("MikroTik client initialized successfully");
                    Some(std::sync::Arc::new(client))
                }
                Err(e) => {
                    tracing::error!("Failed to initialize MikroTik client: {}", e);
                    None
                }
            }
        } else {
            None
        }
    } else {
        None
    };

    // Create WireGuard service
    let wireguard_service =
        std::sync::Arc::new(services::wireguard_service::WireguardService::new(
            db_pool.clone(),
            mikrotik_client.clone(),
        ));

    // Create API service with OpenAPI documentation.
    // poem-openapi реализует OpenApi для кортежей не длиннее 16 элементов,
    // поэтому список API вложен: сначала основной набор, затем новые разделы.
    let core_apis = (
        routes::auth::AuthApi::new(auth_service.clone()),
        routes::employees::EmployeesApi::new(db_pool.clone()),
        routes::departments::DepartmentsApi::new(db_pool.clone()),
        routes::positions::PositionsApi::new(db_pool.clone()),
        routes::activity_log::ActivityLogApi::new(db_pool.clone()),
        routes::software::SoftwareApi::new(db_pool.clone(), activity_log.clone()),
        routes::computers::ComputersApi::new(db_pool.clone()),
        routes::requests::RequestsApi::new(db_pool.clone()),
        routes::consumables::ConsumablesApi::new(db_pool.clone()),
        routes::equipment::EquipmentApi::new(db_pool.clone()),
        routes::licenses::LicensesApi::new(db_pool.clone()),
        routes::dashboard::DashboardApi::new(db_pool.clone()),
        routes::network::NetworkApi::new(db_pool.clone()),
        routes::ad::AdApi::new(db_pool.clone()),
        routes::monitoring::MonitoringApi::new(db_pool.clone()),
        routes::wireguard::WireguardApi::new(
            db_pool.clone(),
            wireguard_service.clone(),
            mikrotik_client.clone(),
        ),
    );

    let api_service = OpenApiService::new(
        (
            core_apis,
            routes::inventory::InventoryApi::new(db_pool.clone()),
            routes::imports::ImportApi::new(db_pool.clone()),
        ),
        &config.app.name,
        env!("CARGO_PKG_VERSION"),
    )
    .server(format!(
        "http://{}:{}/api",
        config.app.host, config.app.port
    ));

    // Create Swagger UI
    let ui = api_service.swagger_ui();
    let spec = api_service.spec_endpoint();

    // Build CORS middleware from config
    let mut cors = Cors::new()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
        .allow_credentials(true)
        .max_age(3600);

    // Add allowed origins from config
    for origin in &config.cors.allowed_origins {
        cors = cors.allow_origin(origin.as_str());
    }

    // Build application routes. Документация отключается в production через DOCS_ENABLED.
    let mut app = Route::new()
        .nest("/api", api_service)
        .at("/health", poem::endpoint::make_sync(|_| "OK"));

    if config.docs_enabled {
        app = app.nest("/docs", ui).nest("/api-spec", spec);
    }

    // Ограничиваем частоту попыток входа: 20 запросов за 5 минут с одного адреса.
    let login_limiter = Arc::new(utils::rate_limit::FixedWindowLimiter::new(
        20,
        Duration::from_secs(300),
    ));

    let app = app
        .with(middleware::auth::RequireAuth::new(login_limiter))
        .data(auth_service) // AuthService доступен middleware и jwt_checker
        .with(cors)
        .with(Tracing);

    // Start server
    let addr = format!("{}:{}", config.app.host, config.app.port);
    tracing::info!("Server listening on http://{}", addr);
    tracing::info!("API documentation available at http://{}/docs", addr);

    Server::new(TcpListener::bind(&addr)).run(app).await
}
