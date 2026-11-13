use poem::Request;
use poem_openapi::SecurityScheme;
use poem_openapi::auth::Bearer;
use std::sync::Arc;
use uuid::Uuid;

use crate::services::auth_service::{AuthService, Claims};

/// Admin-only JWT Authentication
#[derive(SecurityScheme)]
#[oai(ty = "bearer", bearer_format = "JWT", checker = "admin_checker")]
pub struct AdminAuth(pub Claims);

/// Check if admin
async fn admin_checker(req: &Request, bearer: Bearer) -> Option<Claims> {
    let auth_service = req.data::<Arc<AuthService>>()?;

    match auth_service.verify_token(&bearer.token) {
        Ok(claims) => {
            if claims.role == "admin" {
                tracing::debug!("Admin JWT verified for user: {}", claims.sub);
                Some(claims)
            } else {
                tracing::warn!(
                    "Access denied: user {} is not admin (role: {})",
                    claims.sub,
                    claims.role
                );
                None
            }
        }
        Err(e) => {
            tracing::warn!("JWT verification failed: {}", e);
            None
        }
    }
}

/// Helper trait для извлечения user_id из Claims
pub trait ClaimsExt {
    fn user_id(&self) -> Option<Uuid>;
}

impl ClaimsExt for Claims {
    fn user_id(&self) -> Option<Uuid> {
        self.sub.parse().ok()
    }
}
