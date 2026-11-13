use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::config::database::Pool;
use crate::models::computer_component::{
    ComponentRequest, ComputerComponentResponse, ComputerDisplay, ComputerGpu, ComputerMotherboard,
    ComputerNetworkAdapter, ComputerPeripheral, ComputerProcessor, ComputerRam, ComputerStorage,
    InstalledSoftwareResponse, NewComputerDisplay, NewComputerGpu, NewComputerMotherboard,
    NewComputerNetworkAdapter, NewComputerPeripheral, NewComputerProcessor, NewComputerRam,
    NewComputerSoftware, NewComputerStorage, NewInstalledSoftwareRequest, UpdateComputerDisplay,
    UpdateComputerGpu, UpdateComputerMotherboard, UpdateComputerNetworkAdapter,
    UpdateComputerPeripheral, UpdateComputerProcessor, UpdateComputerRam, UpdateComputerStorage,
    format_datetime, format_optional_date,
};
use crate::models::schema::{
    computer_gpus, computer_monitors, computer_motherboards, computer_network_adapters,
    computer_peripherals, computer_processors, computer_ram, computer_software, computer_storage,
    software_catalog,
};
use crate::utils::db_helpers::{DbResult, get_connection};
use crate::utils::error::AppError;

fn decimal_from_f64(value: Option<f64>) -> Option<BigDecimal> {
    value.and_then(|v| v.to_string().parse::<BigDecimal>().ok())
}

fn parse_date(value: &Option<String>) -> Result<Option<NaiveDate>, AppError> {
    match value {
        Some(v) if !v.trim().is_empty() => NaiveDate::parse_from_str(v.trim(), "%Y-%m-%d")
            .map(Some)
            .map_err(|_| AppError::ValidationError("Некорректный формат даты".to_string())),
        _ => Ok(None),
    }
}

fn required_capacity(value: Option<i32>) -> Result<i32, AppError> {
    match value {
        Some(capacity) if capacity > 0 => Ok(capacity),
        _ => Err(AppError::ValidationError(
            "Укажите положительный объем компонента".to_string(),
        )),
    }
}

pub struct ComputerComponentRepository {
    pool: Pool,
}

impl ComputerComponentRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Чтение компонентов
    // ========================================================================

    pub async fn list_components(
        &self,
        computer_id: i32,
    ) -> DbResult<Vec<ComputerComponentResponse>> {
        let mut conn = get_connection(&self.pool).await?;
        let mut components = Vec::new();

        let processors = computer_processors::table
            .filter(computer_processors::computer_id.eq(computer_id))
            .order(computer_processors::id.asc())
            .load::<ComputerProcessor>(&mut conn)
            .await?;
        components.extend(processors.into_iter().map(Into::into));

        let motherboards = computer_motherboards::table
            .filter(computer_motherboards::computer_id.eq(computer_id))
            .order(computer_motherboards::id.asc())
            .load::<ComputerMotherboard>(&mut conn)
            .await?;
        components.extend(motherboards.into_iter().map(Into::into));

        let ram = computer_ram::table
            .filter(computer_ram::computer_id.eq(computer_id))
            .order(computer_ram::slot_number.asc().nulls_last())
            .load::<ComputerRam>(&mut conn)
            .await?;
        components.extend(ram.into_iter().map(Into::into));

        let storage = computer_storage::table
            .filter(computer_storage::computer_id.eq(computer_id))
            .order(computer_storage::id.asc())
            .load::<ComputerStorage>(&mut conn)
            .await?;
        components.extend(storage.into_iter().map(Into::into));

        let gpus = computer_gpus::table
            .filter(computer_gpus::computer_id.eq(computer_id))
            .order(computer_gpus::id.asc())
            .load::<ComputerGpu>(&mut conn)
            .await?;
        components.extend(gpus.into_iter().map(Into::into));

        let adapters = computer_network_adapters::table
            .filter(computer_network_adapters::computer_id.eq(computer_id))
            .order(computer_network_adapters::id.asc())
            .load::<ComputerNetworkAdapter>(&mut conn)
            .await?;
        components.extend(adapters.into_iter().map(Into::into));

        let displays = computer_monitors::table
            .filter(computer_monitors::computer_id.eq(computer_id))
            .order(computer_monitors::id.asc())
            .load::<ComputerDisplay>(&mut conn)
            .await?;
        components.extend(displays.into_iter().map(Into::into));

        let peripherals = computer_peripherals::table
            .filter(computer_peripherals::computer_id.eq(computer_id))
            .order(computer_peripherals::id.asc())
            .load::<ComputerPeripheral>(&mut conn)
            .await?;
        components.extend(peripherals.into_iter().map(Into::into));

        Ok(components)
    }

    pub async fn get_component(
        &self,
        computer_id: i32,
        component_type: &str,
        component_id: i32,
    ) -> DbResult<ComputerComponentResponse> {
        let mut conn = get_connection(&self.pool).await?;

        match component_type {
            "processor" => computer_processors::table
                .filter(computer_processors::computer_id.eq(computer_id))
                .find(component_id)
                .first::<ComputerProcessor>(&mut conn)
                .await
                .map(Into::into)
                .map_err(Into::into),
            "ram" => computer_ram::table
                .filter(computer_ram::computer_id.eq(computer_id))
                .find(component_id)
                .first::<ComputerRam>(&mut conn)
                .await
                .map(Into::into)
                .map_err(Into::into),
            "storage" => computer_storage::table
                .filter(computer_storage::computer_id.eq(computer_id))
                .find(component_id)
                .first::<ComputerStorage>(&mut conn)
                .await
                .map(Into::into)
                .map_err(Into::into),
            "gpu" => computer_gpus::table
                .filter(computer_gpus::computer_id.eq(computer_id))
                .find(component_id)
                .first::<ComputerGpu>(&mut conn)
                .await
                .map(Into::into)
                .map_err(Into::into),
            "motherboard" => computer_motherboards::table
                .filter(computer_motherboards::computer_id.eq(computer_id))
                .find(component_id)
                .first::<ComputerMotherboard>(&mut conn)
                .await
                .map(Into::into)
                .map_err(Into::into),
            "network" => computer_network_adapters::table
                .filter(computer_network_adapters::computer_id.eq(computer_id))
                .find(component_id)
                .first::<ComputerNetworkAdapter>(&mut conn)
                .await
                .map(Into::into)
                .map_err(Into::into),
            "monitor" => computer_monitors::table
                .filter(computer_monitors::computer_id.eq(computer_id))
                .find(component_id)
                .first::<ComputerDisplay>(&mut conn)
                .await
                .map(Into::into)
                .map_err(Into::into),
            "peripheral" => computer_peripherals::table
                .filter(computer_peripherals::computer_id.eq(computer_id))
                .find(component_id)
                .first::<ComputerPeripheral>(&mut conn)
                .await
                .map(Into::into)
                .map_err(Into::into),
            other => Err(AppError::ValidationError(format!(
                "Неизвестный тип компонента: {other}"
            ))),
        }
    }

    // ========================================================================
    // Создание компонентов
    // ========================================================================

    pub async fn create_component(
        &self,
        computer_id: i32,
        req: ComponentRequest,
    ) -> DbResult<ComputerComponentResponse> {
        let mut conn = get_connection(&self.pool).await?;
        let component_type = req.component_type.clone();

        match component_type.as_str() {
            "processor" => {
                let new_processor = NewComputerProcessor {
                    computer_id,
                    name: req.name,
                    manufacturer: req.manufacturer,
                    cores: req.cores,
                    threads: req.threads,
                    base_frequency: req.base_frequency,
                    max_frequency: req.max_frequency,
                    cache_size: req.cache_size,
                    socket: req.socket,
                    tdp: req.tdp,
                    is_primary: req.is_primary.or(Some(true)),
                    notes: req.notes,
                };

                let processor = diesel::insert_into(computer_processors::table)
                    .values(&new_processor)
                    .get_result::<ComputerProcessor>(&mut conn)
                    .await?;
                Ok(processor.into())
            }
            "ram" => {
                let capacity_gb = required_capacity(req.capacity_gb)?;
                let new_ram = NewComputerRam {
                    computer_id,
                    manufacturer: req.manufacturer,
                    model: req.model.or(Some(req.name)),
                    capacity_gb,
                    type_: req.memory_type,
                    frequency: req.frequency,
                    slot_number: req.slot_number,
                    serial_number: req.serial_number,
                    notes: req.notes,
                };

                let ram = diesel::insert_into(computer_ram::table)
                    .values(&new_ram)
                    .get_result::<ComputerRam>(&mut conn)
                    .await?;
                Ok(ram.into())
            }
            "storage" => {
                let capacity_gb = required_capacity(req.capacity_gb)?;
                let storage_type = req.storage_type.ok_or_else(|| {
                    AppError::ValidationError("Укажите тип накопителя".to_string())
                })?;

                let new_storage = NewComputerStorage {
                    computer_id,
                    type_: storage_type,
                    manufacturer: req.manufacturer,
                    model: req.model.or(Some(req.name)),
                    capacity_gb,
                    interface: req.interface,
                    serial_number: req.serial_number,
                    health_status: None,
                    is_system_drive: req.is_system_drive,
                    mount_point: req.mount_point,
                    notes: req.notes,
                };

                let storage = diesel::insert_into(computer_storage::table)
                    .values(&new_storage)
                    .get_result::<ComputerStorage>(&mut conn)
                    .await?;
                Ok(storage.into())
            }
            "gpu" => {
                let new_gpu = NewComputerGpu {
                    computer_id,
                    name: req.name,
                    manufacturer: req.manufacturer,
                    memory_gb: req.memory_gb,
                    memory_type: req.gpu_memory_type.or(req.memory_type),
                    driver_version: req.driver_version,
                    is_primary: req.is_primary.or(Some(true)),
                    notes: req.notes,
                };

                let gpu = diesel::insert_into(computer_gpus::table)
                    .values(&new_gpu)
                    .get_result::<ComputerGpu>(&mut conn)
                    .await?;
                Ok(gpu.into())
            }
            "motherboard" => {
                let existing = computer_motherboards::table
                    .filter(computer_motherboards::computer_id.eq(computer_id))
                    .count()
                    .get_result::<i64>(&mut conn)
                    .await?;
                if existing > 0 {
                    return Err(AppError::Conflict(
                        "У компьютера уже есть материнская плата".to_string(),
                    ));
                }

                let new_board = NewComputerMotherboard {
                    computer_id,
                    manufacturer: req.manufacturer,
                    model: req.model.or(Some(req.name)),
                    chipset: req.chipset,
                    bios_version: req.bios_version,
                    bios_date: None,
                    form_factor: req.form_factor,
                    serial_number: req.serial_number,
                    notes: req.notes,
                };

                let board = diesel::insert_into(computer_motherboards::table)
                    .values(&new_board)
                    .get_result::<ComputerMotherboard>(&mut conn)
                    .await?;
                Ok(board.into())
            }
            "network" => {
                let new_adapter = NewComputerNetworkAdapter {
                    computer_id,
                    name: req.name,
                    manufacturer: req.manufacturer,
                    mac_address: req.mac_address,
                    ip_address: req.ip_address,
                    adapter_type: req.adapter_type,
                    speed: req.speed,
                    is_primary: req.is_primary,
                    status: req.network_status.or(Some("active".to_string())),
                    notes: req.notes,
                };

                let adapter = diesel::insert_into(computer_network_adapters::table)
                    .values(&new_adapter)
                    .get_result::<ComputerNetworkAdapter>(&mut conn)
                    .await?;
                Ok(adapter.into())
            }
            "monitor" => {
                let new_display = NewComputerDisplay {
                    computer_id,
                    name: req.name,
                    manufacturer: req.manufacturer,
                    model: req.model,
                    serial_number: req.serial_number,
                    size_inches: decimal_from_f64(req.size_inches),
                    resolution: req.resolution,
                    panel_type: req.panel_type,
                    refresh_rate: req.refresh_rate,
                    is_primary: req.is_primary,
                    notes: req.notes,
                };

                let display = diesel::insert_into(computer_monitors::table)
                    .values(&new_display)
                    .get_result::<ComputerDisplay>(&mut conn)
                    .await?;
                Ok(display.into())
            }
            "peripheral" => {
                let peripheral_type = req.peripheral_type.ok_or_else(|| {
                    AppError::ValidationError("Укажите тип периферийного устройства".to_string())
                })?;

                let new_peripheral = NewComputerPeripheral {
                    computer_id,
                    name: req.name,
                    type_: peripheral_type,
                    manufacturer: req.manufacturer,
                    model: req.model,
                    serial_number: req.serial_number,
                    connection_type: req.connection_type,
                    status: req.peripheral_status.or(Some("active".to_string())),
                    purchase_date: None,
                    warranty_end_date: None,
                    notes: req.notes,
                };

                let peripheral = diesel::insert_into(computer_peripherals::table)
                    .values(&new_peripheral)
                    .get_result::<ComputerPeripheral>(&mut conn)
                    .await?;
                Ok(peripheral.into())
            }
            other => Err(AppError::ValidationError(format!(
                "Неизвестный тип компонента: {other}"
            ))),
        }
    }

    // ========================================================================
    // Обновление компонентов
    // ========================================================================

    pub async fn update_component(
        &self,
        computer_id: i32,
        component_type: &str,
        component_id: i32,
        req: ComponentRequest,
    ) -> DbResult<ComputerComponentResponse> {
        let mut conn = get_connection(&self.pool).await?;

        match component_type {
            "processor" => {
                let changes = UpdateComputerProcessor {
                    name: Some(req.name),
                    manufacturer: req.manufacturer,
                    cores: req.cores,
                    threads: req.threads,
                    base_frequency: req.base_frequency,
                    max_frequency: req.max_frequency,
                    cache_size: req.cache_size,
                    socket: req.socket,
                    tdp: req.tdp,
                    is_primary: req.is_primary,
                    notes: req.notes,
                };

                let processor = diesel::update(
                    computer_processors::table
                        .filter(computer_processors::computer_id.eq(computer_id))
                        .find(component_id),
                )
                .set(&changes)
                .get_result::<ComputerProcessor>(&mut conn)
                .await?;
                Ok(processor.into())
            }
            "ram" => {
                let changes = UpdateComputerRam {
                    manufacturer: req.manufacturer,
                    model: req.model.or(Some(req.name)),
                    capacity_gb: req.capacity_gb,
                    type_: req.memory_type,
                    frequency: req.frequency,
                    slot_number: req.slot_number,
                    serial_number: req.serial_number,
                    notes: req.notes,
                };

                let ram = diesel::update(
                    computer_ram::table
                        .filter(computer_ram::computer_id.eq(computer_id))
                        .find(component_id),
                )
                .set(&changes)
                .get_result::<ComputerRam>(&mut conn)
                .await?;
                Ok(ram.into())
            }
            "storage" => {
                let changes = UpdateComputerStorage {
                    type_: req.storage_type,
                    manufacturer: req.manufacturer,
                    model: req.model.or(Some(req.name)),
                    capacity_gb: req.capacity_gb,
                    interface: req.interface,
                    serial_number: req.serial_number,
                    health_status: None,
                    is_system_drive: req.is_system_drive,
                    mount_point: req.mount_point,
                    notes: req.notes,
                };

                let storage = diesel::update(
                    computer_storage::table
                        .filter(computer_storage::computer_id.eq(computer_id))
                        .find(component_id),
                )
                .set(&changes)
                .get_result::<ComputerStorage>(&mut conn)
                .await?;
                Ok(storage.into())
            }
            "gpu" => {
                let changes = UpdateComputerGpu {
                    name: Some(req.name),
                    manufacturer: req.manufacturer,
                    memory_gb: req.memory_gb,
                    memory_type: req.gpu_memory_type.or(req.memory_type),
                    driver_version: req.driver_version,
                    is_primary: req.is_primary,
                    notes: req.notes,
                };

                let gpu = diesel::update(
                    computer_gpus::table
                        .filter(computer_gpus::computer_id.eq(computer_id))
                        .find(component_id),
                )
                .set(&changes)
                .get_result::<ComputerGpu>(&mut conn)
                .await?;
                Ok(gpu.into())
            }
            "motherboard" => {
                let changes = UpdateComputerMotherboard {
                    manufacturer: req.manufacturer,
                    model: req.model.or(Some(req.name)),
                    chipset: req.chipset,
                    bios_version: req.bios_version,
                    bios_date: None,
                    form_factor: req.form_factor,
                    serial_number: req.serial_number,
                    notes: req.notes,
                };

                let board = diesel::update(
                    computer_motherboards::table
                        .filter(computer_motherboards::computer_id.eq(computer_id))
                        .find(component_id),
                )
                .set(&changes)
                .get_result::<ComputerMotherboard>(&mut conn)
                .await?;
                Ok(board.into())
            }
            "network" => {
                let changes = UpdateComputerNetworkAdapter {
                    name: Some(req.name),
                    manufacturer: req.manufacturer,
                    mac_address: req.mac_address,
                    ip_address: req.ip_address,
                    adapter_type: req.adapter_type,
                    speed: req.speed,
                    is_primary: req.is_primary,
                    status: req.network_status,
                    notes: req.notes,
                };

                let adapter = diesel::update(
                    computer_network_adapters::table
                        .filter(computer_network_adapters::computer_id.eq(computer_id))
                        .find(component_id),
                )
                .set(&changes)
                .get_result::<ComputerNetworkAdapter>(&mut conn)
                .await?;
                Ok(adapter.into())
            }
            "monitor" => {
                let changes = UpdateComputerDisplay {
                    name: Some(req.name),
                    manufacturer: req.manufacturer,
                    model: req.model,
                    serial_number: req.serial_number,
                    size_inches: decimal_from_f64(req.size_inches),
                    resolution: req.resolution,
                    panel_type: req.panel_type,
                    refresh_rate: req.refresh_rate,
                    is_primary: req.is_primary,
                    notes: req.notes,
                };

                let display = diesel::update(
                    computer_monitors::table
                        .filter(computer_monitors::computer_id.eq(computer_id))
                        .find(component_id),
                )
                .set(&changes)
                .get_result::<ComputerDisplay>(&mut conn)
                .await?;
                Ok(display.into())
            }
            "peripheral" => {
                let changes = UpdateComputerPeripheral {
                    name: Some(req.name),
                    type_: req.peripheral_type,
                    manufacturer: req.manufacturer,
                    model: req.model,
                    serial_number: req.serial_number,
                    connection_type: req.connection_type,
                    status: req.peripheral_status,
                    purchase_date: None,
                    warranty_end_date: None,
                    notes: req.notes,
                };

                let peripheral = diesel::update(
                    computer_peripherals::table
                        .filter(computer_peripherals::computer_id.eq(computer_id))
                        .find(component_id),
                )
                .set(&changes)
                .get_result::<ComputerPeripheral>(&mut conn)
                .await?;
                Ok(peripheral.into())
            }
            other => Err(AppError::ValidationError(format!(
                "Неизвестный тип компонента: {other}"
            ))),
        }
    }

    // ========================================================================
    // Удаление компонентов
    // ========================================================================

    pub async fn delete_component(
        &self,
        computer_id: i32,
        component_type: &str,
        component_id: i32,
    ) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        match component_type {
            "processor" => diesel::delete(
                computer_processors::table
                    .filter(computer_processors::computer_id.eq(computer_id))
                    .find(component_id),
            )
            .execute(&mut conn)
            .await
            .map_err(Into::into),
            "ram" => diesel::delete(
                computer_ram::table
                    .filter(computer_ram::computer_id.eq(computer_id))
                    .find(component_id),
            )
            .execute(&mut conn)
            .await
            .map_err(Into::into),
            "storage" => diesel::delete(
                computer_storage::table
                    .filter(computer_storage::computer_id.eq(computer_id))
                    .find(component_id),
            )
            .execute(&mut conn)
            .await
            .map_err(Into::into),
            "gpu" => diesel::delete(
                computer_gpus::table
                    .filter(computer_gpus::computer_id.eq(computer_id))
                    .find(component_id),
            )
            .execute(&mut conn)
            .await
            .map_err(Into::into),
            "motherboard" => diesel::delete(
                computer_motherboards::table
                    .filter(computer_motherboards::computer_id.eq(computer_id))
                    .find(component_id),
            )
            .execute(&mut conn)
            .await
            .map_err(Into::into),
            "network" => diesel::delete(
                computer_network_adapters::table
                    .filter(computer_network_adapters::computer_id.eq(computer_id))
                    .find(component_id),
            )
            .execute(&mut conn)
            .await
            .map_err(Into::into),
            "monitor" => diesel::delete(
                computer_monitors::table
                    .filter(computer_monitors::computer_id.eq(computer_id))
                    .find(component_id),
            )
            .execute(&mut conn)
            .await
            .map_err(Into::into),
            "peripheral" => diesel::delete(
                computer_peripherals::table
                    .filter(computer_peripherals::computer_id.eq(computer_id))
                    .find(component_id),
            )
            .execute(&mut conn)
            .await
            .map_err(Into::into),
            other => Err(AppError::ValidationError(format!(
                "Неизвестный тип компонента: {other}"
            ))),
        }
    }

    // ========================================================================
    // Установленное ПО
    // ========================================================================

    pub async fn list_software(
        &self,
        computer_id: i32,
    ) -> DbResult<Vec<InstalledSoftwareResponse>> {
        let mut conn = get_connection(&self.pool).await?;

        let rows = computer_software::table
            .inner_join(software_catalog::table)
            .filter(computer_software::computer_id.eq(computer_id))
            .select((
                computer_software::id,
                computer_software::software_catalog_id,
                software_catalog::name,
                software_catalog::manufacturer,
                software_catalog::category,
                computer_software::version,
                computer_software::license_type,
                computer_software::license_end_date,
                computer_software::installation_date,
                computer_software::last_seen_date,
                computer_software::status,
                computer_software::notes,
            ))
            .order(software_catalog::name.asc())
            .load::<(
                i32,
                i32,
                String,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<NaiveDate>,
                Option<NaiveDate>,
                Option<chrono::NaiveDateTime>,
                Option<String>,
                Option<String>,
            )>(&mut conn)
            .await?;

        Ok(rows
            .into_iter()
            .map(
                |(
                    id,
                    software_id,
                    name,
                    manufacturer,
                    category,
                    version,
                    license_type,
                    license_end_date,
                    installation_date,
                    last_seen_date,
                    status,
                    notes,
                )| InstalledSoftwareResponse {
                    id,
                    software_id,
                    name,
                    manufacturer,
                    category,
                    version,
                    license_type,
                    license_end_date: format_optional_date(license_end_date),
                    installation_date: format_optional_date(installation_date),
                    last_seen_date: last_seen_date.map(format_datetime),
                    status,
                    notes,
                },
            )
            .collect())
    }

    pub async fn install_software(
        &self,
        computer_id: i32,
        req: NewInstalledSoftwareRequest,
    ) -> DbResult<InstalledSoftwareResponse> {
        let mut conn = get_connection(&self.pool).await?;

        let catalog = software_catalog::table
            .find(req.software_catalog_id)
            .first::<crate::models::software::Software>(&mut conn)
            .await?;

        let existing = computer_software::table
            .filter(computer_software::computer_id.eq(computer_id))
            .filter(computer_software::software_catalog_id.eq(req.software_catalog_id))
            .count()
            .get_result::<i64>(&mut conn)
            .await?;
        if existing > 0 {
            return Err(AppError::Conflict(
                "Это ПО уже установлено на компьютер".to_string(),
            ));
        }

        let new_software = NewComputerSoftware {
            computer_id,
            software_catalog_id: req.software_catalog_id,
            version: req.version,
            license_key: req.license_key,
            license_type: req.license_type.or(Some("unknown".to_string())),
            license_end_date: parse_date(&req.license_end_date)?,
            installation_date: parse_date(&req.installation_date)?,
            status: req.status.or(Some("active".to_string())),
            notes: req.notes,
        };

        let installed = diesel::insert_into(computer_software::table)
            .values(&new_software)
            .get_result::<crate::models::computer_component::ComputerSoftware>(&mut conn)
            .await?;

        Ok(InstalledSoftwareResponse {
            id: installed.id,
            software_id: installed.software_catalog_id,
            name: catalog.name,
            manufacturer: catalog.manufacturer,
            category: catalog.category,
            version: installed.version,
            license_type: installed.license_type,
            license_end_date: format_optional_date(installed.license_end_date),
            installation_date: format_optional_date(installed.installation_date),
            last_seen_date: installed.last_seen_date.map(format_datetime),
            status: installed.status,
            notes: installed.notes,
        })
    }

    pub async fn uninstall_software(&self, computer_id: i32, software_id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(
            computer_software::table
                .filter(computer_software::computer_id.eq(computer_id))
                .find(software_id),
        )
        .execute(&mut conn)
        .await
        .map_err(Into::into)
    }
}
