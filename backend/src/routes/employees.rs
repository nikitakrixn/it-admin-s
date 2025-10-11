use diesel::prelude::*;
use poem_openapi::{param::Path, param::Query, payload::Json, ApiResponse, Object, OpenApi};
use serde::{Deserialize, Serialize};

use crate::config::database::Pool;
use crate::models::employee::{
    Department, DepartmentResponse, Employee, EmployeeResponse, NewDepartment, NewEmployeeRequest,
    NewPosition, Position, PositionResponse, UpdateEmployeeRequest,
};
use crate::models::schema::{departments, employees, positions};
use crate::routes::api::ApiTags;

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

#[derive(Serialize, Object)]
#[oai(rename = "EmployeeErrorResponse")]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
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

#[derive(ApiResponse)]
pub enum DepartmentsListResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<DepartmentResponse>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum PositionsListResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<PositionResponse>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// API Implementation
// ============================================================================

pub struct EmployeesApi {
    db_pool: Pool,
}

impl EmployeesApi {
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
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
        let offset = (page - 1) * per_page;

        let mut conn = match self.db_pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                return EmployeesListResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }))
            }
        };

        // Build base query for counting
        let mut count_query = employees::table.into_boxed();

        if let Some(ref s) = status {
            count_query = count_query.filter(employees::status.eq(s));
        }

        if let Some(dept_id) = department_id {
            count_query = count_query.filter(employees::department_id.eq(dept_id));
        }

        // Get total count
        let total = match count_query
            .count()
            .get_result::<i64>(&mut conn)
        {
            Ok(count) => count,
            Err(e) => {
                return EmployeesListResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to count employees: {}", e),
                }))
            }
        };

        // Build query for data retrieval
        let mut query = employees::table
            .left_join(positions::table)
            .left_join(departments::table)
            .into_boxed();

        if let Some(s) = status {
            query = query.filter(employees::status.eq(s));
        }

        if let Some(dept_id) = department_id {
            query = query.filter(employees::department_id.eq(dept_id));
        }

        // Get employees with pagination
        let results = match query
            .select((
                Employee::as_select(),
                positions::name.nullable(),
                departments::name.nullable(),
            ))
            .order(employees::last_name.asc())
            .limit(per_page)
            .offset(offset)
            .load::<(Employee, Option<String>, Option<String>)>(&mut conn)
        {
            Ok(results) => results,
            Err(e) => {
                return EmployeesListResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to load employees: {}", e),
                }))
            }
        };

        let employees_list = results
            .into_iter()
            .map(|(emp, pos_name, dept_name)| Self::employee_to_response(emp, pos_name, dept_name))
            .collect();

        EmployeesListResponse::Ok(Json(EmployeeListResponse {
            employees: employees_list,
            total,
            page,
            per_page,
        }))
    }

    /// Get employee by ID
    #[oai(path = "/:id", method = "get")]
    async fn get_employee(&self, Path(id): Path<i32>) -> EmployeeDetailResponse {
        let mut conn = match self.db_pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                return EmployeeDetailResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }))
            }
        };

        let result = employees::table
            .find(id)
            .left_join(positions::table)
            .left_join(departments::table)
            .select((
                Employee::as_select(),
                positions::name.nullable(),
                departments::name.nullable(),
            ))
            .first::<(Employee, Option<String>, Option<String>)>(&mut conn);

        match result {
            Ok((employee, pos_name, dept_name)) => {
                EmployeeDetailResponse::Ok(Json(Self::employee_to_response(
                    employee, pos_name, dept_name,
                )))
            }
            Err(diesel::NotFound) => {
                EmployeeDetailResponse::NotFound(Json(ErrorResponse {
                    error: "not_found".to_string(),
                    message: format!("Employee with id {} not found", id),
                }))
            }
            Err(e) => EmployeeDetailResponse::InternalError(Json(ErrorResponse {
                error: "database_error".to_string(),
                message: format!("Failed to get employee: {}", e),
            })),
        }
    }

    /// Create new employee
    #[oai(path = "/", method = "post")]
    async fn create_employee(&self, Json(req): Json<NewEmployeeRequest>) -> EmployeeCreateResponse {
        let new_employee = match req.to_new_employee() {
            Ok(emp) => emp,
            Err(e) => {
                return EmployeeCreateResponse::BadRequest(Json(ErrorResponse {
                    error: "validation_error".to_string(),
                    message: e,
                }))
            }
        };
        let mut conn = match self.db_pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                return EmployeeCreateResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }))
            }
        };

        let result = diesel::insert_into(employees::table)
            .values(&new_employee)
            .get_result::<Employee>(&mut conn);

        match result {
            Ok(employee) => {
                // Get position and department names
                let (pos_name, dept_name) = if employee.position_id.is_some() || employee.department_id.is_some() {
                    let pos = employee.position_id.and_then(|pid| {
                        positions::table.find(pid).select(positions::name).first::<String>(&mut conn).ok()
                    });
                    let dept = employee.department_id.and_then(|did| {
                        departments::table.find(did).select(departments::name).first::<String>(&mut conn).ok()
                    });
                    (pos, dept)
                } else {
                    (None, None)
                };

                EmployeeCreateResponse::Created(Json(Self::employee_to_response(
                    employee, pos_name, dept_name,
                )))
            }
            Err(e) => EmployeeCreateResponse::InternalError(Json(ErrorResponse {
                error: "database_error".to_string(),
                message: format!("Failed to create employee: {}", e),
            })),
        }
    }

    /// Update employee
    #[oai(path = "/:id", method = "put")]
    async fn update_employee(
        &self,
        Path(id): Path<i32>,
        Json(req): Json<UpdateEmployeeRequest>,
    ) -> EmployeeUpdateResponse {
        let update_data = match req.to_update_employee() {
            Ok(data) => data,
            Err(e) => {
                return EmployeeUpdateResponse::InternalError(Json(ErrorResponse {
                    error: "validation_error".to_string(),
                    message: e,
                }))
            }
        };
        let mut conn = match self.db_pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                return EmployeeUpdateResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }))
            }
        };

        let result = diesel::update(employees::table.find(id))
            .set(&update_data)
            .get_result::<Employee>(&mut conn);

        match result {
            Ok(employee) => {
                let (pos_name, dept_name) = if employee.position_id.is_some() || employee.department_id.is_some() {
                    let pos = employee.position_id.and_then(|pid| {
                        positions::table.find(pid).select(positions::name).first::<String>(&mut conn).ok()
                    });
                    let dept = employee.department_id.and_then(|did| {
                        departments::table.find(did).select(departments::name).first::<String>(&mut conn).ok()
                    });
                    (pos, dept)
                } else {
                    (None, None)
                };

                EmployeeUpdateResponse::Ok(Json(Self::employee_to_response(
                    employee, pos_name, dept_name,
                )))
            }
            Err(diesel::NotFound) => {
                EmployeeUpdateResponse::NotFound(Json(ErrorResponse {
                    error: "not_found".to_string(),
                    message: format!("Employee with id {} not found", id),
                }))
            }
            Err(e) => EmployeeUpdateResponse::InternalError(Json(ErrorResponse {
                error: "database_error".to_string(),
                message: format!("Failed to update employee: {}", e),
            })),
        }
    }

    /// Delete employee
    #[oai(path = "/:id", method = "delete")]
    async fn delete_employee(&self, Path(id): Path<i32>) -> EmployeeDeleteResponse {
        let mut conn = match self.db_pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                return EmployeeDeleteResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }))
            }
        };

        let result = diesel::delete(employees::table.find(id)).execute(&mut conn);

        match result {
            Ok(0) => EmployeeDeleteResponse::NotFound(Json(ErrorResponse {
                error: "not_found".to_string(),
                message: format!("Employee with id {} not found", id),
            })),
            Ok(_) => EmployeeDeleteResponse::NoContent,
            Err(e) => EmployeeDeleteResponse::InternalError(Json(ErrorResponse {
                error: "database_error".to_string(),
                message: format!("Failed to delete employee: {}", e),
            })),
        }
    }

    /// Bulk delete employees
    #[oai(path = "/bulk-delete", method = "post")]
    async fn bulk_delete_employees(&self, Json(req): Json<BulkDeleteRequest>) -> BulkDeleteEmployeesResponse {
        if req.ids.is_empty() {
            return BulkDeleteEmployeesResponse::BadRequest(Json(ErrorResponse {
                error: "validation_error".to_string(),
                message: "No employee IDs provided".to_string(),
            }));
        }

        let mut conn = match self.db_pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                return BulkDeleteEmployeesResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }))
            }
        };

        let mut deleted_count = 0;
        let mut failed_ids = Vec::new();

        for id in req.ids {
            match diesel::delete(employees::table.find(id)).execute(&mut conn) {
                Ok(count) if count > 0 => deleted_count += 1,
                Ok(_) => failed_ids.push(id),
                Err(_) => failed_ids.push(id),
            }
        }

        BulkDeleteEmployeesResponse::Ok(Json(BulkDeleteResponse {
            deleted_count,
            failed_ids,
        }))
    }

    /// Get all departments
    #[oai(path = "/departments", method = "get")]
    async fn list_departments(&self) -> DepartmentsListResponse {
        let mut conn = match self.db_pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                return DepartmentsListResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }))
            }
        };

        let departments_result = departments::table
            .order(departments::name.asc())
            .load::<Department>(&mut conn);

        match departments_result {
            Ok(depts) => {
                let mut responses = Vec::new();
                for dept in depts {
                    let count = employees::table
                        .filter(employees::department_id.eq(dept.id))
                        .count()
                        .get_result::<i64>(&mut conn)
                        .unwrap_or(0);

                    responses.push(DepartmentResponse {
                        id: dept.id,
                        name: dept.name,
                        description: dept.description,
                        employee_count: count,
                        created_at: dept.created_at.to_string(),
                        updated_at: dept.updated_at.to_string(),
                    });
                }
                DepartmentsListResponse::Ok(Json(responses))
            }
            Err(e) => DepartmentsListResponse::InternalError(Json(ErrorResponse {
                error: "database_error".to_string(),
                message: format!("Failed to load departments: {}", e),
            })),
        }
    }

    /// Create new department
    #[oai(path = "/departments", method = "post")]
    async fn create_department(&self, Json(new_dept): Json<NewDepartment>) -> DepartmentsListResponse {
        let mut conn = match self.db_pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                return DepartmentsListResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }))
            }
        };

        match diesel::insert_into(departments::table)
            .values(&new_dept)
            .execute(&mut conn)
        {
            Ok(_) => {
                // Return updated list
                self.list_departments().await
            }
            Err(e) => DepartmentsListResponse::InternalError(Json(ErrorResponse {
                error: "database_error".to_string(),
                message: format!("Failed to create department: {}", e),
            })),
        }
    }

    /// Delete department
    #[oai(path = "/departments/:id", method = "delete")]
    async fn delete_department(&self, Path(id): Path<i32>) -> EmployeeDeleteResponse {
        let mut conn = match self.db_pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                return EmployeeDeleteResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }))
            }
        };

        let result = diesel::delete(departments::table.find(id)).execute(&mut conn);

        match result {
            Ok(0) => EmployeeDeleteResponse::NotFound(Json(ErrorResponse {
                error: "not_found".to_string(),
                message: format!("Department with id {} not found", id),
            })),
            Ok(_) => EmployeeDeleteResponse::NoContent,
            Err(e) => EmployeeDeleteResponse::InternalError(Json(ErrorResponse {
                error: "database_error".to_string(),
                message: format!("Failed to delete department: {}", e),
            })),
        }
    }

    /// Create new position
    #[oai(path = "/positions", method = "post")]
    async fn create_position(&self, Json(new_pos): Json<NewPosition>) -> PositionsListResponse {
        let mut conn = match self.db_pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                return PositionsListResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }))
            }
        };

        match diesel::insert_into(positions::table)
            .values(&new_pos)
            .execute(&mut conn)
        {
            Ok(_) => {
                // Return updated list
                self.list_positions().await
            }
            Err(e) => PositionsListResponse::InternalError(Json(ErrorResponse {
                error: "database_error".to_string(),
                message: format!("Failed to create position: {}", e),
            })),
        }
    }

    /// Delete position
    #[oai(path = "/positions/:id", method = "delete")]
    async fn delete_position(&self, Path(id): Path<i32>) -> EmployeeDeleteResponse {
        let mut conn = match self.db_pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                return EmployeeDeleteResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }))
            }
        };

        let result = diesel::delete(positions::table.find(id)).execute(&mut conn);

        match result {
            Ok(0) => EmployeeDeleteResponse::NotFound(Json(ErrorResponse {
                error: "not_found".to_string(),
                message: format!("Position with id {} not found", id),
            })),
            Ok(_) => EmployeeDeleteResponse::NoContent,
            Err(e) => EmployeeDeleteResponse::InternalError(Json(ErrorResponse {
                error: "database_error".to_string(),
                message: format!("Failed to delete position: {}", e),
            })),
        }
    }

    /// Get all positions
    #[oai(path = "/positions", method = "get")]
    async fn list_positions(&self) -> PositionsListResponse {
        let mut conn = match self.db_pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                return PositionsListResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }))
            }
        };

        let positions_result = positions::table
            .left_join(departments::table)
            .select((Position::as_select(), departments::name.nullable()))
            .order(positions::name.asc())
            .load::<(Position, Option<String>)>(&mut conn);

        match positions_result {
            Ok(positions_list) => {
                let responses = positions_list
                    .into_iter()
                    .map(|(pos, dept_name)| PositionResponse {
                        id: pos.id,
                        name: pos.name,
                        department_id: pos.department_id,
                        department_name: dept_name,
                        created_at: pos.created_at.to_string(),
                        updated_at: pos.updated_at.to_string(),
                    })
                    .collect();

                PositionsListResponse::Ok(Json(responses))
            }
            Err(e) => PositionsListResponse::InternalError(Json(ErrorResponse {
                error: "database_error".to_string(),
                message: format!("Failed to load positions: {}", e),
            })),
        }
    }
}
