use crate::config::database::Pool;
use crate::middleware;
use crate::models::monitoring::{
    ComputerMonitorListResponse, ComputerMonitorResponse, ComputerMonitoringListResponse,
    ComputerMonitoringResponse, NewComputerMonitorRequest, NewComputerMonitoringRequest,
    UpdateComputerMonitorRequest,
};
use crate::repositories::monitoring_repository::{
    ComputerMonitorRepository, ComputerMonitoringRepository,
};
use crate::routes::api::ApiTags;
use crate::services::activity_log_service::ActivityLogService;
use crate::utils::change_tracker::ChangeTracker;
use crate::utils::error::{AppError, ErrorResponse};
use poem_openapi::{ApiResponse, OpenApi, param::Path, param::Query, payload::Json};
use serde::Serialize;

// ============================================================================
// Computer monitoring metrics
// ============================================================================

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerMonitoringLatestResponse {
    #[oai(status = 200)]
    Ok(Json<ComputerMonitoringResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerMonitoringHistoryResponse {
    #[oai(status = 200)]
    Ok(Json<ComputerMonitoringListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerMonitoringCreateResponse {
    #[oai(status = 201)]
    Created(Json<ComputerMonitoringResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerMonitoringDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// Computer monitor response types
// ============================================================================

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerMonitorsListResponse {
    #[oai(status = 200)]
    Ok(Json<ComputerMonitorListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerMonitorDetailResponse {
    #[oai(status = 200)]
    Ok(Json<ComputerMonitorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerMonitorCreateResponse {
    #[oai(status = 201)]
    Created(Json<ComputerMonitorResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ComputerMonitorUpdateResponse {
    #[oai(status = 200)]
    Ok(Json<ComputerMonitorResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum ComputerMonitorDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// Dashboard summary response types
// ============================================================================

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum MonitoringDashboardResponse {
    #[oai(status = 200)]
    Ok(Json<MonitoringDashboardListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct MonitoringDashboardSummary {
    pub total_computers_monitored: i64,
    pub total_monitoring_records: i64,
    pub avg_cpu_usage: Option<f64>,
    pub avg_ram_usage_percent: Option<f64>,
    pub computers_with_high_cpu: i64,
    pub computers_with_high_ram: i64,
    pub last_collected_at: Option<String>,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct MonitoringDashboardListResponse {
    pub summary: MonitoringDashboardSummary,
    pub recent_monitoring: Vec<ComputerMonitoringResponse>,
}

// ============================================================================
// API implementation
// ============================================================================

pub struct MonitoringApi {
    monitoring_repo: ComputerMonitoringRepository,
    monitor_repo: ComputerMonitorRepository,
    activity_log: ActivityLogService,
}

impl MonitoringApi {
    pub fn new(db_pool: Pool) -> Self {
        Self {
            monitoring_repo: ComputerMonitoringRepository::new(db_pool.clone()),
            monitor_repo: ComputerMonitorRepository::new(db_pool.clone()),
            activity_log: ActivityLogService::new(db_pool),
        }
    }
}

#[OpenApi(prefix_path = "/monitoring", tag = "ApiTags::Monitoring")]
impl MonitoringApi {
    // ------------------------------------------------------------------------
    // Computer monitoring — latest metrics
    // ------------------------------------------------------------------------
    /// Get latest monitoring data for a computer
    #[oai(path = "/computers/:id/latest", method = "get")]
    async fn get_latest_monitoring(&self, Path(id): Path<i32>) -> ComputerMonitoringLatestResponse {
        match self.monitoring_repo.get_latest_monitoring(id).await {
            Ok(Some(monitoring)) => ComputerMonitoringLatestResponse::Ok(Json(
                ComputerMonitoringResponse::from(monitoring),
            )),
            Ok(None) => {
                let err =
                    AppError::NotFound(format!("No monitoring data for computer with id {}", id));
                ComputerMonitoringLatestResponse::NotFound(Json(err.to_error_response()))
            }
            Err(e) => ComputerMonitoringLatestResponse::InternalError(Json(e.to_error_response())),
        }
    }

    // ------------------------------------------------------------------------
    // Computer monitoring — history
    // ------------------------------------------------------------------------
    /// Get monitoring history for a computer
    #[oai(path = "/computers/:id/history", method = "get")]
    async fn list_monitoring_history(
        &self,
        Path(id): Path<i32>,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
    ) -> ComputerMonitoringHistoryResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self
            .monitoring_repo
            .list_monitoring_history(id, page, per_page)
            .await
        {
            Ok((monitoring_list, total)) => {
                let responses: Vec<ComputerMonitoringResponse> = monitoring_list
                    .into_iter()
                    .map(ComputerMonitoringResponse::from)
                    .collect();

                ComputerMonitoringHistoryResponse::Ok(Json(ComputerMonitoringListResponse {
                    monitoring: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => ComputerMonitoringHistoryResponse::InternalError(Json(e.to_error_response())),
        }
    }

    // ------------------------------------------------------------------------
    // Computer monitoring — create
    // ------------------------------------------------------------------------
    /// Create new monitoring data for a computer
    #[oai(path = "/computers/:id", method = "post")]
    async fn create_monitoring(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<NewComputerMonitoringRequest>,
    ) -> ComputerMonitoringCreateResponse {
        let mut new_monitoring = req.into_new_computer_monitoring();
        new_monitoring.computer_id = id;

        match self.monitoring_repo.create_monitoring(new_monitoring).await {
            Ok(monitoring) => {
                let details = serde_json::json!({
                    "computer_id": monitoring.computer_id,
                    "cpu_usage": monitoring.cpu_usage,
                    "ram_usage_percent": monitoring.ram_usage_percent,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "computer_monitoring",
                    monitoring.id,
                    details,
                );

                ComputerMonitoringCreateResponse::Created(Json(ComputerMonitoringResponse::from(
                    monitoring,
                )))
            }
            Err(e) => ComputerMonitoringCreateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    // ------------------------------------------------------------------------
    // Computer monitoring — delete
    // ------------------------------------------------------------------------
    /// Delete monitoring record
    #[oai(path = "/computers/:computer_id/history/:id", method = "delete")]
    async fn delete_monitoring(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(computer_id): Path<i32>,
        Path(id): Path<i32>,
    ) -> ComputerMonitoringDeleteResponse {
        match self.monitoring_repo.delete_monitoring(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("Monitoring record with id {} not found", id));
                ComputerMonitoringDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "deleted",
                    "computer_monitoring",
                    id,
                    serde_json::json!({ "id": id, "computer_id": computer_id }),
                );
                ComputerMonitoringDeleteResponse::NoContent
            }
            Err(e) => ComputerMonitoringDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }

    // ------------------------------------------------------------------------
    // Computer monitors (physical displays)
    // ------------------------------------------------------------------------
    /// Get all computer monitors with pagination
    #[oai(path = "/monitors", method = "get")]
    async fn list_computer_monitors(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(computer_id): Query<Option<i32>>,
    ) -> ComputerMonitorsListResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self
            .monitor_repo
            .list_computer_monitors(page, per_page, computer_id)
            .await
        {
            Ok((monitors, total)) => {
                let responses: Vec<ComputerMonitorResponse> = monitors
                    .into_iter()
                    .map(ComputerMonitorResponse::from)
                    .collect();

                ComputerMonitorsListResponse::Ok(Json(ComputerMonitorListResponse {
                    monitors: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => ComputerMonitorsListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get computer monitor by ID
    #[oai(path = "/monitors/:id", method = "get")]
    async fn get_computer_monitor(&self, Path(id): Path<i32>) -> ComputerMonitorDetailResponse {
        match self.monitor_repo.get_computer_monitor_by_id(id).await {
            Ok(monitor) => {
                ComputerMonitorDetailResponse::Ok(Json(ComputerMonitorResponse::from(monitor)))
            }
            Err(e) => match e {
                AppError::NotFound(_) => {
                    ComputerMonitorDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => ComputerMonitorDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new computer monitor
    #[oai(path = "/monitors", method = "post")]
    async fn create_computer_monitor(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewComputerMonitorRequest>,
    ) -> ComputerMonitorCreateResponse {
        let new_monitor = req.into_new_computer_monitor();

        match self.monitor_repo.create_computer_monitor(new_monitor).await {
            Ok(monitor) => {
                let details = serde_json::json!({
                    "computer_id": monitor.computer_id,
                    "name": monitor.name,
                    "manufacturer": monitor.manufacturer,
                    "model": monitor.model,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "computer_monitor",
                    monitor.id,
                    details,
                );

                ComputerMonitorCreateResponse::Created(Json(ComputerMonitorResponse::from(monitor)))
            }
            Err(e) => ComputerMonitorCreateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update computer monitor
    #[oai(path = "/monitors/:id", method = "put")]
    async fn update_computer_monitor(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateComputerMonitorRequest>,
    ) -> ComputerMonitorUpdateResponse {
        let old_monitor = match self.monitor_repo.get_computer_monitor_by_id(id).await {
            Ok(m) => m,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        ComputerMonitorUpdateResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => ComputerMonitorUpdateResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let update_data = req.into_update_computer_monitor();

        match self
            .monitor_repo
            .update_computer_monitor(id, update_data)
            .await
        {
            Ok(monitor) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("name", &old_monitor.name, &monitor.name);
                tracker.track_option_string(
                    "manufacturer",
                    &old_monitor.manufacturer,
                    &monitor.manufacturer,
                );
                tracker.track_option_string("model", &old_monitor.model, &monitor.model);
                tracker.track_option_string(
                    "serial_number",
                    &old_monitor.serial_number,
                    &monitor.serial_number,
                );
                tracker.track_option_string(
                    "resolution",
                    &old_monitor.resolution,
                    &monitor.resolution,
                );
                tracker.track_option_string(
                    "panel_type",
                    &old_monitor.panel_type,
                    &monitor.panel_type,
                );
                tracker.track_option_i32(
                    "refresh_rate",
                    &old_monitor.refresh_rate,
                    &monitor.refresh_rate,
                );
                tracker.track_option_bool(
                    "is_primary",
                    &old_monitor.is_primary,
                    &monitor.is_primary,
                );
                tracker.track_option_string("notes", &old_monitor.notes, &monitor.notes);

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "monitor_name": monitor.name,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "computer_monitor",
                        monitor.id,
                        details,
                    );
                }

                ComputerMonitorUpdateResponse::Ok(Json(ComputerMonitorResponse::from(monitor)))
            }
            Err(e) => ComputerMonitorUpdateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete computer monitor
    #[oai(path = "/monitors/:id", method = "delete")]
    async fn delete_computer_monitor(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> ComputerMonitorDeleteResponse {
        let monitor_info = self.monitor_repo.get_computer_monitor_by_id(id).await.ok();

        match self.monitor_repo.delete_computer_monitor(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("Computer monitor with id {} not found", id));
                ComputerMonitorDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(monitor) = monitor_info {
                    let details = serde_json::json!({
                        "name": monitor.name,
                        "computer_id": monitor.computer_id,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "computer_monitor",
                        id,
                        details,
                    );
                }
                ComputerMonitorDeleteResponse::NoContent
            }
            Err(e) => ComputerMonitorDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }

    // ------------------------------------------------------------------------
    // Monitoring dashboard — summary
    // ------------------------------------------------------------------------
    /// Get monitoring dashboard summary
    #[oai(path = "/dashboard", method = "get")]
    async fn get_dashboard(&self) -> MonitoringDashboardResponse {
        let aggregate = match self.monitoring_repo.get_dashboard_summary().await {
            Ok(aggregate) => aggregate,
            Err(e) => {
                return MonitoringDashboardResponse::InternalError(Json(e.to_error_response()));
            }
        };

        let recent_monitoring = match self.monitoring_repo.list_recent_monitoring(10).await {
            Ok(monitoring) => monitoring,
            Err(e) => {
                return MonitoringDashboardResponse::InternalError(Json(e.to_error_response()));
            }
        };

        let summary = MonitoringDashboardSummary {
            total_computers_monitored: aggregate.total_computers_monitored,
            total_monitoring_records: aggregate.total_monitoring_records,
            avg_cpu_usage: aggregate.avg_cpu_usage,
            avg_ram_usage_percent: aggregate.avg_ram_usage_percent,
            computers_with_high_cpu: aggregate.computers_with_high_cpu,
            computers_with_high_ram: aggregate.computers_with_high_ram,
            last_collected_at: aggregate
                .last_collected_at
                .map(|value| value.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()),
        };

        MonitoringDashboardResponse::Ok(Json(MonitoringDashboardListResponse {
            summary,
            recent_monitoring: recent_monitoring
                .into_iter()
                .map(ComputerMonitoringResponse::from)
                .collect(),
        }))
    }
}
