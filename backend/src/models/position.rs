use diesel::prelude::*;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};

use super::schema::positions;

#[derive(Queryable, Selectable, Serialize, Clone, Debug, Identifiable)]
#[diesel(table_name = positions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Position {
    pub id: i32,
    pub name: String,
    pub department_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize, poem_openapi::Object)]
#[diesel(table_name = positions)]
pub struct NewPosition {
    pub name: String,
    pub department_id: Option<i32>,
}

#[derive(AsChangeset, Deserialize, poem_openapi::Object)]
#[diesel(table_name = positions)]
pub struct UpdatePosition {
    pub name: Option<String>,
    pub department_id: Option<i32>,
}

#[derive(Serialize, poem_openapi::Object, Clone)]
pub struct PositionResponse {
    pub id: i32,
    pub name: String,
    pub department_id: Option<i32>,
    pub department_name: Option<String>,
    pub employee_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(QueryableByName, Serialize, Clone, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PositionWithDetails {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Nullable<Integer>)]
    pub department_id: Option<i32>,
    #[diesel(sql_type = Nullable<Text>)]
    pub department_name: Option<String>,
    #[diesel(sql_type = Timestamp)]
    pub created_at: chrono::NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    pub updated_at: chrono::NaiveDateTime,
    #[diesel(sql_type = BigInt)]
    pub employee_count: i64,
}
