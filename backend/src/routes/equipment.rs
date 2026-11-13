use poem_openapi::{ApiResponse, OpenApi, param::Path, param::Query, payload::Json};

use crate::config::database::Pool;
use crate::middleware;
use crate::models::equipment::{
    Equipment, EquipmentListResponse, EquipmentResponse, NewEquipmentRequest,
    UpdateEquipmentRequest,
};
use crate::repositories::equipment_repository::EquipmentRepository;
use crate::routes::api::ApiTags;
use crate::services::activity_log_service::ActivityLogService;
use crate::utils::change_tracker::ChangeTracker;
use crate::utils::error::{AppError, ErrorResponse};

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum EquipmentListApiResponse {
    #[oai(status = 200)]
    Ok(Json<EquipmentListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum EquipmentDetailResponse {
    #[oai(status = 200)]
    Ok(Json<EquipmentResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum EquipmentCreateResponse {
    #[oai(status = 201)]
    Created(Json<EquipmentResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum EquipmentUpdateResponse {
    #[oai(status = 200)]
    Ok(Json<EquipmentResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum EquipmentDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

pub struct EquipmentApi {
    repository: EquipmentRepository,
    activity_log: ActivityLogService,
}

impl EquipmentApi {
    pub fn new(db_pool: Pool) -> Self {
        Self {
            repository: EquipmentRepository::new(db_pool.clone()),
            activity_log: ActivityLogService::new(db_pool),
        }
    }

    fn equipment_to_response(equipment: Equipment) -> EquipmentResponse {
        EquipmentResponse::from(equipment)
    }
}

#[OpenApi(prefix_path = "/equipment", tag = "ApiTags::Equipment")]
impl EquipmentApi {
    /// Get all equipment with pagination
    #[oai(path = "/", method = "get")]
    async fn list_equipment(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(status): Query<Option<String>>,
        Query(location): Query<Option<String>>,
    ) -> EquipmentListApiResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self
            .repository
            .list_equipment(page, per_page, status, location)
            .await
        {
            Ok((equipment_list, total)) => {
                let responses: Vec<EquipmentResponse> = equipment_list
                    .into_iter()
                    .map(Self::equipment_to_response)
                    .collect();

                EquipmentListApiResponse::Ok(Json(EquipmentListResponse {
                    equipment: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => EquipmentListApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get equipment by ID
    #[oai(path = "/:id", method = "get")]
    async fn get_equipment(&self, Path(id): Path<i32>) -> EquipmentDetailResponse {
        match self.repository.get_equipment_by_id(id).await {
            Ok(equipment) => {
                EquipmentDetailResponse::Ok(Json(Self::equipment_to_response(equipment)))
            }
            Err(e) => match e {
                AppError::NotFound(_) => {
                    EquipmentDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => EquipmentDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new equipment
    #[oai(path = "/", method = "post")]
    async fn create_equipment(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewEquipmentRequest>,
    ) -> EquipmentCreateResponse {
        let new_equipment = match req.into_new_equipment() {
            Ok(eq) => eq,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return EquipmentCreateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.repository.create_equipment(new_equipment).await {
            Ok(equipment) => {
                let details = serde_json::json!({
                    "name": equipment.name,
                    "inventory_number": equipment.inventory_number,
                    "status": equipment.status,
                    "location": equipment.location,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "equipment",
                    equipment.id,
                    details,
                );

                EquipmentCreateResponse::Created(Json(Self::equipment_to_response(equipment)))
            }
            Err(e) => EquipmentCreateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update equipment
    #[oai(path = "/:id", method = "put")]
    async fn update_equipment(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateEquipmentRequest>,
    ) -> EquipmentUpdateResponse {
        let old_equipment = match self.repository.get_equipment_by_id(id).await {
            Ok(eq) => eq,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        EquipmentUpdateResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => EquipmentUpdateResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let update_data = match req.into_update_equipment() {
            Ok(data) => data,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return EquipmentUpdateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.repository.update_equipment(id, update_data).await {
            Ok(equipment) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("name", &old_equipment.name, &equipment.name);
                tracker.track_string("status", &old_equipment.status, &equipment.status);
                tracker.track_option_string(
                    "inventory_number",
                    &old_equipment.inventory_number,
                    &equipment.inventory_number,
                );
                tracker.track_option_string(
                    "manufacturer",
                    &old_equipment.manufacturer,
                    &equipment.manufacturer,
                );
                tracker.track_option_string("model", &old_equipment.model, &equipment.model);
                tracker.track_option_string(
                    "serial_number",
                    &old_equipment.serial_number,
                    &equipment.serial_number,
                );
                tracker.track_option_string(
                    "location",
                    &old_equipment.location,
                    &equipment.location,
                );
                tracker.track_option_string("notes", &old_equipment.notes, &equipment.notes);
                tracker.track_option_i32(
                    "employee_id",
                    &old_equipment.employee_id,
                    &equipment.employee_id,
                );
                tracker.track_option_i32(
                    "location_id",
                    &old_equipment.location_id,
                    &equipment.location_id,
                );

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "equipment_name": equipment.name,
                        "inventory_number": equipment.inventory_number,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "equipment",
                        equipment.id,
                        details,
                    );
                }

                EquipmentUpdateResponse::Ok(Json(Self::equipment_to_response(equipment)))
            }
            Err(e) => EquipmentUpdateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete equipment
    #[oai(path = "/:id", method = "delete")]
    async fn delete_equipment(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> EquipmentDeleteResponse {
        let equipment_info = self.repository.get_equipment_by_id(id).await.ok();

        match self.repository.delete_equipment(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("Equipment with id {} not found", id));
                EquipmentDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(equipment) = equipment_info {
                    let details = serde_json::json!({
                        "name": equipment.name,
                        "inventory_number": equipment.inventory_number,
                        "status": equipment.status,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "equipment",
                        id,
                        details,
                    );
                }
                EquipmentDeleteResponse::NoContent
            }
            Err(e) => EquipmentDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }
}
