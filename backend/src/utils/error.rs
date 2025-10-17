use serde::Serialize;
use std::fmt;

/// Unified error response structure
#[derive(Serialize, poem_openapi::Object, Clone)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

/// Application error types
#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    NotFound(String),
    ValidationError(String),
    Unauthorized(String),
    Conflict(String),
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl AppError {
    pub fn to_error_response(&self) -> ErrorResponse {
        let error_type = match self {
            AppError::DatabaseError(_) => "database_error",
            AppError::NotFound(_) => "not_found",
            AppError::ValidationError(_) => "validation_error",
            AppError::Unauthorized(_) => "unauthorized",
            AppError::Conflict(_) => "conflict",
            AppError::InternalError(_) => "internal_error",
        };

        ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
        }
    }
}

/// Convert diesel errors to AppError
impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => AppError::NotFound("Resource not found".to_string()),
            diesel::result::Error::DatabaseError(kind, _info) => {
                AppError::DatabaseError(format!("Database error: {:?}", kind))
            }
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

/// Convert pool errors to AppError
impl From<diesel_async::pooled_connection::deadpool::PoolError> for AppError {
    fn from(err: diesel_async::pooled_connection::deadpool::PoolError) -> Self {
        AppError::DatabaseError(format!("Failed to get database connection: {}", err))
    }
}
