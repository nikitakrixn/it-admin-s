use chrono::NaiveDateTime;
use diesel::prelude::*;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::schema::activity_log;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = activity_log)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ActivityLog {
    pub id: i32,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: i32,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = activity_log)]
pub struct NewActivityLog {
    pub user_id: Option<Uuid>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: i32,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Serialize, Object)]
pub struct ActivityLogResponse {
    pub id: i32,
    pub user_id: Option<String>,
    pub user_email: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: i32,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: String,
}

impl NewActivityLog {
    pub fn new(
        user_id: Option<Uuid>,
        action: impl Into<String>,
        entity_type: impl Into<String>,
        entity_id: i32,
    ) -> Self {
        Self {
            user_id,
            action: action.into(),
            entity_type: entity_type.into(),
            entity_id,
            details: None,
            ip_address: None,
            user_agent: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    pub fn with_ip(mut self, ip: impl Into<String>) -> Self {
        self.ip_address = Some(ip.into());
        self
    }

    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
}
