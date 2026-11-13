use poem_openapi::{ApiResponse, OpenApi, param::Query, payload::Json};

use crate::config::database::Pool;
use crate::models::analytics::AnalyticsOverview;
use crate::models::dashboard::DashboardSummary;
use crate::models::schema::{
    computers, consumables, employees, equipment, license_pools, requests, software_catalog,
};
use crate::repositories::analytics_repository::AnalyticsRepository;
use crate::utils::db_helpers::get_connection;
use crate::utils::error::ErrorResponse;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;

#[derive(ApiResponse)]
pub enum DashboardSummaryResponse {
    #[oai(status = 200)]
    Ok(Json<DashboardSummary>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum AnalyticsOverviewResponse {
    #[oai(status = 200)]
    Ok(Json<AnalyticsOverview>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

pub struct DashboardApi {
    db_pool: Pool,
}

impl DashboardApi {
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }
}

#[OpenApi(
    prefix_path = "/dashboard",
    tag = "crate::routes::api::ApiTags::System"
)]
impl DashboardApi {
    /// Агрегированная сводка по основным разделам инфраструктуры для дашборда
    #[oai(path = "/summary", method = "get")]
    async fn get_dashboard_summary(&self) -> DashboardSummaryResponse {
        let mut conn = match get_connection(&self.db_pool).await {
            Ok(conn) => conn,
            Err(e) => return DashboardSummaryResponse::InternalError(Json(e.to_error_response())),
        };

        let total_computers = computers::table
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .unwrap_or(0);
        let total_employees = employees::table
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .unwrap_or(0);
        let total_requests = requests::table
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .unwrap_or(0);
        let total_licenses = license_pools::table
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .unwrap_or(0);
        let total_equipment = equipment::table
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .unwrap_or(0);
        let total_consumables = consumables::table
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .unwrap_or(0);
        let total_software = software_catalog::table
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .unwrap_or(0);

        DashboardSummaryResponse::Ok(Json(DashboardSummary {
            total_computers,
            total_employees,
            total_requests,
            total_licenses,
            total_equipment,
            total_consumables,
            total_software,
            extra: None,
        }))
    }

    /// Расширенная аналитика по инфраструктуре за указанный период
    #[oai(path = "/analytics", method = "get")]
    async fn get_analytics_overview(
        &self,
        Query(days): Query<Option<i64>>,
    ) -> AnalyticsOverviewResponse {
        let days = days.unwrap_or(30).clamp(1, 365);
        let repository = AnalyticsRepository::new(self.db_pool.clone());

        match repository.get_overview(days).await {
            Ok(overview) => AnalyticsOverviewResponse::Ok(Json(overview)),
            Err(e) => AnalyticsOverviewResponse::InternalError(Json(e.to_error_response())),
        }
    }
}
