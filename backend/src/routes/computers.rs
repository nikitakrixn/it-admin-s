use poem_openapi::{ApiResponse, OpenApi, param::Path, param::Query, payload::Json};

use crate::config::database::Pool;
use crate::middleware;
use crate::models::computer::{
    Computer, ComputerListResponse, ComputerResponse, NewComputerRequest, UpdateComputerRequest,
};
use crate::models::computer_component::{
    ComponentRequest, ComputerComponentListResponse, ComputerSoftwareListResponse,
    NewInstalledSoftwareRequest,
};
use crate::repositories::computer_component_repository::ComputerComponentRepository;
use crate::repositories::computer_repository::ComputerRepository;
use crate::routes::api::ApiTags;
use crate::services::activity_log_service::ActivityLogService;
use crate::utils::change_tracker::ChangeTracker;
use crate::utils::error::{AppError, ErrorResponse};

// Вариант с данными больше варианта ошибки на ~380 байт, но в poem-openapi
// каждый вариант должен реализовывать IntoResponse, а `Box<Json<T>>` — нет.
// Поэтому оставляем как есть, заглушая clippy точечно.
#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputersListResponse {
    #[oai(status = 200)]
    Ok(Json<ComputerListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerDetailResponse {
    #[oai(status = 200)]
    Ok(Json<ComputerResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerCreateResponse {
    #[oai(status = 201)]
    Created(Json<ComputerResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerUpdateResponse {
    #[oai(status = 200)]
    Ok(Json<ComputerResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum ComputerDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// Компоненты компьютера
// ============================================================================

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerComponentsListResponse {
    #[oai(status = 200)]
    Ok(Json<ComputerComponentListResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerComponentDetailResponse {
    #[oai(status = 200)]
    Ok(Json<crate::models::computer_component::ComputerComponentResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerComponentCreateResponse {
    #[oai(status = 201)]
    Created(Json<crate::models::computer_component::ComputerComponentResponse>),
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
pub enum ComputerComponentUpdateResponse {
    #[oai(status = 200)]
    Ok(Json<crate::models::computer_component::ComputerComponentResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 409)]
    Conflict(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum ComputerComponentDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// Установленное ПО
// ============================================================================

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerSoftwareListApiResponse {
    #[oai(status = 200)]
    Ok(Json<ComputerSoftwareListResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerSoftwareCreateResponse {
    #[oai(status = 201)]
    Created(Json<crate::models::computer_component::InstalledSoftwareResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 409)]
    Conflict(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum ComputerSoftwareDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

pub struct ComputersApi {
    repository: ComputerRepository,
    component_repository: ComputerComponentRepository,
    activity_log: ActivityLogService,
}

impl ComputersApi {
    pub fn new(db_pool: Pool) -> Self {
        Self {
            repository: ComputerRepository::new(db_pool.clone()),
            component_repository: ComputerComponentRepository::new(db_pool.clone()),
            activity_log: ActivityLogService::new(db_pool),
        }
    }

    fn computer_to_response(computer: Computer) -> ComputerResponse {
        ComputerResponse::from(computer)
    }
}

#[OpenApi(prefix_path = "/computers", tag = "ApiTags::Computers")]
impl ComputersApi {
    /// Get all computers with pagination
    #[oai(path = "/", method = "get")]
    async fn list_computers(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(status): Query<Option<String>>,
        Query(computer_type): Query<Option<String>>,
    ) -> ComputersListResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self
            .repository
            .list_computers(page, per_page, status, computer_type)
            .await
        {
            Ok((computers_list, total)) => {
                let responses: Vec<ComputerResponse> = computers_list
                    .into_iter()
                    .map(Self::computer_to_response)
                    .collect();

                ComputersListResponse::Ok(Json(ComputerListResponse {
                    computers: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => ComputersListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get computer by ID
    #[oai(path = "/:id", method = "get")]
    async fn get_computer(&self, Path(id): Path<i32>) -> ComputerDetailResponse {
        match self.repository.get_computer_by_id(id).await {
            Ok(computer) => ComputerDetailResponse::Ok(Json(Self::computer_to_response(computer))),
            Err(e) => match e {
                AppError::NotFound(_) => {
                    ComputerDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => ComputerDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new computer
    #[oai(path = "/", method = "post")]
    async fn create_computer(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewComputerRequest>,
    ) -> ComputerCreateResponse {
        let new_computer = match req.into_new_computer() {
            Ok(comp) => comp,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return ComputerCreateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.repository.create_computer(new_computer).await {
            Ok(computer) => {
                let details = serde_json::json!({
                    "hostname": computer.hostname,
                    "inventory_number": computer.inventory_number,
                    "computer_type": computer.computer_type,
                    "os": computer.os,
                    "status": computer.status,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "computer",
                    computer.id,
                    details,
                );

                ComputerCreateResponse::Created(Json(Self::computer_to_response(computer)))
            }
            Err(e) => ComputerCreateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update computer
    #[oai(path = "/:id", method = "put")]
    async fn update_computer(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateComputerRequest>,
    ) -> ComputerUpdateResponse {
        let old_computer = match self.repository.get_computer_by_id(id).await {
            Ok(comp) => comp,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        ComputerUpdateResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => ComputerUpdateResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let update_data = match req.into_update_computer() {
            Ok(data) => data,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return ComputerUpdateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.repository.update_computer(id, update_data).await {
            Ok(computer) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string(
                    "computer_type",
                    &old_computer.computer_type,
                    &computer.computer_type,
                );
                tracker.track_string("status", &old_computer.status, &computer.status);
                tracker.track_option_string("hostname", &old_computer.hostname, &computer.hostname);
                tracker.track_option_string(
                    "inventory_number",
                    &old_computer.inventory_number,
                    &computer.inventory_number,
                );
                tracker.track_option_string("os", &old_computer.os, &computer.os);
                tracker.track_option_string(
                    "os_version",
                    &old_computer.os_version,
                    &computer.os_version,
                );
                tracker.track_option_string(
                    "manufacturer",
                    &old_computer.manufacturer,
                    &computer.manufacturer,
                );
                tracker.track_option_string("model", &old_computer.model, &computer.model);
                tracker.track_option_string(
                    "serial_number",
                    &old_computer.serial_number,
                    &computer.serial_number,
                );
                tracker.track_option_string("location", &old_computer.location, &computer.location);
                tracker.track_option_string("notes", &old_computer.notes, &computer.notes);
                tracker.track_option_i32(
                    "employee_id",
                    &old_computer.employee_id,
                    &computer.employee_id,
                );
                tracker.track_option_bool(
                    "is_online",
                    &old_computer.is_online,
                    &computer.is_online,
                );
                tracker.track_option_bool(
                    "domain_joined",
                    &old_computer.domain_joined,
                    &computer.domain_joined,
                );
                tracker.track_option_bool(
                    "bitlocker_enabled",
                    &old_computer.bitlocker_enabled,
                    &computer.bitlocker_enabled,
                );

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "computer_name": computer
                            .hostname
                            .as_deref()
                            .unwrap_or(computer.inventory_number.as_deref().unwrap_or("")),
                        "inventory_number": computer.inventory_number,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "computer",
                        computer.id,
                        details,
                    );
                }

                ComputerUpdateResponse::Ok(Json(Self::computer_to_response(computer)))
            }
            Err(e) => ComputerUpdateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete computer
    #[oai(path = "/:id", method = "delete")]
    async fn delete_computer(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> ComputerDeleteResponse {
        let computer_info = self.repository.get_computer_by_id(id).await.ok();

        match self.repository.delete_computer(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("Computer with id {} not found", id));
                ComputerDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(computer) = computer_info {
                    let details = serde_json::json!({
                        "hostname": computer.hostname,
                        "inventory_number": computer.inventory_number,
                        "computer_type": computer.computer_type,
                        "status": computer.status,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "computer",
                        id,
                        details,
                    );
                }
                ComputerDeleteResponse::NoContent
            }
            Err(e) => ComputerDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }

    // ------------------------------------------------------------------------
    // Компоненты компьютера
    // ------------------------------------------------------------------------

    /// Get all components of a computer
    #[oai(path = "/:id/components", method = "get")]
    async fn list_computer_components(
        &self,
        Path(id): Path<i32>,
    ) -> ComputerComponentsListResponse {
        if let Err(e) = self.repository.get_computer_by_id(id).await {
            return match e {
                AppError::NotFound(_) => {
                    ComputerComponentsListResponse::NotFound(Json(e.to_error_response()))
                }
                _ => ComputerComponentsListResponse::InternalError(Json(e.to_error_response())),
            };
        }

        match self.component_repository.list_components(id).await {
            Ok(components) => {
                let total = components.len() as i64;
                ComputerComponentsListResponse::Ok(Json(ComputerComponentListResponse {
                    computer_id: id,
                    components,
                    total,
                }))
            }
            Err(e) => ComputerComponentsListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get one component of a computer
    #[oai(path = "/:id/components/:component_type/:component_id", method = "get")]
    async fn get_computer_component(
        &self,
        Path(id): Path<i32>,
        Path(component_type): Path<String>,
        Path(component_id): Path<i32>,
    ) -> ComputerComponentDetailResponse {
        match self
            .component_repository
            .get_component(id, &component_type, component_id)
            .await
        {
            Ok(component) => ComputerComponentDetailResponse::Ok(Json(component)),
            Err(e) => match e {
                AppError::NotFound(_) => {
                    ComputerComponentDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => ComputerComponentDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Add a component to a computer
    #[oai(path = "/:id/components", method = "post")]
    async fn create_computer_component(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<ComponentRequest>,
    ) -> ComputerComponentCreateResponse {
        if let Err(e) = self.repository.get_computer_by_id(id).await {
            return match e {
                AppError::NotFound(_) => {
                    ComputerComponentCreateResponse::NotFound(Json(e.to_error_response()))
                }
                _ => ComputerComponentCreateResponse::InternalError(Json(e.to_error_response())),
            };
        }

        match self.component_repository.create_component(id, req).await {
            Ok(component) => {
                let details = serde_json::json!({
                    "computer_id": id,
                    "component_type": component.component_type,
                    "component_id": component.id,
                    "name": component.name,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "component_added",
                    "computer",
                    id,
                    details,
                );

                ComputerComponentCreateResponse::Created(Json(component))
            }
            Err(e) => match e {
                AppError::ValidationError(_) => {
                    ComputerComponentCreateResponse::BadRequest(Json(e.to_error_response()))
                }
                AppError::Conflict(_) => {
                    ComputerComponentCreateResponse::Conflict(Json(e.to_error_response()))
                }
                AppError::NotFound(_) => {
                    ComputerComponentCreateResponse::NotFound(Json(e.to_error_response()))
                }
                _ => ComputerComponentCreateResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Update a component of a computer
    #[oai(path = "/:id/components/:component_type/:component_id", method = "put")]
    async fn update_computer_component(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Path(component_type): Path<String>,
        Path(component_id): Path<i32>,
        Json(req): Json<ComponentRequest>,
    ) -> ComputerComponentUpdateResponse {
        let old_component = match self
            .component_repository
            .get_component(id, &component_type, component_id)
            .await
        {
            Ok(component) => component,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        ComputerComponentUpdateResponse::NotFound(Json(e.to_error_response()))
                    }
                    AppError::ValidationError(_) => {
                        ComputerComponentUpdateResponse::BadRequest(Json(e.to_error_response()))
                    }
                    _ => {
                        ComputerComponentUpdateResponse::InternalError(Json(e.to_error_response()))
                    }
                };
            }
        };

        match self
            .component_repository
            .update_component(id, &component_type, component_id, req)
            .await
        {
            Ok(component) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("name", &old_component.name, &component.name);
                tracker.track_option_string(
                    "manufacturer",
                    &old_component.manufacturer,
                    &component.manufacturer,
                );
                tracker.track_option_string("model", &old_component.model, &component.model);
                tracker.track_option_string("notes", &old_component.notes, &component.notes);

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "computer_id": id,
                        "component_type": component.component_type,
                        "component_id": component.id,
                        "name": component.name,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "computer",
                        id,
                        details,
                    );
                }

                ComputerComponentUpdateResponse::Ok(Json(component))
            }
            Err(e) => match e {
                AppError::ValidationError(_) => {
                    ComputerComponentUpdateResponse::BadRequest(Json(e.to_error_response()))
                }
                AppError::Conflict(_) => {
                    ComputerComponentUpdateResponse::Conflict(Json(e.to_error_response()))
                }
                AppError::NotFound(_) => {
                    ComputerComponentUpdateResponse::NotFound(Json(e.to_error_response()))
                }
                _ => ComputerComponentUpdateResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Delete a component of a computer
    #[oai(
        path = "/:id/components/:component_type/:component_id",
        method = "delete"
    )]
    async fn delete_computer_component(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Path(component_type): Path<String>,
        Path(component_id): Path<i32>,
    ) -> ComputerComponentDeleteResponse {
        let component = self
            .component_repository
            .get_component(id, &component_type, component_id)
            .await
            .ok();

        match self
            .component_repository
            .delete_component(id, &component_type, component_id)
            .await
        {
            Ok(0) => {
                let err = AppError::NotFound(format!(
                    "Component {component_type} with id {component_id} not found"
                ));
                ComputerComponentDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(component) = component {
                    let details = serde_json::json!({
                        "computer_id": id,
                        "component_type": component.component_type,
                        "component_id": component.id,
                        "name": component.name,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "component_removed",
                        "computer",
                        id,
                        details,
                    );
                }
                ComputerComponentDeleteResponse::NoContent
            }
            Err(e) => match e {
                AppError::ValidationError(_) => {
                    let err = e.to_error_response();
                    ComputerComponentDeleteResponse::NotFound(Json(err))
                }
                _ => ComputerComponentDeleteResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    // ------------------------------------------------------------------------
    // Установленное ПО
    // ------------------------------------------------------------------------

    /// Get software installed on a computer
    #[oai(path = "/:id/software", method = "get")]
    async fn list_computer_software(&self, Path(id): Path<i32>) -> ComputerSoftwareListApiResponse {
        if let Err(e) = self.repository.get_computer_by_id(id).await {
            return match e {
                AppError::NotFound(_) => {
                    ComputerSoftwareListApiResponse::NotFound(Json(e.to_error_response()))
                }
                _ => ComputerSoftwareListApiResponse::InternalError(Json(e.to_error_response())),
            };
        }

        match self.component_repository.list_software(id).await {
            Ok(software) => {
                let total = software.len() as i64;
                ComputerSoftwareListApiResponse::Ok(Json(ComputerSoftwareListResponse {
                    computer_id: id,
                    software,
                    total,
                }))
            }
            Err(e) => ComputerSoftwareListApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Install software on a computer
    #[oai(path = "/:id/software", method = "post")]
    async fn install_computer_software(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<NewInstalledSoftwareRequest>,
    ) -> ComputerSoftwareCreateResponse {
        match self.component_repository.install_software(id, req).await {
            Ok(software) => {
                let details = serde_json::json!({
                    "computer_id": id,
                    "software_id": software.software_id,
                    "name": software.name,
                    "version": software.version,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "software_installed",
                    "computer",
                    id,
                    details,
                );

                ComputerSoftwareCreateResponse::Created(Json(software))
            }
            Err(e) => match e {
                AppError::ValidationError(_) => {
                    ComputerSoftwareCreateResponse::BadRequest(Json(e.to_error_response()))
                }
                AppError::Conflict(_) => {
                    ComputerSoftwareCreateResponse::Conflict(Json(e.to_error_response()))
                }
                AppError::NotFound(_) => {
                    ComputerSoftwareCreateResponse::NotFound(Json(e.to_error_response()))
                }
                _ => ComputerSoftwareCreateResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Remove software from a computer
    #[oai(path = "/:id/software/:software_id", method = "delete")]
    async fn uninstall_computer_software(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Path(software_id): Path<i32>,
    ) -> ComputerSoftwareDeleteResponse {
        let installed = self
            .component_repository
            .list_software(id)
            .await
            .ok()
            .and_then(|list| list.into_iter().find(|item| item.id == software_id));

        match self
            .component_repository
            .uninstall_software(id, software_id)
            .await
        {
            Ok(0) => {
                let err = AppError::NotFound(format!("Software with id {software_id} not found"));
                ComputerSoftwareDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(software) = installed {
                    let details = serde_json::json!({
                        "computer_id": id,
                        "software_id": software.software_id,
                        "name": software.name,
                        "version": software.version,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "software_removed",
                        "computer",
                        id,
                        details,
                    );
                }
                ComputerSoftwareDeleteResponse::NoContent
            }
            Err(e) => ComputerSoftwareDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }
}
