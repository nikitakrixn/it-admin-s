use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::{
    ip_addresses, network_device_ports, network_devices, network_scans, network_segments,
};

// ============================================================================
// Network segments
// ============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = network_segments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NetworkSegment {
    pub id: i32,
    pub name: String,
    pub network_address: String,
    pub subnet_mask: String,
    pub gateway: Option<String>,
    pub vlan_id: Option<i32>,
    pub dns_servers: Option<String>,
    pub dhcp_enabled: Option<bool>,
    pub dhcp_range_start: Option<String>,
    pub dhcp_range_end: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = network_segments)]
pub struct NewNetworkSegment {
    pub name: String,
    pub network_address: String,
    pub subnet_mask: String,
    pub gateway: Option<String>,
    pub vlan_id: Option<i32>,
    pub dns_servers: Option<String>,
    pub dhcp_enabled: Option<bool>,
    pub dhcp_range_start: Option<String>,
    pub dhcp_range_end: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = network_segments)]
pub struct UpdateNetworkSegment {
    pub name: Option<String>,
    pub network_address: Option<String>,
    pub subnet_mask: Option<String>,
    pub gateway: Option<String>,
    pub vlan_id: Option<i32>,
    pub dns_servers: Option<String>,
    pub dhcp_enabled: Option<bool>,
    pub dhcp_range_start: Option<String>,
    pub dhcp_range_end: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewNetworkSegmentRequest {
    pub name: String,
    pub network_address: String,
    pub subnet_mask: String,
    pub gateway: Option<String>,
    pub vlan_id: Option<i32>,
    pub dns_servers: Option<String>,
    pub dhcp_enabled: Option<bool>,
    pub dhcp_range_start: Option<String>,
    pub dhcp_range_end: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
}

impl NewNetworkSegmentRequest {
    pub fn into_new_network_segment(self) -> NewNetworkSegment {
        NewNetworkSegment {
            name: self.name,
            network_address: self.network_address,
            subnet_mask: self.subnet_mask,
            gateway: self.gateway,
            vlan_id: self.vlan_id,
            dns_servers: self.dns_servers,
            dhcp_enabled: self.dhcp_enabled,
            dhcp_range_start: self.dhcp_range_start,
            dhcp_range_end: self.dhcp_range_end,
            description: self.description,
            location: self.location,
        }
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateNetworkSegmentRequest {
    pub name: Option<String>,
    pub network_address: Option<String>,
    pub subnet_mask: Option<String>,
    pub gateway: Option<String>,
    pub vlan_id: Option<i32>,
    pub dns_servers: Option<String>,
    pub dhcp_enabled: Option<bool>,
    pub dhcp_range_start: Option<String>,
    pub dhcp_range_end: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
}

impl UpdateNetworkSegmentRequest {
    pub fn into_update_network_segment(self) -> UpdateNetworkSegment {
        UpdateNetworkSegment {
            name: self.name,
            network_address: self.network_address,
            subnet_mask: self.subnet_mask,
            gateway: self.gateway,
            vlan_id: self.vlan_id,
            dns_servers: self.dns_servers,
            dhcp_enabled: self.dhcp_enabled,
            dhcp_range_start: self.dhcp_range_start,
            dhcp_range_end: self.dhcp_range_end,
            description: self.description,
            location: self.location,
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct NetworkSegmentResponse {
    pub id: i32,
    pub name: String,
    pub network_address: String,
    pub subnet_mask: String,
    pub gateway: Option<String>,
    pub vlan_id: Option<i32>,
    pub dns_servers: Option<String>,
    pub dhcp_enabled: Option<bool>,
    pub dhcp_range_start: Option<String>,
    pub dhcp_range_end: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<NetworkSegment> for NetworkSegmentResponse {
    fn from(segment: NetworkSegment) -> Self {
        Self {
            id: segment.id,
            name: segment.name,
            network_address: segment.network_address,
            subnet_mask: segment.subnet_mask,
            gateway: segment.gateway,
            vlan_id: segment.vlan_id,
            dns_servers: segment.dns_servers,
            dhcp_enabled: segment.dhcp_enabled,
            dhcp_range_start: segment.dhcp_range_start,
            dhcp_range_end: segment.dhcp_range_end,
            description: segment.description,
            location: segment.location,
            created_at: segment
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            updated_at: segment
                .updated_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct NetworkSegmentListResponse {
    pub segments: Vec<NetworkSegmentResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

// ============================================================================
// Network devices
// ============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = network_devices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NetworkDevice {
    pub id: i32,
    pub device_type: String,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub management_url: Option<String>,
    pub location: Option<String>,
    pub firmware_version: Option<String>,
    pub port_count: Option<i32>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = network_devices)]
pub struct NewNetworkDevice {
    pub device_type: String,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub management_url: Option<String>,
    pub location: Option<String>,
    pub firmware_version: Option<String>,
    pub port_count: Option<i32>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = network_devices)]
pub struct UpdateNetworkDevice {
    pub device_type: Option<String>,
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub management_url: Option<String>,
    pub location: Option<String>,
    pub firmware_version: Option<String>,
    pub port_count: Option<i32>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewNetworkDeviceRequest {
    pub device_type: String,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub management_url: Option<String>,
    pub location: Option<String>,
    pub firmware_version: Option<String>,
    pub port_count: Option<i32>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

impl NewNetworkDeviceRequest {
    pub fn into_new_network_device(self) -> NewNetworkDevice {
        NewNetworkDevice {
            device_type: self.device_type,
            name: self.name,
            manufacturer: self.manufacturer,
            model: self.model,
            serial_number: self.serial_number,
            ip_address: self.ip_address,
            mac_address: self.mac_address,
            management_url: self.management_url,
            location: self.location,
            firmware_version: self.firmware_version,
            port_count: self.port_count,
            status: self.status,
            notes: self.notes,
        }
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateNetworkDeviceRequest {
    pub device_type: Option<String>,
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub management_url: Option<String>,
    pub location: Option<String>,
    pub firmware_version: Option<String>,
    pub port_count: Option<i32>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

impl UpdateNetworkDeviceRequest {
    pub fn into_update_network_device(self) -> UpdateNetworkDevice {
        UpdateNetworkDevice {
            device_type: self.device_type,
            name: self.name,
            manufacturer: self.manufacturer,
            model: self.model,
            serial_number: self.serial_number,
            ip_address: self.ip_address,
            mac_address: self.mac_address,
            management_url: self.management_url,
            location: self.location,
            firmware_version: self.firmware_version,
            port_count: self.port_count,
            status: self.status,
            notes: self.notes,
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct NetworkDeviceResponse {
    pub id: i32,
    pub device_type: String,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub management_url: Option<String>,
    pub location: Option<String>,
    pub firmware_version: Option<String>,
    pub port_count: Option<i32>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<NetworkDevice> for NetworkDeviceResponse {
    fn from(device: NetworkDevice) -> Self {
        Self {
            id: device.id,
            device_type: device.device_type,
            name: device.name,
            manufacturer: device.manufacturer,
            model: device.model,
            serial_number: device.serial_number,
            ip_address: device.ip_address,
            mac_address: device.mac_address,
            management_url: device.management_url,
            location: device.location,
            firmware_version: device.firmware_version,
            port_count: device.port_count,
            status: device.status,
            notes: device.notes,
            created_at: device
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            updated_at: device
                .updated_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct NetworkDeviceListResponse {
    pub devices: Vec<NetworkDeviceResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

// ============================================================================
// IP addresses
// ============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = ip_addresses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IpAddress {
    pub id: i32,
    pub network_segment_id: Option<i32>,
    pub ip_address: String,
    pub status: Option<String>,
    pub assigned_to_type: Option<String>,
    pub assigned_to_id: Option<i32>,
    pub hostname: Option<String>,
    pub mac_address: Option<String>,
    pub is_reserved: Option<bool>,
    pub reservation_reason: Option<String>,
    pub last_seen: Option<NaiveDateTime>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = ip_addresses)]
pub struct NewIpAddress {
    pub network_segment_id: Option<i32>,
    pub ip_address: String,
    pub status: Option<String>,
    pub assigned_to_type: Option<String>,
    pub assigned_to_id: Option<i32>,
    pub hostname: Option<String>,
    pub mac_address: Option<String>,
    pub is_reserved: Option<bool>,
    pub reservation_reason: Option<String>,
    pub last_seen: Option<NaiveDateTime>,
    pub notes: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = ip_addresses)]
pub struct UpdateIpAddress {
    pub network_segment_id: Option<i32>,
    pub ip_address: Option<String>,
    pub status: Option<String>,
    pub assigned_to_type: Option<String>,
    pub assigned_to_id: Option<i32>,
    pub hostname: Option<String>,
    pub mac_address: Option<String>,
    pub is_reserved: Option<bool>,
    pub reservation_reason: Option<String>,
    pub last_seen: Option<NaiveDateTime>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewIpAddressRequest {
    pub network_segment_id: Option<i32>,
    pub ip_address: String,
    pub status: Option<String>,
    pub assigned_to_type: Option<String>,
    pub assigned_to_id: Option<i32>,
    pub hostname: Option<String>,
    pub mac_address: Option<String>,
    pub is_reserved: Option<bool>,
    pub reservation_reason: Option<String>,
    pub last_seen: Option<String>,
    pub notes: Option<String>,
}

impl NewIpAddressRequest {
    pub fn into_new_ip_address(self) -> Result<NewIpAddress, String> {
        let parse_datetime = |s: &Option<String>| -> Result<Option<NaiveDateTime>, String> {
            match s {
                Some(v) if !v.is_empty() => {
                    NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S%.3fZ")
                        .or_else(|_| NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%SZ"))
                        .map(Some)
                        .map_err(|e| format!("Invalid datetime format: {}", e))
                }
                _ => Ok(None),
            }
        };

        Ok(NewIpAddress {
            network_segment_id: self.network_segment_id,
            ip_address: self.ip_address,
            status: self.status,
            assigned_to_type: self.assigned_to_type,
            assigned_to_id: self.assigned_to_id,
            hostname: self.hostname,
            mac_address: self.mac_address,
            is_reserved: self.is_reserved,
            reservation_reason: self.reservation_reason,
            last_seen: parse_datetime(&self.last_seen)?,
            notes: self.notes,
        })
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateIpAddressRequest {
    pub network_segment_id: Option<i32>,
    pub ip_address: Option<String>,
    pub status: Option<String>,
    pub assigned_to_type: Option<String>,
    pub assigned_to_id: Option<i32>,
    pub hostname: Option<String>,
    pub mac_address: Option<String>,
    pub is_reserved: Option<bool>,
    pub reservation_reason: Option<String>,
    pub last_seen: Option<String>,
    pub notes: Option<String>,
}

impl UpdateIpAddressRequest {
    pub fn into_update_ip_address(self) -> Result<UpdateIpAddress, String> {
        let parse_datetime = |s: &Option<String>| -> Result<Option<NaiveDateTime>, String> {
            match s {
                Some(v) if !v.is_empty() => {
                    NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S%.3fZ")
                        .or_else(|_| NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%SZ"))
                        .map(Some)
                        .map_err(|e| format!("Invalid datetime format: {}", e))
                }
                _ => Ok(None),
            }
        };

        Ok(UpdateIpAddress {
            network_segment_id: self.network_segment_id,
            ip_address: self.ip_address,
            status: self.status,
            assigned_to_type: self.assigned_to_type,
            assigned_to_id: self.assigned_to_id,
            hostname: self.hostname,
            mac_address: self.mac_address,
            is_reserved: self.is_reserved,
            reservation_reason: self.reservation_reason,
            last_seen: parse_datetime(&self.last_seen)?,
            notes: self.notes,
        })
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct IpAddressResponse {
    pub id: i32,
    pub network_segment_id: Option<i32>,
    pub ip_address: String,
    pub status: Option<String>,
    pub assigned_to_type: Option<String>,
    pub assigned_to_id: Option<i32>,
    pub hostname: Option<String>,
    pub mac_address: Option<String>,
    pub is_reserved: Option<bool>,
    pub reservation_reason: Option<String>,
    pub last_seen: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<IpAddress> for IpAddressResponse {
    fn from(addr: IpAddress) -> Self {
        let fmt_dt =
            |dt: Option<NaiveDateTime>| dt.map(|d| d.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string());
        Self {
            id: addr.id,
            network_segment_id: addr.network_segment_id,
            ip_address: addr.ip_address,
            status: addr.status,
            assigned_to_type: addr.assigned_to_type,
            assigned_to_id: addr.assigned_to_id,
            hostname: addr.hostname,
            mac_address: addr.mac_address,
            is_reserved: addr.is_reserved,
            reservation_reason: addr.reservation_reason,
            last_seen: fmt_dt(addr.last_seen),
            notes: addr.notes,
            created_at: addr.created_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            updated_at: addr.updated_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct IpAddressListResponse {
    pub addresses: Vec<IpAddressResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

// ============================================================================
// Network scans
// ============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = network_scans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NetworkScan {
    pub id: i32,
    pub scan_type: String,
    pub network_segment_id: Option<i32>,
    pub ip_range: Option<String>,
    pub devices_found: Option<i32>,
    pub scan_duration: Option<i32>,
    pub scan_results: Option<serde_json::Value>,
    pub initiated_by: Option<uuid::Uuid>,
    pub started_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
    pub status: Option<String>,
    pub error_message: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = network_scans)]
pub struct NewNetworkScan {
    pub scan_type: String,
    pub network_segment_id: Option<i32>,
    pub ip_range: Option<String>,
    pub devices_found: Option<i32>,
    pub scan_duration: Option<i32>,
    pub scan_results: Option<serde_json::Value>,
    pub initiated_by: Option<uuid::Uuid>,
    pub started_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
    pub status: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = network_scans)]
pub struct UpdateNetworkScan {
    pub scan_type: Option<String>,
    pub network_segment_id: Option<i32>,
    pub ip_range: Option<String>,
    pub devices_found: Option<i32>,
    pub scan_duration: Option<i32>,
    pub scan_results: Option<serde_json::Value>,
    pub initiated_by: Option<uuid::Uuid>,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub status: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewNetworkScanRequest {
    pub scan_type: String,
    pub network_segment_id: Option<i32>,
    pub ip_range: Option<String>,
    pub devices_found: Option<i32>,
    pub scan_duration: Option<i32>,
    pub scan_results: Option<serde_json::Value>,
    pub initiated_by: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub status: Option<String>,
    pub error_message: Option<String>,
}

impl NewNetworkScanRequest {
    #[allow(deprecated)]
    pub fn into_new_network_scan(self) -> Result<NewNetworkScan, String> {
        let parse_uuid = |s: &Option<String>| -> Result<Option<uuid::Uuid>, String> {
            match s {
                Some(v) if !v.is_empty() => uuid::Uuid::parse_str(v)
                    .map(Some)
                    .map_err(|e| format!("Invalid UUID format: {}", e)),
                _ => Ok(None),
            }
        };
        let parse_datetime = |s: &Option<String>| -> Result<Option<NaiveDateTime>, String> {
            match s {
                Some(v) if !v.is_empty() => {
                    NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S%.3fZ")
                        .or_else(|_| NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%SZ"))
                        .map(Some)
                        .map_err(|e| format!("Invalid datetime format: {}", e))
                }
                _ => Ok(None),
            }
        };

        Ok(NewNetworkScan {
            scan_type: self.scan_type,
            network_segment_id: self.network_segment_id,
            ip_range: self.ip_range,
            devices_found: self.devices_found,
            scan_duration: self.scan_duration,
            scan_results: self.scan_results,
            initiated_by: parse_uuid(&self.initiated_by)?,
            started_at: parse_datetime(&self.started_at)?
                .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
            completed_at: parse_datetime(&self.completed_at)?,
            status: self.status,
            error_message: self.error_message,
        })
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct RunNetworkScanRequest {
    pub start_ip: String,
    pub end_ip: String,
    pub ports: Option<Vec<i32>>,
    pub scan_type: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateNetworkScanRequest {
    pub scan_type: Option<String>,
    pub network_segment_id: Option<i32>,
    pub ip_range: Option<String>,
    pub devices_found: Option<i32>,
    pub scan_duration: Option<i32>,
    pub scan_results: Option<serde_json::Value>,
    pub initiated_by: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub status: Option<String>,
    pub error_message: Option<String>,
}

impl UpdateNetworkScanRequest {
    pub fn into_update_network_scan(self) -> Result<UpdateNetworkScan, String> {
        let parse_uuid = |s: &Option<String>| -> Result<Option<uuid::Uuid>, String> {
            match s {
                Some(v) if !v.is_empty() => uuid::Uuid::parse_str(v)
                    .map(Some)
                    .map_err(|e| format!("Invalid UUID format: {}", e)),
                _ => Ok(None),
            }
        };
        let parse_datetime = |s: &Option<String>| -> Result<Option<NaiveDateTime>, String> {
            match s {
                Some(v) if !v.is_empty() => {
                    NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S%.3fZ")
                        .or_else(|_| NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%SZ"))
                        .map(Some)
                        .map_err(|e| format!("Invalid datetime format: {}", e))
                }
                _ => Ok(None),
            }
        };

        Ok(UpdateNetworkScan {
            scan_type: self.scan_type,
            network_segment_id: self.network_segment_id,
            ip_range: self.ip_range,
            devices_found: self.devices_found,
            scan_duration: self.scan_duration,
            scan_results: self.scan_results,
            initiated_by: parse_uuid(&self.initiated_by)?,
            started_at: parse_datetime(&self.started_at)?,
            completed_at: parse_datetime(&self.completed_at)?,
            status: self.status,
            error_message: self.error_message,
        })
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct NetworkScanResponse {
    pub id: i32,
    pub scan_type: String,
    pub network_segment_id: Option<i32>,
    pub ip_range: Option<String>,
    pub devices_found: Option<i32>,
    pub scan_duration: Option<i32>,
    pub scan_results: Option<serde_json::Value>,
    pub initiated_by: Option<String>,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub status: Option<String>,
    pub error_message: Option<String>,
    pub created_at: String,
}

impl From<NetworkScan> for NetworkScanResponse {
    fn from(scan: NetworkScan) -> Self {
        let fmt_dt =
            |dt: Option<NaiveDateTime>| dt.map(|d| d.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string());
        Self {
            id: scan.id,
            scan_type: scan.scan_type,
            network_segment_id: scan.network_segment_id,
            ip_range: scan.ip_range,
            devices_found: scan.devices_found,
            scan_duration: scan.scan_duration,
            scan_results: scan.scan_results,
            initiated_by: scan.initiated_by.map(|u| u.to_string()),
            started_at: scan.started_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            completed_at: fmt_dt(scan.completed_at),
            status: scan.status,
            error_message: scan.error_message,
            created_at: scan.created_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct NetworkScanListResponse {
    pub scans: Vec<NetworkScanResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

// ============================================================================
// Network device ports
// ============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = network_device_ports)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NetworkDevicePort {
    pub id: i32,
    pub device_id: i32,
    pub port_number: i32,
    pub port_name: Option<String>,
    pub port_type: Option<String>,
    pub speed: Option<String>,
    pub status: Option<String>,
    pub vlan_id: Option<i32>,
    pub connected_device_type: Option<String>,
    pub connected_device_id: Option<i32>,
    pub mac_address: Option<String>,
    pub description: Option<String>,
    pub is_uplink: Option<bool>,
    pub last_activity: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = network_device_ports)]
pub struct NewNetworkDevicePort {
    pub device_id: i32,
    pub port_number: i32,
    pub port_name: Option<String>,
    pub port_type: Option<String>,
    pub speed: Option<String>,
    pub status: Option<String>,
    pub vlan_id: Option<i32>,
    pub connected_device_type: Option<String>,
    pub connected_device_id: Option<i32>,
    pub mac_address: Option<String>,
    pub description: Option<String>,
    pub is_uplink: Option<bool>,
    pub last_activity: Option<NaiveDateTime>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = network_device_ports)]
pub struct UpdateNetworkDevicePort {
    pub device_id: Option<i32>,
    pub port_number: Option<i32>,
    pub port_name: Option<String>,
    pub port_type: Option<String>,
    pub speed: Option<String>,
    pub status: Option<String>,
    pub vlan_id: Option<i32>,
    pub connected_device_type: Option<String>,
    pub connected_device_id: Option<i32>,
    pub mac_address: Option<String>,
    pub description: Option<String>,
    pub is_uplink: Option<bool>,
    pub last_activity: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewNetworkDevicePortRequest {
    pub device_id: i32,
    pub port_number: i32,
    pub port_name: Option<String>,
    pub port_type: Option<String>,
    pub speed: Option<String>,
    pub status: Option<String>,
    pub vlan_id: Option<i32>,
    pub connected_device_type: Option<String>,
    pub connected_device_id: Option<i32>,
    pub mac_address: Option<String>,
    pub description: Option<String>,
    pub is_uplink: Option<bool>,
    pub last_activity: Option<String>,
}

impl NewNetworkDevicePortRequest {
    pub fn into_new_network_device_port(self) -> Result<NewNetworkDevicePort, String> {
        let parse_datetime = |s: &Option<String>| -> Result<Option<NaiveDateTime>, String> {
            match s {
                Some(v) if !v.is_empty() => {
                    NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S%.3fZ")
                        .or_else(|_| NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%SZ"))
                        .map(Some)
                        .map_err(|e| format!("Invalid datetime format: {}", e))
                }
                _ => Ok(None),
            }
        };

        Ok(NewNetworkDevicePort {
            device_id: self.device_id,
            port_number: self.port_number,
            port_name: self.port_name,
            port_type: self.port_type,
            speed: self.speed,
            status: self.status,
            vlan_id: self.vlan_id,
            connected_device_type: self.connected_device_type,
            connected_device_id: self.connected_device_id,
            mac_address: self.mac_address,
            description: self.description,
            is_uplink: self.is_uplink,
            last_activity: parse_datetime(&self.last_activity)?,
        })
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateNetworkDevicePortRequest {
    pub device_id: Option<i32>,
    pub port_number: Option<i32>,
    pub port_name: Option<String>,
    pub port_type: Option<String>,
    pub speed: Option<String>,
    pub status: Option<String>,
    pub vlan_id: Option<i32>,
    pub connected_device_type: Option<String>,
    pub connected_device_id: Option<i32>,
    pub mac_address: Option<String>,
    pub description: Option<String>,
    pub is_uplink: Option<bool>,
    pub last_activity: Option<String>,
}

impl UpdateNetworkDevicePortRequest {
    pub fn into_update_network_device_port(self) -> Result<UpdateNetworkDevicePort, String> {
        let parse_datetime = |s: &Option<String>| -> Result<Option<NaiveDateTime>, String> {
            match s {
                Some(v) if !v.is_empty() => {
                    NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S%.3fZ")
                        .or_else(|_| NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%SZ"))
                        .map(Some)
                        .map_err(|e| format!("Invalid datetime format: {}", e))
                }
                _ => Ok(None),
            }
        };

        Ok(UpdateNetworkDevicePort {
            device_id: self.device_id,
            port_number: self.port_number,
            port_name: self.port_name,
            port_type: self.port_type,
            speed: self.speed,
            status: self.status,
            vlan_id: self.vlan_id,
            connected_device_type: self.connected_device_type,
            connected_device_id: self.connected_device_id,
            mac_address: self.mac_address,
            description: self.description,
            is_uplink: self.is_uplink,
            last_activity: parse_datetime(&self.last_activity)?,
        })
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct NetworkDevicePortResponse {
    pub id: i32,
    pub device_id: i32,
    pub port_number: i32,
    pub port_name: Option<String>,
    pub port_type: Option<String>,
    pub speed: Option<String>,
    pub status: Option<String>,
    pub vlan_id: Option<i32>,
    pub connected_device_type: Option<String>,
    pub connected_device_id: Option<i32>,
    pub mac_address: Option<String>,
    pub description: Option<String>,
    pub is_uplink: Option<bool>,
    pub last_activity: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<NetworkDevicePort> for NetworkDevicePortResponse {
    fn from(port: NetworkDevicePort) -> Self {
        let fmt_dt =
            |dt: Option<NaiveDateTime>| dt.map(|d| d.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string());
        Self {
            id: port.id,
            device_id: port.device_id,
            port_number: port.port_number,
            port_name: port.port_name,
            port_type: port.port_type,
            speed: port.speed,
            status: port.status,
            vlan_id: port.vlan_id,
            connected_device_type: port.connected_device_type,
            connected_device_id: port.connected_device_id,
            mac_address: port.mac_address,
            description: port.description,
            is_uplink: port.is_uplink,
            last_activity: fmt_dt(port.last_activity),
            created_at: port.created_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            updated_at: port.updated_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct NetworkDevicePortListResponse {
    pub ports: Vec<NetworkDevicePortResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
