use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema::{request_comments, requests};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Request {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub requester_id: i32,
    pub assignee_id: Option<i32>,
    pub computer_id: Option<i32>,
    pub equipment_id: Option<i32>,
    pub priority: String,
    pub status: String,
    pub category: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub resolved_at: Option<NaiveDateTime>,
    pub closed_at: Option<NaiveDateTime>,
    pub first_response_at: Option<NaiveDateTime>,
    pub sla_deadline: Option<NaiveDateTime>,
    pub sla_breached: Option<bool>,
    pub estimated_time: Option<i32>,
    pub actual_time: Option<i32>,
    pub parent_request_id: Option<i32>,
    pub recurring_schedule: Option<String>,
    pub knowledge_base_article_id: Option<i32>,
    pub satisfaction_rating: Option<i32>,
    pub satisfaction_comment: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = requests)]
pub struct NewRequest {
    pub title: String,
    pub description: String,
    pub requester_id: i32,
    pub assignee_id: Option<i32>,
    pub computer_id: Option<i32>,
    pub equipment_id: Option<i32>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub category: Option<String>,
    pub resolved_at: Option<NaiveDateTime>,
    pub closed_at: Option<NaiveDateTime>,
    pub first_response_at: Option<NaiveDateTime>,
    pub sla_deadline: Option<NaiveDateTime>,
    pub sla_breached: Option<bool>,
    pub estimated_time: Option<i32>,
    pub actual_time: Option<i32>,
    pub parent_request_id: Option<i32>,
    pub recurring_schedule: Option<String>,
    pub knowledge_base_article_id: Option<i32>,
    pub satisfaction_rating: Option<i32>,
    pub satisfaction_comment: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = requests)]
pub struct UpdateRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub requester_id: Option<i32>,
    pub assignee_id: Option<i32>,
    pub computer_id: Option<i32>,
    pub equipment_id: Option<i32>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub category: Option<String>,
    pub resolved_at: Option<NaiveDateTime>,
    pub closed_at: Option<NaiveDateTime>,
    pub first_response_at: Option<NaiveDateTime>,
    pub sla_deadline: Option<NaiveDateTime>,
    pub sla_breached: Option<bool>,
    pub estimated_time: Option<i32>,
    pub actual_time: Option<i32>,
    pub parent_request_id: Option<i32>,
    pub recurring_schedule: Option<String>,
    pub knowledge_base_article_id: Option<i32>,
    pub satisfaction_rating: Option<i32>,
    pub satisfaction_comment: Option<String>,
}

// ============================================================================
// API request/response types (строковые даты, т.к. poem_openapi::Object не
// поддерживает NaiveDateTime)
// ============================================================================

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewRequestRequest {
    pub title: String,
    pub description: String,
    pub requester_id: i32,
    pub assignee_id: Option<i32>,
    pub computer_id: Option<i32>,
    pub equipment_id: Option<i32>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub category: Option<String>,
    pub resolved_at: Option<String>,
    pub closed_at: Option<String>,
    pub first_response_at: Option<String>,
    pub sla_deadline: Option<String>,
    pub sla_breached: Option<bool>,
    pub estimated_time: Option<i32>,
    pub actual_time: Option<i32>,
    pub parent_request_id: Option<i32>,
    pub recurring_schedule: Option<String>,
    pub knowledge_base_article_id: Option<i32>,
    pub satisfaction_rating: Option<i32>,
    pub satisfaction_comment: Option<String>,
}

impl NewRequestRequest {
    pub fn into_new_request(self) -> Result<NewRequest, String> {
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

        Ok(NewRequest {
            title: self.title,
            description: self.description,
            requester_id: self.requester_id,
            assignee_id: self.assignee_id,
            computer_id: self.computer_id,
            equipment_id: self.equipment_id,
            priority: self.priority,
            status: self.status,
            category: self.category,
            resolved_at: parse_datetime(&self.resolved_at)?,
            closed_at: parse_datetime(&self.closed_at)?,
            first_response_at: parse_datetime(&self.first_response_at)?,
            sla_deadline: parse_datetime(&self.sla_deadline)?,
            sla_breached: self.sla_breached,
            estimated_time: self.estimated_time,
            actual_time: self.actual_time,
            parent_request_id: self.parent_request_id,
            recurring_schedule: self.recurring_schedule,
            knowledge_base_article_id: self.knowledge_base_article_id,
            satisfaction_rating: self.satisfaction_rating,
            satisfaction_comment: self.satisfaction_comment,
        })
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateRequestRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub requester_id: Option<i32>,
    pub assignee_id: Option<i32>,
    pub computer_id: Option<i32>,
    pub equipment_id: Option<i32>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub category: Option<String>,
    pub resolved_at: Option<String>,
    pub closed_at: Option<String>,
    pub first_response_at: Option<String>,
    pub sla_deadline: Option<String>,
    pub sla_breached: Option<bool>,
    pub estimated_time: Option<i32>,
    pub actual_time: Option<i32>,
    pub parent_request_id: Option<i32>,
    pub recurring_schedule: Option<String>,
    pub knowledge_base_article_id: Option<i32>,
    pub satisfaction_rating: Option<i32>,
    pub satisfaction_comment: Option<String>,
}

impl UpdateRequestRequest {
    pub fn into_update_request(self) -> Result<UpdateRequest, String> {
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

        Ok(UpdateRequest {
            title: self.title,
            description: self.description,
            requester_id: self.requester_id,
            assignee_id: self.assignee_id,
            computer_id: self.computer_id,
            equipment_id: self.equipment_id,
            priority: self.priority,
            status: self.status,
            category: self.category,
            resolved_at: parse_datetime(&self.resolved_at)?,
            closed_at: parse_datetime(&self.closed_at)?,
            first_response_at: parse_datetime(&self.first_response_at)?,
            sla_deadline: parse_datetime(&self.sla_deadline)?,
            sla_breached: self.sla_breached,
            estimated_time: self.estimated_time,
            actual_time: self.actual_time,
            parent_request_id: self.parent_request_id,
            recurring_schedule: self.recurring_schedule,
            knowledge_base_article_id: self.knowledge_base_article_id,
            satisfaction_rating: self.satisfaction_rating,
            satisfaction_comment: self.satisfaction_comment,
        })
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct RequestResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub requester_id: i32,
    pub assignee_id: Option<i32>,
    pub computer_id: Option<i32>,
    pub equipment_id: Option<i32>,
    pub priority: String,
    pub status: String,
    pub category: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub resolved_at: Option<String>,
    pub closed_at: Option<String>,
    pub first_response_at: Option<String>,
    pub sla_deadline: Option<String>,
    pub sla_breached: Option<bool>,
    pub estimated_time: Option<i32>,
    pub actual_time: Option<i32>,
    pub parent_request_id: Option<i32>,
    pub recurring_schedule: Option<String>,
    pub knowledge_base_article_id: Option<i32>,
    pub satisfaction_rating: Option<i32>,
    pub satisfaction_comment: Option<String>,
}

impl From<Request> for RequestResponse {
    fn from(request: Request) -> Self {
        let fmt =
            |dt: Option<NaiveDateTime>| dt.map(|d| d.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string());
        Self {
            id: request.id,
            title: request.title,
            description: request.description,
            requester_id: request.requester_id,
            assignee_id: request.assignee_id,
            computer_id: request.computer_id,
            equipment_id: request.equipment_id,
            priority: request.priority,
            status: request.status,
            category: request.category,
            created_at: request
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            updated_at: request
                .updated_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            resolved_at: fmt(request.resolved_at),
            closed_at: fmt(request.closed_at),
            first_response_at: fmt(request.first_response_at),
            sla_deadline: fmt(request.sla_deadline),
            sla_breached: request.sla_breached,
            estimated_time: request.estimated_time,
            actual_time: request.actual_time,
            parent_request_id: request.parent_request_id,
            recurring_schedule: request.recurring_schedule,
            knowledge_base_article_id: request.knowledge_base_article_id,
            satisfaction_rating: request.satisfaction_rating,
            satisfaction_comment: request.satisfaction_comment,
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct RequestListResponse {
    pub requests: Vec<RequestResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

// ============================================================================
// Комментарии к заявкам
// ============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = request_comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RequestComment {
    pub id: i32,
    pub request_id: i32,
    pub user_id: Uuid,
    pub comment: String,
    pub is_internal: Option<bool>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = request_comments)]
pub struct NewRequestComment {
    pub request_id: i32,
    pub user_id: Uuid,
    pub comment: String,
    pub is_internal: Option<bool>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewRequestCommentRequest {
    pub comment: String,
    pub is_internal: Option<bool>,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct RequestCommentResponse {
    pub id: i32,
    pub request_id: i32,
    pub user_id: String,
    pub user_email: Option<String>,
    pub comment: String,
    pub is_internal: bool,
    pub created_at: String,
}

impl RequestCommentResponse {
    pub fn new(comment: RequestComment, user_email: Option<String>) -> Self {
        Self {
            id: comment.id,
            request_id: comment.request_id,
            user_id: comment.user_id.to_string(),
            user_email,
            comment: comment.comment,
            is_internal: comment.is_internal.unwrap_or(false),
            created_at: comment
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct RequestCommentListResponse {
    pub comments: Vec<RequestCommentResponse>,
    pub total: i64,
}
