use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::equipment;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = equipment)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Equipment {
    pub id: i32,
    pub inventory_number: Option<String>,
    pub type_id: Option<i32>,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub employee_id: Option<i32>,
    pub location: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub warranty_end_date: Option<NaiveDate>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub location_id: Option<i32>,
    pub last_maintenance_date: Option<NaiveDate>,
    pub next_maintenance_date: Option<NaiveDate>,
    pub toner_level: Option<i32>,
    pub page_count: Option<i32>,
    pub paper_jam_count: Option<i32>,
    pub total_pages_printed: Option<i32>,
    pub color_pages_printed: Option<i32>,
    pub mono_pages_printed: Option<i32>,
    pub last_error_code: Option<String>,
    pub last_error_time: Option<NaiveDateTime>,
    pub supplies_status: Option<serde_json::Value>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = equipment)]
pub struct NewEquipment {
    pub inventory_number: Option<String>,
    pub type_id: Option<i32>,
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub employee_id: Option<i32>,
    pub location: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub warranty_end_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub location_id: Option<i32>,
    pub last_maintenance_date: Option<NaiveDate>,
    pub next_maintenance_date: Option<NaiveDate>,
    pub toner_level: Option<i32>,
    pub page_count: Option<i32>,
    pub paper_jam_count: Option<i32>,
    pub total_pages_printed: Option<i32>,
    pub color_pages_printed: Option<i32>,
    pub mono_pages_printed: Option<i32>,
    pub last_error_code: Option<String>,
    pub last_error_time: Option<NaiveDateTime>,
    pub supplies_status: Option<serde_json::Value>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = equipment)]
pub struct UpdateEquipment {
    pub inventory_number: Option<String>,
    pub type_id: Option<i32>,
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub employee_id: Option<i32>,
    pub location: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub warranty_end_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub location_id: Option<i32>,
    pub last_maintenance_date: Option<NaiveDate>,
    pub next_maintenance_date: Option<NaiveDate>,
    pub toner_level: Option<i32>,
    pub page_count: Option<i32>,
    pub paper_jam_count: Option<i32>,
    pub total_pages_printed: Option<i32>,
    pub color_pages_printed: Option<i32>,
    pub mono_pages_printed: Option<i32>,
    pub last_error_code: Option<String>,
    pub last_error_time: Option<NaiveDateTime>,
    pub supplies_status: Option<serde_json::Value>,
}

// ============================================================================
// API request/response types (строковые даты)
// ============================================================================

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewEquipmentRequest {
    pub inventory_number: Option<String>,
    pub type_id: Option<i32>,
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub employee_id: Option<i32>,
    pub location: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub purchase_date: Option<String>,
    pub warranty_end_date: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub location_id: Option<i32>,
    pub last_maintenance_date: Option<String>,
    pub next_maintenance_date: Option<String>,
    pub toner_level: Option<i32>,
    pub page_count: Option<i32>,
    pub paper_jam_count: Option<i32>,
    pub total_pages_printed: Option<i32>,
    pub color_pages_printed: Option<i32>,
    pub mono_pages_printed: Option<i32>,
    pub last_error_code: Option<String>,
    pub last_error_time: Option<String>,
    pub supplies_status: Option<serde_json::Value>,
}

impl NewEquipmentRequest {
    pub fn into_new_equipment(self) -> Result<NewEquipment, String> {
        let parse_date = |s: &Option<String>| -> Result<Option<NaiveDate>, String> {
            match s {
                Some(v) if !v.is_empty() => NaiveDate::parse_from_str(v, "%Y-%m-%d")
                    .map(Some)
                    .map_err(|e| format!("Invalid date format: {}", e)),
                _ => Ok(None),
            }
        };
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

        Ok(NewEquipment {
            inventory_number: self.inventory_number,
            type_id: self.type_id,
            name: self.name,
            manufacturer: self.manufacturer,
            model: self.model,
            serial_number: self.serial_number,
            employee_id: self.employee_id,
            location: self.location,
            ip_address: self.ip_address,
            mac_address: self.mac_address,
            purchase_date: parse_date(&self.purchase_date)?,
            warranty_end_date: parse_date(&self.warranty_end_date)?,
            status: self.status,
            notes: self.notes,
            location_id: self.location_id,
            last_maintenance_date: parse_date(&self.last_maintenance_date)?,
            next_maintenance_date: parse_date(&self.next_maintenance_date)?,
            toner_level: self.toner_level,
            page_count: self.page_count,
            paper_jam_count: self.paper_jam_count,
            total_pages_printed: self.total_pages_printed,
            color_pages_printed: self.color_pages_printed,
            mono_pages_printed: self.mono_pages_printed,
            last_error_code: self.last_error_code,
            last_error_time: parse_datetime(&self.last_error_time)?,
            supplies_status: self.supplies_status,
        })
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateEquipmentRequest {
    pub inventory_number: Option<String>,
    pub type_id: Option<i32>,
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub employee_id: Option<i32>,
    pub location: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub purchase_date: Option<String>,
    pub warranty_end_date: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub location_id: Option<i32>,
    pub last_maintenance_date: Option<String>,
    pub next_maintenance_date: Option<String>,
    pub toner_level: Option<i32>,
    pub page_count: Option<i32>,
    pub paper_jam_count: Option<i32>,
    pub total_pages_printed: Option<i32>,
    pub color_pages_printed: Option<i32>,
    pub mono_pages_printed: Option<i32>,
    pub last_error_code: Option<String>,
    pub last_error_time: Option<String>,
    pub supplies_status: Option<serde_json::Value>,
}

impl UpdateEquipmentRequest {
    pub fn into_update_equipment(self) -> Result<UpdateEquipment, String> {
        let parse_date = |s: &Option<String>| -> Result<Option<NaiveDate>, String> {
            match s {
                Some(v) if !v.is_empty() => NaiveDate::parse_from_str(v, "%Y-%m-%d")
                    .map(Some)
                    .map_err(|e| format!("Invalid date format: {}", e)),
                _ => Ok(None),
            }
        };
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

        Ok(UpdateEquipment {
            inventory_number: self.inventory_number,
            type_id: self.type_id,
            name: self.name,
            manufacturer: self.manufacturer,
            model: self.model,
            serial_number: self.serial_number,
            employee_id: self.employee_id,
            location: self.location,
            ip_address: self.ip_address,
            mac_address: self.mac_address,
            purchase_date: parse_date(&self.purchase_date)?,
            warranty_end_date: parse_date(&self.warranty_end_date)?,
            status: self.status,
            notes: self.notes,
            location_id: self.location_id,
            last_maintenance_date: parse_date(&self.last_maintenance_date)?,
            next_maintenance_date: parse_date(&self.next_maintenance_date)?,
            toner_level: self.toner_level,
            page_count: self.page_count,
            paper_jam_count: self.paper_jam_count,
            total_pages_printed: self.total_pages_printed,
            color_pages_printed: self.color_pages_printed,
            mono_pages_printed: self.mono_pages_printed,
            last_error_code: self.last_error_code,
            last_error_time: parse_datetime(&self.last_error_time)?,
            supplies_status: self.supplies_status,
        })
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct EquipmentResponse {
    pub id: i32,
    pub inventory_number: Option<String>,
    pub type_id: Option<i32>,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub employee_id: Option<i32>,
    pub location: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub purchase_date: Option<String>,
    pub warranty_end_date: Option<String>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub location_id: Option<i32>,
    pub last_maintenance_date: Option<String>,
    pub next_maintenance_date: Option<String>,
    pub toner_level: Option<i32>,
    pub page_count: Option<i32>,
    pub paper_jam_count: Option<i32>,
    pub total_pages_printed: Option<i32>,
    pub color_pages_printed: Option<i32>,
    pub mono_pages_printed: Option<i32>,
    pub last_error_code: Option<String>,
    pub last_error_time: Option<String>,
    pub supplies_status: Option<serde_json::Value>,
}

impl From<Equipment> for EquipmentResponse {
    fn from(equipment: Equipment) -> Self {
        let fmt_date = |d: Option<NaiveDate>| d.map(|x| x.to_string());
        let fmt_dt =
            |dt: Option<NaiveDateTime>| dt.map(|d| d.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string());
        Self {
            id: equipment.id,
            inventory_number: equipment.inventory_number,
            type_id: equipment.type_id,
            name: equipment.name,
            manufacturer: equipment.manufacturer,
            model: equipment.model,
            serial_number: equipment.serial_number,
            employee_id: equipment.employee_id,
            location: equipment.location,
            ip_address: equipment.ip_address,
            mac_address: equipment.mac_address,
            purchase_date: fmt_date(equipment.purchase_date),
            warranty_end_date: fmt_date(equipment.warranty_end_date),
            status: equipment.status,
            notes: equipment.notes,
            created_at: equipment
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            updated_at: equipment
                .updated_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            location_id: equipment.location_id,
            last_maintenance_date: fmt_date(equipment.last_maintenance_date),
            next_maintenance_date: fmt_date(equipment.next_maintenance_date),
            toner_level: equipment.toner_level,
            page_count: equipment.page_count,
            paper_jam_count: equipment.paper_jam_count,
            total_pages_printed: equipment.total_pages_printed,
            color_pages_printed: equipment.color_pages_printed,
            mono_pages_printed: equipment.mono_pages_printed,
            last_error_code: equipment.last_error_code,
            last_error_time: fmt_dt(equipment.last_error_time),
            supplies_status: equipment.supplies_status,
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct EquipmentListResponse {
    pub equipment: Vec<EquipmentResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
