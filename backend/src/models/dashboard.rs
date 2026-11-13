use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Агрегированная сводка по основным разделам IT-инфраструктуры для дашборда.
#[derive(Debug, Serialize, Deserialize, Object, Default)]
pub struct DashboardSummary {
    pub total_computers: i64,
    pub total_employees: i64,
    pub total_requests: i64,
    pub total_licenses: i64,
    pub total_equipment: i64,
    pub total_consumables: i64,
    pub total_software: i64,
    /// Дополнительные показатели (свободная структура для расширения)
    pub extra: Option<Value>,
}
