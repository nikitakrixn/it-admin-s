use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::database::Pool;
use crate::models::schema::users;
use crate::models::user::{NewUser, User};
use crate::utils::db_helpers::{DbResult, get_connection};
use crate::utils::error::AppError;
use crate::utils::password;

/// Сколько неудачных попыток входа допускается до временной блокировки.
const MAX_FAILED_ATTEMPTS: i32 = 5;

/// Длительность блокировки после исчерпания попыток.
const LOCKOUT_MINUTES: i64 = 15;

/// Допустимые роли пользователей.
const ALLOWED_ROLES: [&str; 2] = ["admin", "user"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

pub struct AuthService {
    db_pool: Pool,
    jwt_secret: String,
    jwt_expiration: i64,
}

impl AuthService {
    pub fn new(db_pool: Pool, jwt_secret: String, jwt_expiration: i64) -> Self {
        Self {
            db_pool,
            jwt_secret,
            jwt_expiration,
        }
    }

    pub fn generate_token(&self, user: &User) -> DbResult<String> {
        let now = Utc::now();
        let exp = (now + Duration::seconds(self.jwt_expiration)).timestamp();

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: user.role.clone(),
            exp,
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalError(format!("Token generation failed: {}", e)))
    }

    pub fn verify_token(&self, token: &str) -> DbResult<Claims> {
        let validation = Validation::default();

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))
    }

    /// Создаёт пользователя. Пароль проверяется по политике и хешируется
    /// в отдельном потоке, чтобы не блокировать async-рантайм.
    pub async fn register_user(
        &self,
        email: String,
        password: String,
        role: Option<String>,
    ) -> DbResult<User> {
        password::validate_strength(&password)?;

        let role = role.unwrap_or_else(|| "user".to_string());
        Self::validate_role(&role)?;

        let mut conn = get_connection(&self.db_pool).await?;

        let existing_user = users::table
            .filter(users::email.eq(&email))
            .first::<User>(&mut conn)
            .await
            .optional()
            .map_err(AppError::from)?;

        if existing_user.is_some() {
            return Err(AppError::Conflict(
                "User with this email already exists".to_string(),
            ));
        }

        let password_hash = Self::hash_password_async(password).await?;

        let new_user = NewUser {
            email,
            password_hash,
            employee_id: None,
            role,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut conn)
            .await
            .map_err(AppError::from)
    }

    /// Проверяет учётные данные с блокировкой после серии неудачных попыток.
    pub async fn authenticate(&self, email: String, password: String) -> DbResult<(User, String)> {
        let mut conn = get_connection(&self.db_pool).await?;

        let user = users::table
            .filter(users::email.eq(&email))
            .first::<User>(&mut conn)
            .await
            .optional()
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

        if !user.is_active {
            return Err(AppError::Unauthorized(
                "Учётная запись отключена администратором".to_string(),
            ));
        }

        let now = Utc::now().naive_utc();

        if let Some(locked_until) = user.locked_until
            && locked_until > now
        {
            let minutes = (locked_until - now).num_minutes() + 1;
            return Err(AppError::Unauthorized(format!(
                "Слишком много неудачных попыток входа. Повторите через {minutes} мин."
            )));
        }

        let password_ok = Self::verify_password_async(password, user.password_hash.clone()).await?;

        if !password_ok {
            let attempts = user.failed_login_attempts.unwrap_or(0) + 1;
            let locked_until = if attempts >= MAX_FAILED_ATTEMPTS {
                Some(now + Duration::minutes(LOCKOUT_MINUTES))
            } else {
                None
            };

            diesel::update(users::table.find(user.id))
                .set((
                    users::failed_login_attempts.eq(Some(attempts)),
                    users::locked_until.eq(locked_until),
                ))
                .execute(&mut conn)
                .await
                .map_err(AppError::from)?;

            return Err(AppError::Unauthorized(
                "Invalid email or password".to_string(),
            ));
        }

        diesel::update(users::table.find(user.id))
            .set((
                users::last_login_at.eq(Some(now)),
                users::failed_login_attempts.eq(Some(0)),
                users::locked_until.eq(None::<chrono::NaiveDateTime>),
            ))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        let token = self.generate_token(&user)?;

        Ok((user, token))
    }

    /// Меняет пароль пользователя, проверяя текущий пароль.
    pub async fn change_password(
        &self,
        user_id: Uuid,
        current_password: String,
        new_password: String,
    ) -> DbResult<()> {
        password::validate_strength(&new_password)?;

        let mut conn = get_connection(&self.db_pool).await?;

        let user = users::table
            .find(user_id)
            .first::<User>(&mut conn)
            .await
            .map_err(AppError::from)?;

        let current_ok =
            Self::verify_password_async(current_password, user.password_hash.clone()).await?;

        if !current_ok {
            return Err(AppError::Unauthorized(
                "Текущий пароль указан неверно".to_string(),
            ));
        }

        let password_hash = Self::hash_password_async(new_password).await?;
        let now = Utc::now().naive_utc();

        diesel::update(users::table.find(user_id))
            .set((
                users::password_hash.eq(password_hash),
                users::password_changed_at.eq(Some(now)),
                users::must_change_password.eq(Some(false)),
                users::failed_login_attempts.eq(Some(0)),
                users::locked_until.eq(None::<chrono::NaiveDateTime>),
            ))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(())
    }

    /// Список пользователей для административной панели.
    pub async fn list_users(&self) -> DbResult<Vec<User>> {
        let mut conn = get_connection(&self.db_pool).await?;

        users::table
            .order(users::created_at.desc())
            .load::<User>(&mut conn)
            .await
            .map_err(AppError::from)
    }

    /// Обновляет роль и признак активности пользователя.
    pub async fn update_user(
        &self,
        user_id: Uuid,
        role: Option<String>,
        is_active: Option<bool>,
    ) -> DbResult<User> {
        if let Some(ref role) = role {
            Self::validate_role(role)?;
        }

        let mut conn = get_connection(&self.db_pool).await?;

        let user = users::table
            .find(user_id)
            .first::<User>(&mut conn)
            .await
            .map_err(AppError::from)?;

        let new_role = role.unwrap_or(user.role);
        let new_is_active = is_active.unwrap_or(user.is_active);

        diesel::update(users::table.find(user_id))
            .set((users::role.eq(new_role), users::is_active.eq(new_is_active)))
            .get_result::<User>(&mut conn)
            .await
            .map_err(AppError::from)
    }

    pub async fn count_users(&self) -> DbResult<i64> {
        let mut conn = get_connection(&self.db_pool).await?;

        users::table
            .count()
            .get_result(&mut conn)
            .await
            .map_err(AppError::from)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> DbResult<User> {
        let mut conn = get_connection(&self.db_pool).await?;

        users::table
            .find(user_id)
            .first::<User>(&mut conn)
            .await
            .map_err(AppError::from)
    }

    #[allow(dead_code)]
    pub async fn get_user_by_email(&self, email: &str) -> DbResult<Option<User>> {
        let mut conn = get_connection(&self.db_pool).await?;

        users::table
            .filter(users::email.eq(email))
            .first::<User>(&mut conn)
            .await
            .optional()
            .map_err(AppError::from)
    }

    fn validate_role(role: &str) -> DbResult<()> {
        if ALLOWED_ROLES.contains(&role) {
            Ok(())
        } else {
            Err(AppError::ValidationError(format!(
                "Недопустимая роль: {role}. Допустимые значения: admin, user"
            )))
        }
    }

    async fn hash_password_async(password: String) -> DbResult<String> {
        tokio::task::spawn_blocking(move || password::hash_password(&password))
            .await
            .map_err(|e| AppError::InternalError(format!("Password hashing task failed: {e}")))?
    }

    async fn verify_password_async(password: String, hash: String) -> DbResult<bool> {
        tokio::task::spawn_blocking(move || password::verify_password(&password, &hash))
            .await
            .map_err(|e| {
                AppError::InternalError(format!("Password verification task failed: {e}"))
            })?
    }
}
