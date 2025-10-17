use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Queryable, Selectable, Serialize, Clone, Debug)]
#[diesel(table_name = crate::models::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub employee_id: Option<i32>,
    pub role: String,
    pub is_active: bool,
    pub last_login_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub two_factor_enabled: Option<bool>,
    #[serde(skip_serializing)]
    pub two_factor_secret: Option<String>,
    pub password_changed_at: Option<NaiveDateTime>,
    pub must_change_password: Option<bool>,
    pub failed_login_attempts: Option<i32>,
    pub locked_until: Option<NaiveDateTime>,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::models::schema::users)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub employee_id: Option<i32>,
    pub role: String,
}


#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::models::schema::users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub employee_id: Option<i32>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub last_login_at: Option<NaiveDateTime>,
}


#[derive(Serialize, Clone, Debug, poem_openapi::Object)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub employee_id: Option<i32>,
    pub role: String,
    pub is_active: bool,
    pub last_login_at: Option<String>,
    pub created_at: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email,
            employee_id: user.employee_id,
            role: user.role,
            is_active: user.is_active,
            last_login_at: user.last_login_at.map(|dt| dt.to_string()),
            created_at: user.created_at.to_string(),
        }
    }
}
