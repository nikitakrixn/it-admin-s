use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::schema::{wireguard_config_downloads, wireguard_interfaces, wireguard_peers};

// ============================================================================
// WireGuard Interface Models
// ============================================================================

#[derive(Queryable, Selectable, Serialize, Clone, Debug, Identifiable)]
#[diesel(table_name = wireguard_interfaces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WireguardInterface {
    pub id: i32,
    pub name: String,
    pub listen_port: i32,
    pub public_key: String,
    pub private_key: Option<String>,
    pub mtu: Option<i32>,
    pub is_active: Option<bool>,
    pub mikrotik_id: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, poem_openapi::Object)]
#[diesel(table_name = wireguard_interfaces)]
pub struct NewWireguardInterface {
    pub name: String,
    pub listen_port: i32,
    pub public_key: String,
    pub private_key: Option<String>,
    pub mtu: Option<i32>,
    pub is_active: Option<bool>,
    pub mikrotik_id: Option<String>,
    pub notes: Option<String>,
}

#[derive(AsChangeset, Deserialize, poem_openapi::Object)]
#[diesel(table_name = wireguard_interfaces)]
pub struct UpdateWireguardInterface {
    pub name: Option<String>,
    pub listen_port: Option<i32>,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
    pub mtu: Option<i32>,
    pub is_active: Option<bool>,
    pub mikrotik_id: Option<String>,
    pub notes: Option<String>,
}

// ============================================================================
// WireGuard Peer Models
// ============================================================================

#[derive(Queryable, Selectable, Serialize, Clone, Debug, Identifiable)]
#[diesel(table_name = wireguard_peers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WireguardPeer {
    pub id: i32,
    pub interface_id: i32,
    pub employee_id: Option<i32>,
    pub name: String,
    pub public_key: String,
    pub private_key: Option<String>,
    pub preshared_key: Option<String>,
    pub allowed_ips: String,
    pub client_address: String,
    pub client_dns: Option<String>,
    pub endpoint_address: Option<String>,
    pub endpoint_port: Option<i32>,
    pub persistent_keepalive: Option<i32>,
    pub mikrotik_peer_id: Option<String>,
    pub status: String,
    pub last_handshake: Option<NaiveDateTime>,
    pub rx_bytes: Option<i64>,
    pub tx_bytes: Option<i64>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: Option<Uuid>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = wireguard_peers)]
pub struct NewWireguardPeer {
    pub interface_id: i32,
    pub employee_id: Option<i32>,
    pub name: String,
    pub public_key: String,
    pub private_key: Option<String>,
    pub preshared_key: Option<String>,
    pub allowed_ips: String,
    pub client_address: String,
    pub client_dns: Option<String>,
    pub endpoint_address: Option<String>,
    pub endpoint_port: Option<i32>,
    pub persistent_keepalive: Option<i32>,
    pub mikrotik_peer_id: Option<String>,
    pub status: String,
    pub notes: Option<String>,
    pub created_by: Option<Uuid>,
}

#[derive(Deserialize, Clone, poem_openapi::Object)]
pub struct CreateWireguardPeerRequest {
    pub interface_id: i32,
    pub employee_id: Option<i32>,
    pub name: String,
    pub allowed_ips: Option<String>,
    pub client_address: String,
    pub client_dns: Option<String>,
    pub endpoint_address: Option<String>,
    pub endpoint_port: Option<i32>,
    pub persistent_keepalive: Option<i32>,
    pub notes: Option<String>,
}

#[derive(AsChangeset, Deserialize, poem_openapi::Object)]
#[diesel(table_name = wireguard_peers)]
pub struct UpdateWireguardPeer {
    pub employee_id: Option<i32>,
    pub name: Option<String>,
    pub allowed_ips: Option<String>,
    pub client_address: Option<String>,
    pub client_dns: Option<String>,
    pub endpoint_address: Option<String>,
    pub endpoint_port: Option<i32>,
    pub persistent_keepalive: Option<i32>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

// ============================================================================
// Response Models
// ============================================================================

#[derive(Serialize, poem_openapi::Object, Clone)]
pub struct WireguardPeerResponse {
    pub id: i32,
    pub interface_id: i32,
    pub interface_name: Option<String>,
    pub employee_id: Option<i32>,
    pub employee_name: Option<String>,
    pub name: String,
    pub public_key: String,
    pub allowed_ips: String,
    pub client_address: String,
    pub client_dns: Option<String>,
    pub endpoint_address: Option<String>,
    pub endpoint_port: Option<i32>,
    pub persistent_keepalive: Option<i32>,
    pub status: String,
    pub last_handshake: Option<String>,
    pub rx_bytes: Option<i64>,
    pub tx_bytes: Option<i64>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, poem_openapi::Object)]
pub struct WireguardPeerListResponse {
    pub peers: Vec<WireguardPeerResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Serialize, poem_openapi::Object)]
pub struct WireguardConfigResponse {
    pub config: String,
    pub qr_code: Option<String>,
}

// ============================================================================
// Config Download Log
// ============================================================================

#[derive(Queryable, Selectable, Serialize, Clone, Debug)]
#[diesel(table_name = wireguard_config_downloads)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WireguardConfigDownload {
    pub id: i32,
    pub peer_id: i32,
    pub downloaded_by: Option<Uuid>,
    pub downloaded_at: NaiveDateTime,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = wireguard_config_downloads)]
pub struct NewWireguardConfigDownload {
    pub peer_id: i32,
    pub downloaded_by: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}
