use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::{
    computer_gpus, computer_monitors, computer_motherboards, computer_network_adapters,
    computer_peripherals, computer_processors, computer_ram, computer_software, computer_storage,
};

// ============================================================================
// Таблицы компонентов компьютера
// ============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = computer_processors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ComputerProcessor {
    pub id: i32,
    pub computer_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub cores: Option<i32>,
    pub threads: Option<i32>,
    pub base_frequency: Option<String>,
    pub max_frequency: Option<String>,
    pub cache_size: Option<String>,
    pub socket: Option<String>,
    pub tdp: Option<String>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = computer_processors)]
pub struct NewComputerProcessor {
    pub computer_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub cores: Option<i32>,
    pub threads: Option<i32>,
    pub base_frequency: Option<String>,
    pub max_frequency: Option<String>,
    pub cache_size: Option<String>,
    pub socket: Option<String>,
    pub tdp: Option<String>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = computer_processors)]
pub struct UpdateComputerProcessor {
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub cores: Option<i32>,
    pub threads: Option<i32>,
    pub base_frequency: Option<String>,
    pub max_frequency: Option<String>,
    pub cache_size: Option<String>,
    pub socket: Option<String>,
    pub tdp: Option<String>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = computer_ram)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ComputerRam {
    pub id: i32,
    pub computer_id: i32,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub capacity_gb: i32,
    pub type_: Option<String>,
    pub frequency: Option<String>,
    pub slot_number: Option<i32>,
    pub serial_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = computer_ram)]
pub struct NewComputerRam {
    pub computer_id: i32,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub capacity_gb: i32,
    pub type_: Option<String>,
    pub frequency: Option<String>,
    pub slot_number: Option<i32>,
    pub serial_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = computer_ram)]
pub struct UpdateComputerRam {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub capacity_gb: Option<i32>,
    pub type_: Option<String>,
    pub frequency: Option<String>,
    pub slot_number: Option<i32>,
    pub serial_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = computer_storage)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ComputerStorage {
    pub id: i32,
    pub computer_id: i32,
    pub type_: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub capacity_gb: i32,
    pub interface: Option<String>,
    pub serial_number: Option<String>,
    pub health_status: Option<String>,
    pub is_system_drive: Option<bool>,
    pub mount_point: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub smart_status: Option<String>,
    pub temperature: Option<i32>,
    pub power_on_hours: Option<i32>,
    pub reallocated_sectors: Option<i32>,
    pub last_defrag_date: Option<NaiveDate>,
    pub fragmentation_percent: Option<i32>,
    pub read_errors: Option<i32>,
    pub write_errors: Option<i32>,
    pub wear_level_percent: Option<i32>,
    pub total_bytes_written: Option<i64>,
    pub power_cycle_count: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = computer_storage)]
pub struct NewComputerStorage {
    pub computer_id: i32,
    pub type_: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub capacity_gb: i32,
    pub interface: Option<String>,
    pub serial_number: Option<String>,
    pub health_status: Option<String>,
    pub is_system_drive: Option<bool>,
    pub mount_point: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = computer_storage)]
pub struct UpdateComputerStorage {
    pub type_: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub capacity_gb: Option<i32>,
    pub interface: Option<String>,
    pub serial_number: Option<String>,
    pub health_status: Option<String>,
    pub is_system_drive: Option<bool>,
    pub mount_point: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = computer_gpus)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ComputerGpu {
    pub id: i32,
    pub computer_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub memory_gb: Option<i32>,
    pub memory_type: Option<String>,
    pub driver_version: Option<String>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = computer_gpus)]
pub struct NewComputerGpu {
    pub computer_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub memory_gb: Option<i32>,
    pub memory_type: Option<String>,
    pub driver_version: Option<String>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = computer_gpus)]
pub struct UpdateComputerGpu {
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub memory_gb: Option<i32>,
    pub memory_type: Option<String>,
    pub driver_version: Option<String>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = computer_motherboards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ComputerMotherboard {
    pub id: i32,
    pub computer_id: i32,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub chipset: Option<String>,
    pub bios_version: Option<String>,
    pub bios_date: Option<NaiveDate>,
    pub form_factor: Option<String>,
    pub serial_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = computer_motherboards)]
pub struct NewComputerMotherboard {
    pub computer_id: i32,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub chipset: Option<String>,
    pub bios_version: Option<String>,
    pub bios_date: Option<NaiveDate>,
    pub form_factor: Option<String>,
    pub serial_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = computer_motherboards)]
pub struct UpdateComputerMotherboard {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub chipset: Option<String>,
    pub bios_version: Option<String>,
    pub bios_date: Option<NaiveDate>,
    pub form_factor: Option<String>,
    pub serial_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = computer_network_adapters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ComputerNetworkAdapter {
    pub id: i32,
    pub computer_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
    pub adapter_type: Option<String>,
    pub speed: Option<String>,
    pub is_primary: Option<bool>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = computer_network_adapters)]
pub struct NewComputerNetworkAdapter {
    pub computer_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
    pub adapter_type: Option<String>,
    pub speed: Option<String>,
    pub is_primary: Option<bool>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = computer_network_adapters)]
pub struct UpdateComputerNetworkAdapter {
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
    pub adapter_type: Option<String>,
    pub speed: Option<String>,
    pub is_primary: Option<bool>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = computer_monitors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ComputerDisplay {
    pub id: i32,
    pub computer_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub size_inches: Option<BigDecimal>,
    pub resolution: Option<String>,
    pub panel_type: Option<String>,
    pub refresh_rate: Option<i32>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = computer_monitors)]
pub struct NewComputerDisplay {
    pub computer_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub size_inches: Option<BigDecimal>,
    pub resolution: Option<String>,
    pub panel_type: Option<String>,
    pub refresh_rate: Option<i32>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = computer_monitors)]
pub struct UpdateComputerDisplay {
    pub name: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub size_inches: Option<BigDecimal>,
    pub resolution: Option<String>,
    pub panel_type: Option<String>,
    pub refresh_rate: Option<i32>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = computer_peripherals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ComputerPeripheral {
    pub id: i32,
    pub computer_id: i32,
    pub name: String,
    pub type_: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub connection_type: Option<String>,
    pub status: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub warranty_end_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = computer_peripherals)]
pub struct NewComputerPeripheral {
    pub computer_id: i32,
    pub name: String,
    pub type_: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub connection_type: Option<String>,
    pub status: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub warranty_end_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = computer_peripherals)]
pub struct UpdateComputerPeripheral {
    pub name: Option<String>,
    pub type_: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub connection_type: Option<String>,
    pub status: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub warranty_end_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = computer_software)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ComputerSoftware {
    pub id: i32,
    pub computer_id: i32,
    pub software_catalog_id: i32,
    pub version: Option<String>,
    pub license_key: Option<String>,
    pub license_type: Option<String>,
    pub license_end_date: Option<NaiveDate>,
    pub installation_date: Option<NaiveDate>,
    pub last_seen_date: Option<NaiveDateTime>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = computer_software)]
pub struct NewComputerSoftware {
    pub computer_id: i32,
    pub software_catalog_id: i32,
    pub version: Option<String>,
    pub license_key: Option<String>,
    pub license_type: Option<String>,
    pub license_end_date: Option<NaiveDate>,
    pub installation_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

// ============================================================================
// Ответы API
// ============================================================================

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ComponentDetail {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ComputerComponentResponse {
    pub id: i32,
    pub component_type: String,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub is_primary: bool,
    pub details: Vec<ComponentDetail>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ComputerComponentListResponse {
    pub computer_id: i32,
    pub components: Vec<ComputerComponentResponse>,
    pub total: i64,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct InstalledSoftwareResponse {
    pub id: i32,
    pub software_id: i32,
    pub name: String,
    pub manufacturer: Option<String>,
    pub category: Option<String>,
    pub version: Option<String>,
    pub license_type: Option<String>,
    pub license_end_date: Option<String>,
    pub installation_date: Option<String>,
    pub last_seen_date: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct ComputerSoftwareListResponse {
    pub computer_id: i32,
    pub software: Vec<InstalledSoftwareResponse>,
    pub total: i64,
}

// ============================================================================
// Запросы API
// ============================================================================

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct ComponentRequest {
    pub component_type: String,
    pub name: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub notes: Option<String>,
    pub is_primary: Option<bool>,

    // Процессор
    pub cores: Option<i32>,
    pub threads: Option<i32>,
    pub base_frequency: Option<String>,
    pub max_frequency: Option<String>,
    pub cache_size: Option<String>,
    pub socket: Option<String>,
    pub tdp: Option<String>,

    // Оперативная память
    pub capacity_gb: Option<i32>,
    pub memory_type: Option<String>,
    pub frequency: Option<String>,
    pub slot_number: Option<i32>,

    // Накопитель
    pub storage_type: Option<String>,
    pub interface: Option<String>,
    pub is_system_drive: Option<bool>,
    pub mount_point: Option<String>,

    // Видеокарта
    pub memory_gb: Option<i32>,
    pub gpu_memory_type: Option<String>,
    pub driver_version: Option<String>,

    // Материнская плата
    pub chipset: Option<String>,
    pub bios_version: Option<String>,
    pub form_factor: Option<String>,

    // Сетевой адаптер
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
    pub adapter_type: Option<String>,
    pub speed: Option<String>,
    pub network_status: Option<String>,

    // Монитор
    pub size_inches: Option<f64>,
    pub resolution: Option<String>,
    pub panel_type: Option<String>,
    pub refresh_rate: Option<i32>,

    // Периферия
    pub peripheral_type: Option<String>,
    pub connection_type: Option<String>,
    pub peripheral_status: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewInstalledSoftwareRequest {
    pub software_catalog_id: i32,
    pub version: Option<String>,
    pub license_type: Option<String>,
    pub license_key: Option<String>,
    pub license_end_date: Option<String>,
    pub installation_date: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

// ============================================================================
// Преобразование в ответы API
// ============================================================================

pub fn format_datetime(value: NaiveDateTime) -> String {
    value.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

fn detail(label: &str, value: impl Into<String>) -> ComponentDetail {
    ComponentDetail {
        label: label.to_string(),
        value: value.into(),
    }
}

fn join_name(manufacturer: &Option<String>, model: &Option<String>, fallback: String) -> String {
    match (manufacturer, model) {
        (Some(manufacturer), Some(model)) if manufacturer != model => {
            format!("{manufacturer} {model}")
        }
        (_, Some(model)) => model.clone(),
        (Some(manufacturer), None) => manufacturer.clone(),
        (None, None) => fallback,
    }
}

impl From<ComputerProcessor> for ComputerComponentResponse {
    fn from(processor: ComputerProcessor) -> Self {
        let mut details = Vec::new();
        if let Some(cores) = processor.cores {
            details.push(detail("Ядра", cores.to_string()));
        }
        if let Some(threads) = processor.threads {
            details.push(detail("Потоки", threads.to_string()));
        }
        if let Some(frequency) = processor.base_frequency {
            details.push(detail("Базовая частота", frequency));
        }
        if let Some(frequency) = processor.max_frequency {
            details.push(detail("Макс. частота", frequency));
        }
        if let Some(cache) = processor.cache_size {
            details.push(detail("Кэш", cache));
        }
        if let Some(socket) = processor.socket {
            details.push(detail("Сокет", socket));
        }
        if let Some(tdp) = processor.tdp {
            details.push(detail("TDP", tdp));
        }

        Self {
            id: processor.id,
            component_type: "processor".to_string(),
            name: processor.name,
            manufacturer: processor.manufacturer,
            model: None,
            is_primary: processor.is_primary.unwrap_or(true),
            details,
            notes: processor.notes,
            created_at: format_datetime(processor.created_at),
            updated_at: format_datetime(processor.updated_at),
        }
    }
}

impl From<ComputerRam> for ComputerComponentResponse {
    fn from(ram: ComputerRam) -> Self {
        let mut details = vec![detail("Объем", format!("{} ГБ", ram.capacity_gb))];
        if let Some(memory_type) = ram.type_ {
            details.push(detail("Тип", memory_type));
        }
        if let Some(frequency) = ram.frequency {
            details.push(detail("Частота", frequency));
        }
        if let Some(slot) = ram.slot_number {
            details.push(detail("Слот", slot.to_string()));
        }
        if let Some(serial) = ram.serial_number {
            details.push(detail("Серийный номер", serial));
        }

        Self {
            id: ram.id,
            component_type: "ram".to_string(),
            name: join_name(
                &ram.manufacturer,
                &ram.model,
                format!("{} ГБ", ram.capacity_gb),
            ),
            manufacturer: ram.manufacturer,
            model: ram.model,
            is_primary: false,
            details,
            notes: ram.notes,
            created_at: format_datetime(ram.created_at),
            updated_at: format_datetime(ram.updated_at),
        }
    }
}

impl From<ComputerStorage> for ComputerComponentResponse {
    fn from(storage: ComputerStorage) -> Self {
        let mut details = vec![
            detail("Тип", storage.type_.clone()),
            detail("Объем", format!("{} ГБ", storage.capacity_gb)),
        ];
        if let Some(interface) = storage.interface {
            details.push(detail("Интерфейс", interface));
        }
        if let Some(health) = storage.health_status {
            details.push(detail("Здоровье", health));
        }
        if let Some(mount_point) = storage.mount_point {
            details.push(detail("Точка монтирования", mount_point));
        }
        if let Some(serial) = storage.serial_number {
            details.push(detail("Серийный номер", serial));
        }
        if storage.is_system_drive.unwrap_or(false) {
            details.push(detail("Системный диск", "Да"));
        }

        Self {
            id: storage.id,
            component_type: "storage".to_string(),
            name: join_name(
                &storage.manufacturer,
                &storage.model,
                format!("{} {} ГБ", storage.type_, storage.capacity_gb),
            ),
            manufacturer: storage.manufacturer,
            model: storage.model,
            is_primary: storage.is_system_drive.unwrap_or(false),
            details,
            notes: storage.notes,
            created_at: format_datetime(storage.created_at),
            updated_at: format_datetime(storage.updated_at),
        }
    }
}

impl From<ComputerGpu> for ComputerComponentResponse {
    fn from(gpu: ComputerGpu) -> Self {
        let mut details = Vec::new();
        if let Some(memory) = gpu.memory_gb {
            details.push(detail("Память", format!("{memory} ГБ")));
        }
        if let Some(memory_type) = gpu.memory_type {
            details.push(detail("Тип памяти", memory_type));
        }
        if let Some(driver) = gpu.driver_version {
            details.push(detail("Драйвер", driver));
        }

        Self {
            id: gpu.id,
            component_type: "gpu".to_string(),
            name: gpu.name,
            manufacturer: gpu.manufacturer,
            model: None,
            is_primary: gpu.is_primary.unwrap_or(false),
            details,
            notes: gpu.notes,
            created_at: format_datetime(gpu.created_at),
            updated_at: format_datetime(gpu.updated_at),
        }
    }
}

impl From<ComputerMotherboard> for ComputerComponentResponse {
    fn from(board: ComputerMotherboard) -> Self {
        let mut details = Vec::new();
        if let Some(chipset) = board.chipset {
            details.push(detail("Чипсет", chipset));
        }
        if let Some(bios) = board.bios_version {
            details.push(detail("BIOS", bios));
        }
        if let Some(bios_date) = board.bios_date {
            details.push(detail("Дата BIOS", bios_date.to_string()));
        }
        if let Some(form_factor) = board.form_factor {
            details.push(detail("Форм-фактор", form_factor));
        }
        if let Some(serial) = board.serial_number {
            details.push(detail("Серийный номер", serial));
        }

        Self {
            id: board.id,
            component_type: "motherboard".to_string(),
            name: join_name(
                &board.manufacturer,
                &board.model,
                "Материнская плата".to_string(),
            ),
            manufacturer: board.manufacturer,
            model: board.model,
            is_primary: true,
            details,
            notes: board.notes,
            created_at: format_datetime(board.created_at),
            updated_at: format_datetime(board.updated_at),
        }
    }
}

impl From<ComputerNetworkAdapter> for ComputerComponentResponse {
    fn from(adapter: ComputerNetworkAdapter) -> Self {
        let mut details = Vec::new();
        if let Some(adapter_type) = adapter.adapter_type {
            details.push(detail("Тип", adapter_type));
        }
        if let Some(mac) = adapter.mac_address {
            details.push(detail("MAC-адрес", mac));
        }
        if let Some(ip) = adapter.ip_address {
            details.push(detail("IP-адрес", ip));
        }
        if let Some(speed) = adapter.speed {
            details.push(detail("Скорость", speed));
        }
        if let Some(status) = adapter.status {
            details.push(detail("Статус", status));
        }

        Self {
            id: adapter.id,
            component_type: "network".to_string(),
            name: adapter.name,
            manufacturer: adapter.manufacturer,
            model: None,
            is_primary: adapter.is_primary.unwrap_or(false),
            details,
            notes: adapter.notes,
            created_at: format_datetime(adapter.created_at),
            updated_at: format_datetime(adapter.updated_at),
        }
    }
}

impl From<ComputerDisplay> for ComputerComponentResponse {
    fn from(display: ComputerDisplay) -> Self {
        let mut details = Vec::new();
        if let Some(size) = display.size_inches {
            details.push(detail("Размер", format!("{size}\"")));
        }
        if let Some(resolution) = display.resolution {
            details.push(detail("Разрешение", resolution));
        }
        if let Some(panel_type) = display.panel_type {
            details.push(detail("Тип матрицы", panel_type));
        }
        if let Some(refresh_rate) = display.refresh_rate {
            details.push(detail("Частота обновления", format!("{refresh_rate} Гц")));
        }
        if let Some(serial) = display.serial_number {
            details.push(detail("Серийный номер", serial));
        }

        Self {
            id: display.id,
            component_type: "monitor".to_string(),
            name: display.name,
            manufacturer: display.manufacturer,
            model: display.model,
            is_primary: display.is_primary.unwrap_or(false),
            details,
            notes: display.notes,
            created_at: format_datetime(display.created_at),
            updated_at: format_datetime(display.updated_at),
        }
    }
}

impl From<ComputerPeripheral> for ComputerComponentResponse {
    fn from(peripheral: ComputerPeripheral) -> Self {
        let mut details = vec![detail("Тип устройства", peripheral.type_.clone())];
        if let Some(connection) = peripheral.connection_type {
            details.push(detail("Подключение", connection));
        }
        if let Some(status) = peripheral.status {
            details.push(detail("Статус", status));
        }
        if let Some(serial) = peripheral.serial_number {
            details.push(detail("Серийный номер", serial));
        }
        if let Some(warranty) = peripheral.warranty_end_date {
            details.push(detail("Гарантия до", warranty.to_string()));
        }

        Self {
            id: peripheral.id,
            component_type: "peripheral".to_string(),
            name: peripheral.name,
            manufacturer: peripheral.manufacturer,
            model: peripheral.model,
            is_primary: false,
            details,
            notes: peripheral.notes,
            created_at: format_datetime(peripheral.created_at),
            updated_at: format_datetime(peripheral.updated_at),
        }
    }
}

pub fn format_optional_date(value: Option<NaiveDate>) -> Option<String> {
    value.map(|date| date.to_string())
}
