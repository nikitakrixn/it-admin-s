use poem_openapi::{ApiResponse, Object, OpenApi, param::Header, payload::Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::user::UserResponse;
use crate::routes::api::ApiTags;
use crate::services::auth_service::AuthService;

// ============================================================================
// Request/Response DTOs
// ============================================================================

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

#[derive(Debug, Serialize, Object)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

// ============================================================================
// API Responses
// ============================================================================

#[derive(ApiResponse)]
pub enum RegisterResponse {
    /// User successfully registered
    #[oai(status = 201)]
    Created(Json<AuthResponse>),

    /// Bad request (validation error)
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),

    /// User already exists
    #[oai(status = 409)]
    Conflict(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum LoginResponse {
    /// Successfully authenticated
    #[oai(status = 200)]
    Ok(Json<AuthResponse>),

    /// Invalid credentials
    #[oai(status = 401)]
    Unauthorized(Json<ErrorResponse>),

    /// Bad request (validation error)
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum MeResponse {
    /// Current user info
    #[oai(status = 200)]
    Ok(Json<UserResponse>),

    /// Unauthorized
    #[oai(status = 401)]
    Unauthorized(Json<ErrorResponse>),
}

// ============================================================================
// Auth API
// ============================================================================

pub struct AuthApi {
    auth_service: AuthService,
}

impl AuthApi {
    pub fn new(auth_service: AuthService) -> Self {
        Self { auth_service }
    }
}

#[OpenApi(prefix_path = "/auth", tag = "ApiTags::Auth")]
impl AuthApi {
    /// Register a new user
    ///
    /// Creates a new user account with the provided credentials.
    /// The password will be securely hashed using Argon2.
    #[oai(path = "/register", method = "post")]
    async fn register(&self, req: Json<RegisterRequest>) -> RegisterResponse {
        // Валидация
        if let Err(e) = req.0.validate() {
            return RegisterResponse::BadRequest(Json(ErrorResponse {
                error: "validation_error".to_string(),
                message: format!("Validation failed: {}", e),
            }));
        }

        // Регистрация пользователя
        match self
            .auth_service
            .register_user(req.0.email, req.0.password, Some("user".to_string()))
        {
            Ok(user) => {
                // Генерация токена
                match self.auth_service.generate_token(&user) {
                    Ok(token) => RegisterResponse::Created(Json(AuthResponse {
                        token,
                        user: user.into(),
                    })),
                    Err(e) => RegisterResponse::BadRequest(Json(ErrorResponse {
                        error: "token_generation_failed".to_string(),
                        message: format!("Failed to generate token: {}", e),
                    })),
                }
            }
            Err(e) => {
                let error_msg = e.to_string();
                if error_msg.contains("already exists") {
                    RegisterResponse::Conflict(Json(ErrorResponse {
                        error: "user_exists".to_string(),
                        message: "User with this email already exists".to_string(),
                    }))
                } else {
                    RegisterResponse::BadRequest(Json(ErrorResponse {
                        error: "registration_failed".to_string(),
                        message: format!("Registration failed: {}", error_msg),
                    }))
                }
            }
        }
    }

    /// Login with email and password
    ///
    /// Authenticates a user and returns a JWT token.
    /// The token should be included in subsequent requests as a Bearer token.
    #[oai(path = "/login", method = "post")]
    async fn login(&self, req: Json<LoginRequest>) -> LoginResponse {
        // Валидация
        if let Err(e) = req.0.validate() {
            return LoginResponse::BadRequest(Json(ErrorResponse {
                error: "validation_error".to_string(),
                message: format!("Validation failed: {}", e),
            }));
        }

        // Аутентификация
        match self.auth_service.authenticate(req.0.email, req.0.password) {
            Ok((user, token)) => LoginResponse::Ok(Json(AuthResponse {
                token,
                user: user.into(),
            })),
            Err(e) => LoginResponse::Unauthorized(Json(ErrorResponse {
                error: "authentication_failed".to_string(),
                message: e.to_string(),
            })),
        }
    }

    /// Get current user information
    ///
    /// Returns information about the currently authenticated user.
    /// Requires a valid JWT token in the Authorization header.
    #[oai(path = "/me", method = "get")]
    async fn me(&self, authorization: Header<Option<String>>) -> MeResponse {
        // Проверить наличие токена
        let token = match authorization.0 {
            Some(ref auth_header) if auth_header.starts_with("Bearer ") => &auth_header[7..],
            _ => {
                return MeResponse::Unauthorized(Json(ErrorResponse {
                    error: "unauthorized".to_string(),
                    message: "Missing or invalid authorization header".to_string(),
                }));
            }
        };

        // Валидировать токен
        let claims = match self.auth_service.verify_token(token) {
            Ok(claims) => claims,
            Err(_) => {
                return MeResponse::Unauthorized(Json(ErrorResponse {
                    error: "unauthorized".to_string(),
                    message: "Invalid or expired token".to_string(),
                }));
            }
        };

        // Получить пользователя из БД
        let user_id = match uuid::Uuid::parse_str(&claims.sub) {
            Ok(id) => id,
            Err(_) => {
                return MeResponse::Unauthorized(Json(ErrorResponse {
                    error: "invalid_token".to_string(),
                    message: "Invalid user ID in token".to_string(),
                }));
            }
        };

        match self.auth_service.get_user_by_id(user_id) {
            Ok(user) => MeResponse::Ok(Json(user.into())),
            Err(_) => MeResponse::Unauthorized(Json(ErrorResponse {
                error: "user_not_found".to_string(),
                message: "User not found".to_string(),
            })),
        }
    }
}
