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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub role: String,
    pub exp: i64, // expiration time
    pub iat: i64, // issued at
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

    /// Хеширование пароля с использованием Argon2
    pub fn hash_password(&self, password: &str) -> Result<String, Box<dyn std::error::Error>> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Password hashing failed: {}", e))?;
        Ok(password_hash.to_string())
    }

    /// Проверка пароля
    pub fn verify_password(
        &self,
        password: &str,
        hash: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| format!("Invalid password hash: {}", e))?;
        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Генерация JWT токена
    pub fn generate_token(&self, user: &User) -> Result<String, jsonwebtoken::errors::Error> {
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
    }

    /// Валидация JWT токена
    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )?;
        Ok(token_data.claims)
    }

    /// Регистрация нового пользователя
    pub async fn register_user(
        &self,
        email: String,
        password: String,
        role: Option<String>,
    ) -> Result<User, Box<dyn std::error::Error>> {
        let mut conn = self.db_pool.get().await?;

        // Проверка существования пользователя
        let existing_user = users::table
            .filter(users::email.eq(&email))
            .first::<User>(&mut conn).await
            .optional()?;

        if existing_user.is_some() {
            return Err("User with this email already exists".into());
        }

        // Хеширование пароля
        let password_hash = self.hash_password(&password)?;

        // Создание нового пользователя
        let new_user = NewUser {
            email,
            password_hash,
            employee_id: None,
            role: role.unwrap_or_else(|| "user".to_string()),
        };

        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut conn).await?;

        Ok(user)
    }

    /// Аутентификация пользователя
    pub async fn authenticate(
        &self,
        email: String,
        password: String,
    ) -> Result<(User, String), Box<dyn std::error::Error>> {
        let mut conn = self.db_pool.get().await?;

        // Поиск пользователя
        let user = users::table
            .filter(users::email.eq(&email))
            .first::<User>(&mut conn).await
            .optional()?
            .ok_or("Invalid email or password")?;

        // Проверка активности
        if !user.is_active {
            return Err("User account is disabled".into());
        }

        // Проверка пароля
        if !self.verify_password(&password, &user.password_hash)? {
            return Err("Invalid email or password".into());
        }

        // Обновление времени последнего входа
        diesel::update(users::table.find(user.id))
            .set(users::last_login_at.eq(Some(Utc::now().naive_utc())))
            .execute(&mut conn).await?;

        // Генерация токена
        let token = self.generate_token(&user)?;

        Ok((user, token))
    }

    /// Получить пользователя по ID
    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, Box<dyn std::error::Error>> {
        let mut conn = self.db_pool.get().await?;
        let user = users::table.find(user_id).first::<User>(&mut conn).await?;
        Ok(user)
    }

    /// Получить пользователя по email
    pub async fn get_user_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let mut conn = self.db_pool.get().await?;
        let user = users::table
            .filter(users::email.eq(email))
            .first::<User>(&mut conn).await
            .optional()?;
        Ok(user)
    }
}
