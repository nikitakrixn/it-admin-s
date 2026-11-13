use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::{computer_monitoring, computer_monitors};

fn decimal_from_f64(value: Option<f64>) -> Option<BigDecimal> {
    value.and_then(|v| v.to_string().parse::<BigDecimal>().ok())
}

fn decimal_to_string(value: Option<BigDecimal>) -> Option<String> {
    value.map(|v| v.to_string())
}

// ============================================================================
// Computer monitoring metrics
// ============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = computer_monitoring)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ComputerMonitoring {
    pub id: i32,
    pub computer_id: i32,
    pub cpu_usage: Option<BigDecimal>,
    pub ram_usage_percent: Option<BigDecimal>,
    pub ram_total_gb: Option<i32>,
    pub ram_used_gb: Option<BigDecimal>,
    pub disk_usage: Option<serde_json::Value>,
    pub cpu_temperature: Option<BigDecimal>,
    pub gpu_temperature: Option<BigDecimal>,
    pub uptime_seconds: Option<i64>,
    pub network_stats: Option<serde_json::Value>,
    pub processes_count: Option<i32>,
    pub logged_users: Option<serde_json::Value>,
    pub collected_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = computer_monitoring)]
pub struct NewComputerMonitoring {
    pub computer_id: i32,
    pub cpu_usage: Option<BigDecimal>,
    pub ram_usage_percent: Option<BigDecimal>,
    pub ram_total_gb: Option<i32>,
    pub ram_used_gb: Option<BigDecimal>,
    pub disk_usage: Option<serde_json::Value>,
    pub cpu_temperature: Option<BigDecimal>,
    pub gpu_temperature: Option<BigDecimal>,
    pub uptime_seconds: Option<i64>,
    pub network_stats: Option<serde_json::Value>,
    pub processes_count: Option<i32>,
    pub logged_users: Option<serde_json::Value>,
    pub collected_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewComputerMonitoringRequest {
    pub computer_id: i32,
    pub cpu_usage: Option<f64>,
    pub ram_usage_percent: Option<f64>,
    pub ram_total_gb: Option<i32>,
    pub ram_used_gb: Option<f64>,
    pub disk_usage: Option<serde_json::Value>,
    pub cpu_temperature: Option<f64>,
    pub gpu_temperature: Option<f64>,
    pub uptime_seconds: Option<i64>,
    pub network_stats: Option<serde_json::Value>,
    pub processes_count: Option<i32>,
    pub logged_users: Option<serde_json::Value>,
    pub collected_at: Option<String>,
}

impl NewComputerMonitoringRequest {
    pub fn into_new_computer_monitoring(self) -> NewComputerMonitoring {
        let parse_datetime = |s: &Option<String>| -> Result<Option<NaiveDateTime>, String> {
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

        NewComputerMonitoring {
            computer_id: self.computer_id,
            cpu_usage: decimal_from_f64(self.cpu_usage),
            ram_usage_percent: decimal_from_f64(self.ram_usage_percent),
            ram_total_gb: self.ram_total_gb,
            ram_used_gb: decimal_from_f64(self.ram_used_gb),
            disk_usage: self.disk_usage,
            cpu_temperature: decimal_from_f64(self.cpu_temperature),
            gpu_temperature: decimal_from_f64(self.gpu_temperature),
            uptime_seconds: self.uptime_seconds,
            network_stats: self.network_stats,
            processes_count: self.processes_count,
            logged_users: self.logged_users,
            collected_at: parse_datetime(&self.collected_at)
                .ok()
                .flatten()
                .unwrap_or_else(|| {
                    NaiveDate::from_ymd_opt(1970, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap()
                }),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ComputerMonitoringResponse {
    pub id: i32,
    pub computer_id: i32,
    pub cpu_usage: Option<String>,
    pub ram_usage_percent: Option<String>,
    pub ram_total_gb: Option<i32>,
    pub ram_used_gb: Option<String>,
    pub disk_usage: Option<serde_json::Value>,
    pub cpu_temperature: Option<String>,
    pub gpu_temperature: Option<String>,
    pub uptime_seconds: Option<i64>,
    pub network_stats: Option<serde_json::Value>,
    pub processes_count: Option<i32>,
    pub logged_users: Option<serde_json::Value>,
    pub collected_at: String,
}

impl From<ComputerMonitoring> for ComputerMonitoringResponse {
    fn from(monitoring: ComputerMonitoring) -> Self {
        Self {
            id: monitoring.id,
            computer_id: monitoring.computer_id,
            cpu_usage: decimal_to_string(monitoring.cpu_usage),
            ram_usage_percent: decimal_to_string(monitoring.ram_usage_percent),
            ram_total_gb: monitoring.ram_total_gb,
            ram_used_gb: decimal_to_string(monitoring.ram_used_gb),
            disk_usage: monitoring.disk_usage,
            cpu_temperature: decimal_to_string(monitoring.cpu_temperature),
            gpu_temperature: decimal_to_string(monitoring.gpu_temperature),
            uptime_seconds: monitoring.uptime_seconds,
            network_stats: monitoring.network_stats,
            processes_count: monitoring.processes_count,
            logged_users: monitoring.logged_users,
            collected_at: monitoring
                .collected_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ComputerMonitoringListResponse {
    pub monitoring: Vec<ComputerMonitoringResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

// ============================================================================
// Computer monitors (physical displays)
// ============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = computer_monitors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ComputerMonitor {
    pub id: i32,
    pub computer_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub size_inches: Option<BigDecimal>,
    pub resolution: Option<String>,
    pub panel_type: Option<String>,
    pub refresh_rate: Option<i32>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = computer_monitors)]
pub struct NewComputerMonitor {
    pub computer_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub size_inches: Option<BigDecimal>,
    pub resolution: Option<String>,
    pub panel_type: Option<String>,
    pub refresh_rate: Option<i32>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = computer_monitors)]
pub struct UpdateComputerMonitor {
    pub computer_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub size_inches: Option<BigDecimal>,
    pub resolution: Option<String>,
    pub panel_type: Option<String>,
    pub refresh_rate: Option<i32>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewComputerMonitorRequest {
    pub computer_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub size_inches: Option<f64>,
    pub resolution: Option<String>,
    pub panel_type: Option<String>,
    pub refresh_rate: Option<i32>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
}

impl NewComputerMonitorRequest {
    pub fn into_new_computer_monitor(self) -> NewComputerMonitor {
        NewComputerMonitor {
            computer_id: self.computer_id,
            name: self.name,
            manufacturer: self.manufacturer,
            model: self.model,
            serial_number: self.serial_number,
            size_inches: decimal_from_f64(self.size_inches),
            resolution: self.resolution,
            panel_type: self.panel_type,
            refresh_rate: self.refresh_rate,
            is_primary: self.is_primary,
            notes: self.notes,
        }
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateComputerMonitorRequest {
    pub computer_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub size_inches: Option<f64>,
    pub resolution: Option<String>,
    pub panel_type: Option<String>,
    pub refresh_rate: Option<i32>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
}

impl UpdateComputerMonitorRequest {
    pub fn into_update_computer_monitor(self) -> UpdateComputerMonitor {
        UpdateComputerMonitor {
            computer_id: self.computer_id,
            name: self.name,
            manufacturer: self.manufacturer,
            model: self.model,
            serial_number: self.serial_number,
            size_inches: decimal_from_f64(self.size_inches),
            resolution: self.resolution,
            panel_type: self.panel_type,
            refresh_rate: self.refresh_rate,
            is_primary: self.is_primary,
            notes: self.notes,
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ComputerMonitorResponse {
    pub id: i32,
    pub computer_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub size_inches: Option<String>,
    pub resolution: Option<String>,
    pub panel_type: Option<String>,
    pub refresh_rate: Option<i32>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<ComputerMonitor> for ComputerMonitorResponse {
    fn from(monitor: ComputerMonitor) -> Self {
        Self {
            id: monitor.id,
            computer_id: monitor.computer_id,
            name: monitor.name,
            manufacturer: monitor.manufacturer,
            model: monitor.model,
            serial_number: monitor.serial_number,
            size_inches: decimal_to_string(monitor.size_inches),
            resolution: monitor.resolution,
            panel_type: monitor.panel_type,
            refresh_rate: monitor.refresh_rate,
            is_primary: monitor.is_primary,
            notes: monitor.notes,
            created_at: monitor
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            updated_at: monitor
                .updated_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ComputerMonitorListResponse {
    pub monitors: Vec<ComputerMonitorResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
