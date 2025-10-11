use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::{departments, employees, positions};

// ============================================================================
// Employee Models
// ============================================================================

#[derive(Queryable, Selectable, Serialize, Clone, Debug, Identifiable)]
#[diesel(table_name = employees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub position_id: Option<i32>,
    pub department_id: Option<i32>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ad_username: Option<String>,
    pub hire_date: Option<NaiveDate>,
    pub termination_date: Option<NaiveDate>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize, poem_openapi::Object)]
#[diesel(table_name = employees)]
pub struct NewEmployee {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub position_id: Option<i32>,
    pub department_id: Option<i32>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ad_username: Option<String>,
    pub hire_date: Option<NaiveDate>,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(AsChangeset, Deserialize, poem_openapi::Object)]
#[diesel(table_name = employees)]
pub struct UpdateEmployee {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub position_id: Option<i32>,
    pub department_id: Option<i32>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ad_username: Option<String>,
    pub hire_date: Option<NaiveDate>,
    pub termination_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

// Response DTO with joined data
#[derive(Serialize, poem_openapi::Object, Clone)]
pub struct EmployeeResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub full_name: String,
    pub position_id: Option<i32>,
    pub position_name: Option<String>,
    pub department_id: Option<i32>,
    pub department_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ad_username: Option<String>,
    pub hire_date: Option<String>,
    pub termination_date: Option<String>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Employee {
    pub fn full_name(&self) -> String {
        if let Some(middle) = &self.middle_name {
            format!("{} {} {}", self.last_name, self.first_name, middle)
        } else {
            format!("{} {}", self.last_name, self.first_name)
        }
    }
}

// ============================================================================
// Department Models
// ============================================================================

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

#[derive(Serialize, poem_openapi::Object, Clone)]
pub struct DepartmentResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub employee_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

// ============================================================================
// Position Models
// ============================================================================

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

#[derive(Serialize, poem_openapi::Object, Clone)]
pub struct PositionResponse {
    pub id: i32,
    pub name: String,
    pub department_id: Option<i32>,
    pub department_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
