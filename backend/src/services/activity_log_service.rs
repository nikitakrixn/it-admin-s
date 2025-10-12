use diesel::prelude::*;
use uuid::Uuid;

use crate::config::database::Pool;
use crate::models::activity_log::NewActivityLog;
use crate::models::schema::activity_log;

pub struct ActivityLogService {
    db_pool: Pool,
}

impl ActivityLogService {
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }

    /// Log an activity with minimal information
    pub fn log(
        &self,
        user_id: Option<Uuid>,
        action: impl Into<String>,
        entity_type: impl Into<String>,
        entity_id: i32,
    ) -> Result<(), diesel::result::Error> {
        let new_log = NewActivityLog::new(user_id, action, entity_type, entity_id);

        let mut conn = self.db_pool.get().map_err(|e| {
            tracing::error!("Failed to get DB connection for activity log: {}", e);
            diesel::result::Error::BrokenTransactionManager
        })?;

        diesel::insert_into(activity_log::table)
            .values(&new_log)
            .execute(&mut conn)?;

        Ok(())
    }

    /// Log an activity with details (JSONB)
    pub fn log_with_details(
        &self,
        user_id: Option<Uuid>,
        action: impl Into<String>,
        entity_type: impl Into<String>,
        entity_id: i32,
        details: serde_json::Value,
    ) -> Result<(), diesel::result::Error> {
        let new_log =
            NewActivityLog::new(user_id, action, entity_type, entity_id).with_details(details);

        let mut conn = self.db_pool.get().map_err(|e| {
            tracing::error!("Failed to get DB connection for activity log: {}", e);
            diesel::result::Error::BrokenTransactionManager
        })?;

        diesel::insert_into(activity_log::table)
            .values(&new_log)
            .execute(&mut conn)?;

        Ok(())
    }

    /// Log an activity with full context (IP, user agent, details)
    pub fn log_full(
        &self,
        user_id: Option<Uuid>,
        action: impl Into<String>,
        entity_type: impl Into<String>,
        entity_id: i32,
        details: Option<serde_json::Value>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), diesel::result::Error> {
        let mut new_log = NewActivityLog::new(user_id, action, entity_type, entity_id);

        if let Some(d) = details {
            new_log = new_log.with_details(d);
        }

        if let Some(ip) = ip_address {
            new_log = new_log.with_ip(ip);
        }

        if let Some(ua) = user_agent {
            new_log = new_log.with_user_agent(ua);
        }

        let mut conn = self.db_pool.get().map_err(|e| {
            tracing::error!("Failed to get DB connection for activity log: {}", e);
            diesel::result::Error::BrokenTransactionManager
        })?;

        diesel::insert_into(activity_log::table)
            .values(&new_log)
            .execute(&mut conn)?;

        Ok(())
    }

    /// Log in background (fire and forget - doesn't return errors)
    pub fn log_async(
        &self,
        user_id: Option<Uuid>,
        action: impl Into<String>,
        entity_type: impl Into<String>,
        entity_id: i32,
    ) {
        if let Err(e) = self.log(user_id, action, entity_type, entity_id) {
            tracing::warn!("Failed to log activity: {}", e);
        }
    }

    /// Log with details in background
    pub fn log_with_details_async(
        &self,
        user_id: Option<Uuid>,
        action: impl Into<String>,
        entity_type: impl Into<String>,
        entity_id: i32,
        details: serde_json::Value,
    ) {
        if let Err(e) = self.log_with_details(user_id, action, entity_type, entity_id, details) {
            tracing::warn!("Failed to log activity with details: {}", e);
        }
    }
}
