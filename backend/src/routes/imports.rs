use poem_openapi::{ApiResponse, OpenApi, payload::Json};

use crate::config::database::Pool;
use crate::middleware::auth::{AdminAuth, ClaimsExt};
use crate::models::import::{ImportCsvRequest, ImportReport};
use crate::routes::api::ApiTags;
use crate::services::activity_log_service::ActivityLogService;
use crate::services::import_service::ImportService;
use crate::utils::error::{AppError, ErrorResponse};

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum ImportApiResponse {
    #[oai(status = 200)]
    Ok(Json<ImportReport>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

pub struct ImportApi {
    service: ImportService,
    activity_log: ActivityLogService,
}

impl ImportApi {
    pub fn new(db_pool: Pool) -> Self {
        Self {
            service: ImportService::new(db_pool.clone()),
            activity_log: ActivityLogService::new(db_pool),
        }
    }

    fn finish(
        report: ImportReport,
        dry_run: bool,
        action: &'static str,
        user_id: Option<uuid::Uuid>,
        activity_log: &ActivityLogService,
    ) -> ImportApiResponse {
        if !dry_run {
            activity_log.log_with_details_async(
                user_id,
                action,
                "import",
                0,
                serde_json::json!({
                    "total": report.total_rows,
                    "created": report.created,
                    "updated": report.updated,
                    "skipped": report.skipped,
                }),
            );
        }

        ImportApiResponse::Ok(Json(report))
    }
}

#[OpenApi(prefix_path = "/import", tag = "ApiTags::Import")]
impl ImportApi {
    /// Импорт сотрудников из CSV
    ///
    /// Совпадение ищется по email (без учёта регистра), затем по логину AD.
    /// Строки без обоих идентификаторов отклоняются, чтобы повторный импорт
    /// не создавал дубликаты. Подразделения и должности создаются по названию,
    /// если их ещё нет в справочнике.
    #[oai(path = "/employees", method = "post")]
    async fn import_employees(
        &self,
        auth: AdminAuth,
        Json(payload): Json<ImportCsvRequest>,
    ) -> ImportApiResponse {
        let dry_run = payload.dry_run.unwrap_or(false);

        match self.service.import_employees(&payload.csv, dry_run).await {
            Ok(report) => Self::finish(
                report,
                dry_run,
                "import_employees",
                auth.0.user_id(),
                &self.activity_log,
            ),
            Err(error @ AppError::ValidationError(_)) => {
                ImportApiResponse::BadRequest(Json(error.to_error_response()))
            }
            Err(error) => ImportApiResponse::InternalError(Json(error.to_error_response())),
        }
    }

    /// Импорт компьютеров из CSV
    ///
    /// Совпадение ищется по инвентарному номеру, затем по имени компьютера.
    /// Ответственный указывается email-адресом уже существующего сотрудника:
    /// сначала импортируйте сотрудников, затем компьютеры.
    #[oai(path = "/computers", method = "post")]
    async fn import_computers(
        &self,
        auth: AdminAuth,
        Json(payload): Json<ImportCsvRequest>,
    ) -> ImportApiResponse {
        let dry_run = payload.dry_run.unwrap_or(false);

        match self.service.import_computers(&payload.csv, dry_run).await {
            Ok(report) => Self::finish(
                report,
                dry_run,
                "import_computers",
                auth.0.user_id(),
                &self.activity_log,
            ),
            Err(error @ AppError::ValidationError(_)) => {
                ImportApiResponse::BadRequest(Json(error.to_error_response()))
            }
            Err(error) => ImportApiResponse::InternalError(Json(error.to_error_response())),
        }
    }
}
