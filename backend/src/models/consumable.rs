use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema::{consumable_movements, consumables};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consumable {
    pub id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub category: Option<String>,
    pub compatible_with: Option<i32>,
    pub compatible_with_name: Option<String>,
    pub quantity: i32,
    pub min_quantity: i32,
    pub unit: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableMovement {
    pub id: i32,
    pub consumable_id: i32,
    pub equipment_id: Option<i32>,
    pub computer_id: Option<i32>,
    pub quantity: i32,
    pub movement_type: String,
    pub employee_id: Option<i32>,
    pub performed_by: Option<Uuid>,
    pub performed_by_email: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = consumable_movements)]
pub struct NewConsumableMovement {
    pub consumable_id: i32,
    pub equipment_id: Option<i32>,
    pub computer_id: Option<i32>,
    pub quantity: i32,
    pub movement_type: String,
    pub employee_id: Option<i32>,
    pub performed_by: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = consumables)]
pub struct NewConsumable {
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub category: Option<String>,
    pub compatible_with: Option<i32>,
    pub quantity: i32,
    pub min_quantity: i32,
    pub unit: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = consumables)]
pub struct UpdateConsumable {
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub category: Option<String>,
    pub compatible_with: Option<i32>,
    pub quantity: Option<i32>,
    pub min_quantity: Option<i32>,
    pub unit: Option<String>,
    pub notes: Option<String>,
}

// ============================================================================
// API request/response types
// ============================================================================

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewConsumableRequest {
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub category: Option<String>,
    pub compatible_with: Option<i32>,
    pub quantity: i32,
    pub min_quantity: i32,
    pub unit: Option<String>,
    pub notes: Option<String>,
}

impl NewConsumableRequest {
    pub fn into_new_consumable(self) -> NewConsumable {
        NewConsumable {
            name: self.name,
            manufacturer: self.manufacturer,
            model: self.model,
            category: self.category,
            compatible_with: self.compatible_with,
            quantity: self.quantity,
            min_quantity: self.min_quantity,
            unit: self.unit,
            notes: self.notes,
        }
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateConsumableRequest {
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub category: Option<String>,
    pub compatible_with: Option<i32>,
    pub quantity: Option<i32>,
    pub min_quantity: Option<i32>,
    pub unit: Option<String>,
    pub notes: Option<String>,
}

impl UpdateConsumableRequest {
    pub fn into_update_consumable(self) -> UpdateConsumable {
        UpdateConsumable {
            name: self.name,
            manufacturer: self.manufacturer,
            model: self.model,
            category: self.category,
            compatible_with: self.compatible_with,
            quantity: self.quantity,
            min_quantity: self.min_quantity,
            unit: self.unit,
            notes: self.notes,
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ConsumableResponse {
    pub id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub category: Option<String>,
    pub compatible_with: Option<i32>,
    pub compatible_with_name: Option<String>,
    pub quantity: i32,
    pub min_quantity: i32,
    pub unit: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Consumable> for ConsumableResponse {
    fn from(consumable: Consumable) -> Self {
        Self {
            id: consumable.id,
            name: consumable.name,
            manufacturer: consumable.manufacturer,
            model: consumable.model,
            category: consumable.category,
            compatible_with: consumable.compatible_with,
            compatible_with_name: consumable.compatible_with_name,
            quantity: consumable.quantity,
            min_quantity: consumable.min_quantity,
            unit: consumable.unit,
            notes: consumable.notes,
            created_at: consumable
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            updated_at: consumable
                .updated_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ConsumableListResponse {
    pub consumables: Vec<ConsumableResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewConsumableMovementRequest {
    pub quantity: i32,
    pub movement_type: String,
    pub employee_id: Option<i32>,
    pub equipment_id: Option<i32>,
    pub computer_id: Option<i32>,
    pub notes: Option<String>,
}

impl NewConsumableMovementRequest {
    pub fn into_new_movement(
        self,
        consumable_id: i32,
        performed_by: Option<Uuid>,
    ) -> NewConsumableMovement {
        NewConsumableMovement {
            consumable_id,
            equipment_id: self.equipment_id,
            computer_id: self.computer_id,
            quantity: self.quantity,
            movement_type: self.movement_type,
            employee_id: self.employee_id,
            performed_by,
            notes: self.notes,
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ConsumableMovementResponse {
    pub id: i32,
    pub consumable_id: i32,
    pub equipment_id: Option<i32>,
    pub computer_id: Option<i32>,
    pub quantity: i32,
    pub movement_type: String,
    pub employee_id: Option<i32>,
    pub performed_by: Option<String>,
    pub performed_by_email: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}

impl From<ConsumableMovement> for ConsumableMovementResponse {
    fn from(movement: ConsumableMovement) -> Self {
        Self {
            id: movement.id,
            consumable_id: movement.consumable_id,
            equipment_id: movement.equipment_id,
            computer_id: movement.computer_id,
            quantity: movement.quantity,
            movement_type: movement.movement_type,
            employee_id: movement.employee_id,
            performed_by: movement.performed_by.map(|id| id.to_string()),
            performed_by_email: movement.performed_by_email,
            notes: movement.notes,
            created_at: movement
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ConsumableMovementListResponse {
    pub movements: Vec<ConsumableMovementResponse>,
    pub total: i64,
    pub total_in: i64,
    pub total_out: i64,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ConsumableMovementCreatedResponse {
    pub movement: ConsumableMovementResponse,
    pub consumable: ConsumableResponse,
}
