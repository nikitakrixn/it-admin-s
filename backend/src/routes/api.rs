use poem_openapi::{OpenApi, payload::Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, poem_openapi::Object)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub struct Api;

impl Api {
    pub fn new() -> Self {
        Self
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
    /// Software catalog
    Software,
    /// Activity log and audit
    ActivityLog,
}
