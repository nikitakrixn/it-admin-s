use crate::config::database::Pool;
use crate::utils::error::AppError;
use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;

/// Get database connection from pool with error handling
pub async fn get_connection(pool: &Pool) -> Result<Object<AsyncPgConnection>, AppError> {
    pool.get().await.map_err(AppError::from)
}

/// Type alias for database result
pub type DbResult<T> = Result<T, AppError>;
