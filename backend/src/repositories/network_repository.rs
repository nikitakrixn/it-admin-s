use crate::config::database::Pool;
use crate::models::network::{
    IpAddress, NetworkDevice, NetworkDevicePort, NetworkScan, NetworkSegment, NewIpAddress,
    NewNetworkDevice, NewNetworkDevicePort, NewNetworkScan, NewNetworkSegment, UpdateIpAddress,
    UpdateNetworkDevice, UpdateNetworkDevicePort, UpdateNetworkScan, UpdateNetworkSegment,
};
use crate::models::schema::{
    ip_addresses, network_device_ports, network_devices, network_scans, network_segments,
};
use crate::utils::db_helpers::{DbResult, get_connection};
use crate::utils::error::AppError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

// ============================================================================
// Network segment operations
// ============================================================================

type NetworkSegmentRow = (
    i32,
    String,
    String,
    String,
    Option<String>,
    Option<i32>,
    Option<String>,
    Option<bool>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    NaiveDateTime,
    NaiveDateTime,
);

fn map_network_segment(row: NetworkSegmentRow) -> NetworkSegment {
    let (
        id,
        name,
        network_address,
        subnet_mask,
        gateway,
        vlan_id,
        dns_servers,
        dhcp_enabled,
        dhcp_range_start,
        dhcp_range_end,
        description,
        location,
        created_at,
        updated_at,
    ) = row;
    NetworkSegment {
        id,
        name,
        network_address,
        subnet_mask,
        gateway,
        vlan_id,
        dns_servers,
        dhcp_enabled,
        dhcp_range_start,
        dhcp_range_end,
        description,
        location,
        created_at,
        updated_at,
    }
}

pub struct NetworkSegmentRepository {
    pool: Pool,
}

impl NetworkSegmentRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn list_network_segments(
        &self,
        page: i64,
        per_page: i64,
    ) -> DbResult<(Vec<NetworkSegment>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let total = network_segments::table
            .count()
            .get_result::<i64>(&mut conn)
            .await?;

        let results = network_segments::table
            .select((
                network_segments::id,
                network_segments::name,
                network_segments::network_address,
                network_segments::subnet_mask,
                network_segments::gateway,
                network_segments::vlan_id,
                network_segments::dns_servers,
                network_segments::dhcp_enabled,
                network_segments::dhcp_range_start,
                network_segments::dhcp_range_end,
                network_segments::description,
                network_segments::location,
                network_segments::created_at,
                network_segments::updated_at,
            ))
            .order(network_segments::name.asc())
            .limit(per_page)
            .offset(offset)
            .load::<NetworkSegmentRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_network_segment)
            .collect();

        Ok((results, total))
    }

    pub async fn get_network_segment_by_id(&self, id: i32) -> DbResult<NetworkSegment> {
        let mut conn = get_connection(&self.pool).await?;

        let row = network_segments::table
            .select((
                network_segments::id,
                network_segments::name,
                network_segments::network_address,
                network_segments::subnet_mask,
                network_segments::gateway,
                network_segments::vlan_id,
                network_segments::dns_servers,
                network_segments::dhcp_enabled,
                network_segments::dhcp_range_start,
                network_segments::dhcp_range_end,
                network_segments::description,
                network_segments::location,
                network_segments::created_at,
                network_segments::updated_at,
            ))
            .filter(network_segments::id.eq(id))
            .first::<NetworkSegmentRow>(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(map_network_segment(row))
    }

    pub async fn create_network_segment(
        &self,
        new_segment: NewNetworkSegment,
    ) -> DbResult<NetworkSegment> {
        let mut conn = get_connection(&self.pool).await?;

        let new_id = diesel::insert_into(network_segments::table)
            .values(&new_segment)
            .returning(network_segments::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_network_segment_by_id(new_id).await
    }

    pub async fn update_network_segment(
        &self,
        id: i32,
        update_data: UpdateNetworkSegment,
    ) -> DbResult<NetworkSegment> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(network_segments::table.find(id))
            .set(&update_data)
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_network_segment_by_id(id).await
    }

    pub async fn delete_network_segment(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(network_segments::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)
    }
}

// ============================================================================
// Network device operations
// ============================================================================

type NetworkDeviceRow = (
    i32,
    String,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<i32>,
    Option<String>,
    Option<String>,
    NaiveDateTime,
    NaiveDateTime,
);

fn map_network_device(row: NetworkDeviceRow) -> NetworkDevice {
    let (
        id,
        device_type,
        name,
        manufacturer,
        model,
        serial_number,
        ip_address,
        mac_address,
        management_url,
        location,
        firmware_version,
        port_count,
        status,
        notes,
        created_at,
        updated_at,
    ) = row;
    NetworkDevice {
        id,
        device_type,
        name,
        manufacturer,
        model,
        serial_number,
        ip_address,
        mac_address,
        management_url,
        location,
        firmware_version,
        port_count,
        status,
        notes,
        created_at,
        updated_at,
    }
}

pub struct NetworkDeviceRepository {
    pool: Pool,
}

impl NetworkDeviceRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn list_network_devices(
        &self,
        page: i64,
        per_page: i64,
    ) -> DbResult<(Vec<NetworkDevice>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let total = network_devices::table
            .count()
            .get_result::<i64>(&mut conn)
            .await?;

        let results = network_devices::table
            .select((
                network_devices::id,
                network_devices::device_type,
                network_devices::name,
                network_devices::manufacturer,
                network_devices::model,
                network_devices::serial_number,
                network_devices::ip_address,
                network_devices::mac_address,
                network_devices::management_url,
                network_devices::location,
                network_devices::firmware_version,
                network_devices::port_count,
                network_devices::status,
                network_devices::notes,
                network_devices::created_at,
                network_devices::updated_at,
            ))
            .order(network_devices::name.asc())
            .limit(per_page)
            .offset(offset)
            .load::<NetworkDeviceRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_network_device)
            .collect();

        Ok((results, total))
    }

    pub async fn get_network_device_by_id(&self, id: i32) -> DbResult<NetworkDevice> {
        let mut conn = get_connection(&self.pool).await?;

        let row = network_devices::table
            .select((
                network_devices::id,
                network_devices::device_type,
                network_devices::name,
                network_devices::manufacturer,
                network_devices::model,
                network_devices::serial_number,
                network_devices::ip_address,
                network_devices::mac_address,
                network_devices::management_url,
                network_devices::location,
                network_devices::firmware_version,
                network_devices::port_count,
                network_devices::status,
                network_devices::notes,
                network_devices::created_at,
                network_devices::updated_at,
            ))
            .filter(network_devices::id.eq(id))
            .first::<NetworkDeviceRow>(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(map_network_device(row))
    }

    pub async fn create_network_device(
        &self,
        new_device: NewNetworkDevice,
    ) -> DbResult<NetworkDevice> {
        let mut conn = get_connection(&self.pool).await?;

        let new_id = diesel::insert_into(network_devices::table)
            .values(&new_device)
            .returning(network_devices::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_network_device_by_id(new_id).await
    }

    pub async fn update_network_device(
        &self,
        id: i32,
        update_data: UpdateNetworkDevice,
    ) -> DbResult<NetworkDevice> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(network_devices::table.find(id))
            .set(&update_data)
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_network_device_by_id(id).await
    }

    pub async fn delete_network_device(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(network_devices::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)
    }
}

// ============================================================================
// IP address operations
// ============================================================================

type IpAddressRow = (
    i32,
    Option<i32>,
    String,
    Option<String>,
    Option<String>,
    Option<i32>,
    Option<String>,
    Option<String>,
    Option<bool>,
    Option<String>,
    Option<NaiveDateTime>,
    Option<String>,
    NaiveDateTime,
    NaiveDateTime,
);

fn map_ip_address(row: IpAddressRow) -> IpAddress {
    let (
        id,
        network_segment_id,
        ip_address,
        status,
        assigned_to_type,
        assigned_to_id,
        hostname,
        mac_address,
        is_reserved,
        reservation_reason,
        last_seen,
        notes,
        created_at,
        updated_at,
    ) = row;
    IpAddress {
        id,
        network_segment_id,
        ip_address,
        status,
        assigned_to_type,
        assigned_to_id,
        hostname,
        mac_address,
        is_reserved,
        reservation_reason,
        last_seen,
        notes,
        created_at,
        updated_at,
    }
}

pub struct IpAddressRepository {
    pool: Pool,
}

impl IpAddressRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn list_ip_addresses(
        &self,
        page: i64,
        per_page: i64,
        segment_id: Option<i32>,
    ) -> DbResult<(Vec<IpAddress>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let mut count_query = ip_addresses::table.into_boxed();
        if let Some(sid) = segment_id {
            count_query = count_query.filter(ip_addresses::network_segment_id.eq(sid));
        }

        let total = count_query.count().get_result::<i64>(&mut conn).await?;

        let mut query = ip_addresses::table
            .select((
                ip_addresses::id,
                ip_addresses::network_segment_id,
                ip_addresses::ip_address,
                ip_addresses::status,
                ip_addresses::assigned_to_type,
                ip_addresses::assigned_to_id,
                ip_addresses::hostname,
                ip_addresses::mac_address,
                ip_addresses::is_reserved,
                ip_addresses::reservation_reason,
                ip_addresses::last_seen,
                ip_addresses::notes,
                ip_addresses::created_at,
                ip_addresses::updated_at,
            ))
            .into_boxed();

        if let Some(sid) = segment_id {
            query = query.filter(ip_addresses::network_segment_id.eq(sid));
        }

        let results = query
            .order(ip_addresses::ip_address.asc())
            .limit(per_page)
            .offset(offset)
            .load::<IpAddressRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_ip_address)
            .collect();

        Ok((results, total))
    }

    pub async fn get_ip_address_by_id(&self, id: i32) -> DbResult<IpAddress> {
        let mut conn = get_connection(&self.pool).await?;

        let row = ip_addresses::table
            .select((
                ip_addresses::id,
                ip_addresses::network_segment_id,
                ip_addresses::ip_address,
                ip_addresses::status,
                ip_addresses::assigned_to_type,
                ip_addresses::assigned_to_id,
                ip_addresses::hostname,
                ip_addresses::mac_address,
                ip_addresses::is_reserved,
                ip_addresses::reservation_reason,
                ip_addresses::last_seen,
                ip_addresses::notes,
                ip_addresses::created_at,
                ip_addresses::updated_at,
            ))
            .filter(ip_addresses::id.eq(id))
            .first::<IpAddressRow>(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(map_ip_address(row))
    }

    pub async fn create_ip_address(&self, new_addr: NewIpAddress) -> DbResult<IpAddress> {
        let mut conn = get_connection(&self.pool).await?;

        let new_id = diesel::insert_into(ip_addresses::table)
            .values(&new_addr)
            .returning(ip_addresses::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_ip_address_by_id(new_id).await
    }

    pub async fn update_ip_address(
        &self,
        id: i32,
        update_data: UpdateIpAddress,
    ) -> DbResult<IpAddress> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(ip_addresses::table.find(id))
            .set(&update_data)
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_ip_address_by_id(id).await
    }

    pub async fn delete_ip_address(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(ip_addresses::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)
    }
}

// ============================================================================
// Network scan operations
// ============================================================================

type NetworkScanRow = (
    i32,
    String,
    Option<i32>,
    Option<String>,
    Option<i32>,
    Option<i32>,
    Option<serde_json::Value>,
    Option<Uuid>,
    NaiveDateTime,
    Option<NaiveDateTime>,
    Option<String>,
    Option<String>,
    NaiveDateTime,
);

fn map_network_scan(row: NetworkScanRow) -> NetworkScan {
    let (
        id,
        scan_type,
        network_segment_id,
        ip_range,
        devices_found,
        scan_duration,
        scan_results,
        initiated_by,
        started_at,
        completed_at,
        status,
        error_message,
        created_at,
    ) = row;
    NetworkScan {
        id,
        scan_type,
        network_segment_id,
        ip_range,
        devices_found,
        scan_duration,
        scan_results,
        initiated_by,
        started_at,
        completed_at,
        status,
        error_message,
        created_at,
    }
}

pub struct NetworkScanRepository {
    pool: Pool,
}

impl NetworkScanRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn list_network_scans(
        &self,
        page: i64,
        per_page: i64,
    ) -> DbResult<(Vec<NetworkScan>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let total = network_scans::table
            .count()
            .get_result::<i64>(&mut conn)
            .await?;

        let results = network_scans::table
            .select((
                network_scans::id,
                network_scans::scan_type,
                network_scans::network_segment_id,
                network_scans::ip_range,
                network_scans::devices_found,
                network_scans::scan_duration,
                network_scans::scan_results,
                network_scans::initiated_by,
                network_scans::started_at,
                network_scans::completed_at,
                network_scans::status,
                network_scans::error_message,
                network_scans::created_at,
            ))
            .order(network_scans::created_at.desc())
            .limit(per_page)
            .offset(offset)
            .load::<NetworkScanRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_network_scan)
            .collect();

        Ok((results, total))
    }

    pub async fn get_network_scan_by_id(&self, id: i32) -> DbResult<NetworkScan> {
        let mut conn = get_connection(&self.pool).await?;

        let row = network_scans::table
            .select((
                network_scans::id,
                network_scans::scan_type,
                network_scans::network_segment_id,
                network_scans::ip_range,
                network_scans::devices_found,
                network_scans::scan_duration,
                network_scans::scan_results,
                network_scans::initiated_by,
                network_scans::started_at,
                network_scans::completed_at,
                network_scans::status,
                network_scans::error_message,
                network_scans::created_at,
            ))
            .filter(network_scans::id.eq(id))
            .first::<NetworkScanRow>(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(map_network_scan(row))
    }

    pub async fn create_network_scan(&self, new_scan: NewNetworkScan) -> DbResult<NetworkScan> {
        let mut conn = get_connection(&self.pool).await?;

        let new_id = diesel::insert_into(network_scans::table)
            .values(&new_scan)
            .returning(network_scans::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_network_scan_by_id(new_id).await
    }

    pub async fn update_network_scan(
        &self,
        id: i32,
        update_data: UpdateNetworkScan,
    ) -> DbResult<NetworkScan> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(network_scans::table.find(id))
            .set(&update_data)
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_network_scan_by_id(id).await
    }

    pub async fn delete_network_scan(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(network_scans::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)
    }
}

// ============================================================================
// Network device port operations
// ============================================================================

type NetworkDevicePortRow = (
    i32,
    i32,
    i32,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<i32>,
    Option<String>,
    Option<i32>,
    Option<String>,
    Option<String>,
    Option<bool>,
    Option<NaiveDateTime>,
    NaiveDateTime,
    NaiveDateTime,
);

fn map_network_device_port(row: NetworkDevicePortRow) -> NetworkDevicePort {
    let (
        id,
        device_id,
        port_number,
        port_name,
        port_type,
        speed,
        status,
        vlan_id,
        connected_device_type,
        connected_device_id,
        mac_address,
        description,
        is_uplink,
        last_activity,
        created_at,
        updated_at,
    ) = row;
    NetworkDevicePort {
        id,
        device_id,
        port_number,
        port_name,
        port_type,
        speed,
        status,
        vlan_id,
        connected_device_type,
        connected_device_id,
        mac_address,
        description,
        is_uplink,
        last_activity,
        created_at,
        updated_at,
    }
}

pub struct NetworkDevicePortRepository {
    pool: Pool,
}

impl NetworkDevicePortRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn list_network_device_ports(
        &self,
        page: i64,
        per_page: i64,
        device_id: Option<i32>,
    ) -> DbResult<(Vec<NetworkDevicePort>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let mut count_query = network_device_ports::table.into_boxed();
        if let Some(did) = device_id {
            count_query = count_query.filter(network_device_ports::device_id.eq(did));
        }

        let total = count_query.count().get_result::<i64>(&mut conn).await?;

        let mut query = network_device_ports::table
            .select((
                network_device_ports::id,
                network_device_ports::device_id,
                network_device_ports::port_number,
                network_device_ports::port_name,
                network_device_ports::port_type,
                network_device_ports::speed,
                network_device_ports::status,
                network_device_ports::vlan_id,
                network_device_ports::connected_device_type,
                network_device_ports::connected_device_id,
                network_device_ports::mac_address,
                network_device_ports::description,
                network_device_ports::is_uplink,
                network_device_ports::last_activity,
                network_device_ports::created_at,
                network_device_ports::updated_at,
            ))
            .into_boxed();

        if let Some(did) = device_id {
            query = query.filter(network_device_ports::device_id.eq(did));
        }

        let results = query
            .order(network_device_ports::port_number.asc())
            .limit(per_page)
            .offset(offset)
            .load::<NetworkDevicePortRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_network_device_port)
            .collect();

        Ok((results, total))
    }

    pub async fn get_network_device_port_by_id(&self, id: i32) -> DbResult<NetworkDevicePort> {
        let mut conn = get_connection(&self.pool).await?;

        let row = network_device_ports::table
            .select((
                network_device_ports::id,
                network_device_ports::device_id,
                network_device_ports::port_number,
                network_device_ports::port_name,
                network_device_ports::port_type,
                network_device_ports::speed,
                network_device_ports::status,
                network_device_ports::vlan_id,
                network_device_ports::connected_device_type,
                network_device_ports::connected_device_id,
                network_device_ports::mac_address,
                network_device_ports::description,
                network_device_ports::is_uplink,
                network_device_ports::last_activity,
                network_device_ports::created_at,
                network_device_ports::updated_at,
            ))
            .filter(network_device_ports::id.eq(id))
            .first::<NetworkDevicePortRow>(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(map_network_device_port(row))
    }

    pub async fn create_network_device_port(
        &self,
        new_port: NewNetworkDevicePort,
    ) -> DbResult<NetworkDevicePort> {
        let mut conn = get_connection(&self.pool).await?;

        let new_id = diesel::insert_into(network_device_ports::table)
            .values(&new_port)
            .returning(network_device_ports::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_network_device_port_by_id(new_id).await
    }

    pub async fn update_network_device_port(
        &self,
        id: i32,
        update_data: UpdateNetworkDevicePort,
    ) -> DbResult<NetworkDevicePort> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(network_device_ports::table.find(id))
            .set(&update_data)
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_network_device_port_by_id(id).await
    }

    pub async fn delete_network_device_port(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(network_device_ports::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)
    }
}
