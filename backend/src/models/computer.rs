use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::computers;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = computers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Computer {
    pub id: i32,
    pub inventory_number: Option<String>,
    pub hostname: Option<String>,
    pub computer_type: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub os_build: Option<String>,
    pub os_architecture: Option<String>,
    pub domain_joined: Option<bool>,
    pub last_boot_time: Option<NaiveDateTime>,
    pub employee_id: Option<i32>,
    pub location: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub warranty_end_date: Option<NaiveDate>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_seen_online: Option<NaiveDateTime>,
    pub is_online: Option<bool>,
    pub last_user_logged: Option<String>,
    pub bitlocker_enabled: Option<bool>,
    pub antivirus_status: Option<String>,
    pub windows_activation_status: Option<String>,
    pub location_id: Option<i32>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = computers)]
pub struct NewComputer {
    pub inventory_number: Option<String>,
    pub hostname: Option<String>,
    pub computer_type: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub os_build: Option<String>,
    pub os_architecture: Option<String>,
    pub domain_joined: Option<bool>,
    pub last_boot_time: Option<NaiveDateTime>,
    pub employee_id: Option<i32>,
    pub location: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub warranty_end_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub last_seen_online: Option<NaiveDateTime>,
    pub is_online: Option<bool>,
    pub last_user_logged: Option<String>,
    pub bitlocker_enabled: Option<bool>,
    pub antivirus_status: Option<String>,
    pub windows_activation_status: Option<String>,
    pub location_id: Option<i32>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = computers)]
pub struct UpdateComputer {
    pub inventory_number: Option<String>,
    pub hostname: Option<String>,
    pub computer_type: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub os_build: Option<String>,
    pub os_architecture: Option<String>,
    pub domain_joined: Option<bool>,
    pub last_boot_time: Option<NaiveDateTime>,
    pub employee_id: Option<i32>,
    pub location: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub warranty_end_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub last_seen_online: Option<NaiveDateTime>,
    pub is_online: Option<bool>,
    pub last_user_logged: Option<String>,
    pub bitlocker_enabled: Option<bool>,
    pub antivirus_status: Option<String>,
    pub windows_activation_status: Option<String>,
    pub location_id: Option<i32>,
}

// ============================================================================
// API request/response types (poem_openapi::Object — только простые типы,
// т.к. derive Object не поддерживает NaiveDate/NaiveDateTime)
// ============================================================================

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewComputerRequest {
    pub inventory_number: Option<String>,
    pub hostname: Option<String>,
    pub computer_type: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub os_build: Option<String>,
    pub os_architecture: Option<String>,
    pub domain_joined: Option<bool>,
    pub last_boot_time: Option<String>,
    pub employee_id: Option<i32>,
    pub location: Option<String>,
    pub purchase_date: Option<String>,
    pub warranty_end_date: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub last_seen_online: Option<String>,
    pub is_online: Option<bool>,
    pub last_user_logged: Option<String>,
    pub bitlocker_enabled: Option<bool>,
    pub antivirus_status: Option<String>,
    pub windows_activation_status: Option<String>,
    pub location_id: Option<i32>,
}

impl NewComputerRequest {
    pub fn into_new_computer(self) -> Result<NewComputer, String> {
        let parse_naive_date = |s: &Option<String>| -> Result<Option<NaiveDate>, String> {
            match s {
                Some(v) if !v.is_empty() => NaiveDate::parse_from_str(v, "%Y-%m-%d")
                    .map(Some)
                    .map_err(|e| format!("Invalid date format: {}", e)),
                _ => Ok(None),
            }
        };

        let parse_naive_datetime = |s: &Option<String>| -> Result<Option<NaiveDateTime>, String> {
            match s {
                Some(v) if !v.is_empty() => {
                    NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S%.3fZ")
                        .or_else(|_| NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%SZ"))
                        .map(Some)
                        .map_err(|e| format!("Invalid datetime format: {}", e))
                }
                _ => Ok(None),
            }
        };

        Ok(NewComputer {
            inventory_number: self.inventory_number,
            hostname: self.hostname,
            computer_type: self.computer_type,
            manufacturer: self.manufacturer,
            model: self.model,
            serial_number: self.serial_number,
            os: self.os,
            os_version: self.os_version,
            os_build: self.os_build,
            os_architecture: self.os_architecture,
            domain_joined: self.domain_joined,
            last_boot_time: parse_naive_datetime(&self.last_boot_time)?,
            employee_id: self.employee_id,
            location: self.location,
            purchase_date: parse_naive_date(&self.purchase_date)?,
            warranty_end_date: parse_naive_date(&self.warranty_end_date)?,
            status: self.status,
            notes: self.notes,
            last_seen_online: parse_naive_datetime(&self.last_seen_online)?,
            is_online: self.is_online,
            last_user_logged: self.last_user_logged,
            bitlocker_enabled: self.bitlocker_enabled,
            antivirus_status: self.antivirus_status,
            windows_activation_status: self.windows_activation_status,
            location_id: self.location_id,
        })
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateComputerRequest {
    pub inventory_number: Option<String>,
    pub hostname: Option<String>,
    pub computer_type: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub os_build: Option<String>,
    pub os_architecture: Option<String>,
    pub domain_joined: Option<bool>,
    pub last_boot_time: Option<String>,
    pub employee_id: Option<i32>,
    pub location: Option<String>,
    pub purchase_date: Option<String>,
    pub warranty_end_date: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub last_seen_online: Option<String>,
    pub is_online: Option<bool>,
    pub last_user_logged: Option<String>,
    pub bitlocker_enabled: Option<bool>,
    pub antivirus_status: Option<String>,
    pub windows_activation_status: Option<String>,
    pub location_id: Option<i32>,
}

impl UpdateComputerRequest {
    pub fn into_update_computer(self) -> Result<UpdateComputer, String> {
        let parse_naive_date = |s: &Option<String>| -> Result<Option<NaiveDate>, String> {
            match s {
                Some(v) if !v.is_empty() => NaiveDate::parse_from_str(v, "%Y-%m-%d")
                    .map(Some)
                    .map_err(|e| format!("Invalid date format: {}", e)),
                _ => Ok(None),
            }
        };

        let parse_naive_datetime = |s: &Option<String>| -> Result<Option<NaiveDateTime>, String> {
            match s {
                Some(v) if !v.is_empty() => {
                    NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S%.3fZ")
                        .or_else(|_| NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%SZ"))
                        .map(Some)
                        .map_err(|e| format!("Invalid datetime format: {}", e))
                }
                _ => Ok(None),
            }
        };

        Ok(UpdateComputer {
            inventory_number: self.inventory_number,
            hostname: self.hostname,
            computer_type: self.computer_type,
            manufacturer: self.manufacturer,
            model: self.model,
            serial_number: self.serial_number,
            os: self.os,
            os_version: self.os_version,
            os_build: self.os_build,
            os_architecture: self.os_architecture,
            domain_joined: self.domain_joined,
            last_boot_time: parse_naive_datetime(&self.last_boot_time)?,
            employee_id: self.employee_id,
            location: self.location,
            purchase_date: parse_naive_date(&self.purchase_date)?,
            warranty_end_date: parse_naive_date(&self.warranty_end_date)?,
            status: self.status,
            notes: self.notes,
            last_seen_online: parse_naive_datetime(&self.last_seen_online)?,
            is_online: self.is_online,
            last_user_logged: self.last_user_logged,
            bitlocker_enabled: self.bitlocker_enabled,
            antivirus_status: self.antivirus_status,
            windows_activation_status: self.windows_activation_status,
            location_id: self.location_id,
        })
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ComputerResponse {
    pub id: i32,
    pub inventory_number: Option<String>,
    pub hostname: Option<String>,
    pub computer_type: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub os_build: Option<String>,
    pub os_architecture: Option<String>,
    pub domain_joined: Option<bool>,
    pub last_boot_time: Option<String>,
    pub employee_id: Option<i32>,
    pub location: Option<String>,
    pub purchase_date: Option<String>,
    pub warranty_end_date: Option<String>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub last_seen_online: Option<String>,
    pub is_online: Option<bool>,
    pub last_user_logged: Option<String>,
    pub bitlocker_enabled: Option<bool>,
    pub antivirus_status: Option<String>,
    pub windows_activation_status: Option<String>,
    pub location_id: Option<i32>,
}

impl From<Computer> for ComputerResponse {
    fn from(computer: Computer) -> Self {
        Self {
            id: computer.id,
            inventory_number: computer.inventory_number,
            hostname: computer.hostname,
            computer_type: computer.computer_type,
            manufacturer: computer.manufacturer,
            model: computer.model,
            serial_number: computer.serial_number,
            os: computer.os,
            os_version: computer.os_version,
            os_build: computer.os_build,
            os_architecture: computer.os_architecture,
            domain_joined: computer.domain_joined,
            last_boot_time: computer
                .last_boot_time
                .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()),
            employee_id: computer.employee_id,
            location: computer.location,
            purchase_date: computer.purchase_date.map(|d| d.to_string()),
            warranty_end_date: computer.warranty_end_date.map(|d| d.to_string()),
            status: computer.status,
            notes: computer.notes,
            created_at: computer
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            updated_at: computer
                .updated_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            last_seen_online: computer
                .last_seen_online
                .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()),
            is_online: computer.is_online,
            last_user_logged: computer.last_user_logged,
            bitlocker_enabled: computer.bitlocker_enabled,
            antivirus_status: computer.antivirus_status,
            windows_activation_status: computer.windows_activation_status,
            location_id: computer.location_id,
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ComputerListResponse {
    pub computers: Vec<ComputerResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
