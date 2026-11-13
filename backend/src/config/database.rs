use diesel::PgConnection;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool as AsyncPool;

// Синхронный пул для миграций
pub type SyncPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;

// Асинхронный пул для основной работы
pub type Pool = AsyncPool<AsyncPgConnection>;

use super::DatabaseConfig;

/// Создает асинхронный пул соединений для основной работы приложения
pub fn create_async_pool(config: &DatabaseConfig) -> Result<Pool, Box<dyn std::error::Error>> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&config.url);

    let pool = AsyncPool::builder(manager)
        .max_size(config.max_connections as usize)
        .build()?;

    tracing::info!("Async database connection pool created successfully");

    Ok(pool)
}

/// Создает синхронный пул для миграций (используется только при старте)
pub fn create_sync_pool(config: &DatabaseConfig) -> Result<SyncPool, Box<dyn std::error::Error>> {
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(&config.url);

    let pool = diesel::r2d2::Pool::builder()
        .max_size(config.max_connections)
        .build(manager)?;

    // Test connection
    let _conn = pool.get()?;
    tracing::info!("Sync database connection pool created for migrations");

    Ok(pool)
}
