use poem_openapi::{ApiResponse, OpenApi, param::Path, param::Query, payload::Json};

use crate::config::database::Pool;
use crate::middleware;
use crate::models::consumable::{
    Consumable, ConsumableListResponse, ConsumableMovementCreatedResponse,
    ConsumableMovementListResponse, ConsumableMovementResponse, ConsumableResponse,
    NewConsumableMovementRequest, NewConsumableRequest, UpdateConsumableRequest,
};
use crate::repositories::consumable_repository::ConsumableRepository;
use crate::routes::api::ApiTags;
use crate::services::activity_log_service::ActivityLogService;
use crate::utils::change_tracker::ChangeTracker;
use crate::utils::error::{AppError, ErrorResponse};

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ConsumablesListResponse {
    #[oai(status = 200)]
    Ok(Json<ConsumableListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ConsumableDetailResponse {
    #[oai(status = 200)]
    Ok(Json<ConsumableResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ConsumableCreateResponse {
    #[oai(status = 201)]
    Created(Json<ConsumableResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ConsumableUpdateResponse {
    #[oai(status = 200)]
    Ok(Json<ConsumableResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum ConsumableDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ConsumableMovementsResponse {
    #[oai(status = 200)]
    Ok(Json<ConsumableMovementListResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ConsumableMovementCreateResponse {
    #[oai(status = 201)]
    Created(Json<ConsumableMovementCreatedResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

const MOVEMENT_TYPES: [&str; 5] = ["purchase", "usage", "return", "write_off", "transfer"];

fn validate_movement(req: &NewConsumableMovementRequest) -> Result<(), AppError> {
    if !MOVEMENT_TYPES.contains(&req.movement_type.as_str()) {
        return Err(AppError::ValidationError(format!(
            "Invalid movement type '{}'. Allowed values: {}",
            req.movement_type,
            MOVEMENT_TYPES.join(", ")
        )));
    }

    if req.quantity == 0 {
        return Err(AppError::ValidationError(
            "Movement quantity cannot be zero".to_string(),
        ));
    }

    let requires_positive = matches!(req.movement_type.as_str(), "purchase" | "return");
    let requires_negative = matches!(req.movement_type.as_str(), "usage" | "write_off");

    if requires_positive && req.quantity < 0 {
        return Err(AppError::ValidationError(format!(
            "Movement type '{}' requires a positive quantity",
            req.movement_type
        )));
    }

    if requires_negative && req.quantity > 0 {
        return Err(AppError::ValidationError(format!(
            "Movement type '{}' requires a negative quantity",
            req.movement_type
        )));
    }

    Ok(())
}

pub struct ConsumablesApi {
    repository: ConsumableRepository,
    activity_log: ActivityLogService,
}

impl ConsumablesApi {
    pub fn new(db_pool: Pool) -> Self {
        Self {
            repository: ConsumableRepository::new(db_pool.clone()),
            activity_log: ActivityLogService::new(db_pool),
        }
    }

    fn consumable_to_response(consumable: Consumable) -> ConsumableResponse {
        ConsumableResponse::from(consumable)
    }
}

#[OpenApi(prefix_path = "/consumables", tag = "ApiTags::Consumables")]
impl ConsumablesApi {
    /// Get all consumables with pagination
    #[oai(path = "/", method = "get")]
    async fn list_consumables(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(category): Query<Option<String>>,
    ) -> ConsumablesListResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self
            .repository
            .list_consumables(page, per_page, category)
            .await
        {
            Ok((consumables_list, total)) => {
                let responses: Vec<ConsumableResponse> = consumables_list
                    .into_iter()
                    .map(Self::consumable_to_response)
                    .collect();

                ConsumablesListResponse::Ok(Json(ConsumableListResponse {
                    consumables: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => ConsumablesListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get consumable by ID
    #[oai(path = "/:id", method = "get")]
    async fn get_consumable(&self, Path(id): Path<i32>) -> ConsumableDetailResponse {
        match self.repository.get_consumable_by_id(id).await {
            Ok(consumable) => {
                ConsumableDetailResponse::Ok(Json(Self::consumable_to_response(consumable)))
            }
            Err(e) => match e {
                AppError::NotFound(_) => {
                    ConsumableDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => ConsumableDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new consumable
    #[oai(path = "/", method = "post")]
    async fn create_consumable(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewConsumableRequest>,
    ) -> ConsumableCreateResponse {
        let new_consumable = req.into_new_consumable();

        match self.repository.create_consumable(new_consumable).await {
            Ok(consumable) => {
                let details = serde_json::json!({
                    "name": consumable.name,
                    "category": consumable.category,
                    "quantity": consumable.quantity,
                    "unit": consumable.unit,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "consumable",
                    consumable.id,
                    details,
                );

                ConsumableCreateResponse::Created(Json(Self::consumable_to_response(consumable)))
            }
            Err(e) => ConsumableCreateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update consumable
    #[oai(path = "/:id", method = "put")]
    async fn update_consumable(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateConsumableRequest>,
    ) -> ConsumableUpdateResponse {
        let old_consumable = match self.repository.get_consumable_by_id(id).await {
            Ok(c) => c,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        ConsumableUpdateResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => ConsumableUpdateResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let update_data = req.into_update_consumable();

        match self.repository.update_consumable(id, update_data).await {
            Ok(consumable) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("name", &old_consumable.name, &consumable.name);
                tracker.track_option_string(
                    "category",
                    &old_consumable.category,
                    &consumable.category,
                );
                tracker.track_option_string(
                    "manufacturer",
                    &old_consumable.manufacturer,
                    &consumable.manufacturer,
                );
                tracker.track_option_string("unit", &old_consumable.unit, &consumable.unit);
                tracker.track_option_string("notes", &old_consumable.notes, &consumable.notes);
                tracker.track_string(
                    "quantity",
                    &old_consumable.quantity.to_string(),
                    &consumable.quantity.to_string(),
                );
                tracker.track_string(
                    "min_quantity",
                    &old_consumable.min_quantity.to_string(),
                    &consumable.min_quantity.to_string(),
                );

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "consumable_name": consumable.name,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "consumable",
                        consumable.id,
                        details,
                    );
                }

                ConsumableUpdateResponse::Ok(Json(Self::consumable_to_response(consumable)))
            }
            Err(e) => ConsumableUpdateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete consumable
    #[oai(path = "/:id", method = "delete")]
    async fn delete_consumable(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> ConsumableDeleteResponse {
        let consumable_info = self.repository.get_consumable_by_id(id).await.ok();

        match self.repository.delete_consumable(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("Consumable with id {} not found", id));
                ConsumableDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(consumable) = consumable_info {
                    let details = serde_json::json!({
                        "name": consumable.name,
                        "category": consumable.category,
                        "quantity": consumable.quantity,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "consumable",
                        id,
                        details,
                    );
                }
                ConsumableDeleteResponse::NoContent
            }
            Err(e) => ConsumableDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get movement history for a consumable
    #[oai(path = "/:id/movements", method = "get")]
    async fn list_consumable_movements(
        &self,
        Path(id): Path<i32>,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
    ) -> ConsumableMovementsResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(50).min(100);

        if let Err(e) = self.repository.get_consumable_by_id(id).await {
            return match e {
                AppError::NotFound(_) => {
                    ConsumableMovementsResponse::NotFound(Json(e.to_error_response()))
                }
                _ => ConsumableMovementsResponse::InternalError(Json(e.to_error_response())),
            };
        }

        match self.repository.list_movements(id, page, per_page).await {
            Ok((movements, total, total_in, total_out)) => {
                ConsumableMovementsResponse::Ok(Json(ConsumableMovementListResponse {
                    movements: movements
                        .into_iter()
                        .map(ConsumableMovementResponse::from)
                        .collect(),
                    total,
                    total_in,
                    total_out,
                }))
            }
            Err(e) => ConsumableMovementsResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Record a consumable movement and update the current stock
    #[oai(path = "/:id/movements", method = "post")]
    async fn create_consumable_movement(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<NewConsumableMovementRequest>,
    ) -> ConsumableMovementCreateResponse {
        if let Err(e) = validate_movement(&req) {
            return ConsumableMovementCreateResponse::BadRequest(Json(e.to_error_response()));
        }

        if let Err(e) = self.repository.get_consumable_by_id(id).await {
            return match e {
                AppError::NotFound(_) => {
                    ConsumableMovementCreateResponse::NotFound(Json(e.to_error_response()))
                }
                _ => ConsumableMovementCreateResponse::InternalError(Json(e.to_error_response())),
            };
        }

        let movement_type = req.movement_type.clone();
        let quantity = req.quantity;

        use crate::middleware::auth::ClaimsExt;
        let user_id = auth.0.user_id();
        let new_movement = req.into_new_movement(id, user_id);

        match self.repository.create_movement(id, new_movement).await {
            Ok((movement, consumable)) => {
                let details = serde_json::json!({
                    "movement_type": movement_type,
                    "quantity": quantity,
                    "resulting_quantity": consumable.quantity,
                });

                self.activity_log.log_with_details_async(
                    user_id,
                    "updated",
                    "consumable",
                    id,
                    details,
                );

                ConsumableMovementCreateResponse::Created(Json(ConsumableMovementCreatedResponse {
                    movement: ConsumableMovementResponse::from(movement),
                    consumable: Self::consumable_to_response(consumable),
                }))
            }
            Err(e) => match e {
                AppError::ValidationError(_) => {
                    ConsumableMovementCreateResponse::BadRequest(Json(e.to_error_response()))
                }
                AppError::NotFound(_) => {
                    ConsumableMovementCreateResponse::NotFound(Json(e.to_error_response()))
                }
                _ => ConsumableMovementCreateResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }
}
