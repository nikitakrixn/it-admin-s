use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
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

    fn hash_password(&self, password: &str) -> DbResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))
    }

    fn verify_password(&self, password: &str, hash: &str) -> DbResult<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::InternalError(format!("Invalid password hash: {}", e)))?;

        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
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

    pub async fn register_user(
        &self,
        email: String,
        password: String,
        role: Option<String>,
    ) -> DbResult<User> {
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

        let password_hash = self.hash_password(&password)?;

        let new_user = NewUser {
            email,
            password_hash,
            employee_id: None,
            role: role.unwrap_or_else(|| "user".to_string()),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut conn)
            .await
            .map_err(AppError::from)
    }

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
                "User account is disabled".to_string(),
            ));
        }

        if !self.verify_password(&password, &user.password_hash)? {
            return Err(AppError::Unauthorized(
                "Invalid email or password".to_string(),
            ));
        }

        diesel::update(users::table.find(user.id))
            .set(users::last_login_at.eq(Some(Utc::now().naive_utc())))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        let token = self.generate_token(&user)?;

        Ok((user, token))
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
}
