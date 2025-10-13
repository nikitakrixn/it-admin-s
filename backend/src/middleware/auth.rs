use poem::Request;
use poem_openapi::auth::Bearer;
use poem_openapi::SecurityScheme;
use std::sync::Arc;
use uuid::Uuid;

use crate::services::auth_service::{AuthService, Claims};

/// JWT Bearer Authentication для OpenAPI
/// 
/// Использование в handlers:
/// ```rust
/// #[oai(path = "/protected", method = "get")]
/// async fn protected_endpoint(&self, auth: JwtAuth) -> Response {
///     let user_id = auth.0.sub; // UUID пользователя
///     // ...
/// }
/// ```
#[derive(SecurityScheme)]
#[oai(
    ty = "bearer",
    bearer_format = "JWT",
    checker = "jwt_checker"
)]
pub struct JwtAuth(pub Claims);

/// Функция проверки JWT токена
/// 
/// Вызывается автоматически poem-openapi при каждом запросе к защищенному endpoint
async fn jwt_checker(req: &Request, bearer: Bearer) -> Option<Claims> {
    // Получаем AuthService из app data
    let auth_service = req.data::<Arc<AuthService>>()?;
    
    // Проверяем токен
    match auth_service.verify_token(&bearer.token) {
        Ok(claims) => {
            tracing::debug!("JWT verified for user: {}", claims.sub);
            Some(claims)
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
