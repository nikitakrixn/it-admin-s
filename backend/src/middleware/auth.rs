use poem::{
    http::StatusCode,
    web::headers::{authorization::Bearer, Authorization},
    Endpoint, Error, Middleware, Request, Result,
};
use std::sync::Arc;

use crate::services::auth_service::{AuthService, Claims};

/// Middleware для проверки JWT токена
pub struct AuthMiddleware {
    auth_service: Arc<AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }
}

impl<E: Endpoint> Middleware<E> for AuthMiddleware {
    type Output = AuthMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthMiddlewareImpl {
            ep,
            auth_service: self.auth_service.clone(),
        }
    }
}

pub struct AuthMiddlewareImpl<E> {
    ep: E,
    auth_service: Arc<AuthService>,
}

impl<E: Endpoint> Endpoint for AuthMiddlewareImpl<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        // Извлечь Authorization header
        let auth_header = req
            .headers()
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| {
                Error::from_string(
                    "Missing authorization header",
                    StatusCode::UNAUTHORIZED,
                )
            })?;

        // Проверить формат "Bearer <token>"
        if !auth_header.starts_with("Bearer ") {
            return Err(Error::from_string(
                "Invalid authorization header format",
                StatusCode::UNAUTHORIZED,
            ));
        }

        let token = &auth_header[7..]; // Убрать "Bearer "

        // Валидировать токен
        let claims = self
            .auth_service
            .verify_token(token)
            .map_err(|_| {
                Error::from_string(
                    "Invalid or expired token",
                    StatusCode::UNAUTHORIZED,
                )
            })?;

        // Сохранить claims в extensions для использования в handlers
        req.extensions_mut().insert(claims);

        // Продолжить обработку запроса
        self.ep.call(req).await
    }
}

/// Helper для извлечения Claims из request
pub trait RequestExt {
    fn claims(&self) -> Option<&Claims>;
}

impl RequestExt for Request {
    fn claims(&self) -> Option<&Claims> {
        self.extensions().get::<Claims>()
    }
}
