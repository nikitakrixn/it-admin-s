use crate::config::database::Pool;
use crate::models::monitoring::{
    ComputerMonitor, ComputerMonitoring, NewComputerMonitor, NewComputerMonitoring,
    UpdateComputerMonitor,
};
use crate::models::schema::{computer_monitoring, computer_monitors};
use crate::utils::db_helpers::{DbResult, get_connection};
use crate::utils::error::AppError;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Double, Nullable, Timestamp};
use diesel_async::RunQueryDsl;

pub struct ComputerMonitoringRepository {
    pub pool: Pool,
}

impl ComputerMonitoringRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_latest_monitoring(
        &self,
        computer_id: i32,
    ) -> DbResult<Option<ComputerMonitoring>> {
        let mut conn = get_connection(&self.pool).await?;

        let row = computer_monitoring::table
            .select((
                computer_monitoring::id,
                computer_monitoring::computer_id,
                computer_monitoring::cpu_usage,
                computer_monitoring::ram_usage_percent,
                computer_monitoring::ram_total_gb,
                computer_monitoring::ram_used_gb,
                computer_monitoring::disk_usage,
                computer_monitoring::cpu_temperature,
                computer_monitoring::gpu_temperature,
                computer_monitoring::uptime_seconds,
                computer_monitoring::network_stats,
                computer_monitoring::processes_count,
                computer_monitoring::logged_users,
                computer_monitoring::collected_at,
            ))
            .filter(computer_monitoring::computer_id.eq(computer_id))
            .order(computer_monitoring::collected_at.desc())
            .first::<ComputerMonitoringRow>(&mut conn)
            .await
            .optional()
            .map_err(AppError::from)?;

        Ok(row.map(map_computer_monitoring))
    }

    pub async fn list_monitoring_history(
        &self,
        computer_id: i32,
        page: i64,
        per_page: i64,
    ) -> DbResult<(Vec<ComputerMonitoring>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let total = computer_monitoring::table
            .filter(computer_monitoring::computer_id.eq(computer_id))
            .count()
            .get_result::<i64>(&mut conn)
            .await?;

        let results = computer_monitoring::table
            .select((
                computer_monitoring::id,
                computer_monitoring::computer_id,
                computer_monitoring::cpu_usage,
                computer_monitoring::ram_usage_percent,
                computer_monitoring::ram_total_gb,
                computer_monitoring::ram_used_gb,
                computer_monitoring::disk_usage,
                computer_monitoring::cpu_temperature,
                computer_monitoring::gpu_temperature,
                computer_monitoring::uptime_seconds,
                computer_monitoring::network_stats,
                computer_monitoring::processes_count,
                computer_monitoring::logged_users,
                computer_monitoring::collected_at,
            ))
            .filter(computer_monitoring::computer_id.eq(computer_id))
            .order(computer_monitoring::collected_at.desc())
            .limit(per_page)
            .offset(offset)
            .load::<ComputerMonitoringRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_computer_monitoring)
            .collect();

        Ok((results, total))
    }

    pub async fn create_monitoring(
        &self,
        new_monitoring: NewComputerMonitoring,
    ) -> DbResult<ComputerMonitoring> {
        let mut conn = get_connection(&self.pool).await?;

        let new_id = diesel::insert_into(computer_monitoring::table)
            .values((
                computer_monitoring::computer_id.eq(new_monitoring.computer_id),
                computer_monitoring::cpu_usage.eq(new_monitoring.cpu_usage),
                computer_monitoring::ram_usage_percent.eq(new_monitoring.ram_usage_percent),
                computer_monitoring::ram_total_gb.eq(new_monitoring.ram_total_gb),
                computer_monitoring::ram_used_gb.eq(new_monitoring.ram_used_gb),
                computer_monitoring::disk_usage.eq(new_monitoring.disk_usage),
                computer_monitoring::cpu_temperature.eq(new_monitoring.cpu_temperature),
                computer_monitoring::gpu_temperature.eq(new_monitoring.gpu_temperature),
                computer_monitoring::uptime_seconds.eq(new_monitoring.uptime_seconds),
                computer_monitoring::network_stats.eq(new_monitoring.network_stats),
                computer_monitoring::processes_count.eq(new_monitoring.processes_count),
                computer_monitoring::logged_users.eq(new_monitoring.logged_users),
                computer_monitoring::collected_at.eq(new_monitoring.collected_at),
            ))
            .returning(computer_monitoring::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_monitoring_by_id(new_id).await
    }

    pub async fn get_monitoring_by_id(&self, id: i32) -> DbResult<ComputerMonitoring> {
        let mut conn = get_connection(&self.pool).await?;

        let row = computer_monitoring::table
            .select((
                computer_monitoring::id,
                computer_monitoring::computer_id,
                computer_monitoring::cpu_usage,
                computer_monitoring::ram_usage_percent,
                computer_monitoring::ram_total_gb,
                computer_monitoring::ram_used_gb,
                computer_monitoring::disk_usage,
                computer_monitoring::cpu_temperature,
                computer_monitoring::gpu_temperature,
                computer_monitoring::uptime_seconds,
                computer_monitoring::network_stats,
                computer_monitoring::processes_count,
                computer_monitoring::logged_users,
                computer_monitoring::collected_at,
            ))
            .filter(computer_monitoring::id.eq(id))
            .first::<ComputerMonitoringRow>(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(map_computer_monitoring(row))
    }

    pub async fn delete_monitoring(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(computer_monitoring::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)
    }

    pub async fn get_dashboard_summary(&self) -> DbResult<MonitoringAggregateRow> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::sql_query(
            r#"
            WITH latest AS (
                SELECT DISTINCT ON (computer_id)
                    computer_id,
                    cpu_usage,
                    ram_usage_percent,
                    collected_at
                FROM computer_monitoring
                ORDER BY computer_id, collected_at DESC
            )
            SELECT
                COUNT(*)::bigint AS total_computers_monitored,
                (SELECT COUNT(*) FROM computer_monitoring)::bigint AS total_monitoring_records,
                AVG(cpu_usage)::float8 AS avg_cpu_usage,
                AVG(ram_usage_percent)::float8 AS avg_ram_usage_percent,
                COUNT(*) FILTER (WHERE cpu_usage > 80)::bigint AS computers_with_high_cpu,
                COUNT(*) FILTER (WHERE ram_usage_percent > 80)::bigint AS computers_with_high_ram,
                MAX(collected_at) AS last_collected_at
            FROM latest
            "#,
        )
        .get_result::<MonitoringAggregateRow>(&mut conn)
        .await
        .map_err(AppError::from)
    }

    pub async fn list_recent_monitoring(&self, limit: i64) -> DbResult<Vec<ComputerMonitoring>> {
        let mut conn = get_connection(&self.pool).await?;

        let results = computer_monitoring::table
            .select((
                computer_monitoring::id,
                computer_monitoring::computer_id,
                computer_monitoring::cpu_usage,
                computer_monitoring::ram_usage_percent,
                computer_monitoring::ram_total_gb,
                computer_monitoring::ram_used_gb,
                computer_monitoring::disk_usage,
                computer_monitoring::cpu_temperature,
                computer_monitoring::gpu_temperature,
                computer_monitoring::uptime_seconds,
                computer_monitoring::network_stats,
                computer_monitoring::processes_count,
                computer_monitoring::logged_users,
                computer_monitoring::collected_at,
            ))
            .order(computer_monitoring::collected_at.desc())
            .limit(limit)
            .load::<ComputerMonitoringRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_computer_monitoring)
            .collect();

        Ok(results)
    }
}

#[derive(Debug, QueryableByName)]
pub struct MonitoringAggregateRow {
    #[diesel(sql_type = BigInt)]
    pub total_computers_monitored: i64,
    #[diesel(sql_type = BigInt)]
    pub total_monitoring_records: i64,
    #[diesel(sql_type = Nullable<Double>)]
    pub avg_cpu_usage: Option<f64>,
    #[diesel(sql_type = Nullable<Double>)]
    pub avg_ram_usage_percent: Option<f64>,
    #[diesel(sql_type = BigInt)]
    pub computers_with_high_cpu: i64,
    #[diesel(sql_type = BigInt)]
    pub computers_with_high_ram: i64,
    #[diesel(sql_type = Nullable<Timestamp>)]
    pub last_collected_at: Option<NaiveDateTime>,
}

type ComputerMonitoringRow = (
    i32,
    i32,
    Option<BigDecimal>,
    Option<BigDecimal>,
    Option<i32>,
    Option<BigDecimal>,
    Option<serde_json::Value>,
    Option<BigDecimal>,
    Option<BigDecimal>,
    Option<i64>,
    Option<serde_json::Value>,
    Option<i32>,
    Option<serde_json::Value>,
    NaiveDateTime,
);

fn map_computer_monitoring(row: ComputerMonitoringRow) -> ComputerMonitoring {
    let (
        id,
        computer_id,
        cpu_usage,
        ram_usage_percent,
        ram_total_gb,
        ram_used_gb,
        disk_usage,
        cpu_temperature,
        gpu_temperature,
        uptime_seconds,
        network_stats,
        processes_count,
        logged_users,
        collected_at,
    ) = row;
    ComputerMonitoring {
        id,
        computer_id,
        cpu_usage,
        ram_usage_percent,
        ram_total_gb,
        ram_used_gb,
        disk_usage,
        cpu_temperature,
        gpu_temperature,
        uptime_seconds,
        network_stats,
        processes_count,
        logged_users,
        collected_at,
    }
}

pub struct ComputerMonitorRepository {
    pub pool: Pool,
}

impl ComputerMonitorRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn list_computer_monitors(
        &self,
        page: i64,
        per_page: i64,
        computer_id: Option<i32>,
    ) -> DbResult<(Vec<ComputerMonitor>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let mut count_query = computer_monitors::table.into_boxed();
        if let Some(cid) = computer_id {
            count_query = count_query.filter(computer_monitors::computer_id.eq(cid));
        }

        let total = count_query.count().get_result::<i64>(&mut conn).await?;

        let mut query = computer_monitors::table
            .select((
                computer_monitors::id,
                computer_monitors::computer_id,
                computer_monitors::name,
                computer_monitors::manufacturer,
                computer_monitors::model,
                computer_monitors::serial_number,
                computer_monitors::size_inches,
                computer_monitors::resolution,
                computer_monitors::panel_type,
                computer_monitors::refresh_rate,
                computer_monitors::is_primary,
                computer_monitors::notes,
                computer_monitors::created_at,
                computer_monitors::updated_at,
            ))
            .into_boxed();

        if let Some(cid) = computer_id {
            query = query.filter(computer_monitors::computer_id.eq(cid));
        }

        let results = query
            .order(computer_monitors::name.asc())
            .limit(per_page)
            .offset(offset)
            .load::<ComputerMonitorRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_computer_monitor)
            .collect();

        Ok((results, total))
    }

    pub async fn get_computer_monitor_by_id(&self, id: i32) -> DbResult<ComputerMonitor> {
        let mut conn = get_connection(&self.pool).await?;

        let row = computer_monitors::table
            .select((
                computer_monitors::id,
                computer_monitors::computer_id,
                computer_monitors::name,
                computer_monitors::manufacturer,
                computer_monitors::model,
                computer_monitors::serial_number,
                computer_monitors::size_inches,
                computer_monitors::resolution,
                computer_monitors::panel_type,
                computer_monitors::refresh_rate,
                computer_monitors::is_primary,
                computer_monitors::notes,
                computer_monitors::created_at,
                computer_monitors::updated_at,
            ))
            .filter(computer_monitors::id.eq(id))
            .first::<ComputerMonitorRow>(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(map_computer_monitor(row))
    }

    pub async fn create_computer_monitor(
        &self,
        new_monitor: NewComputerMonitor,
    ) -> DbResult<ComputerMonitor> {
        let mut conn = get_connection(&self.pool).await?;

        let new_id = diesel::insert_into(computer_monitors::table)
            .values((
                computer_monitors::computer_id.eq(new_monitor.computer_id),
                computer_monitors::name.eq(new_monitor.name),
                computer_monitors::manufacturer.eq(new_monitor.manufacturer),
                computer_monitors::model.eq(new_monitor.model),
                computer_monitors::serial_number.eq(new_monitor.serial_number),
                computer_monitors::size_inches.eq(new_monitor.size_inches),
                computer_monitors::resolution.eq(new_monitor.resolution),
                computer_monitors::panel_type.eq(new_monitor.panel_type),
                computer_monitors::refresh_rate.eq(new_monitor.refresh_rate),
                computer_monitors::is_primary.eq(new_monitor.is_primary),
                computer_monitors::notes.eq(new_monitor.notes),
            ))
            .returning(computer_monitors::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_computer_monitor_by_id(new_id).await
    }

    pub async fn update_computer_monitor(
        &self,
        id: i32,
        update_data: UpdateComputerMonitor,
    ) -> DbResult<ComputerMonitor> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(computer_monitors::table.find(id))
            .set((
                computer_monitors::computer_id.eq(update_data.computer_id),
                computer_monitors::name.eq(update_data.name),
                computer_monitors::manufacturer.eq(update_data.manufacturer),
                computer_monitors::model.eq(update_data.model),
                computer_monitors::serial_number.eq(update_data.serial_number),
                computer_monitors::size_inches.eq(update_data.size_inches),
                computer_monitors::resolution.eq(update_data.resolution),
                computer_monitors::panel_type.eq(update_data.panel_type),
                computer_monitors::refresh_rate.eq(update_data.refresh_rate),
                computer_monitors::is_primary.eq(update_data.is_primary),
                computer_monitors::notes.eq(update_data.notes),
            ))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_computer_monitor_by_id(id).await
    }

    pub async fn delete_computer_monitor(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(computer_monitors::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)
    }
}

type ComputerMonitorRow = (
    i32,
    i32,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<BigDecimal>,
    Option<String>,
    Option<String>,
    Option<i32>,
    Option<bool>,
    Option<String>,
    NaiveDateTime,
    NaiveDateTime,
);

fn map_computer_monitor(row: ComputerMonitorRow) -> ComputerMonitor {
    let (
        id,
        computer_id,
        name,
        manufacturer,
        model,
        serial_number,
        size_inches,
        resolution,
        panel_type,
        refresh_rate,
        is_primary,
        notes,
        created_at,
        updated_at,
    ) = row;
    ComputerMonitor {
        id,
        computer_id,
        name,
        manufacturer,
        model,
        serial_number,
        size_inches,
        resolution,
        panel_type,
        refresh_rate,
        is_primary,
        notes,
        created_at,
        updated_at,
    }
}
