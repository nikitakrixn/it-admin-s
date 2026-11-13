use poem_openapi::Object;
use serde::{Deserialize, Serialize};

/// Пара «ключ — количество» для агрегатов по статусам, приоритетам и категориям.
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct CountItem {
    pub key: String,
    pub count: i64,
}

/// Среднее время решения заявок в разрезе приоритета.
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct PriorityResolution {
    pub priority: String,
    pub count: i64,
    pub avg_hours: Option<f64>,
}

/// Часто встречающаяся проблема (группировка заявок по названию).
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct TopIssue {
    pub title: String,
    pub count: i64,
    pub avg_hours: Option<f64>,
}

/// Количество записей журнала по дню недели (1 — понедельник).
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct WeekdayActivity {
    pub weekday: i32,
    pub count: i64,
}

/// Сводка для страницы аналитики и главного дашборда.
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct AnalyticsOverview {
    pub period_days: i64,

    pub total_employees: i64,
    pub active_employees: i64,
    pub total_computers: i64,
    pub active_computers: i64,
    pub total_requests: i64,
    pub open_requests: i64,
    pub requests_created_in_period: i64,
    pub requests_resolved_in_period: i64,
    pub avg_resolution_hours: Option<f64>,

    pub total_licenses: i64,
    pub used_licenses: i64,
    pub expiring_licenses: i64,
    pub expired_licenses: i64,

    pub total_equipment: i64,
    pub equipment_needs_attention: i64,
    pub low_stock_consumables: i64,

    pub requests_by_status: Vec<CountItem>,
    pub requests_by_priority: Vec<CountItem>,
    pub requests_by_category: Vec<CountItem>,
    pub resolution_by_priority: Vec<PriorityResolution>,
    pub top_issues: Vec<TopIssue>,
    pub activity_by_weekday: Vec<WeekdayActivity>,
}
