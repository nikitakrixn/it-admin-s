use poem_openapi::{ApiResponse, Object, OpenApi, param::Header, payload::Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::user::UserResponse;
use crate::routes::api::ApiTags;
use crate::services::auth_service::AuthService;
use crate::utils::error::{AppError, ErrorResponse};

#[derive(Debug, Deserialize, Validate, Object)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(length(min = 2, message = "Name must be at least 2 characters"))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, Object)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize, Object)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(ApiResponse)]
pub enum RegisterResponse {
    #[oai(status = 201)]
    Created(Json<AuthResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 409)]
    Conflict(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum LoginResponse {
    #[oai(status = 200)]
    Ok(Json<AuthResponse>),
    #[oai(status = 401)]
    Unauthorized(Json<ErrorResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum MeResponse {
    #[oai(status = 200)]
    Ok(Json<UserResponse>),
    #[oai(status = 401)]
    Unauthorized(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// Auth API
// ============================================================================

pub struct AuthApi {
    auth_service: std::sync::Arc<AuthService>,
}

impl AuthApi {
    pub fn new(auth_service: std::sync::Arc<AuthService>) -> Self {
        Self { auth_service }
    }
}

#[OpenApi(prefix_path = "/auth", tag = "ApiTags::Auth")]
impl AuthApi {
    #[oai(path = "/register", method = "post")]
    async fn register(&self, req: Json<RegisterRequest>) -> RegisterResponse {
        if let Err(e) = req.0.validate() {
            let validation_err = AppError::ValidationError(format!("Validation failed: {}", e));
            return RegisterResponse::BadRequest(Json(validation_err.to_error_response()));
        }

        match self
            .auth_service
            .register_user(req.0.email, req.0.password, Some("user".to_string()))
            .await
        {
            Ok(user) => match self.auth_service.generate_token(&user) {
                Ok(token) => RegisterResponse::Created(Json(AuthResponse {
                    token,
                    user: user.into(),
                })),
                Err(e) => {
                    RegisterResponse::InternalError(Json(e.to_error_response()))
                }
            },
            Err(e) => match e {
                AppError::Conflict(_) => RegisterResponse::Conflict(Json(e.to_error_response())),
                AppError::ValidationError(_) => RegisterResponse::BadRequest(Json(e.to_error_response())),
                _ => RegisterResponse::InternalError(Json(e.to_error_response())),
            }
        }
    }

    #[oai(path = "/login", method = "post")]
    async fn login(&self, req: Json<LoginRequest>) -> LoginResponse {
        if let Err(e) = req.0.validate() {
            let validation_err = AppError::ValidationError(format!("Validation failed: {}", e));
            return LoginResponse::BadRequest(Json(validation_err.to_error_response()));
        }

        match self.auth_service.authenticate(req.0.email, req.0.password).await {
            Ok((user, token)) => LoginResponse::Ok(Json(AuthResponse {
                token,
                user: user.into(),
            })),
            Err(e) => match e {
                AppError::Unauthorized(_) => LoginResponse::Unauthorized(Json(e.to_error_response())),
                AppError::ValidationError(_) => LoginResponse::BadRequest(Json(e.to_error_response())),
                _ => LoginResponse::InternalError(Json(e.to_error_response())),
            }
        }
    }

    #[oai(path = "/me", method = "get")]
    async fn me(&self, authorization: Header<Option<String>>) -> MeResponse {
        let token = match authorization.0 {
            Some(ref auth_header) if auth_header.starts_with("Bearer ") => &auth_header[7..],
            _ => {
                return MeResponse::Unauthorized(Json(ErrorResponse {
                    error: "unauthorized".to_string(),
                    message: "Missing or invalid authorization header".to_string(),
                }));
            }
        };

        let claims = match self.auth_service.verify_token(token) {
            Ok(claims) => claims,
            Err(e) => {
                return MeResponse::Unauthorized(Json(e.to_error_response()));
            }
        };

        let user_id = match uuid::Uuid::parse_str(&claims.sub) {
            Ok(id) => id,
            Err(_) => {
                return MeResponse::Unauthorized(Json(ErrorResponse {
                    error: "invalid_token".to_string(),
                    message: "Invalid user ID in token".to_string(),
                }));
            }
        };

        match self.auth_service.get_user_by_id(user_id).await {
            Ok(user) => MeResponse::Ok(Json(user.into())),
            Err(_) => MeResponse::Unauthorized(Json(ErrorResponse {
                error: "user_not_found".to_string(),
                message: "User not found".to_string(),
            }))
        }
    }
}
