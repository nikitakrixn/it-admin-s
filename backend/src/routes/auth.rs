use poem_openapi::{
    ApiResponse, Object, OpenApi,
    param::{Header, Path},
    payload::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::middleware::auth::{AdminAuth, ClaimsExt, UserAuth};
use crate::models::user::UserResponse;
use crate::routes::api::ApiTags;
use crate::services::auth_service::AuthService;
use crate::utils::error::{AppError, ErrorResponse};

#[derive(Debug, Deserialize, Validate, Object)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 12, message = "Password must be at least 12 characters"))]
    pub password: String,

    /// Роль нового пользователя: `admin` или `user`. По умолчанию `user`.
    pub role: Option<String>,
}

#[derive(Debug, Deserialize, Validate, Object)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, Object)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, message = "Current password is required"))]
    pub current_password: String,

    #[validate(length(min = 12, message = "Password must be at least 12 characters"))]
    pub new_password: String,
}

#[derive(Debug, Deserialize, Object)]
pub struct UpdateUserRequest {
    pub role: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Object)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Object)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(ApiResponse)]
pub enum RegisterResponse {
    #[oai(status = 201)]
    Created(Json<UserResponse>),
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
    #[oai(status = 429)]
    TooManyRequests(Json<ErrorResponse>),
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

#[derive(ApiResponse)]
pub enum ChangePasswordResponse {
    #[oai(status = 200)]
    Ok(Json<MessageResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 401)]
    Unauthorized(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum UsersListResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<UserResponse>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum UpdateUserResponse {
    #[oai(status = 200)]
    Ok(Json<UserResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
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
    /// Создание пользователя. Доступно только администраторам.
    #[oai(path = "/register", method = "post")]
    async fn register(&self, _auth: AdminAuth, req: Json<RegisterRequest>) -> RegisterResponse {
        if let Err(e) = req.0.validate() {
            let validation_err = AppError::ValidationError(format!("Validation failed: {}", e));
            return RegisterResponse::BadRequest(Json(validation_err.to_error_response()));
        }

        match self
            .auth_service
            .register_user(req.0.email, req.0.password, req.0.role)
            .await
        {
            Ok(user) => RegisterResponse::Created(Json(user.into())),
            Err(e) => match e {
                AppError::Conflict(_) => RegisterResponse::Conflict(Json(e.to_error_response())),
                AppError::ValidationError(_) => {
                    RegisterResponse::BadRequest(Json(e.to_error_response()))
                }
                _ => RegisterResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    #[oai(path = "/login", method = "post")]
    async fn login(&self, req: Json<LoginRequest>) -> LoginResponse {
        if let Err(e) = req.0.validate() {
            let validation_err = AppError::ValidationError(format!("Validation failed: {}", e));
            return LoginResponse::BadRequest(Json(validation_err.to_error_response()));
        }

        match self
            .auth_service
            .authenticate(req.0.email, req.0.password)
            .await
        {
            Ok((user, token)) => LoginResponse::Ok(Json(AuthResponse {
                token,
                user: user.into(),
            })),
            Err(e) => match e {
                AppError::Unauthorized(_) => {
                    LoginResponse::Unauthorized(Json(e.to_error_response()))
                }
                AppError::ValidationError(_) => {
                    LoginResponse::BadRequest(Json(e.to_error_response()))
                }
                _ => LoginResponse::InternalError(Json(e.to_error_response())),
            },
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
            })),
        }
    }

    /// Смена собственного пароля.
    #[oai(path = "/change-password", method = "post")]
    async fn change_password(
        &self,
        auth: UserAuth,
        req: Json<ChangePasswordRequest>,
    ) -> ChangePasswordResponse {
        if let Err(e) = req.0.validate() {
            let validation_err = AppError::ValidationError(format!("Validation failed: {}", e));
            return ChangePasswordResponse::BadRequest(Json(validation_err.to_error_response()));
        }

        let Some(user_id) = auth.0.user_id() else {
            return ChangePasswordResponse::Unauthorized(Json(ErrorResponse {
                error: "invalid_token".to_string(),
                message: "Invalid user ID in token".to_string(),
            }));
        };

        match self
            .auth_service
            .change_password(user_id, req.0.current_password, req.0.new_password)
            .await
        {
            Ok(()) => ChangePasswordResponse::Ok(Json(MessageResponse {
                message: "Пароль изменён".to_string(),
            })),
            Err(e) => match e {
                AppError::ValidationError(_) => {
                    ChangePasswordResponse::BadRequest(Json(e.to_error_response()))
                }
                AppError::Unauthorized(_) => {
                    ChangePasswordResponse::Unauthorized(Json(e.to_error_response()))
                }
                _ => ChangePasswordResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Список пользователей для административной панели.
    #[oai(path = "/users", method = "get")]
    async fn list_users(&self, _auth: AdminAuth) -> UsersListResponse {
        match self.auth_service.list_users().await {
            Ok(users) => {
                UsersListResponse::Ok(Json(users.into_iter().map(UserResponse::from).collect()))
            }
            Err(e) => UsersListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Изменение роли или блокировка пользователя.
    #[oai(path = "/users/:id", method = "patch")]
    async fn update_user(
        &self,
        auth: AdminAuth,
        Path(user_id): Path<String>,
        req: Json<UpdateUserRequest>,
    ) -> UpdateUserResponse {
        let Ok(user_id) = Uuid::parse_str(&user_id) else {
            let err =
                AppError::ValidationError("Некорректный идентификатор пользователя".to_string());
            return UpdateUserResponse::BadRequest(Json(err.to_error_response()));
        };

        if auth.0.user_id() == Some(user_id) {
            let is_self_demotion = req.0.role.as_deref().is_some_and(|role| role != "admin");
            let is_self_disable = req.0.is_active == Some(false);

            if is_self_demotion || is_self_disable {
                let err = AppError::ValidationError(
                    "Нельзя отключить или понизить собственную учётную запись".to_string(),
                );
                return UpdateUserResponse::BadRequest(Json(err.to_error_response()));
            }
        }

        match self
            .auth_service
            .update_user(user_id, req.0.role, req.0.is_active)
            .await
        {
            Ok(user) => UpdateUserResponse::Ok(Json(user.into())),
            Err(e) => match e {
                AppError::ValidationError(_) => {
                    UpdateUserResponse::BadRequest(Json(e.to_error_response()))
                }
                AppError::NotFound(_) => UpdateUserResponse::NotFound(Json(e.to_error_response())),
                _ => UpdateUserResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }
}
