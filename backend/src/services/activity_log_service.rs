use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::config::database::Pool;
use crate::models::activity_log::NewActivityLog;
use crate::models::schema::activity_log;

#[derive(Clone)]
pub struct ActivityLogService {
    db_pool: Pool,
}

impl ActivityLogService {
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }

    /// Log an activity with minimal information
    pub async fn log(
        &self,
        user_id: Option<Uuid>,
        action: impl Into<String>,
        entity_type: impl Into<String>,
        entity_id: i32,
    ) -> Result<(), diesel::result::Error> {
        let new_log = NewActivityLog::new(user_id, action, entity_type, entity_id);

        let mut conn = self.db_pool.get().await.map_err(|e| {
            tracing::error!("Failed to get DB connection for activity log: {}", e);
            diesel::result::Error::BrokenTransactionManager
        })?;

        diesel::insert_into(activity_log::table)
            .values(&new_log)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Log an activity with details (JSONB)
    pub async fn log_with_details(
        &self,
        user_id: Option<Uuid>,
        action: impl Into<String>,
        entity_type: impl Into<String>,
        entity_id: i32,
        details: serde_json::Value,
    ) -> Result<(), diesel::result::Error> {
        let new_log =
            NewActivityLog::new(user_id, action, entity_type, entity_id).with_details(details);

        let mut conn = self.db_pool.get().await.map_err(|e| {
            tracing::error!("Failed to get DB connection for activity log: {}", e);
            diesel::result::Error::BrokenTransactionManager
        })?;

        diesel::insert_into(activity_log::table)
            .values(&new_log)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Log in background (fire and forget - doesn't return errors)
    pub fn log_async(
        &self,
        user_id: Option<Uuid>,
        action: impl Into<String> + Send + 'static,
        entity_type: impl Into<String> + Send + 'static,
        entity_id: i32,
    ) {
        let pool = self.db_pool.clone();
        let action = action.into();
        let entity_type = entity_type.into();

        tokio::spawn(async move {
            let service = ActivityLogService::new(pool);
            if let Err(e) = service.log(user_id, action, entity_type, entity_id).await {
                tracing::warn!("Failed to log activity: {}", e);
            }
        });
    }

    /// Log with details in background
    pub fn log_with_details_async(
        &self,
        user_id: Option<Uuid>,
        action: impl Into<String> + Send + 'static,
        entity_type: impl Into<String> + Send + 'static,
        entity_id: i32,
        details: serde_json::Value,
    ) {
        let pool = self.db_pool.clone();
        let action = action.into();
        let entity_type = entity_type.into();

        tokio::spawn(async move {
            let service = ActivityLogService::new(pool);
            if let Err(e) = service
                .log_with_details(user_id, action, entity_type, entity_id, details)
                .await
            {
                tracing::warn!("Failed to log activity with details: {}", e);
            }
        });
    }
}
