use poem_openapi::{ApiResponse, OpenApi, param::Path, param::Query, payload::Json};
use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

use crate::config::database::Pool;
use crate::middleware;
use crate::models::network::{
    IpAddressListResponse, IpAddressResponse, NetworkDeviceListResponse,
    NetworkDevicePortListResponse, NetworkDevicePortResponse, NetworkDeviceResponse,
    NetworkScanListResponse, NetworkScanResponse, NetworkSegmentListResponse,
    NetworkSegmentResponse, NewIpAddressRequest, NewNetworkDevicePortRequest,
    NewNetworkDeviceRequest, NewNetworkScan, NewNetworkScanRequest, NewNetworkSegmentRequest,
    RunNetworkScanRequest, UpdateIpAddressRequest, UpdateNetworkDevicePortRequest,
    UpdateNetworkDeviceRequest, UpdateNetworkScanRequest, UpdateNetworkSegmentRequest,
};
use crate::repositories::network_repository::{
    IpAddressRepository, NetworkDevicePortRepository, NetworkDeviceRepository,
    NetworkScanRepository, NetworkSegmentRepository,
};
use crate::routes::api::ApiTags;
use crate::services::activity_log_service::ActivityLogService;
use crate::services::network_scan_service;
use crate::utils::change_tracker::ChangeTracker;
use crate::utils::error::{AppError, ErrorResponse};

// ============================================================================
// Network segment response types
// ============================================================================

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkSegmentsListResponse {
    #[oai(status = 200)]
    Ok(Json<NetworkSegmentListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkSegmentDetailResponse {
    #[oai(status = 200)]
    Ok(Json<NetworkSegmentResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkSegmentCreateResponse {
    #[oai(status = 201)]
    Created(Json<NetworkSegmentResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkSegmentUpdateResponse {
    #[oai(status = 200)]
    Ok(Json<NetworkSegmentResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum NetworkSegmentDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// Network device response types
// ============================================================================

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkDevicesListResponse {
    #[oai(status = 200)]
    Ok(Json<NetworkDeviceListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkDeviceDetailResponse {
    #[oai(status = 200)]
    Ok(Json<NetworkDeviceResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkDeviceCreateResponse {
    #[oai(status = 201)]
    Created(Json<NetworkDeviceResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkDeviceUpdateResponse {
    #[oai(status = 200)]
    Ok(Json<NetworkDeviceResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum NetworkDeviceDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// IP address response types
// ============================================================================

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum IpAddressesListResponse {
    #[oai(status = 200)]
    Ok(Json<IpAddressListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum IpAddressDetailResponse {
    #[oai(status = 200)]
    Ok(Json<IpAddressResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum IpAddressCreateResponse {
    #[oai(status = 201)]
    Created(Json<IpAddressResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum IpAddressUpdateResponse {
    #[oai(status = 200)]
    Ok(Json<IpAddressResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum IpAddressDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// Network scan response types
// ============================================================================

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkScansListResponse {
    #[oai(status = 200)]
    Ok(Json<NetworkScanListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkScanDetailResponse {
    #[oai(status = 200)]
    Ok(Json<NetworkScanResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkScanCreateResponse {
    #[oai(status = 201)]
    Created(Json<NetworkScanResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkScanRunResponse {
    #[oai(status = 201)]
    Created(Json<NetworkScanResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkScanUpdateResponse {
    #[oai(status = 200)]
    Ok(Json<NetworkScanResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum NetworkScanDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// Network device port response types
// ============================================================================

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkDevicePortsListResponse {
    #[oai(status = 200)]
    Ok(Json<NetworkDevicePortListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkDevicePortDetailResponse {
    #[oai(status = 200)]
    Ok(Json<NetworkDevicePortResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkDevicePortCreateResponse {
    #[oai(status = 201)]
    Created(Json<NetworkDevicePortResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum NetworkDevicePortUpdateResponse {
    #[oai(status = 200)]
    Ok(Json<NetworkDevicePortResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum NetworkDevicePortDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// API implementation — single OpenApi block with prefix /network
// ============================================================================

pub struct NetworkApi {
    segment_repo: NetworkSegmentRepository,
    device_repo: NetworkDeviceRepository,
    ip_repo: IpAddressRepository,
    scan_repo: NetworkScanRepository,
    port_repo: NetworkDevicePortRepository,
    activity_log: ActivityLogService,
}

impl NetworkApi {
    pub fn new(db_pool: Pool) -> Self {
        Self {
            segment_repo: NetworkSegmentRepository::new(db_pool.clone()),
            device_repo: NetworkDeviceRepository::new(db_pool.clone()),
            ip_repo: IpAddressRepository::new(db_pool.clone()),
            scan_repo: NetworkScanRepository::new(db_pool.clone()),
            port_repo: NetworkDevicePortRepository::new(db_pool.clone()),
            activity_log: ActivityLogService::new(db_pool),
        }
    }
}

#[OpenApi(prefix_path = "/network", tag = "ApiTags::Network")]
impl NetworkApi {
    // ------------------------------------------------------------------------
    // Network segments — GET /network/segments
    // ------------------------------------------------------------------------
    /// Get all network segments with pagination
    #[oai(path = "/segments", method = "get")]
    async fn list_network_segments(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
    ) -> NetworkSegmentsListResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self
            .segment_repo
            .list_network_segments(page, per_page)
            .await
        {
            Ok((segments, total)) => {
                let responses: Vec<NetworkSegmentResponse> = segments
                    .into_iter()
                    .map(NetworkSegmentResponse::from)
                    .collect();

                NetworkSegmentsListResponse::Ok(Json(NetworkSegmentListResponse {
                    segments: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => NetworkSegmentsListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get network segment by ID
    #[oai(path = "/segments/:id", method = "get")]
    async fn get_network_segment(&self, Path(id): Path<i32>) -> NetworkSegmentDetailResponse {
        match self.segment_repo.get_network_segment_by_id(id).await {
            Ok(segment) => {
                NetworkSegmentDetailResponse::Ok(Json(NetworkSegmentResponse::from(segment)))
            }
            Err(e) => match e {
                AppError::NotFound(_) => {
                    NetworkSegmentDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => NetworkSegmentDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new network segment
    #[oai(path = "/segments", method = "post")]
    async fn create_network_segment(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewNetworkSegmentRequest>,
    ) -> NetworkSegmentCreateResponse {
        let new_segment = req.into_new_network_segment();

        match self.segment_repo.create_network_segment(new_segment).await {
            Ok(segment) => {
                let details = serde_json::json!({
                    "name": segment.name,
                    "network_address": segment.network_address,
                    "subnet_mask": segment.subnet_mask,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "network_segment",
                    segment.id,
                    details,
                );

                NetworkSegmentCreateResponse::Created(Json(NetworkSegmentResponse::from(segment)))
            }
            Err(e) => NetworkSegmentCreateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update network segment
    #[oai(path = "/segments/:id", method = "put")]
    async fn update_network_segment(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateNetworkSegmentRequest>,
    ) -> NetworkSegmentUpdateResponse {
        let old_segment = match self.segment_repo.get_network_segment_by_id(id).await {
            Ok(s) => s,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        NetworkSegmentUpdateResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => NetworkSegmentUpdateResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let update_data = req.into_update_network_segment();

        match self
            .segment_repo
            .update_network_segment(id, update_data)
            .await
        {
            Ok(segment) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("name", &old_segment.name, &segment.name);
                tracker.track_string(
                    "network_address",
                    &old_segment.network_address,
                    &segment.network_address,
                );
                tracker.track_string(
                    "subnet_mask",
                    &old_segment.subnet_mask,
                    &segment.subnet_mask,
                );
                tracker.track_option_string("gateway", &old_segment.gateway, &segment.gateway);
                tracker.track_option_string(
                    "dns_servers",
                    &old_segment.dns_servers,
                    &segment.dns_servers,
                );
                tracker.track_option_bool(
                    "dhcp_enabled",
                    &old_segment.dhcp_enabled,
                    &segment.dhcp_enabled,
                );

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "segment_name": segment.name,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "network_segment",
                        segment.id,
                        details,
                    );
                }

                NetworkSegmentUpdateResponse::Ok(Json(NetworkSegmentResponse::from(segment)))
            }
            Err(e) => NetworkSegmentUpdateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete network segment
    #[oai(path = "/segments/:id", method = "delete")]
    async fn delete_network_segment(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> NetworkSegmentDeleteResponse {
        let segment_info = self.segment_repo.get_network_segment_by_id(id).await.ok();

        match self.segment_repo.delete_network_segment(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("Network segment with id {} not found", id));
                NetworkSegmentDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(segment) = segment_info {
                    let details = serde_json::json!({
                        "name": segment.name,
                        "network_address": segment.network_address,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "network_segment",
                        id,
                        details,
                    );
                }
                NetworkSegmentDeleteResponse::NoContent
            }
            Err(e) => NetworkSegmentDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }

    // ------------------------------------------------------------------------
    // Network devices — GET /network/devices
    // ------------------------------------------------------------------------
    /// Get all network devices with pagination
    #[oai(path = "/devices", method = "get")]
    async fn list_network_devices(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
    ) -> NetworkDevicesListResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self.device_repo.list_network_devices(page, per_page).await {
            Ok((devices, total)) => {
                let responses: Vec<NetworkDeviceResponse> = devices
                    .into_iter()
                    .map(NetworkDeviceResponse::from)
                    .collect();

                NetworkDevicesListResponse::Ok(Json(NetworkDeviceListResponse {
                    devices: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => NetworkDevicesListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get network device by ID
    #[oai(path = "/devices/:id", method = "get")]
    async fn get_network_device(&self, Path(id): Path<i32>) -> NetworkDeviceDetailResponse {
        match self.device_repo.get_network_device_by_id(id).await {
            Ok(device) => {
                NetworkDeviceDetailResponse::Ok(Json(NetworkDeviceResponse::from(device)))
            }
            Err(e) => match e {
                AppError::NotFound(_) => {
                    NetworkDeviceDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => NetworkDeviceDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new network device
    #[oai(path = "/devices", method = "post")]
    async fn create_network_device(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewNetworkDeviceRequest>,
    ) -> NetworkDeviceCreateResponse {
        let new_device = req.into_new_network_device();

        match self.device_repo.create_network_device(new_device).await {
            Ok(device) => {
                let details = serde_json::json!({
                    "name": device.name,
                    "device_type": device.device_type,
                    "ip_address": device.ip_address,
                    "status": device.status,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "network_device",
                    device.id,
                    details,
                );

                NetworkDeviceCreateResponse::Created(Json(NetworkDeviceResponse::from(device)))
            }
            Err(e) => NetworkDeviceCreateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update network device
    #[oai(path = "/devices/:id", method = "put")]
    async fn update_network_device(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateNetworkDeviceRequest>,
    ) -> NetworkDeviceUpdateResponse {
        let old_device = match self.device_repo.get_network_device_by_id(id).await {
            Ok(d) => d,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        NetworkDeviceUpdateResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => NetworkDeviceUpdateResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let update_data = req.into_update_network_device();

        match self
            .device_repo
            .update_network_device(id, update_data)
            .await
        {
            Ok(device) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("name", &old_device.name, &device.name);
                tracker.track_string("device_type", &old_device.device_type, &device.device_type);
                tracker.track_option_string(
                    "ip_address",
                    &old_device.ip_address,
                    &device.ip_address,
                );
                tracker.track_option_string(
                    "mac_address",
                    &old_device.mac_address,
                    &device.mac_address,
                );
                tracker.track_option_string("status", &old_device.status, &device.status);
                tracker.track_option_string("location", &old_device.location, &device.location);

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "device_name": device.name,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "network_device",
                        device.id,
                        details,
                    );
                }

                NetworkDeviceUpdateResponse::Ok(Json(NetworkDeviceResponse::from(device)))
            }
            Err(e) => NetworkDeviceUpdateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete network device
    #[oai(path = "/devices/:id", method = "delete")]
    async fn delete_network_device(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> NetworkDeviceDeleteResponse {
        let device_info = self.device_repo.get_network_device_by_id(id).await.ok();

        match self.device_repo.delete_network_device(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("Network device with id {} not found", id));
                NetworkDeviceDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(device) = device_info {
                    let details = serde_json::json!({
                        "name": device.name,
                        "device_type": device.device_type,
                        "ip_address": device.ip_address,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "network_device",
                        id,
                        details,
                    );
                }
                NetworkDeviceDeleteResponse::NoContent
            }
            Err(e) => NetworkDeviceDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }

    // ------------------------------------------------------------------------
    // IP addresses — GET /network/ip-addresses
    // ------------------------------------------------------------------------
    /// Get all IP addresses with pagination
    #[oai(path = "/ip-addresses", method = "get")]
    async fn list_ip_addresses(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(segment_id): Query<Option<i32>>,
    ) -> IpAddressesListResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self
            .ip_repo
            .list_ip_addresses(page, per_page, segment_id)
            .await
        {
            Ok((addresses, total)) => {
                let responses: Vec<IpAddressResponse> =
                    addresses.into_iter().map(IpAddressResponse::from).collect();

                IpAddressesListResponse::Ok(Json(IpAddressListResponse {
                    addresses: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => IpAddressesListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get IP address by ID
    #[oai(path = "/ip-addresses/:id", method = "get")]
    async fn get_ip_address(&self, Path(id): Path<i32>) -> IpAddressDetailResponse {
        match self.ip_repo.get_ip_address_by_id(id).await {
            Ok(addr) => IpAddressDetailResponse::Ok(Json(IpAddressResponse::from(addr))),
            Err(e) => match e {
                AppError::NotFound(_) => {
                    IpAddressDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => IpAddressDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new IP address
    #[oai(path = "/ip-addresses", method = "post")]
    async fn create_ip_address(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewIpAddressRequest>,
    ) -> IpAddressCreateResponse {
        let new_addr = match req.into_new_ip_address() {
            Ok(a) => a,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return IpAddressCreateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.ip_repo.create_ip_address(new_addr).await {
            Ok(addr) => {
                let details = serde_json::json!({
                    "ip_address": addr.ip_address,
                    "status": addr.status,
                    "assigned_to_type": addr.assigned_to_type,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "ip_address",
                    addr.id,
                    details,
                );

                IpAddressCreateResponse::Created(Json(IpAddressResponse::from(addr)))
            }
            Err(e) => IpAddressCreateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update IP address
    #[oai(path = "/ip-addresses/:id", method = "put")]
    async fn update_ip_address(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateIpAddressRequest>,
    ) -> IpAddressUpdateResponse {
        let old_addr = match self.ip_repo.get_ip_address_by_id(id).await {
            Ok(a) => a,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        IpAddressUpdateResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => IpAddressUpdateResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let update_data = match req.into_update_ip_address() {
            Ok(d) => d,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return IpAddressUpdateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.ip_repo.update_ip_address(id, update_data).await {
            Ok(addr) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("ip_address", &old_addr.ip_address, &addr.ip_address);
                tracker.track_option_string("status", &old_addr.status, &addr.status);
                tracker.track_option_string("hostname", &old_addr.hostname, &addr.hostname);
                tracker.track_option_string(
                    "mac_address",
                    &old_addr.mac_address,
                    &addr.mac_address,
                );
                tracker.track_option_bool("is_reserved", &old_addr.is_reserved, &addr.is_reserved);
                tracker.track_option_i32(
                    "network_segment_id",
                    &old_addr.network_segment_id,
                    &addr.network_segment_id,
                );

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "ip_address": addr.ip_address,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "ip_address",
                        addr.id,
                        details,
                    );
                }

                IpAddressUpdateResponse::Ok(Json(IpAddressResponse::from(addr)))
            }
            Err(e) => IpAddressUpdateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete IP address
    #[oai(path = "/ip-addresses/:id", method = "delete")]
    async fn delete_ip_address(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> IpAddressDeleteResponse {
        let addr_info = self.ip_repo.get_ip_address_by_id(id).await.ok();

        match self.ip_repo.delete_ip_address(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("IP address with id {} not found", id));
                IpAddressDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(addr) = addr_info {
                    let details = serde_json::json!({
                        "ip_address": addr.ip_address,
                        "status": addr.status,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "ip_address",
                        id,
                        details,
                    );
                }
                IpAddressDeleteResponse::NoContent
            }
            Err(e) => IpAddressDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }

    // ------------------------------------------------------------------------
    // Network scans — GET /network/scans
    // ------------------------------------------------------------------------
    /// Get all network scans with pagination
    #[oai(path = "/scans", method = "get")]
    async fn list_network_scans(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
    ) -> NetworkScansListResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self.scan_repo.list_network_scans(page, per_page).await {
            Ok((scans, total)) => {
                let responses: Vec<NetworkScanResponse> =
                    scans.into_iter().map(NetworkScanResponse::from).collect();

                NetworkScansListResponse::Ok(Json(NetworkScanListResponse {
                    scans: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => NetworkScansListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get network scan by ID
    #[oai(path = "/scans/:id", method = "get")]
    async fn get_network_scan(&self, Path(id): Path<i32>) -> NetworkScanDetailResponse {
        match self.scan_repo.get_network_scan_by_id(id).await {
            Ok(scan) => NetworkScanDetailResponse::Ok(Json(NetworkScanResponse::from(scan))),
            Err(e) => match e {
                AppError::NotFound(_) => {
                    NetworkScanDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => NetworkScanDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new network scan
    #[oai(path = "/scans", method = "post")]
    async fn create_network_scan(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewNetworkScanRequest>,
    ) -> NetworkScanCreateResponse {
        let new_scan = match req.into_new_network_scan() {
            Ok(s) => s,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return NetworkScanCreateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.scan_repo.create_network_scan(new_scan).await {
            Ok(scan) => {
                let details = serde_json::json!({
                    "scan_type": scan.scan_type,
                    "ip_range": scan.ip_range,
                    "devices_found": scan.devices_found,
                    "status": scan.status,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "network_scan",
                    scan.id,
                    details,
                );

                NetworkScanCreateResponse::Created(Json(NetworkScanResponse::from(scan)))
            }
            Err(e) => NetworkScanCreateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Run a TCP scan over the range and store the result
    #[oai(path = "/scans/run", method = "post")]
    async fn run_network_scan(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<RunNetworkScanRequest>,
    ) -> NetworkScanRunResponse {
        let start_ip = match req.start_ip.trim().parse::<Ipv4Addr>() {
            Ok(ip) => ip,
            Err(_) => {
                let err = AppError::ValidationError("Некорректный начальный IP-адрес".to_string());
                return NetworkScanRunResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        let end_ip = match req.end_ip.trim().parse::<Ipv4Addr>() {
            Ok(ip) => ip,
            Err(_) => {
                let err = AppError::ValidationError("Некорректный конечный IP-адрес".to_string());
                return NetworkScanRunResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        let start_value = u32::from(start_ip);
        let end_value = u32::from(end_ip);
        if start_value > end_value {
            let err = AppError::ValidationError(
                "Начальный адрес должен быть меньше конечного".to_string(),
            );
            return NetworkScanRunResponse::BadRequest(Json(err.to_error_response()));
        }

        let address_count = end_value - start_value + 1;
        if address_count > 256 {
            let err = AppError::ValidationError(
                "За один раз можно проверить не больше 256 адресов".to_string(),
            );
            return NetworkScanRunResponse::BadRequest(Json(err.to_error_response()));
        }

        let mut ports: Vec<u16> = req
            .ports
            .unwrap_or_default()
            .into_iter()
            .filter(|port| (1..=65535).contains(port))
            .map(|port| port as u16)
            .collect();
        ports.sort_unstable();
        ports.dedup();
        if ports.is_empty() {
            ports = vec![22, 80, 443, 3389];
        }
        if ports.len() > 32 {
            let err = AppError::ValidationError(
                "За один раз можно проверить не больше 32 портов".to_string(),
            );
            return NetworkScanRunResponse::BadRequest(Json(err.to_error_response()));
        }

        let scan_type = req.scan_type.unwrap_or_else(|| "port_scan".to_string());
        let allowed_scan_types = ["ping_sweep", "port_scan", "full_scan", "arp_scan"];
        if !allowed_scan_types.contains(&scan_type.as_str()) {
            let err =
                AppError::ValidationError(format!("Неизвестный тип сканирования: {scan_type}"));
            return NetworkScanRunResponse::BadRequest(Json(err.to_error_response()));
        }

        let started_at = chrono::Utc::now().naive_utc();
        let started = Instant::now();
        let hosts = network_scan_service::scan_range(
            start_ip,
            end_ip,
            &ports,
            Duration::from_millis(400),
            256,
        )
        .await;
        let duration = started.elapsed().as_secs().min(i32::MAX as u64) as i32;
        let devices_found = hosts.len() as i32;

        use crate::middleware::auth::ClaimsExt;
        let new_scan = NewNetworkScan {
            scan_type,
            network_segment_id: None,
            ip_range: Some(format!("{start_ip}-{end_ip}")),
            devices_found: Some(devices_found),
            scan_duration: Some(duration),
            scan_results: Some(serde_json::json!({ "hosts": hosts })),
            initiated_by: auth.0.user_id(),
            started_at,
            completed_at: Some(chrono::Utc::now().naive_utc()),
            status: Some("completed".to_string()),
            error_message: None,
        };

        match self.scan_repo.create_network_scan(new_scan).await {
            Ok(scan) => {
                let details = serde_json::json!({
                    "scan_type": scan.scan_type,
                    "ip_range": scan.ip_range,
                    "devices_found": scan.devices_found,
                    "ports": ports,
                });

                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "scanned",
                    "network_scan",
                    scan.id,
                    details,
                );

                NetworkScanRunResponse::Created(Json(NetworkScanResponse::from(scan)))
            }
            Err(e) => NetworkScanRunResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update network scan
    #[oai(path = "/scans/:id", method = "put")]
    async fn update_network_scan(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateNetworkScanRequest>,
    ) -> NetworkScanUpdateResponse {
        let old_scan = match self.scan_repo.get_network_scan_by_id(id).await {
            Ok(s) => s,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        NetworkScanUpdateResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => NetworkScanUpdateResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let update_data = match req.into_update_network_scan() {
            Ok(d) => d,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return NetworkScanUpdateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.scan_repo.update_network_scan(id, update_data).await {
            Ok(scan) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("scan_type", &old_scan.scan_type, &scan.scan_type);
                tracker.track_option_string("status", &old_scan.status, &scan.status);
                tracker.track_option_string("ip_range", &old_scan.ip_range, &scan.ip_range);
                tracker.track_option_i32(
                    "devices_found",
                    &old_scan.devices_found,
                    &scan.devices_found,
                );

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "scan_type": scan.scan_type,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "network_scan",
                        scan.id,
                        details,
                    );
                }

                NetworkScanUpdateResponse::Ok(Json(NetworkScanResponse::from(scan)))
            }
            Err(e) => NetworkScanUpdateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete network scan
    #[oai(path = "/scans/:id", method = "delete")]
    async fn delete_network_scan(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> NetworkScanDeleteResponse {
        let scan_info = self.scan_repo.get_network_scan_by_id(id).await.ok();

        match self.scan_repo.delete_network_scan(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("Network scan with id {} not found", id));
                NetworkScanDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(scan) = scan_info {
                    let details = serde_json::json!({
                        "scan_type": scan.scan_type,
                        "status": scan.status,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "network_scan",
                        id,
                        details,
                    );
                }
                NetworkScanDeleteResponse::NoContent
            }
            Err(e) => NetworkScanDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }

    // ------------------------------------------------------------------------
    // Network device ports — GET /network/device-ports
    // ------------------------------------------------------------------------
    /// Get all network device ports with pagination
    #[oai(path = "/device-ports", method = "get")]
    async fn list_network_device_ports(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(device_id): Query<Option<i32>>,
    ) -> NetworkDevicePortsListResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self
            .port_repo
            .list_network_device_ports(page, per_page, device_id)
            .await
        {
            Ok((ports, total)) => {
                let responses: Vec<NetworkDevicePortResponse> = ports
                    .into_iter()
                    .map(NetworkDevicePortResponse::from)
                    .collect();

                NetworkDevicePortsListResponse::Ok(Json(NetworkDevicePortListResponse {
                    ports: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => NetworkDevicePortsListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get network device port by ID
    #[oai(path = "/device-ports/:id", method = "get")]
    async fn get_network_device_port(
        &self,
        Path(id): Path<i32>,
    ) -> NetworkDevicePortDetailResponse {
        match self.port_repo.get_network_device_port_by_id(id).await {
            Ok(port) => {
                NetworkDevicePortDetailResponse::Ok(Json(NetworkDevicePortResponse::from(port)))
            }
            Err(e) => match e {
                AppError::NotFound(_) => {
                    NetworkDevicePortDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => NetworkDevicePortDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new network device port
    #[oai(path = "/device-ports", method = "post")]
    async fn create_network_device_port(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewNetworkDevicePortRequest>,
    ) -> NetworkDevicePortCreateResponse {
        let new_port = match req.into_new_network_device_port() {
            Ok(p) => p,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return NetworkDevicePortCreateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.port_repo.create_network_device_port(new_port).await {
            Ok(port) => {
                let details = serde_json::json!({
                    "device_id": port.device_id,
                    "port_number": port.port_number,
                    "port_type": port.port_type,
                    "status": port.status,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "network_device_port",
                    port.id,
                    details,
                );

                NetworkDevicePortCreateResponse::Created(Json(NetworkDevicePortResponse::from(
                    port,
                )))
            }
            Err(e) => NetworkDevicePortCreateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update network device port
    #[oai(path = "/device-ports/:id", method = "put")]
    async fn update_network_device_port(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateNetworkDevicePortRequest>,
    ) -> NetworkDevicePortUpdateResponse {
        let old_port = match self.port_repo.get_network_device_port_by_id(id).await {
            Ok(p) => p,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        NetworkDevicePortUpdateResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => {
                        NetworkDevicePortUpdateResponse::InternalError(Json(e.to_error_response()))
                    }
                };
            }
        };

        let update_data = match req.into_update_network_device_port() {
            Ok(d) => d,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return NetworkDevicePortUpdateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self
            .port_repo
            .update_network_device_port(id, update_data)
            .await
        {
            Ok(port) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_option_string("port_name", &old_port.port_name, &port.port_name);
                tracker.track_option_string("port_type", &old_port.port_type, &port.port_type);
                tracker.track_option_string("status", &old_port.status, &port.status);
                tracker.track_option_string("speed", &old_port.speed, &port.speed);
                tracker.track_option_i32("vlan_id", &old_port.vlan_id, &port.vlan_id);
                tracker.track_option_bool("is_uplink", &old_port.is_uplink, &port.is_uplink);

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "device_id": port.device_id,
                        "port_number": port.port_number,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "network_device_port",
                        port.id,
                        details,
                    );
                }

                NetworkDevicePortUpdateResponse::Ok(Json(NetworkDevicePortResponse::from(port)))
            }
            Err(e) => NetworkDevicePortUpdateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete network device port
    #[oai(path = "/device-ports/:id", method = "delete")]
    async fn delete_network_device_port(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> NetworkDevicePortDeleteResponse {
        let port_info = self.port_repo.get_network_device_port_by_id(id).await.ok();

        match self.port_repo.delete_network_device_port(id).await {
            Ok(0) => {
                let err =
                    AppError::NotFound(format!("Network device port with id {} not found", id));
                NetworkDevicePortDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(port) = port_info {
                    let details = serde_json::json!({
                        "device_id": port.device_id,
                        "port_number": port.port_number,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "network_device_port",
                        id,
                        details,
                    );
                }
                NetworkDevicePortDeleteResponse::NoContent
            }
            Err(e) => NetworkDevicePortDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }
}
