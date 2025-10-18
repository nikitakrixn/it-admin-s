use poem_openapi::{ApiResponse, Object, OpenApi, param::Path, param::Query, payload::Json};
use serde::{Deserialize, Serialize};

use crate::config::database::Pool;
use crate::middleware;
use crate::models::employee::{
    Employee, EmployeeResponse, NewEmployeeRequest, UpdateEmployeeRequest,
};
use crate::repositories::employee_repository::EmployeeRepository;
use crate::routes::api::ApiTags;
use crate::services::activity_log_service::ActivityLogService;
use crate::utils::change_tracker::ChangeTracker;
use crate::utils::error::{AppError, ErrorResponse};

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Serialize, Object)]
pub struct EmployeeListResponse {
    pub employees: Vec<EmployeeResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Deserialize, Object)]
pub struct BulkDeleteRequest {
    pub ids: Vec<i32>,
}

#[derive(Serialize, Object)]
pub struct BulkDeleteResponse {
    pub deleted_count: usize,
    pub failed_ids: Vec<i32>,
}

#[derive(ApiResponse)]
pub enum EmployeesListResponse {
    #[oai(status = 200)]
    Ok(Json<EmployeeListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum EmployeeDetailResponse {
    #[oai(status = 200)]
    Ok(Json<EmployeeResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum EmployeeCreateResponse {
    #[oai(status = 201)]
    Created(Json<EmployeeResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum EmployeeUpdateResponse {
    #[oai(status = 200)]
    Ok(Json<EmployeeResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum EmployeeDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum BulkDeleteEmployeesResponse {
    #[oai(status = 200)]
    Ok(Json<BulkDeleteResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// API Implementation
// ============================================================================

pub struct EmployeesApi {
    repository: EmployeeRepository,
    activity_log: ActivityLogService,
}

impl EmployeesApi {
    pub fn new(db_pool: Pool) -> Self {
        Self {
            repository: EmployeeRepository::new(db_pool.clone()),
            activity_log: ActivityLogService::new(db_pool),
        }
    }

    fn employee_to_response(
        employee: Employee,
        position_name: Option<String>,
        department_name: Option<String>,
    ) -> EmployeeResponse {
        EmployeeResponse {
            id: employee.id,
            first_name: employee.first_name.clone(),
            last_name: employee.last_name.clone(),
            middle_name: employee.middle_name.clone(),
            full_name: employee.full_name(),
            position_id: employee.position_id,
            position_name,
            department_id: employee.department_id,
            department_name,
            email: employee.email,
            phone: employee.phone,
            ad_username: employee.ad_username,
            hire_date: employee.hire_date.map(|d| d.to_string()),
            termination_date: employee.termination_date.map(|d| d.to_string()),
            status: employee.status,
            notes: employee.notes,
            created_at: employee.created_at.to_string(),
            updated_at: employee.updated_at.to_string(),
        }
    }
}

#[OpenApi(prefix_path = "/employees", tag = "ApiTags::Employees")]
impl EmployeesApi {
    /// Get all employees with pagination
    #[oai(path = "/", method = "get")]
    async fn list_employees(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(status): Query<Option<String>>,
        Query(department_id): Query<Option<i32>>,
    ) -> EmployeesListResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self
            .repository
            .list_employees(page, per_page, status, department_id)
            .await
        {
            Ok((results, total)) => {
                let employees_list = results
                    .into_iter()
                    .map(|(emp, pos_name, dept_name)| {
                        Self::employee_to_response(emp, pos_name, dept_name)
                    })
                    .collect();

                EmployeesListResponse::Ok(Json(EmployeeListResponse {
                    employees: employees_list,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => EmployeesListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get employee by ID
    #[oai(path = "/:id", method = "get")]
    async fn get_employee(&self, Path(id): Path<i32>) -> EmployeeDetailResponse {
        match self.repository.get_employee_by_id(id).await {
            Ok((employee, pos_name, dept_name)) => EmployeeDetailResponse::Ok(Json(
                Self::employee_to_response(employee, pos_name, dept_name),
            )),
            Err(e) => match e {
                AppError::NotFound(_) => {
                    EmployeeDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => EmployeeDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new employee
    #[oai(path = "/", method = "post")]
    async fn create_employee(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewEmployeeRequest>,
    ) -> EmployeeCreateResponse {
        let new_employee = match req.to_new_employee() {
            Ok(emp) => emp,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return EmployeeCreateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.repository.create_employee(new_employee).await {
            Ok(employee) => {
                let details = serde_json::json!({
                    "employee_name": employee.full_name(),
                    "email": employee.email,
                    "department_id": employee.department_id,
                    "position_id": employee.position_id,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "employee",
                    employee.id,
                    details,
                );

                let (pos_name, dept_name) = self
                    .repository
                    .get_position_and_department_names(employee.position_id, employee.department_id)
                    .await
                    .unwrap_or((None, None));

                EmployeeCreateResponse::Created(Json(Self::employee_to_response(
                    employee, pos_name, dept_name,
                )))
            }
            Err(e) => EmployeeCreateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update employee
    #[oai(path = "/:id", method = "put")]
    async fn update_employee(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateEmployeeRequest>,
    ) -> EmployeeUpdateResponse {
        let old_employee = match self.repository.get_employee_by_id(id).await {
            Ok((emp, _, _)) => emp,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        EmployeeUpdateResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => EmployeeUpdateResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let update_data = match req.to_update_employee() {
            Ok(data) => data,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return EmployeeUpdateResponse::InternalError(Json(err.to_error_response()));
            }
        };

        match self.repository.update_employee(id, update_data).await {
            Ok(employee) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("first_name", &old_employee.first_name, &employee.first_name);
                tracker.track_string("last_name", &old_employee.last_name, &employee.last_name);
                tracker.track_option_string(
                    "middle_name",
                    &old_employee.middle_name,
                    &employee.middle_name,
                );
                tracker.track_option_string("email", &old_employee.email, &employee.email);
                tracker.track_option_string("phone", &old_employee.phone, &employee.phone);
                tracker.track_string("status", &old_employee.status, &employee.status);
                tracker.track_option_i32(
                    "position_id",
                    &old_employee.position_id,
                    &employee.position_id,
                );
                tracker.track_option_i32(
                    "department_id",
                    &old_employee.department_id,
                    &employee.department_id,
                );

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "employee_name": employee.full_name(),
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "employee",
                        employee.id,
                        details,
                    );
                }

                let (pos_name, dept_name) = self
                    .repository
                    .get_position_and_department_names(employee.position_id, employee.department_id)
                    .await
                    .unwrap_or((None, None));

                EmployeeUpdateResponse::Ok(Json(Self::employee_to_response(
                    employee, pos_name, dept_name,
                )))
            }
            Err(e) => EmployeeUpdateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete employee
    #[oai(path = "/:id", method = "delete")]
    async fn delete_employee(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> EmployeeDeleteResponse {
        let employee_info = self
            .repository
            .get_employee_info_for_log(id)
            .await
            .ok()
            .flatten();

        match self.repository.delete_employee(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("Employee with id {} not found", id));
                EmployeeDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some((
                    first_name,
                    last_name,
                    middle_name,
                    email,
                    phone,
                    status,
                    position,
                    department,
                )) = employee_info
                {
                    let full_name = if let Some(mn) = middle_name {
                        format!("{} {} {}", last_name, first_name, mn)
                    } else {
                        format!("{} {}", last_name, first_name)
                    };

                    let details = serde_json::json!({
                        "employee_name": full_name,
                        "email": email,
                        "phone": phone,
                        "status": status,
                        "position": position,
                        "department": department,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "employee",
                        id,
                        details,
                    );
                }
                EmployeeDeleteResponse::NoContent
            }
            Err(e) => EmployeeDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Bulk delete employees
    #[oai(path = "/bulk-delete", method = "post")]
    async fn bulk_delete_employees(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<BulkDeleteRequest>,
    ) -> BulkDeleteEmployeesResponse {
        if req.ids.is_empty() {
            let err = AppError::ValidationError("No employee IDs provided".to_string());
            return BulkDeleteEmployeesResponse::BadRequest(Json(err.to_error_response()));
        }

        let mut deleted_count = 0;
        let mut failed_ids = Vec::new();

        for id in req.ids {
            match self.repository.delete_employee(id).await {
                Ok(count) if count > 0 => {
                    deleted_count += 1;
                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log
                        .log_async(auth.0.user_id(), "deleted", "employee", id);
                }
                _ => failed_ids.push(id),
            }
        }

        BulkDeleteEmployeesResponse::Ok(Json(BulkDeleteResponse {
            deleted_count,
            failed_ids,
        }))
    }
}
