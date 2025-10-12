use crate::config::database::Pool;
use poem_openapi::{OpenApi, payload::Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, poem_openapi::Object)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub struct Api {
    db_pool: Pool,
}

impl Api {
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }
}

#[OpenApi]
impl Api {
    /// Health check endpoint
    #[oai(path = "/health", method = "get", tag = "ApiTags::System")]
    async fn health(&self) -> Json<HealthResponse> {
        Json(HealthResponse {
            status: "ok".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }
}

#[derive(poem_openapi::Tags)]
pub enum ApiTags {
    /// System endpoints
    System,
    /// Authentication endpoints
    Auth,
    /// Computer management
    Computers,
    /// Employee management
    Employees,
    /// Department management
    Departments,
    /// Position management
    Positions,
    /// Equipment management
    Equipment,
    /// Activity log and audit
    ActivityLog,
}
