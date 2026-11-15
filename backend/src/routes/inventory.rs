use std::collections::HashMap;

use chrono::Utc;
use poem_openapi::{
    ApiResponse, OpenApi,
    param::{Path, Query},
    payload::Json,
};

use crate::config::database::Pool;
use crate::middleware::auth::{ClaimsExt, UserAuth};
use crate::models::inventory::{
    CompleteInventoryAuditRequest, CreateInventoryAuditRequest, InventoryAudit,
    InventoryAuditDetailResponse, InventoryAuditListResponse, InventoryAuditResponse,
    InventoryCounts, InventoryItemResponse, NewInventoryAudit, NewInventoryAuditItem,
    UpdateInventoryAuditItem, UpdateInventoryItemRequest, is_valid_audit_type,
    is_valid_item_status,
};
use crate::repositories::inventory_repository::InventoryRepository;
use crate::routes::api::ApiTags;
use crate::services::activity_log_service::ActivityLogService;
use crate::utils::error::{AppError, ErrorResponse};

const CONDITION_VALUES: [&str; 5] = ["new", "good", "fair", "poor", "broken"];

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum InventoryListApiResponse {
    #[oai(status = 200)]
    Ok(Json<InventoryAuditListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum InventoryDetailApiResponse {
    #[oai(status = 200)]
    Ok(Json<InventoryAuditDetailResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum InventoryCreateApiResponse {
    #[oai(status = 201)]
    Created(Json<InventoryAuditDetailResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum InventoryItemApiResponse {
    #[oai(status = 200)]
    Ok(Json<InventoryItemResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 409)]
    Conflict(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum InventoryFinishApiResponse {
    #[oai(status = 200)]
    Ok(Json<InventoryAuditDetailResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 409)]
    Conflict(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

pub struct InventoryApi {
    repository: InventoryRepository,
    activity_log: ActivityLogService,
}

impl InventoryApi {
    pub fn new(db_pool: Pool) -> Self {
        Self {
            repository: InventoryRepository::new(db_pool.clone()),
            activity_log: ActivityLogService::new(db_pool),
        }
    }

    /// Собирает ответ с позициями: подтягивает названия компьютеров
    /// и email автора сессии одним запросом на каждый тип данных.
    async fn build_detail(
        &self,
        audit: InventoryAudit,
    ) -> Result<InventoryAuditDetailResponse, AppError> {
        let items = self.repository.get_items(audit.id).await?;
        let counts = InventoryCounts::from_statuses(items.iter().map(|item| item.status.as_str()));

        let computer_ids: Vec<i32> = items
            .iter()
            .filter(|item| item.entity_type == "computer")
            .map(|item| item.entity_id)
            .collect();

        let labels: HashMap<i32, (Option<String>, Option<String>)> = self
            .repository
            .get_computer_labels(&computer_ids)
            .await?
            .into_iter()
            .map(|(id, hostname, inventory_number)| (id, (hostname, inventory_number)))
            .collect();

        let emails: HashMap<String, String> = self
            .repository
            .get_user_emails(&[audit.audited_by])
            .await?
            .into_iter()
            .map(|(id, email)| (id.to_string(), email))
            .collect();

        let audited_by_email = emails.get(&audit.audited_by.to_string()).cloned();

        let item_responses = items
            .into_iter()
            .map(|item| {
                let (entity_name, entity_inventory_number) =
                    labels.get(&item.entity_id).cloned().unwrap_or((None, None));
                InventoryItemResponse::from_item(item, entity_name, entity_inventory_number)
            })
            .collect();

        Ok(InventoryAuditDetailResponse {
            audit: InventoryAuditResponse::from_parts(audit, counts, audited_by_email),
            items: item_responses,
        })
    }

    async fn load_detail(&self, id: i32) -> Result<InventoryAuditDetailResponse, AppError> {
        let audit = self.repository.get_audit(id).await?;
        self.build_detail(audit).await
    }
}

#[OpenApi(prefix_path = "/inventory", tag = "ApiTags::Inventory")]
impl InventoryApi {
    /// Список сессий инвентаризации
    #[oai(path = "/audits", method = "get")]
    async fn list_audits(
        &self,
        _auth: UserAuth,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(status): Query<Option<String>>,
    ) -> InventoryListApiResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).clamp(1, 100);

        match self.repository.list_audits(page, per_page, status).await {
            Ok((audits, total, mut counts)) => {
                let emails: HashMap<String, String> = match self
                    .repository
                    .get_user_emails(
                        &audits
                            .iter()
                            .map(|audit| audit.audited_by)
                            .collect::<Vec<_>>(),
                    )
                    .await
                {
                    Ok(rows) => rows
                        .into_iter()
                        .map(|(id, email)| (id.to_string(), email))
                        .collect(),
                    Err(e) => {
                        return InventoryListApiResponse::InternalError(Json(
                            e.to_error_response(),
                        ));
                    }
                };

                let responses = audits
                    .into_iter()
                    .map(|audit| {
                        let audit_counts = counts.remove(&audit.id).unwrap_or_default();
                        let email = emails.get(&audit.audited_by.to_string()).cloned();
                        InventoryAuditResponse::from_parts(audit, audit_counts, email)
                    })
                    .collect();

                InventoryListApiResponse::Ok(Json(InventoryAuditListResponse {
                    audits: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => InventoryListApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Сессия инвентаризации с позициями
    #[oai(path = "/audits/:id", method = "get")]
    async fn get_audit(&self, _auth: UserAuth, Path(id): Path<i32>) -> InventoryDetailApiResponse {
        match self.load_detail(id).await {
            Ok(detail) => InventoryDetailApiResponse::Ok(Json(detail)),
            Err(AppError::NotFound(_)) => InventoryDetailApiResponse::NotFound(Json(
                AppError::NotFound("Сессия инвентаризации не найдена".to_string())
                    .to_error_response(),
            )),
            Err(e) => InventoryDetailApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Начать инвентаризацию: снимок компьютеров на текущий момент
    #[oai(path = "/audits", method = "post")]
    async fn create_audit(
        &self,
        auth: UserAuth,
        Json(payload): Json<CreateInventoryAuditRequest>,
    ) -> InventoryCreateApiResponse {
        let name = payload.audit_name.trim().to_string();
        if name.is_empty() {
            return InventoryCreateApiResponse::BadRequest(Json(
                AppError::ValidationError("Укажите название инвентаризации".to_string())
                    .to_error_response(),
            ));
        }
        if name.chars().count() > 255 {
            return InventoryCreateApiResponse::BadRequest(Json(
                AppError::ValidationError("Название длиннее 255 символов".to_string())
                    .to_error_response(),
            ));
        }

        let audit_type = payload
            .audit_type
            .unwrap_or_else(|| "full".to_string())
            .trim()
            .to_string();

        if !is_valid_audit_type(&audit_type) {
            return InventoryCreateApiResponse::BadRequest(Json(
                AppError::ValidationError(
                    "Тип должен быть full, partial, spot_check или annual".to_string(),
                )
                .to_error_response(),
            ));
        }

        let location = payload
            .location
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        let Some(user_id) = auth.0.user_id() else {
            return InventoryCreateApiResponse::BadRequest(Json(
                AppError::Unauthorized("Не удалось определить пользователя".to_string())
                    .to_error_response(),
            ));
        };

        let snapshot = match self
            .repository
            .snapshot_computers(location.as_deref(), payload.department_id)
            .await
        {
            Ok(rows) => rows,
            Err(e) => {
                return InventoryCreateApiResponse::InternalError(Json(e.to_error_response()));
            }
        };

        if snapshot.is_empty() {
            return InventoryCreateApiResponse::BadRequest(Json(
                AppError::ValidationError(
                    "Под заданные условия не подходит ни один компьютер".to_string(),
                )
                .to_error_response(),
            ));
        }

        let now = Utc::now().naive_utc();
        let new_audit = NewInventoryAudit {
            audit_name: name,
            audit_date: now.date(),
            audit_type,
            location: location.clone(),
            department_id: payload.department_id,
            audited_by: user_id,
            status: Some("in_progress".to_string()),
            total_items: Some(snapshot.len() as i32),
            notes: payload.notes.filter(|value| !value.trim().is_empty()),
            started_at: Some(now),
        };

        let audit = match self.repository.create_audit(new_audit).await {
            Ok(audit) => audit,
            Err(e) => {
                return InventoryCreateApiResponse::InternalError(Json(e.to_error_response()));
            }
        };

        let items: Vec<NewInventoryAuditItem> = snapshot
            .into_iter()
            .map(|(computer_id, expected_location)| NewInventoryAuditItem {
                audit_id: audit.id,
                entity_type: "computer".to_string(),
                entity_id: computer_id,
                expected_location,
                status: "pending".to_string(),
            })
            .collect();

        if let Err(e) = self.repository.create_items(&items).await {
            return InventoryCreateApiResponse::InternalError(Json(e.to_error_response()));
        }

        self.activity_log.log_with_details_async(
            Some(user_id),
            "inventory_started",
            "inventory_audit",
            audit.id,
            serde_json::json!({
                "audit_name": audit.audit_name,
                "audit_type": audit.audit_type,
                "items": items.len(),
                "location": audit.location,
            }),
        );

        match self.build_detail(audit).await {
            Ok(detail) => InventoryCreateApiResponse::Created(Json(detail)),
            Err(e) => InventoryCreateApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Отметить результат проверки позиции
    #[oai(path = "/audits/:id/items/:item_id", method = "patch")]
    async fn update_item(
        &self,
        auth: UserAuth,
        Path(id): Path<i32>,
        Path(item_id): Path<i32>,
        Json(payload): Json<UpdateInventoryItemRequest>,
    ) -> InventoryItemApiResponse {
        let status = payload.status.trim().to_string();
        if !is_valid_item_status(&status) {
            return InventoryItemApiResponse::BadRequest(Json(
                AppError::ValidationError(format!(
                    "Недопустимый статус: {}. Допустимы: {}",
                    status,
                    crate::models::inventory::ITEM_STATUSES.join(", ")
                ))
                .to_error_response(),
            ));
        }

        if let Some(condition) = payload.actual_condition.as_deref()
            && !CONDITION_VALUES.contains(&condition)
        {
            return InventoryItemApiResponse::BadRequest(Json(
                AppError::ValidationError(
                    "Состояние должно быть new, good, fair, poor или broken".to_string(),
                )
                .to_error_response(),
            ));
        }

        let audit = match self.repository.get_audit(id).await {
            Ok(audit) => audit,
            Err(AppError::NotFound(_)) => {
                return InventoryItemApiResponse::NotFound(Json(
                    AppError::NotFound("Сессия инвентаризации не найдена".to_string())
                        .to_error_response(),
                ));
            }
            Err(e) => {
                return InventoryItemApiResponse::InternalError(Json(e.to_error_response()));
            }
        };

        if audit
            .status
            .as_deref()
            .is_some_and(|value| value != "in_progress")
        {
            return InventoryItemApiResponse::Conflict(Json(
                AppError::Conflict("Сессия уже завершена, отметки недоступны".to_string())
                    .to_error_response(),
            ));
        }

        let user_id = auth.0.user_id();
        let checked = status != "pending";
        let changes = UpdateInventoryAuditItem {
            status: Some(status),
            actual_location: payload
                .actual_location
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty()),
            actual_condition: payload
                .actual_condition
                .filter(|value| !value.trim().is_empty()),
            discrepancy_notes: payload
                .discrepancy_notes
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty()),
            checked_at: checked.then(|| Utc::now().naive_utc()),
            checked_by: if checked { user_id } else { None },
        };

        match self.repository.update_item(id, item_id, changes).await {
            Ok(item) => {
                self.activity_log.log_with_details_async(
                    user_id,
                    "inventory_item_checked",
                    "inventory_audit",
                    id,
                    serde_json::json!({
                        "item_id": item.id,
                        "entity_id": item.entity_id,
                        "status": item.status,
                    }),
                );

                let labels = self
                    .repository
                    .get_computer_labels(&[item.entity_id])
                    .await
                    .ok()
                    .and_then(|rows| rows.into_iter().next());
                let (name, inventory_number) = labels
                    .map(|(_, hostname, number)| (hostname, number))
                    .unwrap_or((None, None));

                InventoryItemApiResponse::Ok(Json(InventoryItemResponse::from_item(
                    item,
                    name,
                    inventory_number,
                )))
            }
            Err(AppError::NotFound(_)) => InventoryItemApiResponse::NotFound(Json(
                AppError::NotFound("Позиция не найдена в этой сессии".to_string())
                    .to_error_response(),
            )),
            Err(e) => InventoryItemApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Завершить сессию и зафиксировать итоги
    #[oai(path = "/audits/:id/complete", method = "post")]
    async fn complete_audit(
        &self,
        auth: UserAuth,
        Path(id): Path<i32>,
        Json(payload): Json<CompleteInventoryAuditRequest>,
    ) -> InventoryFinishApiResponse {
        let audit = match self.repository.get_audit(id).await {
            Ok(audit) => audit,
            Err(AppError::NotFound(_)) => {
                return InventoryFinishApiResponse::NotFound(Json(
                    AppError::NotFound("Сессия инвентаризации не найдена".to_string())
                        .to_error_response(),
                ));
            }
            Err(e) => {
                return InventoryFinishApiResponse::InternalError(Json(e.to_error_response()));
            }
        };

        if audit
            .status
            .as_deref()
            .is_some_and(|value| value != "in_progress")
        {
            return InventoryFinishApiResponse::Conflict(Json(
                AppError::Conflict("Сессия уже завершена или отменена".to_string())
                    .to_error_response(),
            ));
        }

        let items = match self.repository.get_items(id).await {
            Ok(items) => items,
            Err(e) => {
                return InventoryFinishApiResponse::InternalError(Json(e.to_error_response()));
            }
        };

        let mut counts = InventoryCounts::from_statuses(items.iter().map(|i| i.status.as_str()));
        let pending = counts.pending();
        let user_id = auth.0.user_id();

        if pending > 0 {
            if payload.mark_pending_missing.unwrap_or(false) {
                if let Err(e) = self.repository.mark_pending_missing(id, user_id).await {
                    return InventoryFinishApiResponse::InternalError(Json(e.to_error_response()));
                }
                counts = match self.repository.get_items(id).await {
                    Ok(items) => {
                        InventoryCounts::from_statuses(items.iter().map(|i| i.status.as_str()))
                    }
                    Err(e) => {
                        return InventoryFinishApiResponse::InternalError(Json(
                            e.to_error_response(),
                        ));
                    }
                };
            } else {
                return InventoryFinishApiResponse::Conflict(Json(
                    AppError::Conflict(format!(
                        "Осталось непроверенных позиций: {}. Отметьте их или завершите с пометкой «отсутствует»",
                        pending
                    ))
                    .to_error_response(),
                ));
            }
        }

        if let Err(e) = self.repository.save_counts(id, &counts).await {
            return InventoryFinishApiResponse::InternalError(Json(e.to_error_response()));
        }

        let audit = match self.repository.set_status(id, "completed", true).await {
            Ok(audit) => audit,
            Err(e) => {
                return InventoryFinishApiResponse::InternalError(Json(e.to_error_response()));
            }
        };

        self.activity_log.log_with_details_async(
            user_id,
            "inventory_completed",
            "inventory_audit",
            id,
            serde_json::json!({
                "total": counts.total,
                "found": counts.found,
                "missing": counts.missing,
                "damaged": counts.damaged,
            }),
        );

        match self.build_detail(audit).await {
            Ok(detail) => InventoryFinishApiResponse::Ok(Json(detail)),
            Err(e) => InventoryFinishApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Отменить сессию
    #[oai(path = "/audits/:id/cancel", method = "post")]
    async fn cancel_audit(
        &self,
        auth: UserAuth,
        Path(id): Path<i32>,
    ) -> InventoryFinishApiResponse {
        let audit = match self.repository.get_audit(id).await {
            Ok(audit) => audit,
            Err(AppError::NotFound(_)) => {
                return InventoryFinishApiResponse::NotFound(Json(
                    AppError::NotFound("Сессия инвентаризации не найдена".to_string())
                        .to_error_response(),
                ));
            }
            Err(e) => {
                return InventoryFinishApiResponse::InternalError(Json(e.to_error_response()));
            }
        };

        if audit
            .status
            .as_deref()
            .is_some_and(|value| value != "in_progress")
        {
            return InventoryFinishApiResponse::Conflict(Json(
                AppError::Conflict("Завершённую сессию нельзя отменить".to_string())
                    .to_error_response(),
            ));
        }

        let audit = match self.repository.set_status(id, "cancelled", false).await {
            Ok(audit) => audit,
            Err(e) => {
                return InventoryFinishApiResponse::InternalError(Json(e.to_error_response()));
            }
        };

        self.activity_log.log_async(
            auth.0.user_id(),
            "inventory_cancelled",
            "inventory_audit",
            id,
        );

        match self.build_detail(audit).await {
            Ok(detail) => InventoryFinishApiResponse::Ok(Json(detail)),
            Err(e) => InventoryFinishApiResponse::InternalError(Json(e.to_error_response())),
        }
    }
}
