use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

use super::DatabaseConfig;

pub fn create_pool(config: &DatabaseConfig) -> Result<Pool, Box<dyn std::error::Error>> {
    let manager = ConnectionManager::<PgConnection>::new(&config.url);

    let pool = r2d2::Pool::builder()
        .max_size(config.max_connections)
        .build(manager)?;

    // Test connection
    let _conn = pool.get()?;
    tracing::info!("Database connection pool created successfully");

    Ok(pool)
}
