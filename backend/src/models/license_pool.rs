use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::license_pools;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensePool {
    pub id: i32,
    pub software_catalog_id: i32,
    pub license_type: String,
    pub total_licenses: i32,
    pub used_licenses: Option<i32>,
    pub available_licenses: Option<i32>,
    pub license_key: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub expiration_date: Option<NaiveDate>,
    pub vendor_id: Option<i32>,
    pub contract_id: Option<i32>,
    pub auto_renewal: Option<bool>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = license_pools)]
pub struct NewLicensePool {
    pub software_catalog_id: i32,
    pub license_type: Option<String>,
    pub total_licenses: Option<i32>,
    pub used_licenses: Option<i32>,
    pub available_licenses: Option<i32>,
    pub license_key: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub expiration_date: Option<NaiveDate>,
    pub vendor_id: Option<i32>,
    pub contract_id: Option<i32>,
    pub auto_renewal: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = license_pools)]
pub struct UpdateLicensePool {
    pub license_type: Option<String>,
    pub total_licenses: Option<i32>,
    pub used_licenses: Option<i32>,
    pub available_licenses: Option<i32>,
    pub license_key: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub expiration_date: Option<NaiveDate>,
    pub vendor_id: Option<i32>,
    pub contract_id: Option<i32>,
    pub auto_renewal: Option<bool>,
    pub notes: Option<String>,
}

// ============================================================================
// API request/response types (строковые даты)
// ============================================================================

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewLicensePoolRequest {
    pub software_catalog_id: i32,
    pub license_type: Option<String>,
    pub total_licenses: Option<i32>,
    pub used_licenses: Option<i32>,
    pub available_licenses: Option<i32>,
    pub license_key: Option<String>,
    pub purchase_date: Option<String>,
    pub expiration_date: Option<String>,
    pub vendor_id: Option<i32>,
    pub contract_id: Option<i32>,
    pub auto_renewal: Option<bool>,
    pub notes: Option<String>,
}

impl NewLicensePoolRequest {
    pub fn into_new_license_pool(self) -> Result<NewLicensePool, String> {
        let parse_date = |s: &Option<String>| -> Result<Option<NaiveDate>, String> {
            match s {
                Some(v) if !v.is_empty() => NaiveDate::parse_from_str(v, "%Y-%m-%d")
                    .map(Some)
                    .map_err(|e| format!("Invalid date format: {}", e)),
                _ => Ok(None),
            }
        };

        Ok(NewLicensePool {
            software_catalog_id: self.software_catalog_id,
            license_type: self.license_type,
            total_licenses: self.total_licenses,
            used_licenses: self.used_licenses,
            available_licenses: self.available_licenses,
            license_key: self.license_key,
            purchase_date: parse_date(&self.purchase_date)?,
            expiration_date: parse_date(&self.expiration_date)?,
            vendor_id: self.vendor_id,
            contract_id: self.contract_id,
            auto_renewal: self.auto_renewal,
            notes: self.notes,
        })
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateLicensePoolRequest {
    pub license_type: Option<String>,
    pub total_licenses: Option<i32>,
    pub used_licenses: Option<i32>,
    pub available_licenses: Option<i32>,
    pub license_key: Option<String>,
    pub purchase_date: Option<String>,
    pub expiration_date: Option<String>,
    pub vendor_id: Option<i32>,
    pub contract_id: Option<i32>,
    pub auto_renewal: Option<bool>,
    pub notes: Option<String>,
}

impl UpdateLicensePoolRequest {
    pub fn into_update_license_pool(self) -> Result<UpdateLicensePool, String> {
        let parse_date = |s: &Option<String>| -> Result<Option<NaiveDate>, String> {
            match s {
                Some(v) if !v.is_empty() => NaiveDate::parse_from_str(v, "%Y-%m-%d")
                    .map(Some)
                    .map_err(|e| format!("Invalid date format: {}", e)),
                _ => Ok(None),
            }
        };

        Ok(UpdateLicensePool {
            license_type: self.license_type,
            total_licenses: self.total_licenses,
            used_licenses: self.used_licenses,
            available_licenses: self.available_licenses,
            license_key: self.license_key,
            purchase_date: parse_date(&self.purchase_date)?,
            expiration_date: parse_date(&self.expiration_date)?,
            vendor_id: self.vendor_id,
            contract_id: self.contract_id,
            auto_renewal: self.auto_renewal,
            notes: self.notes,
        })
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct LicensePoolResponse {
    pub id: i32,
    pub software_catalog_id: i32,
    pub license_type: String,
    pub total_licenses: i32,
    pub used_licenses: Option<i32>,
    pub available_licenses: Option<i32>,
    pub license_key: Option<String>,
    pub purchase_date: Option<String>,
    pub expiration_date: Option<String>,
    pub vendor_id: Option<i32>,
    pub contract_id: Option<i32>,
    pub auto_renewal: Option<bool>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<LicensePool> for LicensePoolResponse {
    fn from(pool: LicensePool) -> Self {
        let fmt_date = |d: Option<NaiveDate>| d.map(|x| x.to_string());
        Self {
            id: pool.id,
            software_catalog_id: pool.software_catalog_id,
            license_type: pool.license_type,
            total_licenses: pool.total_licenses,
            used_licenses: pool.used_licenses,
            available_licenses: pool.available_licenses,
            license_key: pool.license_key,
            purchase_date: fmt_date(pool.purchase_date),
            expiration_date: fmt_date(pool.expiration_date),
            vendor_id: pool.vendor_id,
            contract_id: pool.contract_id,
            auto_renewal: pool.auto_renewal,
            notes: pool.notes,
            created_at: pool.created_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            updated_at: pool.updated_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct LicensePoolListResponse {
    pub licenses: Vec<LicensePoolResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
