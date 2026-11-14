use std::sync::Arc;

use poem::{
    Endpoint, IntoResponse, Middleware, Request, Response,
    http::{StatusCode, header},
};
use poem_openapi::SecurityScheme;
use poem_openapi::auth::Bearer;
use serde_json::json;
use uuid::Uuid;

use crate::services::auth_service::{AuthService, Claims};
use crate::utils::rate_limit::FixedWindowLimiter;

/// JWT-аутентификация для любого авторизованного пользователя.
#[derive(SecurityScheme)]
#[oai(ty = "bearer", bearer_format = "JWT", checker = "user_checker")]
pub struct UserAuth(pub Claims);

/// JWT-аутентификация только для администраторов.
#[derive(SecurityScheme)]
#[oai(ty = "bearer", bearer_format = "JWT", checker = "admin_checker")]
pub struct AdminAuth(pub Claims);

async fn user_checker(req: &Request, bearer: Bearer) -> Option<Claims> {
    let auth_service = req.data::<Arc<AuthService>>()?;

    match auth_service.verify_token(&bearer.token) {
        Ok(claims) => Some(claims),
        Err(e) => {
            tracing::warn!("JWT verification failed: {}", e);
            None
        }
    }
}

async fn admin_checker(req: &Request, bearer: Bearer) -> Option<Claims> {
    let claims = user_checker(req, bearer).await?;

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

/// Helper trait для извлечения user_id из Claims
pub trait ClaimsExt {
    fn user_id(&self) -> Option<Uuid>;
}

impl ClaimsExt for Claims {
    fn user_id(&self) -> Option<Uuid> {
        self.sub.parse().ok()
    }
}

/// Middleware, закрывающий все маршруты API авторизацией.
///
/// Проверка выполняется до обработчика, поэтому новый эндпоинт нельзя
/// случайно оставить публичным. Исключения — вход в систему и CORS-preflight.
pub struct RequireAuth {
    limiter: Arc<FixedWindowLimiter>,
}

impl RequireAuth {
    pub fn new(limiter: Arc<FixedWindowLimiter>) -> Self {
        Self { limiter }
    }
}

impl<E: Endpoint> Middleware<E> for RequireAuth {
    type Output = RequireAuthEndpoint<E>;

    fn transform(&self, next: E) -> Self::Output {
        RequireAuthEndpoint {
            next,
            limiter: self.limiter.clone(),
        }
    }
}

pub struct RequireAuthEndpoint<E> {
    next: E,
    limiter: Arc<FixedWindowLimiter>,
}

impl<E: Endpoint> Endpoint for RequireAuthEndpoint<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> poem::Result<Self::Output> {
        if req.method() == poem::http::Method::OPTIONS {
            return self.next.call(req).await.map(IntoResponse::into_response);
        }

        let path = req.uri().path();

        // Служебные маршруты не требуют токена: healthcheck нужен балансировщику,
        // а Swagger UI не умеет прикладывать Bearer к самому HTML.
        if path == "/health"
            || path == "/docs"
            || path.starts_with("/docs/")
            || path == "/api-spec"
            || path.starts_with("/api-spec/")
        {
            return self.next.call(req).await.map(IntoResponse::into_response);
        }

        // Вход должен быть доступен без токена, но с ограничением частоты.
        if path == "/api/auth/login" {
            let client = client_key(&req);
            if let Err(retry_after) = self.limiter.check(&client) {
                tracing::warn!("Login rate limit exceeded for {}", client);
                return Ok(too_many_requests(retry_after));
            }
            return self.next.call(req).await.map(IntoResponse::into_response);
        }

        let token = req
            .header(header::AUTHORIZATION)
            .and_then(|value| value.strip_prefix("Bearer "))
            .map(str::to_string);

        let Some(token) = token else {
            return Ok(unauthorized("Требуется авторизация"));
        };

        let Some(auth_service) = req.data::<Arc<AuthService>>() else {
            tracing::error!("AuthService is not available in request data");
            return Ok(internal_error());
        };

        match auth_service.verify_token(&token) {
            Ok(claims) => {
                tracing::debug!("JWT verified for user: {}", claims.sub);
                self.next.call(req).await.map(IntoResponse::into_response)
            }
            Err(e) => {
                tracing::warn!("JWT verification failed: {}", e);
                Ok(unauthorized("Сессия истекла, войдите заново"))
            }
        }
    }
}

/// Определяет клиента для rate limit: сначала X-Forwarded-For от reverse proxy,
/// затем адрес TCP-соединения.
fn client_key(req: &Request) -> String {
    req.header("x-forwarded-for")
        .and_then(|value| value.split(',').next())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| req.remote_addr().to_string())
}

fn unauthorized(message: &str) -> Response {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header(header::WWW_AUTHENTICATE, "Bearer")
        .content_type("application/json; charset=utf-8")
        .body(
            json!({
                "error": "unauthorized",
                "message": message,
            })
            .to_string(),
        )
}

fn too_many_requests(retry_after_seconds: u64) -> Response {
    Response::builder()
        .status(StatusCode::TOO_MANY_REQUESTS)
        .header(header::RETRY_AFTER, retry_after_seconds.to_string())
        .content_type("application/json; charset=utf-8")
        .body(
            json!({
                "error": "rate_limited",
                "message": format!(
                    "Слишком много попыток входа. Повторите через {} секунд.",
                    retry_after_seconds
                ),
            })
            .to_string(),
        )
}

fn internal_error() -> Response {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .content_type("application/json; charset=utf-8")
        .body(
            json!({
                "error": "internal_error",
                "message": "Внутренняя ошибка сервера",
            })
            .to_string(),
        )
}
