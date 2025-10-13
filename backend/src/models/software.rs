use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::schema::software_catalog;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = software_catalog)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Software {
    pub id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
    pub is_system: Option<bool>,
    pub requires_license: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize, poem_openapi::Object)]
#[diesel(table_name = software_catalog)]
pub struct NewSoftware {
    pub name: String,
    pub manufacturer: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
    pub is_system: Option<bool>,
    pub requires_license: Option<bool>,
}

#[derive(Debug, AsChangeset, Deserialize, poem_openapi::Object)]
#[diesel(table_name = software_catalog)]
pub struct UpdateSoftware {
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
    pub is_system: Option<bool>,
    pub requires_license: Option<bool>,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct SoftwareResponse {
    pub id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
    pub is_system: bool,
    pub requires_license: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Software> for SoftwareResponse {
    fn from(software: Software) -> Self {
        Self {
            id: software.id,
            name: software.name,
            manufacturer: software.manufacturer,
            category: software.category,
            description: software.description,
            website: software.website,
            is_system: software.is_system.unwrap_or(false),
            requires_license: software.requires_license.unwrap_or(false),
            created_at: software.created_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            updated_at: software.updated_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct SoftwareListResponse {
    pub software: Vec<SoftwareResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
