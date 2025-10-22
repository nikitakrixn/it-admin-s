use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::config::database::Pool;
use crate::models::schema::{employees, wireguard_config_downloads, wireguard_interfaces, wireguard_peers};
use crate::models::wireguard::{
    NewWireguardConfigDownload, NewWireguardPeer, UpdateWireguardPeer, WireguardConfigDownload,
    WireguardInterface, WireguardPeer,
};
use crate::utils::db_helpers::{DbResult, get_connection};

#[derive(Clone)]
pub struct WireguardRepository {
    pool: Pool,
}

impl WireguardRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Interface Operations
    // ========================================================================

    pub async fn list_interfaces(&self) -> DbResult<Vec<WireguardInterface>> {
        let mut conn = get_connection(&self.pool).await?;

        wireguard_interfaces::table
            .order(wireguard_interfaces::name.asc())
            .load::<WireguardInterface>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn get_interface_by_id(&self, id: i32) -> DbResult<WireguardInterface> {
        let mut conn = get_connection(&self.pool).await?;

        wireguard_interfaces::table
            .find(id)
            .first::<WireguardInterface>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn find_interface_by_mikrotik_id(&self, mikrotik_id: &str) -> DbResult<Option<WireguardInterface>> {
        let mut conn = get_connection(&self.pool).await?;

        wireguard_interfaces::table
            .filter(wireguard_interfaces::mikrotik_id.eq(mikrotik_id))
            .first::<WireguardInterface>(&mut conn)
            .await
            .optional()
            .map_err(Into::into)
    }

    pub async fn create_interface(&self, new_interface: crate::models::wireguard::NewWireguardInterface) -> DbResult<WireguardInterface> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::insert_into(wireguard_interfaces::table)
            .values(&new_interface)
            .get_result::<WireguardInterface>(&mut conn)
            .await
            .map_err(Into::into)
    }

    // ========================================================================
    // Peer Operations
    // ========================================================================

    pub async fn list_peers(
        &self,
        page: i64,
        per_page: i64,
        interface_id: Option<i32>,
        employee_id: Option<i32>,
        status: Option<String>,
    ) -> DbResult<(Vec<(WireguardPeer, Option<String>, Option<String>, Option<String>)>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        // Build count query
        let mut count_query = wireguard_peers::table.into_boxed();

        if let Some(iface_id) = interface_id {
            count_query = count_query.filter(wireguard_peers::interface_id.eq(iface_id));
        }

        if let Some(emp_id) = employee_id {
            count_query = count_query.filter(wireguard_peers::employee_id.eq(emp_id));
        }

        if let Some(ref s) = status {
            count_query = count_query.filter(wireguard_peers::status.eq(s));
        }

        let total = count_query.count().get_result::<i64>(&mut conn).await?;

        // Build data query with joins
        let mut data_query = wireguard_peers::table
            .left_join(wireguard_interfaces::table)
            .left_join(employees::table)
            .into_boxed();

        if let Some(iface_id) = interface_id {
            data_query = data_query.filter(wireguard_peers::interface_id.eq(iface_id));
        }

        if let Some(emp_id) = employee_id {
            data_query = data_query.filter(wireguard_peers::employee_id.eq(emp_id));
        }

        if let Some(s) = status {
            data_query = data_query.filter(wireguard_peers::status.eq(s));
        }

        let results = data_query
            .select((
                WireguardPeer::as_select(),
                wireguard_interfaces::name.nullable(),
                employees::first_name.nullable(),
                employees::last_name.nullable(),
            ))
            .order(wireguard_peers::created_at.desc())
            .limit(per_page)
            .offset(offset)
            .load::<(WireguardPeer, Option<String>, Option<String>, Option<String>)>(&mut conn)
            .await?;

        Ok((results, total))
    }

    pub async fn get_peer_by_id(&self, id: i32) -> DbResult<(WireguardPeer, Option<String>, Option<String>, Option<String>)> {
        let mut conn = get_connection(&self.pool).await?;

        wireguard_peers::table
            .find(id)
            .left_join(wireguard_interfaces::table)
            .left_join(employees::table)
            .select((
                WireguardPeer::as_select(),
                wireguard_interfaces::name.nullable(),
                employees::first_name.nullable(),
                employees::last_name.nullable(),
            ))
            .first::<(WireguardPeer, Option<String>, Option<String>, Option<String>)>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn find_peer_by_public_key(&self, public_key: &str) -> DbResult<Option<WireguardPeer>> {
        let mut conn = get_connection(&self.pool).await?;

        wireguard_peers::table
            .filter(wireguard_peers::public_key.eq(public_key))
            .first::<WireguardPeer>(&mut conn)
            .await
            .optional()
            .map_err(Into::into)
    }

    pub async fn create_peer(&self, new_peer: NewWireguardPeer) -> DbResult<WireguardPeer> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::insert_into(wireguard_peers::table)
            .values(&new_peer)
            .get_result::<WireguardPeer>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn update_peer(&self, id: i32, update_data: UpdateWireguardPeer) -> DbResult<WireguardPeer> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(wireguard_peers::table.find(id))
            .set(&update_data)
            .get_result::<WireguardPeer>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn delete_peer(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(wireguard_peers::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn update_peer_stats(
        &self,
        id: i32,
        rx_bytes: i64,
        tx_bytes: i64,
        last_handshake: Option<chrono::NaiveDateTime>,
    ) -> DbResult<WireguardPeer> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(wireguard_peers::table.find(id))
            .set((
                wireguard_peers::rx_bytes.eq(rx_bytes),
                wireguard_peers::tx_bytes.eq(tx_bytes),
                wireguard_peers::last_handshake.eq(last_handshake),
            ))
            .get_result::<WireguardPeer>(&mut conn)
            .await
            .map_err(Into::into)
    }

    // ========================================================================
    // Config Download Log
    // ========================================================================

    pub async fn log_config_download(&self, log: NewWireguardConfigDownload) -> DbResult<WireguardConfigDownload> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::insert_into(wireguard_config_downloads::table)
            .values(&log)
            .get_result::<WireguardConfigDownload>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn get_config_download_history(&self, peer_id: i32) -> DbResult<Vec<WireguardConfigDownload>> {
        let mut conn = get_connection(&self.pool).await?;

        wireguard_config_downloads::table
            .filter(wireguard_config_downloads::peer_id.eq(peer_id))
            .order(wireguard_config_downloads::downloaded_at.desc())
            .load::<WireguardConfigDownload>(&mut conn)
            .await
            .map_err(Into::into)
    }
}
