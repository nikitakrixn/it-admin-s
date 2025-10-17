use poem_openapi::{ApiResponse, OpenApi, param::Path, payload::Json};

use crate::config::database::Pool;
use crate::middleware;
use crate::models::position::{NewPosition, PositionResponse, UpdatePosition};
use crate::repositories::department_repository::DepartmentRepository;
use crate::repositories::position_repository::PositionRepository;
use crate::routes::api::ApiTags;
use crate::services::activity_log_service::ActivityLogService;
use crate::utils::change_tracker::ChangeTracker;
use crate::utils::error::{AppError, ErrorResponse};

#[derive(ApiResponse)]
pub enum PositionsListResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<PositionResponse>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum PositionDetailResponse {
    #[oai(status = 200)]
    Ok(Json<PositionResponse>),
    #[oai(status = 201)]
    Created(Json<PositionResponse>),
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

pub struct PositionsApi {
    repository: PositionRepository,
    dept_repository: DepartmentRepository,
    activity_log: ActivityLogService,
}

impl PositionsApi {
    pub fn new(db_pool: Pool) -> Self {
        Self {
            repository: PositionRepository::new(db_pool.clone()),
            dept_repository: DepartmentRepository::new(db_pool.clone()),
            activity_log: ActivityLogService::new(db_pool),
        }
    }
}

#[OpenApi(prefix_path = "/positions", tag = "ApiTags::Positions")]
impl PositionsApi {
    /// Get all positions
    #[oai(path = "/", method = "get")]
    async fn list_positions(&self) -> PositionsListResponse {
        match self.repository.list_positions().await {
            Ok(positions) => {
                let responses: Vec<PositionResponse> = positions
                    .into_iter()
                    .map(|pos| PositionResponse {
                        id: pos.id,
                        name: pos.name,
                        department_id: pos.department_id,
                        department_name: pos.department_name,
                        employee_count: pos.employee_count,
                        created_at: pos.created_at.to_string(),
                        updated_at: pos.updated_at.to_string(),
                    })
                    .collect();

                PositionsListResponse::Ok(Json(responses))
            }
            Err(e) => PositionsListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get position by ID
    #[oai(path = "/:id", method = "get")]
    async fn get_position(&self, Path(id): Path<i32>) -> PositionDetailResponse {
        match self.repository.get_position_by_id(id).await {
            Ok(pos) => {
                let dept_name = if let Some(dept_id) = pos.department_id {
                    self.dept_repository.get_department_by_id(dept_id).await.ok().map(|d| d.name)
                } else {
                    None
                };

                let response = PositionResponse {
                    id: pos.id,
                    name: pos.name,
                    department_id: pos.department_id,
                    department_name: dept_name,
                    employee_count: 0,
                    created_at: pos.created_at.to_string(),
                    updated_at: pos.updated_at.to_string(),
                };
                PositionDetailResponse::Ok(Json(response))
            }
            Err(e) => match e {
                AppError::NotFound(_) => PositionDetailResponse::NotFound(Json(e.to_error_response())),
                _ => PositionDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new position
    #[oai(path = "/", method = "post")]
    async fn create_position(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(new_pos): Json<NewPosition>,
    ) -> PositionDetailResponse {
        match self.repository.create_position(new_pos).await {
            Ok(pos) => {
                let dept_name = if let Some(dept_id) = pos.department_id {
                    self.dept_repository.get_department_by_id(dept_id).await.ok().map(|d| d.name)
                } else {
                    None
                };

                let details = serde_json::json!({
                    "position_name": pos.name,
                    "department_id": pos.department_id,
                });
                
                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "position",
                    pos.id,
                    details,
                );

                let response = PositionResponse {
                    id: pos.id,
                    name: pos.name,
                    department_id: pos.department_id,
                    department_name: dept_name,
                    employee_count: 0,
                    created_at: pos.created_at.to_string(),
                    updated_at: pos.updated_at.to_string(),
                };
                PositionDetailResponse::Created(Json(response))
            }
            Err(e) => PositionDetailResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update position
    #[oai(path = "/:id", method = "put")]
    async fn update_position(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(update_data): Json<UpdatePosition>,
    ) -> PositionDetailResponse {
        let old_pos = match self.repository.get_position_by_id(id).await {
            Ok(pos) => pos,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => PositionDetailResponse::NotFound(Json(e.to_error_response())),
                    _ => PositionDetailResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        match self.repository.update_position(id, update_data).await {
            Ok(pos) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("name", &old_pos.name, &pos.name);
                tracker.track_option_i32("department_id", &old_pos.department_id, &pos.department_id);

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "position_name": pos.name,
                        "changes": tracker.into_json(),
                    });
                    
                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "position",
                        pos.id,
                        details,
                    );
                }

                let dept_name = if let Some(dept_id) = pos.department_id {
                    self.dept_repository.get_department_by_id(dept_id).await.ok().map(|d| d.name)
                } else {
                    None
                };

                let response = PositionResponse {
                    id: pos.id,
                    name: pos.name,
                    department_id: pos.department_id,
                    department_name: dept_name,
                    employee_count: 0,
                    created_at: pos.created_at.to_string(),
                    updated_at: pos.updated_at.to_string(),
                };
                PositionDetailResponse::Ok(Json(response))
            }
            Err(e) => PositionDetailResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete position
    #[oai(path = "/:id", method = "delete")]
    async fn delete_position(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> PositionDetailResponse {
        let pos_name = self.repository.get_position_by_id(id).await.ok().map(|p| p.name);

        match self.repository.delete_position(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("Position with id {} not found", id));
                PositionDetailResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(name) = pos_name {
                    let details = serde_json::json!({ "position_name": name });
                    
                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "position",
                        id,
                        details,
                    );
                }
                PositionDetailResponse::NoContent
            }
            Err(e) => PositionDetailResponse::InternalError(Json(e.to_error_response())),
        }
    }
}
