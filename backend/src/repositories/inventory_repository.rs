use std::collections::HashMap;

use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::config::database::Pool;
use crate::models::inventory::{
    InventoryAudit, InventoryAuditItem, InventoryCounts, NewInventoryAudit, NewInventoryAuditItem,
    UpdateInventoryAuditItem,
};
use crate::models::schema::{computers, employees, inventory_audit_items, inventory_audits, users};
use crate::utils::db_helpers::{DbResult, get_connection};

pub struct InventoryRepository {
    pool: Pool,
}

impl InventoryRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Список сессий с постраничной выдачей.
    pub async fn list_audits(
        &self,
        page: i64,
        per_page: i64,
        status: Option<String>,
    ) -> DbResult<(Vec<InventoryAudit>, i64, HashMap<i32, InventoryCounts>)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let mut count_query = inventory_audits::table.into_boxed();
        if let Some(ref status) = status {
            count_query = count_query.filter(inventory_audits::status.eq(status));
        }
        let total = count_query.count().get_result::<i64>(&mut conn).await?;

        let mut query = inventory_audits::table.into_boxed();
        if let Some(status) = status {
            query = query.filter(inventory_audits::status.eq(status));
        }

        let audits = query
            .order(inventory_audits::created_at.desc())
            .limit(per_page)
            .offset(offset)
            .load::<InventoryAudit>(&mut conn)
            .await?;

        // Сводку по статусам считаем одним запросом на всю страницу,
        // чтобы не делать по запросу на каждую сессию.
        let audit_ids: Vec<i32> = audits.iter().map(|audit| audit.id).collect();
        let mut counts: HashMap<i32, InventoryCounts> = HashMap::new();

        if !audit_ids.is_empty() {
            let rows = inventory_audit_items::table
                .filter(inventory_audit_items::audit_id.eq_any(&audit_ids))
                .select((
                    inventory_audit_items::audit_id,
                    inventory_audit_items::status,
                ))
                .load::<(i32, String)>(&mut conn)
                .await?;

            let mut statuses: HashMap<i32, Vec<String>> = HashMap::new();
            for (audit_id, status) in rows {
                statuses.entry(audit_id).or_default().push(status);
            }

            for (audit_id, values) in statuses {
                counts.insert(
                    audit_id,
                    InventoryCounts::from_statuses(values.iter().map(String::as_str)),
                );
            }
        }

        Ok((audits, total, counts))
    }

    pub async fn get_audit(&self, id: i32) -> DbResult<InventoryAudit> {
        let mut conn = get_connection(&self.pool).await?;

        inventory_audits::table
            .find(id)
            .first::<InventoryAudit>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn get_items(&self, audit_id: i32) -> DbResult<Vec<InventoryAuditItem>> {
        let mut conn = get_connection(&self.pool).await?;

        inventory_audit_items::table
            .filter(inventory_audit_items::audit_id.eq(audit_id))
            .order(inventory_audit_items::id.asc())
            .load::<InventoryAuditItem>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn create_audit(&self, new_audit: NewInventoryAudit) -> DbResult<InventoryAudit> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::insert_into(inventory_audits::table)
            .values(&new_audit)
            .get_result::<InventoryAudit>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn create_items(&self, items: &[NewInventoryAuditItem]) -> DbResult<usize> {
        if items.is_empty() {
            return Ok(0);
        }

        let mut conn = get_connection(&self.pool).await?;

        diesel::insert_into(inventory_audit_items::table)
            .values(items)
            .execute(&mut conn)
            .await
            .map_err(Into::into)
    }

    /// Снимок компьютеров для новой сессии: id и учётное расположение.
    pub async fn snapshot_computers(
        &self,
        location: Option<&str>,
        department_id: Option<i32>,
    ) -> DbResult<Vec<(i32, Option<String>)>> {
        let mut conn = get_connection(&self.pool).await?;
        let mut query = computers::table.into_boxed();

        if let Some(location) = location {
            query = query.filter(computers::location.eq(location.to_string()));
        }

        if let Some(department_id) = department_id {
            let employee_ids = employees::table
                .filter(employees::department_id.eq(department_id))
                .select(employees::id.nullable());
            query = query.filter(computers::employee_id.eq_any(employee_ids));
        }

        query
            .select((computers::id, computers::location))
            .order(computers::id.asc())
            .load::<(i32, Option<String>)>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn update_item(
        &self,
        audit_id: i32,
        item_id: i32,
        changes: UpdateInventoryAuditItem,
    ) -> DbResult<InventoryAuditItem> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(
            inventory_audit_items::table
                .filter(inventory_audit_items::id.eq(item_id))
                .filter(inventory_audit_items::audit_id.eq(audit_id)),
        )
        .set(&changes)
        .get_result::<InventoryAuditItem>(&mut conn)
        .await
        .map_err(Into::into)
    }

    /// Помечает все непроверенные позиции сессии как отсутствующие.
    pub async fn mark_pending_missing(
        &self,
        audit_id: i32,
        user_id: Option<Uuid>,
    ) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;
        let now = Utc::now().naive_utc();

        diesel::update(
            inventory_audit_items::table
                .filter(inventory_audit_items::audit_id.eq(audit_id))
                .filter(inventory_audit_items::status.eq("pending")),
        )
        .set((
            inventory_audit_items::status.eq("missing"),
            inventory_audit_items::checked_at.eq(Some(now)),
            inventory_audit_items::checked_by.eq(user_id),
        ))
        .execute(&mut conn)
        .await
        .map_err(Into::into)
    }

    pub async fn set_status(
        &self,
        audit_id: i32,
        status: &str,
        completed: bool,
    ) -> DbResult<InventoryAudit> {
        let mut conn = get_connection(&self.pool).await?;
        let completed_at = completed.then(|| Utc::now().naive_utc());

        diesel::update(inventory_audits::table.find(audit_id))
            .set((
                inventory_audits::status.eq(Some(status.to_string())),
                inventory_audits::completed_at.eq(completed_at),
            ))
            .get_result::<InventoryAudit>(&mut conn)
            .await
            .map_err(Into::into)
    }

    /// Сохраняет итоговые счётчики в сессии — они нужны для отчётов.
    pub async fn save_counts(&self, audit_id: i32, counts: &InventoryCounts) -> DbResult<()> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(inventory_audits::table.find(audit_id))
            .set((
                inventory_audits::total_items.eq(Some(counts.total as i32)),
                inventory_audits::found_items.eq(Some(counts.found as i32)),
                inventory_audits::missing_items.eq(Some(counts.missing as i32)),
                inventory_audits::damaged_items.eq(Some(counts.damaged as i32)),
            ))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Названия и инвентарные номера компьютеров для карточек позиций.
    pub async fn get_computer_labels(
        &self,
        ids: &[i32],
    ) -> DbResult<Vec<(i32, Option<String>, Option<String>)>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut conn = get_connection(&self.pool).await?;

        computers::table
            .filter(computers::id.eq_any(ids))
            .select((
                computers::id,
                computers::hostname,
                computers::inventory_number,
            ))
            .load::<(i32, Option<String>, Option<String>)>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn get_user_emails(&self, ids: &[Uuid]) -> DbResult<Vec<(Uuid, String)>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut conn = get_connection(&self.pool).await?;

        users::table
            .filter(users::id.eq_any(ids))
            .select((users::id, users::email))
            .load::<(Uuid, String)>(&mut conn)
            .await
            .map_err(Into::into)
    }
}
