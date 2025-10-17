use diesel::prelude::*;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};

use super::schema::departments;

#[derive(Queryable, Selectable, Serialize, Clone, Debug, Identifiable)]
#[diesel(table_name = departments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Department {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize, poem_openapi::Object)]
#[diesel(table_name = departments)]
pub struct NewDepartment {
    pub name: String,
    pub description: Option<String>,
}

#[derive(AsChangeset, Deserialize, poem_openapi::Object)]
#[diesel(table_name = departments)]
pub struct UpdateDepartment {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, poem_openapi::Object, Clone)]
pub struct DepartmentResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub employee_count: i64,
    pub position_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(QueryableByName, Serialize, Clone, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DepartmentWithCounts {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub description: Option<String>,
    #[diesel(sql_type = Timestamp)]
    pub created_at: chrono::NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    pub updated_at: chrono::NaiveDateTime,
    #[diesel(sql_type = BigInt)]
    pub employee_count: i64,
    #[diesel(sql_type = BigInt)]
    pub position_count: i64,
    #[diesel(sql_type = BigInt)]
    pub computer_count: i64,
}
