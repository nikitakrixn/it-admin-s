mod config;
mod integrations;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use poem::{
    EndpointExt, Route, Server,
    listener::TcpListener,
    middleware::{Cors, Tracing},
};
use poem_openapi::OpenApiService;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration
    let config = config::Config::from_env().expect("Failed to load configuration");

    // Initialize tracing with log level from config
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{},it_admin_backend=debug", config.app.log_level).into()),
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
    if let Some(ad_config) = &config.ad {
        if ad_config.enabled {
            tracing::info!("Active Directory integration enabled: {}", ad_config.server);
        }
    }
    
    if let Some(mikrotik_config) = &config.mikrotik {
        if mikrotik_config.enabled {
            tracing::info!("MikroTik integration enabled: {}:{}", mikrotik_config.host, mikrotik_config.port);
        }
    }

    // Initialize database connection pool
    let db_pool =
        config::database::create_pool(&config.database).expect("Failed to create database pool");

    // Run migrations
    tracing::info!("Running database migrations...");
    {
        use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

        let mut conn = db_pool
            .get()
            .expect("Failed to get DB connection for migrations");
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");
        tracing::info!("Database migrations completed successfully");
    }

    // Create services
    let auth_service = services::auth_service::AuthService::new(
        db_pool.clone(),
        config.jwt.secret.clone(),
        config.jwt.expiration,
    );

    // Create API service with OpenAPI documentation
    let api_service = OpenApiService::new(
        (
            routes::api::Api::new(db_pool.clone()),
            routes::auth::AuthApi::new(auth_service),
            routes::employees::EmployeesApi::new(db_pool.clone()),
            routes::activity_log::ActivityLogApi::new(db_pool.clone()),
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

    // Build application routes
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/docs", ui)
        .nest("/api-spec", spec)
        .at("/health", poem::endpoint::make_sync(|_| "OK"))
        .with(cors)
        .with(Tracing);

    // Start server
    let addr = format!("{}:{}", config.app.host, config.app.port);
    tracing::info!("Server listening on http://{}", addr);
    tracing::info!("API documentation available at http://{}/docs", addr);

    Server::new(TcpListener::bind(&addr)).run(app).await
}
