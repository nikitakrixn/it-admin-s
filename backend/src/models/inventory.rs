use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema::{inventory_audit_items, inventory_audits};

/// Допустимые типы инвентаризации — совпадают с ограничением в БД.
pub const AUDIT_TYPES: [&str; 4] = ["full", "partial", "spot_check", "annual"];

/// Статусы позиции. `pending` означает «ещё не проверено».
pub const ITEM_STATUSES: [&str; 7] = [
    "pending",
    "found",
    "missing",
    "damaged",
    "relocated",
    "surplus",
    "disposed",
];

pub fn is_valid_audit_type(value: &str) -> bool {
    AUDIT_TYPES.contains(&value)
}

pub fn is_valid_item_status(value: &str) -> bool {
    ITEM_STATUSES.contains(&value)
}

// ============================================================================
// Таблицы
// ============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = inventory_audits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InventoryAudit {
    pub id: i32,
    pub audit_name: String,
    pub audit_date: NaiveDate,
    pub audit_type: String,
    pub location: Option<String>,
    pub department_id: Option<i32>,
    pub audited_by: Uuid,
    pub status: Option<String>,
    pub total_items: Option<i32>,
    pub found_items: Option<i32>,
    pub missing_items: Option<i32>,
    pub damaged_items: Option<i32>,
    pub notes: Option<String>,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = inventory_audits)]
pub struct NewInventoryAudit {
    pub audit_name: String,
    pub audit_date: NaiveDate,
    pub audit_type: String,
    pub location: Option<String>,
    pub department_id: Option<i32>,
    pub audited_by: Uuid,
    pub status: Option<String>,
    pub total_items: Option<i32>,
    pub notes: Option<String>,
    pub started_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = inventory_audit_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InventoryAuditItem {
    pub id: i32,
    pub audit_id: i32,
    pub entity_type: String,
    pub entity_id: i32,
    pub expected_location: Option<String>,
    pub actual_location: Option<String>,
    pub expected_condition: Option<String>,
    pub actual_condition: Option<String>,
    pub status: String,
    pub discrepancy_notes: Option<String>,
    pub photo_url: Option<String>,
    pub checked_at: Option<NaiveDateTime>,
    pub checked_by: Option<Uuid>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = inventory_audit_items)]
pub struct NewInventoryAuditItem {
    pub audit_id: i32,
    pub entity_type: String,
    pub entity_id: i32,
    pub expected_location: Option<String>,
    pub status: String,
}

/// Изменения позиции. `None` означает «поле не трогать».
#[derive(Debug, AsChangeset)]
#[diesel(table_name = inventory_audit_items)]
pub struct UpdateInventoryAuditItem {
    pub status: Option<String>,
    pub actual_location: Option<String>,
    pub actual_condition: Option<String>,
    pub discrepancy_notes: Option<String>,
    pub checked_at: Option<NaiveDateTime>,
    pub checked_by: Option<Uuid>,
}

// ============================================================================
// Запросы API
// ============================================================================

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct CreateInventoryAuditRequest {
    /// Название сессии, например «Инвентаризация офиса, ноябрь 2026».
    pub audit_name: String,
    /// `full`, `partial`, `spot_check` или `annual`. По умолчанию `full`.
    pub audit_type: Option<String>,
    /// Ограничить выборку компьютеров одним расположением.
    pub location: Option<String>,
    /// Ограничить выборку компьютерами сотрудников подразделения.
    pub department_id: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateInventoryItemRequest {
    /// `found`, `missing`, `damaged`, `relocated`, `surplus`, `disposed` или `pending`.
    pub status: String,
    /// Фактическое расположение, если оно отличается от учётного.
    pub actual_location: Option<String>,
    /// `new`, `good`, `fair`, `poor` или `broken`.
    pub actual_condition: Option<String>,
    pub discrepancy_notes: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct CompleteInventoryAuditRequest {
    /// Если true, непроверенные позиции помечаются как отсутствующие.
    pub mark_pending_missing: Option<bool>,
}

// ============================================================================
// Ответы API
// ============================================================================

#[derive(Debug, Clone, Default, Serialize, poem_openapi::Object)]
pub struct InventoryCounts {
    pub total: i64,
    pub checked: i64,
    pub found: i64,
    pub missing: i64,
    pub damaged: i64,
}

impl InventoryCounts {
    /// Считает сводку по статусам позиций.
    ///
    /// `relocated` и `surplus` считаются найденными: оборудование физически
    /// есть, просто лежит не там, где записано.
    pub fn from_statuses<'a>(statuses: impl IntoIterator<Item = &'a str>) -> Self {
        let mut counts = Self::default();

        for status in statuses {
            counts.total += 1;
            match status {
                "pending" => {}
                "missing" => {
                    counts.checked += 1;
                    counts.missing += 1;
                }
                "damaged" => {
                    counts.checked += 1;
                    counts.damaged += 1;
                }
                _ => {
                    counts.checked += 1;
                    counts.found += 1;
                }
            }
        }

        counts
    }

    pub fn pending(&self) -> i64 {
        self.total - self.checked
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct InventoryAuditResponse {
    pub id: i32,
    pub audit_name: String,
    pub audit_date: String,
    pub audit_type: String,
    pub location: Option<String>,
    pub department_id: Option<i32>,
    pub audited_by: String,
    pub audited_by_email: Option<String>,
    pub status: String,
    pub notes: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
    pub counts: InventoryCounts,
}

impl InventoryAuditResponse {
    pub fn from_parts(
        audit: InventoryAudit,
        counts: InventoryCounts,
        audited_by_email: Option<String>,
    ) -> Self {
        let fmt_dt = |value: Option<NaiveDateTime>| {
            value.map(|d| d.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string())
        };

        Self {
            id: audit.id,
            audit_name: audit.audit_name,
            audit_date: audit.audit_date.to_string(),
            audit_type: audit.audit_type,
            location: audit.location,
            department_id: audit.department_id,
            audited_by: audit.audited_by.to_string(),
            audited_by_email,
            status: audit.status.unwrap_or_else(|| "in_progress".to_string()),
            notes: audit.notes,
            started_at: fmt_dt(audit.started_at),
            completed_at: fmt_dt(audit.completed_at),
            created_at: audit
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            counts,
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct InventoryItemResponse {
    pub id: i32,
    pub audit_id: i32,
    pub entity_type: String,
    pub entity_id: i32,
    /// Название объекта: для компьютера — hostname или инвентарный номер.
    pub entity_name: Option<String>,
    pub entity_inventory_number: Option<String>,
    pub expected_location: Option<String>,
    pub actual_location: Option<String>,
    pub expected_condition: Option<String>,
    pub actual_condition: Option<String>,
    pub status: String,
    pub discrepancy_notes: Option<String>,
    pub checked_at: Option<String>,
    pub checked_by: Option<String>,
}

impl InventoryItemResponse {
    pub fn from_item(
        item: InventoryAuditItem,
        entity_name: Option<String>,
        entity_inventory_number: Option<String>,
    ) -> Self {
        Self {
            id: item.id,
            audit_id: item.audit_id,
            entity_type: item.entity_type,
            entity_id: item.entity_id,
            entity_name,
            entity_inventory_number,
            expected_location: item.expected_location,
            actual_location: item.actual_location,
            expected_condition: item.expected_condition,
            actual_condition: item.actual_condition,
            status: item.status,
            discrepancy_notes: item.discrepancy_notes,
            checked_at: item
                .checked_at
                .map(|d| d.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()),
            checked_by: item.checked_by.map(|id| id.to_string()),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct InventoryAuditDetailResponse {
    pub audit: InventoryAuditResponse,
    pub items: Vec<InventoryItemResponse>,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct InventoryAuditListResponse {
    pub audits: Vec<InventoryAuditResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_pending_and_checked_items() {
        let counts = InventoryCounts::from_statuses([
            "pending",
            "pending",
            "found",
            "relocated",
            "missing",
            "damaged",
        ]);

        assert_eq!(counts.total, 6);
        assert_eq!(counts.checked, 4);
        assert_eq!(counts.found, 2);
        assert_eq!(counts.missing, 1);
        assert_eq!(counts.damaged, 1);
        assert_eq!(counts.pending(), 2);
    }

    #[test]
    fn validates_statuses_and_types() {
        assert!(is_valid_item_status("pending"));
        assert!(is_valid_item_status("found"));
        assert!(!is_valid_item_status("ok"));

        assert!(is_valid_audit_type("spot_check"));
        assert!(!is_valid_audit_type("full_check"));
    }
}
