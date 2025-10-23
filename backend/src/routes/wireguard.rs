use poem_openapi::{ApiResponse, OpenApi, param::Path, param::Query, payload::Json};
use std::sync::Arc;

use crate::config::database::Pool;
use crate::middleware;
use crate::models::wireguard::{
    CreateWireguardPeerRequest, UpdateWireguardPeer, WireguardConfigResponse,
    WireguardPeerListResponse, WireguardPeerResponse,
};
use crate::repositories::wireguard_repository::WireguardRepository;
use crate::routes::api::ApiTags;
use crate::services::activity_log_service::ActivityLogService;
use crate::services::wireguard_service::WireguardService;
use crate::utils::error::ErrorResponse;
use serde::Serialize;

// ============================================================================
// Response Types
// ============================================================================

#[derive(Serialize, poem_openapi::Object)]
pub struct WireguardInterfaceResponse {
    pub id: i32,
    pub name: String,
    pub listen_port: i32,
    pub public_key: String,
    pub mtu: Option<i32>,
    pub is_active: Option<bool>,
    pub mikrotik_id: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, poem_openapi::Object)]
pub struct WireguardDownloadHistoryItem {
    pub id: i32,
    pub peer_id: i32,
    pub downloaded_by: Option<String>,
    pub downloaded_at: String,
    pub ip_address: Option<String>,
}

#[derive(ApiResponse)]
pub enum WireguardInterfacesListResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<WireguardInterfaceResponse>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum WireguardDownloadHistoryResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<WireguardDownloadHistoryItem>>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(Serialize, poem_openapi::Object)]
pub struct MikrotikInterfaceResponse {
    pub id: String,
    pub name: String,
    pub listen_port: i32,
    pub public_key: String,
    pub mtu: Option<i32>,
    pub disabled: Option<bool>,
}

#[derive(Serialize, poem_openapi::Object)]
pub struct MikrotikPeerResponse {
    pub id: String,
    pub interface: String,
    pub public_key: String,
    pub allowed_address: String,
    pub endpoint_address: Option<String>,
    pub endpoint_port: Option<i32>,
    pub current_endpoint_address: Option<String>,
    pub current_endpoint_port: Option<i32>,
    pub last_handshake: Option<String>,
    pub rx: Option<i64>,
    pub tx: Option<i64>,
    pub comment: Option<String>,
    pub disabled: Option<bool>,
}

#[derive(ApiResponse)]
pub enum WireguardMikrotikInterfacesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<MikrotikInterfaceResponse>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum WireguardMikrotikInterfaceResponse {
    #[oai(status = 200)]
    Ok(Json<MikrotikInterfaceResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum WireguardMikrotikPeersResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<MikrotikPeerResponse>>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum WireguardPeersListResponse {
    #[oai(status = 200)]
    Ok(Json<WireguardPeerListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum WireguardPeerDetailResponse {
    #[oai(status = 200)]
    Ok(Json<WireguardPeerResponse>),
    #[oai(status = 201)]
    Created(Json<WireguardPeerResponse>),
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum WireguardConfigDownloadResponse {
    #[oai(status = 200)]
    Ok(Json<WireguardConfigResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// API Implementation
// ============================================================================

pub struct WireguardApi {
    repository: WireguardRepository,
    service: Arc<WireguardService>,
    activity_log: ActivityLogService,
    mikrotik_client: Option<Arc<crate::integrations::mikrotik::MikrotikClient>>,
}

impl WireguardApi {
    pub fn new(
        db_pool: Pool,
        service: Arc<WireguardService>,
        mikrotik_client: Option<Arc<crate::integrations::mikrotik::MikrotikClient>>,
    ) -> Self {
        Self {
            repository: WireguardRepository::new(db_pool.clone()),
            service,
            activity_log: ActivityLogService::new(db_pool),
            mikrotik_client,
        }
    }

    fn peer_to_response(
        peer: crate::models::wireguard::WireguardPeer,
        interface_name: Option<String>,
        employee_first_name: Option<String>,
        employee_last_name: Option<String>,
    ) -> WireguardPeerResponse {
        let employee_name = match (employee_first_name, employee_last_name) {
            (Some(first), Some(last)) => Some(format!("{} {}", last, first)),
            _ => None,
        };

        WireguardPeerResponse {
            id: peer.id,
            interface_id: peer.interface_id,
            interface_name,
            employee_id: peer.employee_id,
            employee_name,
            name: peer.name,
            public_key: peer.public_key,
            allowed_ips: peer.allowed_ips,
            client_address: peer.client_address,
            client_dns: peer.client_dns,
            endpoint_address: peer.endpoint_address,
            endpoint_port: peer.endpoint_port,
            persistent_keepalive: peer.persistent_keepalive,
            status: peer.status,
            last_handshake: peer.last_handshake.map(|dt| dt.to_string()),
            rx_bytes: peer.rx_bytes,
            tx_bytes: peer.tx_bytes,
            notes: peer.notes,
            created_at: peer.created_at.to_string(),
            updated_at: peer.updated_at.to_string(),
        }
    }
}

#[OpenApi(prefix_path = "/wireguard/peers", tag = "ApiTags::Wireguard")]
impl WireguardApi {
    /// Get all WireGuard peers with pagination
    #[oai(path = "/", method = "get")]
    async fn list_peers(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(interface_id): Query<Option<i32>>,
        Query(employee_id): Query<Option<i32>>,
        Query(status): Query<Option<String>>,
    ) -> WireguardPeersListResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self
            .repository
            .list_peers(page, per_page, interface_id, employee_id, status)
            .await
        {
            Ok((results, total)) => {
                let peers_list = results
                    .into_iter()
                    .map(|(peer, iface_name, emp_first, emp_last)| {
                        Self::peer_to_response(peer, iface_name, emp_first, emp_last)
                    })
                    .collect();

                WireguardPeersListResponse::Ok(Json(WireguardPeerListResponse {
                    peers: peers_list,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => WireguardPeersListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get WireGuard peer by ID
    #[oai(path = "/:id", method = "get")]
    async fn get_peer(&self, Path(id): Path<i32>) -> WireguardPeerDetailResponse {
        match self.repository.get_peer_by_id(id).await {
            Ok((peer, iface_name, emp_first, emp_last)) => {
                WireguardPeerDetailResponse::Ok(Json(Self::peer_to_response(
                    peer, iface_name, emp_first, emp_last,
                )))
            }
            Err(e) => match e {
                crate::utils::error::AppError::NotFound(_) => {
                    WireguardPeerDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => WireguardPeerDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new WireGuard peer
    #[oai(path = "/", method = "post")]
    async fn create_peer(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<CreateWireguardPeerRequest>,
    ) -> WireguardPeerDetailResponse {
        use crate::middleware::auth::ClaimsExt;

        match self.service.create_peer(req.clone(), auth.0.user_id()).await {
            Ok(peer_id) => {
                // Log activity
                let details = serde_json::json!({
                    "peer_name": req.name,
                    "interface_id": req.interface_id,
                    "employee_id": req.employee_id,
                    "client_address": req.client_address,
                });

                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "wireguard_peer",
                    peer_id,
                    details,
                );

                // Get created peer
                match self.repository.get_peer_by_id(peer_id).await {
                    Ok((peer, iface_name, emp_first, emp_last)) => {
                        WireguardPeerDetailResponse::Created(Json(Self::peer_to_response(
                            peer, iface_name, emp_first, emp_last,
                        )))
                    }
                    Err(e) => {
                        WireguardPeerDetailResponse::InternalError(Json(e.to_error_response()))
                    }
                }
            }
            Err(e) => WireguardPeerDetailResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update WireGuard peer
    #[oai(path = "/:id", method = "put")]
    async fn update_peer(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(update_data): Json<UpdateWireguardPeer>,
    ) -> WireguardPeerDetailResponse {
        use crate::middleware::auth::ClaimsExt;

        match self.service.update_peer(id, update_data).await {
            Ok(_) => {
                // Log activity
                self.activity_log.log_async(
                    auth.0.user_id(),
                    "updated",
                    "wireguard_peer",
                    id,
                );

                // Get updated peer
                match self.repository.get_peer_by_id(id).await {
                    Ok((peer, iface_name, emp_first, emp_last)) => {
                        WireguardPeerDetailResponse::Ok(Json(Self::peer_to_response(
                            peer, iface_name, emp_first, emp_last,
                        )))
                    }
                    Err(e) => {
                        WireguardPeerDetailResponse::InternalError(Json(e.to_error_response()))
                    }
                }
            }
            Err(e) => match e {
                crate::utils::error::AppError::NotFound(_) => {
                    WireguardPeerDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => WireguardPeerDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Delete WireGuard peer
    #[oai(path = "/:id", method = "delete")]
    async fn delete_peer(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> WireguardPeerDetailResponse {
        use crate::middleware::auth::ClaimsExt;

        match self.service.delete_peer(id).await {
            Ok(_) => {
                // Log activity
                self.activity_log.log_async(
                    auth.0.user_id(),
                    "deleted",
                    "wireguard_peer",
                    id,
                );

                WireguardPeerDetailResponse::NoContent
            }
            Err(e) => match e {
                crate::utils::error::AppError::NotFound(_) => {
                    WireguardPeerDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => WireguardPeerDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Download WireGuard configuration file
    #[oai(path = "/:id/config", method = "get")]
    async fn download_config(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> WireguardConfigDownloadResponse {
        use crate::middleware::auth::ClaimsExt;

        // Get peer and interface
        let (peer, _, _, _) = match self.repository.get_peer_by_id(id).await {
            Ok(data) => data,
            Err(e) => {
                return match e {
                    crate::utils::error::AppError::NotFound(_) => {
                        WireguardConfigDownloadResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => WireguardConfigDownloadResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let interface = match self.repository.get_interface_by_id(peer.interface_id).await {
            Ok(iface) => iface,
            Err(e) => {
                return WireguardConfigDownloadResponse::InternalError(Json(e.to_error_response()));
            }
        };

        // Generate config
        let server_endpoint = peer.endpoint_address.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("SERVER_IP");

        let config = self.service.generate_client_config(&peer, &interface, server_endpoint);

        // Log download
        let log = crate::models::wireguard::NewWireguardConfigDownload {
            peer_id: id,
            downloaded_by: auth.0.user_id(),
            ip_address: None,
            user_agent: None,
        };

        if let Err(e) = self.repository.log_config_download(log).await {
            tracing::warn!("Failed to log config download: {}", e);
        }

        // Log activity
        self.activity_log.log_async(
            auth.0.user_id(),
            "downloaded_config",
            "wireguard_peer",
            id,
        );

        WireguardConfigDownloadResponse::Ok(Json(WireguardConfigResponse {
            config,
            qr_code: None,
        }))
    }

    /// Sync peer statistics from MikroTik
    #[oai(path = "/:id/sync", method = "post")]
    async fn sync_peer_stats(
        &self,
        _auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> WireguardPeerDetailResponse {
        match self.service.sync_peer_stats(id).await {
            Ok(_) => {
                // Get updated peer
                match self.repository.get_peer_by_id(id).await {
                    Ok((peer, iface_name, emp_first, emp_last)) => {
                        WireguardPeerDetailResponse::Ok(Json(Self::peer_to_response(
                            peer, iface_name, emp_first, emp_last,
                        )))
                    }
                    Err(e) => {
                        WireguardPeerDetailResponse::InternalError(Json(e.to_error_response()))
                    }
                }
            }
            Err(e) => match e {
                crate::utils::error::AppError::NotFound(_) => {
                    WireguardPeerDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => WireguardPeerDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Get all WireGuard interfaces
    #[oai(path = "/interfaces", method = "get")]
    async fn list_interfaces(&self) -> WireguardInterfacesListResponse {
        match self.repository.list_interfaces().await {
            Ok(interfaces) => {
                let response: Vec<WireguardInterfaceResponse> = interfaces
                    .into_iter()
                    .map(|iface| WireguardInterfaceResponse {
                        id: iface.id,
                        name: iface.name,
                        listen_port: iface.listen_port,
                        public_key: iface.public_key,
                        mtu: iface.mtu,
                        is_active: iface.is_active,
                        mikrotik_id: iface.mikrotik_id,
                        notes: iface.notes,
                        created_at: iface.created_at.to_string(),
                        updated_at: iface.updated_at.to_string(),
                    })
                    .collect();

                WireguardInterfacesListResponse::Ok(Json(response))
            }
            Err(e) => WireguardInterfacesListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Sync WireGuard interfaces from MikroTik
    #[oai(path = "/interfaces/sync", method = "post")]
    async fn sync_interfaces(
        &self,
        auth: middleware::auth::AdminAuth,
    ) -> WireguardInterfacesListResponse {
        use crate::middleware::auth::ClaimsExt;

        match self.service.sync_interfaces().await {
            Ok(synced_count) => {
                tracing::info!("Synced {} interfaces from MikroTik", synced_count);

                // Log activity
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "synced_interfaces",
                    "wireguard_interface",
                    0,
                    serde_json::json!({ "synced_count": synced_count }),
                );

                // Return updated list
                match self.repository.list_interfaces().await {
                    Ok(interfaces) => {
                        let response: Vec<WireguardInterfaceResponse> = interfaces
                            .into_iter()
                            .map(|iface| WireguardInterfaceResponse {
                                id: iface.id,
                                name: iface.name,
                                listen_port: iface.listen_port,
                                public_key: iface.public_key,
                                mtu: iface.mtu,
                                is_active: iface.is_active,
                                mikrotik_id: iface.mikrotik_id,
                                notes: iface.notes,
                                created_at: iface.created_at.to_string(),
                                updated_at: iface.updated_at.to_string(),
                            })
                            .collect();

                        WireguardInterfacesListResponse::Ok(Json(response))
                    }
                    Err(e) => WireguardInterfacesListResponse::InternalError(Json(e.to_error_response())),
                }
            }
            Err(e) => WireguardInterfacesListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get config download history for a peer
    #[oai(path = "/:id/downloads", method = "get")]
    async fn get_download_history(
        &self,
        _auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> WireguardDownloadHistoryResponse {
        // Check if peer exists
        if let Err(e) = self.repository.get_peer_by_id(id).await {
            return match e {
                crate::utils::error::AppError::NotFound(_) => {
                    WireguardDownloadHistoryResponse::NotFound(Json(e.to_error_response()))
                }
                _ => WireguardDownloadHistoryResponse::InternalError(Json(e.to_error_response())),
            };
        }

        match self.repository.get_config_download_history(id).await {
            Ok(history) => {
                let response: Vec<WireguardDownloadHistoryItem> = history
                    .into_iter()
                    .map(|download| WireguardDownloadHistoryItem {
                        id: download.id,
                        peer_id: download.peer_id,
                        downloaded_by: download.downloaded_by.map(|u| u.to_string()),
                        downloaded_at: download.downloaded_at.to_string(),
                        ip_address: download.ip_address,
                    })
                    .collect();

                WireguardDownloadHistoryResponse::Ok(Json(response))
            }
            Err(e) => WireguardDownloadHistoryResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Sync all peers statistics from MikroTik
    #[oai(path = "/sync-all", method = "post")]
    async fn sync_all_peers(
        &self
    ) -> WireguardPeersListResponse {
        // Get all peers from database
        match self.repository.list_peers(1, 1000, None, None, None).await {
            Ok((results, _)) => {
                let mut synced_count = 0;
                
                for (peer, _, _, _) in results.iter() {
                    if let Ok(_) = self.service.sync_peer_stats(peer.id).await {
                        synced_count += 1;
                    }
                }

                tracing::info!("Synced {} peers statistics from MikroTik", synced_count);

                // Return updated list
                match self.repository.list_peers(1, 20, None, None, None).await {
                    Ok((results, total)) => {
                        let peers_list = results
                            .into_iter()
                            .map(|(peer, iface_name, emp_first, emp_last)| {
                                Self::peer_to_response(peer, iface_name, emp_first, emp_last)
                            })
                            .collect();

                        WireguardPeersListResponse::Ok(Json(WireguardPeerListResponse {
                            peers: peers_list,
                            total,
                            page: 1,
                            per_page: 20,
                        }))
                    }
                    Err(e) => WireguardPeersListResponse::InternalError(Json(e.to_error_response())),
                }
            }
            Err(e) => WireguardPeersListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Import peers from MikroTik
    #[oai(path = "/import-from-mikrotik", method = "post")]
    async fn import_peers_from_mikrotik(
        &self,
        auth: middleware::auth::AdminAuth,
    ) -> WireguardPeersListResponse {
        use crate::middleware::auth::ClaimsExt;

        match self.service.sync_peers_from_mikrotik().await {
            Ok(imported_count) => {
                tracing::info!("Imported {} peers from MikroTik", imported_count);

                // Log activity
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "imported_peers",
                    "wireguard_peer",
                    0,
                    serde_json::json!({ "imported_count": imported_count }),
                );

                // Return updated list
                match self.repository.list_peers(1, 20, None, None, None).await {
                    Ok((results, total)) => {
                        let peers_list = results
                            .into_iter()
                            .map(|(peer, iface_name, emp_first, emp_last)| {
                                Self::peer_to_response(peer, iface_name, emp_first, emp_last)
                            })
                            .collect();

                        WireguardPeersListResponse::Ok(Json(WireguardPeerListResponse {
                            peers: peers_list,
                            total,
                            page: 1,
                            per_page: 20,
                        }))
                    }
                    Err(e) => WireguardPeersListResponse::InternalError(Json(e.to_error_response())),
                }
            }
            Err(e) => WireguardPeersListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get WireGuard interfaces from MikroTik
    #[oai(path = "/mikrotik/interfaces", method = "get")]
    async fn get_mikrotik_interfaces(
        &self,
        _auth: middleware::auth::AdminAuth,
    ) -> WireguardMikrotikInterfacesResponse {
        if let Some(ref client) = self.mikrotik_client {
            match client.get_wireguard_interfaces().await {
                Ok(interfaces) => {
                    let response: Vec<MikrotikInterfaceResponse> = interfaces
                        .into_iter()
                        .map(|iface| MikrotikInterfaceResponse {
                            id: iface.id,
                            name: iface.name,
                            listen_port: iface.listen_port,
                            public_key: iface.public_key,
                            mtu: iface.mtu,
                            disabled: iface.disabled,
                        })
                        .collect();

                    WireguardMikrotikInterfacesResponse::Ok(Json(response))
                }
                Err(e) => WireguardMikrotikInterfacesResponse::InternalError(Json(e.to_error_response())),
            }
        } else {
            WireguardMikrotikInterfacesResponse::InternalError(Json(ErrorResponse {
                error: "mikrotik_disabled".to_string(),
                message: "MikroTik integration is not enabled".to_string(),
            }))
        }
    }

    /// Get WireGuard peers from MikroTik
    #[oai(path = "/mikrotik/peers", method = "get")]
    async fn get_mikrotik_peers(
        &self,
        _auth: middleware::auth::AdminAuth,
    ) -> WireguardMikrotikPeersResponse {
        if let Some(ref client) = self.mikrotik_client {
            match client.get_wireguard_peers().await {
                Ok(peers) => {
                    let response: Vec<MikrotikPeerResponse> = peers
                        .into_iter()
                        .map(|peer| MikrotikPeerResponse {
                            id: peer.id,
                            interface: peer.interface,
                            public_key: peer.public_key,
                            allowed_address: peer.allowed_address,
                            endpoint_address: peer.endpoint_address,
                            endpoint_port: peer.endpoint_port,
                            current_endpoint_address: peer.current_endpoint_address,
                            current_endpoint_port: peer.current_endpoint_port,
                            last_handshake: peer.last_handshake,
                            rx: peer.rx,
                            tx: peer.tx,
                            comment: peer.comment,
                            disabled: peer.disabled,
                        })
                        .collect();

                    WireguardMikrotikPeersResponse::Ok(Json(response))
                }
                Err(e) => WireguardMikrotikPeersResponse::InternalError(Json(e.to_error_response())),
            }
        } else {
            WireguardMikrotikPeersResponse::InternalError(Json(ErrorResponse {
                error: "mikrotik_disabled".to_string(),
                message: "MikroTik integration is not enabled".to_string(),
            }))
        }
    }

    /// Get specific WireGuard interface from MikroTik
    #[oai(path = "/mikrotik/interfaces/:id", method = "get")]
    async fn get_mikrotik_interface(
        &self,
        _auth: middleware::auth::AdminAuth,
        Path(id): Path<String>,
    ) -> WireguardMikrotikInterfaceResponse {
        if let Some(ref client) = self.mikrotik_client {
            match client.get_wireguard_interface(&id).await {
                Ok(iface) => {
                    let response = MikrotikInterfaceResponse {
                        id: iface.id,
                        name: iface.name,
                        listen_port: iface.listen_port,
                        public_key: iface.public_key,
                        mtu: iface.mtu,
                        disabled: iface.disabled,
                    };

                    WireguardMikrotikInterfaceResponse::Ok(Json(response))
                }
                Err(e) => match e {
                    crate::utils::error::AppError::NotFound(_) => {
                        WireguardMikrotikInterfaceResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => WireguardMikrotikInterfaceResponse::InternalError(Json(e.to_error_response())),
                },
            }
        } else {
            WireguardMikrotikInterfaceResponse::InternalError(Json(ErrorResponse {
                error: "mikrotik_disabled".to_string(),
                message: "MikroTik integration is not enabled".to_string(),
            }))
        }
    }
}
