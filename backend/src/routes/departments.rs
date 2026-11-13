use poem_openapi::{ApiResponse, OpenApi, param::Path, payload::Json};

use crate::config::database::Pool;
use crate::middleware;
use crate::models::department::{DepartmentResponse, NewDepartment, UpdateDepartment};
use crate::repositories::department_repository::DepartmentRepository;
use crate::routes::api::ApiTags;
use crate::services::activity_log_service::ActivityLogService;
use crate::utils::change_tracker::ChangeTracker;
use crate::utils::error::{AppError, ErrorResponse};

#[derive(ApiResponse)]
pub enum DepartmentsListResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<DepartmentResponse>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum DepartmentDetailResponse {
    #[oai(status = 200)]
    Ok(Json<DepartmentResponse>),
    #[oai(status = 201)]
    Created(Json<DepartmentResponse>),
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

pub struct DepartmentsApi {
    repository: DepartmentRepository,
    activity_log: ActivityLogService,
}

impl DepartmentsApi {
    pub fn new(db_pool: Pool) -> Self {
        Self {
            repository: DepartmentRepository::new(db_pool.clone()),
            activity_log: ActivityLogService::new(db_pool),
        }
    }
}

#[OpenApi(prefix_path = "/departments", tag = "ApiTags::Departments")]
impl DepartmentsApi {
    /// Get all departments
    #[oai(path = "/", method = "get")]
    async fn list_departments(&self) -> DepartmentsListResponse {
        match self.repository.list_departments().await {
            Ok(depts) => {
                let responses: Vec<DepartmentResponse> = depts
                    .into_iter()
                    .map(|dept| DepartmentResponse {
                        id: dept.id,
                        name: dept.name,
                        description: dept.description,
                        employee_count: dept.employee_count,
                        position_count: dept.position_count,
                        created_at: dept.created_at.to_string(),
                        updated_at: dept.updated_at.to_string(),
                    })
                    .collect();

                DepartmentsListResponse::Ok(Json(responses))
            }
            Err(e) => DepartmentsListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get department by ID
    #[oai(path = "/:id", method = "get")]
    async fn get_department(&self, Path(id): Path<i32>) -> DepartmentDetailResponse {
        match self.repository.get_department_by_id(id).await {
            Ok(dept) => {
                let response = DepartmentResponse {
                    id: dept.id,
                    name: dept.name,
                    description: dept.description,
                    employee_count: 0,
                    position_count: 0,
                    created_at: dept.created_at.to_string(),
                    updated_at: dept.updated_at.to_string(),
                };
                DepartmentDetailResponse::Ok(Json(response))
            }
            Err(e) => match e {
                AppError::NotFound(_) => {
                    DepartmentDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => DepartmentDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new department
    #[oai(path = "/", method = "post")]
    async fn create_department(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(new_dept): Json<NewDepartment>,
    ) -> DepartmentDetailResponse {
        match self.repository.create_department(new_dept).await {
            Ok(dept) => {
                let details = serde_json::json!({
                    "department_name": dept.name,
                    "description": dept.description,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "department",
                    dept.id,
                    details,
                );

                let response = DepartmentResponse {
                    id: dept.id,
                    name: dept.name,
                    description: dept.description,
                    employee_count: 0,
                    position_count: 0,
                    created_at: dept.created_at.to_string(),
                    updated_at: dept.updated_at.to_string(),
                };
                DepartmentDetailResponse::Created(Json(response))
            }
            Err(e) => DepartmentDetailResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update department
    #[oai(path = "/:id", method = "put")]
    async fn update_department(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(update_data): Json<UpdateDepartment>,
    ) -> DepartmentDetailResponse {
        let old_dept = match self.repository.get_department_by_id(id).await {
            Ok(dept) => dept,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        DepartmentDetailResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => DepartmentDetailResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        match self.repository.update_department(id, update_data).await {
            Ok(dept) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("name", &old_dept.name, &dept.name);
                tracker.track_option_string(
                    "description",
                    &old_dept.description,
                    &dept.description,
                );

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "department_name": dept.name,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "department",
                        dept.id,
                        details,
                    );
                }

                let response = DepartmentResponse {
                    id: dept.id,
                    name: dept.name,
                    description: dept.description,
                    employee_count: 0,
                    position_count: 0,
                    created_at: dept.created_at.to_string(),
                    updated_at: dept.updated_at.to_string(),
                };
                DepartmentDetailResponse::Ok(Json(response))
            }
            Err(e) => DepartmentDetailResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete department
    #[oai(path = "/:id", method = "delete")]
    async fn delete_department(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> DepartmentDetailResponse {
        let dept_name = self
            .repository
            .get_department_by_id(id)
            .await
            .ok()
            .map(|d| d.name);

        match self.repository.delete_department(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("Department with id {} not found", id));
                DepartmentDetailResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(name) = dept_name {
                    let details = serde_json::json!({ "department_name": name });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "department",
                        id,
                        details,
                    );
                }
                DepartmentDetailResponse::NoContent
            }
            Err(e) => DepartmentDetailResponse::InternalError(Json(e.to_error_response())),
        }
    }
}
