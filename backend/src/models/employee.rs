use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::sql_types::*;
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

#[derive(Insertable, Deserialize)]
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

#[derive(Deserialize, poem_openapi::Object)]
pub struct NewEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub position_id: Option<i32>,
    pub department_id: Option<i32>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ad_username: Option<String>,
    pub hire_date: Option<String>,
    pub status: String,
    pub notes: Option<String>,
}

impl NewEmployeeRequest {
    pub fn to_new_employee(self) -> Result<NewEmployee, String> {
        let hire_date = if let Some(date_str) = self.hire_date {
            Some(
                NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                    .map_err(|e| format!("Invalid date format: {}", e))?,
            )
        } else {
            None
        };

        Ok(NewEmployee {
            first_name: self.first_name,
            last_name: self.last_name,
            middle_name: self.middle_name,
            position_id: self.position_id,
            department_id: self.department_id,
            email: self.email,
            phone: self.phone,
            ad_username: self.ad_username,
            hire_date,
            status: self.status,
            notes: self.notes,
        })
    }
}

#[derive(AsChangeset, Deserialize)]
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

#[derive(Serialize, Deserialize, poem_openapi::Object)]
pub struct UpdateEmployeeRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub position_id: Option<i32>,
    pub department_id: Option<i32>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ad_username: Option<String>,
    pub hire_date: Option<String>,
    pub termination_date: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

impl UpdateEmployeeRequest {
    pub fn to_update_employee(self) -> Result<UpdateEmployee, String> {
        let hire_date = if let Some(date_str) = self.hire_date {
            Some(
                NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                    .map_err(|e| format!("Invalid hire_date format: {}", e))?,
            )
        } else {
            None
        };

        let termination_date = if let Some(date_str) = self.termination_date {
            Some(
                NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                    .map_err(|e| format!("Invalid termination_date format: {}", e))?,
            )
        } else {
            None
        };

        Ok(UpdateEmployee {
            first_name: self.first_name,
            last_name: self.last_name,
            middle_name: self.middle_name,
            position_id: self.position_id,
            department_id: self.department_id,
            email: self.email,
            phone: self.phone,
            ad_username: self.ad_username,
            hire_date,
            termination_date,
            status: self.status,
            notes: self.notes,
        })
    }
}

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
