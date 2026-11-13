use crate::config::database::Pool;
use crate::models::analytics::{
    AnalyticsOverview, CountItem, PriorityResolution, TopIssue, WeekdayActivity,
};
use crate::utils::db_helpers::{DbResult, get_connection};
use crate::utils::error::AppError;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Double, Integer, Nullable, Text};
use diesel_async::RunQueryDsl;

pub struct AnalyticsRepository {
    pub pool: Pool,
}

impl AnalyticsRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Собирает сводку по инфраструктуре за указанный период.
    ///
    /// `days` уже ограничен вызывающей стороной, поэтому его можно безопасно
    /// подставлять в интервал: это целое число, а не пользовательская строка.
    pub async fn get_overview(&self, days: i64) -> DbResult<AnalyticsOverview> {
        let mut conn = get_connection(&self.pool).await?;

        let summary = diesel::sql_query(format!(
            r#"
            SELECT
                (SELECT COUNT(*) FROM employees)::bigint AS total_employees,
                (SELECT COUNT(*) FROM employees WHERE status = 'active')::bigint AS active_employees,
                (SELECT COUNT(*) FROM computers)::bigint AS total_computers,
                (SELECT COUNT(*) FROM computers WHERE status = 'active')::bigint AS active_computers,
                (SELECT COUNT(*) FROM requests)::bigint AS total_requests,
                (SELECT COUNT(*) FROM requests
                    WHERE status IN ('new', 'in_progress', 'waiting'))::bigint AS open_requests,
                (SELECT COUNT(*) FROM requests
                    WHERE created_at >= NOW() - INTERVAL '{days} days')::bigint AS requests_created_in_period,
                (SELECT COUNT(*) FROM requests
                    WHERE resolved_at >= NOW() - INTERVAL '{days} days')::bigint AS requests_resolved_in_period,
                (SELECT AVG(EXTRACT(EPOCH FROM (resolved_at - created_at)) / 3600)::float8
                    FROM requests
                    WHERE resolved_at IS NOT NULL
                      AND resolved_at >= NOW() - INTERVAL '{days} days') AS avg_resolution_hours,
                (SELECT COALESCE(SUM(total_licenses), 0) FROM license_pools)::bigint AS total_licenses,
                (SELECT COALESCE(SUM(used_licenses), 0) FROM license_pools)::bigint AS used_licenses,
                (SELECT COUNT(*) FROM license_pools
                    WHERE expiration_date BETWEEN CURRENT_DATE AND CURRENT_DATE + 30)::bigint AS expiring_licenses,
                (SELECT COUNT(*) FROM license_pools
                    WHERE expiration_date < CURRENT_DATE)::bigint AS expired_licenses,
                (SELECT COUNT(*) FROM equipment)::bigint AS total_equipment,
                (SELECT COUNT(*) FROM equipment
                    WHERE status IN ('repair', 'inactive'))::bigint AS equipment_needs_attention,
                (SELECT COUNT(*) FROM consumables
                    WHERE quantity <= min_quantity)::bigint AS low_stock_consumables
            "#
        ))
        .get_result::<AnalyticsSummaryRow>(&mut conn)
        .await
        .map_err(AppError::from)?;

        let requests_by_status = diesel::sql_query(
            r#"
            SELECT status AS key, COUNT(*)::bigint AS count
            FROM requests
            GROUP BY status
            ORDER BY count DESC, status
            "#,
        )
        .load::<CountRow>(&mut conn)
        .await
        .map_err(AppError::from)?
        .into_iter()
        .map(|row| CountItem {
            key: row.key,
            count: row.count,
        })
        .collect();

        let requests_by_priority = diesel::sql_query(
            r#"
            SELECT priority AS key, COUNT(*)::bigint AS count
            FROM requests
            GROUP BY priority
            ORDER BY count DESC, priority
            "#,
        )
        .load::<CountRow>(&mut conn)
        .await
        .map_err(AppError::from)?
        .into_iter()
        .map(|row| CountItem {
            key: row.key,
            count: row.count,
        })
        .collect();

        let requests_by_category = diesel::sql_query(
            r#"
            SELECT COALESCE(category, 'other') AS key, COUNT(*)::bigint AS count
            FROM requests
            GROUP BY COALESCE(category, 'other')
            ORDER BY count DESC, key
            "#,
        )
        .load::<CountRow>(&mut conn)
        .await
        .map_err(AppError::from)?
        .into_iter()
        .map(|row| CountItem {
            key: row.key,
            count: row.count,
        })
        .collect();

        let resolution_by_priority = diesel::sql_query(format!(
            r#"
            SELECT
                priority AS key,
                COUNT(*)::bigint AS count,
                AVG(EXTRACT(EPOCH FROM (resolved_at - created_at)) / 3600)::float8 AS avg_hours
            FROM requests
            WHERE resolved_at IS NOT NULL
              AND resolved_at >= NOW() - INTERVAL '{days} days'
            GROUP BY priority
            ORDER BY count DESC, priority
            "#
        ))
        .load::<PriorityResolutionRow>(&mut conn)
        .await
        .map_err(AppError::from)?
        .into_iter()
        .map(|row| PriorityResolution {
            priority: row.key,
            count: row.count,
            avg_hours: row.avg_hours,
        })
        .collect();

        let top_issues = diesel::sql_query(format!(
            r#"
            SELECT
                title AS key,
                COUNT(*)::bigint AS count,
                AVG(CASE
                    WHEN resolved_at IS NOT NULL
                    THEN EXTRACT(EPOCH FROM (resolved_at - created_at)) / 3600
                END)::float8 AS avg_hours
            FROM requests
            WHERE created_at >= NOW() - INTERVAL '{days} days'
            GROUP BY title
            ORDER BY count DESC, title
            LIMIT 5
            "#
        ))
        .load::<PriorityResolutionRow>(&mut conn)
        .await
        .map_err(AppError::from)?
        .into_iter()
        .map(|row| TopIssue {
            title: row.key,
            count: row.count,
            avg_hours: row.avg_hours,
        })
        .collect();

        let activity_by_weekday = diesel::sql_query(format!(
            r#"
            SELECT
                EXTRACT(ISODOW FROM created_at)::int AS weekday,
                COUNT(*)::bigint AS count
            FROM activity_log
            WHERE created_at >= NOW() - INTERVAL '{days} days'
            GROUP BY weekday
            ORDER BY weekday
            "#
        ))
        .load::<WeekdayRow>(&mut conn)
        .await
        .map_err(AppError::from)?
        .into_iter()
        .map(|row| WeekdayActivity {
            weekday: row.weekday,
            count: row.count,
        })
        .collect();

        Ok(AnalyticsOverview {
            period_days: days,
            total_employees: summary.total_employees,
            active_employees: summary.active_employees,
            total_computers: summary.total_computers,
            active_computers: summary.active_computers,
            total_requests: summary.total_requests,
            open_requests: summary.open_requests,
            requests_created_in_period: summary.requests_created_in_period,
            requests_resolved_in_period: summary.requests_resolved_in_period,
            avg_resolution_hours: summary.avg_resolution_hours,
            total_licenses: summary.total_licenses,
            used_licenses: summary.used_licenses,
            expiring_licenses: summary.expiring_licenses,
            expired_licenses: summary.expired_licenses,
            total_equipment: summary.total_equipment,
            equipment_needs_attention: summary.equipment_needs_attention,
            low_stock_consumables: summary.low_stock_consumables,
            requests_by_status,
            requests_by_priority,
            requests_by_category,
            resolution_by_priority,
            top_issues,
            activity_by_weekday,
        })
    }
}

#[derive(Debug, QueryableByName)]
struct AnalyticsSummaryRow {
    #[diesel(sql_type = BigInt)]
    total_employees: i64,
    #[diesel(sql_type = BigInt)]
    active_employees: i64,
    #[diesel(sql_type = BigInt)]
    total_computers: i64,
    #[diesel(sql_type = BigInt)]
    active_computers: i64,
    #[diesel(sql_type = BigInt)]
    total_requests: i64,
    #[diesel(sql_type = BigInt)]
    open_requests: i64,
    #[diesel(sql_type = BigInt)]
    requests_created_in_period: i64,
    #[diesel(sql_type = BigInt)]
    requests_resolved_in_period: i64,
    #[diesel(sql_type = Nullable<Double>)]
    avg_resolution_hours: Option<f64>,
    #[diesel(sql_type = BigInt)]
    total_licenses: i64,
    #[diesel(sql_type = BigInt)]
    used_licenses: i64,
    #[diesel(sql_type = BigInt)]
    expiring_licenses: i64,
    #[diesel(sql_type = BigInt)]
    expired_licenses: i64,
    #[diesel(sql_type = BigInt)]
    total_equipment: i64,
    #[diesel(sql_type = BigInt)]
    equipment_needs_attention: i64,
    #[diesel(sql_type = BigInt)]
    low_stock_consumables: i64,
}

#[derive(Debug, QueryableByName)]
struct CountRow {
    #[diesel(sql_type = Text)]
    key: String,
    #[diesel(sql_type = BigInt)]
    count: i64,
}

#[derive(Debug, QueryableByName)]
struct PriorityResolutionRow {
    #[diesel(sql_type = Text)]
    key: String,
    #[diesel(sql_type = BigInt)]
    count: i64,
    #[diesel(sql_type = Nullable<Double>)]
    avg_hours: Option<f64>,
}

#[derive(Debug, QueryableByName)]
struct WeekdayRow {
    #[diesel(sql_type = Integer)]
    weekday: i32,
    #[diesel(sql_type = BigInt)]
    count: i64,
}
