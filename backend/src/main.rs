mod config;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;
mod integrations;

use poem::{
    listener::TcpListener,
    middleware::{Cors, Tracing},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,it_admin_backend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = config::Config::from_env().expect("Failed to load configuration");

    tracing::info!("Starting {} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    tracing::info!("Environment: {}", config.app.env);

    // Initialize database connection pool
    let db_pool = config::database::create_pool(&config.database)
        .expect("Failed to create database pool");

    // Run migrations
    tracing::info!("Running database migrations...");
    {
        use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
        
        let mut conn = db_pool.get().expect("Failed to get DB connection for migrations");
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
        ),
        &config.app.name,
        env!("CARGO_PKG_VERSION"),
    )
    .server(format!("http://{}:{}/api", config.app.host, config.app.port));

    // Create Swagger UI
    let ui = api_service.swagger_ui();
    let spec = api_service.spec_endpoint();

    // Build application routes
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/docs", ui)
        .nest("/api-spec", spec)
        .at("/health", poem::endpoint::make_sync(|_| "OK"))
        .with(Cors::new()
            .allow_origin("http://localhost:3000")
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
            .allow_credentials(true)
            .max_age(3600)
        )
        .with(Tracing);

    // Start server
    let addr = format!("{}:{}", config.app.host, config.app.port);
    tracing::info!("Server listening on http://{}", addr);
    tracing::info!("API documentation available at http://{}/docs", addr);

    Server::new(TcpListener::bind(&addr))
        .run(app)
        .await
}
