use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use poem_openapi::{ApiResponse, Object, OpenApi, param::Query, payload::Json};
use serde::Serialize;

use crate::config::database::Pool;
use crate::models::activity_log::{ActivityLog, ActivityLogResponse};
use crate::models::schema::{activity_log, users};
use crate::routes::api::ApiTags;

// ============================================================================
// Response Types
// ============================================================================

#[derive(Serialize, Object)]
pub struct ActivityLogListResponse {
    pub logs: Vec<ActivityLogResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Serialize, Object)]
#[oai(rename = "ActivityLogErrorResponse")]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(ApiResponse)]
pub enum ActivityLogsListResponse {
    #[oai(status = 200)]
    Ok(Json<ActivityLogListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// API Implementation
// ============================================================================

pub struct ActivityLogApi {
    db_pool: Pool,
}

impl ActivityLogApi {
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }
}

#[OpenApi(prefix_path = "/activity-log", tag = "ApiTags::ActivityLog")]
impl ActivityLogApi {
    /// Get activity logs with pagination and filters
    #[oai(path = "/", method = "get")]
    async fn list_activity_logs(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(action): Query<Option<String>>,
        Query(entity_type): Query<Option<String>>,
        Query(user_id): Query<Option<String>>,
    ) -> ActivityLogsListResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(50).min(200);
        let offset = (page - 1) * per_page;

        let mut conn = match self.db_pool.get().await {
            Ok(conn) => conn,
            Err(e) => {
                return ActivityLogsListResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }));
            }
        };

        // Build base query for counting
        let mut count_query = activity_log::table.into_boxed();

        if let Some(ref a) = action {
            count_query = count_query.filter(activity_log::action.eq(a));
        }

        if let Some(ref et) = entity_type {
            count_query = count_query.filter(activity_log::entity_type.eq(et));
        }

        if let Some(ref uid) = user_id {
            if let Ok(uuid) = uid.parse::<uuid::Uuid>() {
                count_query = count_query.filter(activity_log::user_id.eq(uuid));
            }
        }

        // Get total count
        let total = match count_query.count().get_result::<i64>(&mut conn).await {
            Ok(count) => count,
            Err(e) => {
                return ActivityLogsListResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to count activity logs: {}", e),
                }));
            }
        };

        // Build query for data retrieval
        let mut data_query = activity_log::table.into_boxed();

        if let Some(ref a) = action {
            data_query = data_query.filter(activity_log::action.eq(a));
        }

        if let Some(ref et) = entity_type {
            data_query = data_query.filter(activity_log::entity_type.eq(et));
        }

        if let Some(ref uid) = user_id {
            if let Ok(uuid) = uid.parse::<uuid::Uuid>() {
                data_query = data_query.filter(activity_log::user_id.eq(uuid));
            }
        }

        // Get logs with pagination
        let logs = match data_query
            .order(activity_log::created_at.desc())
            .limit(per_page)
            .offset(offset)
            .load::<ActivityLog>(&mut conn)
            .await
        {
            Ok(logs) => logs,
            Err(e) => {
                return ActivityLogsListResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to load activity logs: {}", e),
                }));
            }
        };

        // Get user emails for logs that have user_id
        let user_ids: Vec<uuid::Uuid> = logs.iter().filter_map(|log| log.user_id).collect();

        let user_emails: std::collections::HashMap<uuid::Uuid, String> = if !user_ids.is_empty() {
            users::table
                .filter(users::id.eq_any(&user_ids))
                .select((users::id, users::email))
                .load::<(uuid::Uuid, String)>(&mut conn)
                .await
                .unwrap_or_default()
                .into_iter()
                .collect()
        } else {
            std::collections::HashMap::new()
        };

        let logs_list = logs
            .into_iter()
            .map(|log| {
                let user_email = log.user_id.and_then(|uid| user_emails.get(&uid).cloned());
                ActivityLogResponse {
                    id: log.id,
                    user_id: log.user_id.map(|u| u.to_string()),
                    user_email,
                    action: log.action,
                    entity_type: log.entity_type,
                    entity_id: log.entity_id,
                    details: log.details,
                    ip_address: log.ip_address,
                    user_agent: log.user_agent,
                    created_at: log.created_at.to_string(),
                }
            })
            .collect();

        ActivityLogsListResponse::Ok(Json(ActivityLogListResponse {
            logs: logs_list,
            total,
            page,
            per_page,
        }))
    }
}
