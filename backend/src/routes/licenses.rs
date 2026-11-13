use poem_openapi::{ApiResponse, OpenApi, param::Path, param::Query, payload::Json};

use crate::config::database::Pool;
use crate::middleware;
use crate::models::license_pool::{
    LicensePool, LicensePoolListResponse, LicensePoolResponse, NewLicensePoolRequest,
    UpdateLicensePoolRequest,
};
use crate::repositories::license_pool_repository::LicensePoolRepository;
use crate::routes::api::ApiTags;
use crate::services::activity_log_service::ActivityLogService;
use crate::utils::change_tracker::ChangeTracker;
use crate::utils::error::{AppError, ErrorResponse};

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum LicensesListResponse {
    #[oai(status = 200)]
    Ok(Json<LicensePoolListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum LicenseDetailResponse {
    #[oai(status = 200)]
    Ok(Json<LicensePoolResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum LicenseCreateResponse {
    #[oai(status = 201)]
    Created(Json<LicensePoolResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum LicenseUpdateResponse {
    #[oai(status = 200)]
    Ok(Json<LicensePoolResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum LicenseDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

pub struct LicensesApi {
    repository: LicensePoolRepository,
    activity_log: ActivityLogService,
}

impl LicensesApi {
    pub fn new(db_pool: Pool) -> Self {
        Self {
            repository: LicensePoolRepository::new(db_pool.clone()),
            activity_log: ActivityLogService::new(db_pool),
        }
    }

    fn license_to_response(pool: LicensePool) -> LicensePoolResponse {
        LicensePoolResponse::from(pool)
    }
}

#[OpenApi(prefix_path = "/licenses", tag = "ApiTags::Licenses")]
impl LicensesApi {
    /// Get all license pools with pagination
    #[oai(path = "/", method = "get")]
    async fn list_licenses(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(license_type): Query<Option<String>>,
        Query(software_catalog_id): Query<Option<i32>>,
    ) -> LicensesListResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self
            .repository
            .list_license_pools(page, per_page, license_type, software_catalog_id)
            .await
        {
            Ok((licenses_list, total)) => {
                let responses: Vec<LicensePoolResponse> = licenses_list
                    .into_iter()
                    .map(Self::license_to_response)
                    .collect();

                LicensesListResponse::Ok(Json(LicensePoolListResponse {
                    licenses: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => LicensesListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get license pool by ID
    #[oai(path = "/:id", method = "get")]
    async fn get_license(&self, Path(id): Path<i32>) -> LicenseDetailResponse {
        match self.repository.get_license_pool_by_id(id).await {
            Ok(pool) => LicenseDetailResponse::Ok(Json(Self::license_to_response(pool))),
            Err(e) => match e {
                AppError::NotFound(_) => {
                    LicenseDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => LicenseDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new license pool
    #[oai(path = "/", method = "post")]
    async fn create_license(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewLicensePoolRequest>,
    ) -> LicenseCreateResponse {
        let new_pool = match req.into_new_license_pool() {
            Ok(p) => p,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return LicenseCreateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.repository.create_license_pool(new_pool).await {
            Ok(pool) => {
                let details = serde_json::json!({
                    "software_catalog_id": pool.software_catalog_id,
                    "license_type": pool.license_type,
                    "total_licenses": pool.total_licenses,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "license",
                    pool.id,
                    details,
                );

                LicenseCreateResponse::Created(Json(Self::license_to_response(pool)))
            }
            Err(e) => LicenseCreateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update license pool
    #[oai(path = "/:id", method = "put")]
    async fn update_license(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateLicensePoolRequest>,
    ) -> LicenseUpdateResponse {
        let old_pool = match self.repository.get_license_pool_by_id(id).await {
            Ok(p) => p,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        LicenseUpdateResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => LicenseUpdateResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let update_data = match req.into_update_license_pool() {
            Ok(data) => data,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return LicenseUpdateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.repository.update_license_pool(id, update_data).await {
            Ok(pool) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("license_type", &old_pool.license_type, &pool.license_type);
                tracker.track_string(
                    "total_licenses",
                    &old_pool.total_licenses.to_string(),
                    &pool.total_licenses.to_string(),
                );
                tracker.track_option_i32(
                    "used_licenses",
                    &old_pool.used_licenses,
                    &pool.used_licenses,
                );
                tracker.track_option_i32(
                    "available_licenses",
                    &old_pool.available_licenses,
                    &pool.available_licenses,
                );
                tracker.track_option_string(
                    "license_key",
                    &old_pool.license_key,
                    &pool.license_key,
                );
                tracker.track_option_string("notes", &old_pool.notes, &pool.notes);
                tracker.track_option_i32("vendor_id", &old_pool.vendor_id, &pool.vendor_id);
                tracker.track_option_i32("contract_id", &old_pool.contract_id, &pool.contract_id);
                tracker.track_option_bool(
                    "auto_renewal",
                    &old_pool.auto_renewal,
                    &pool.auto_renewal,
                );

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "license_type": pool.license_type,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "license",
                        pool.id,
                        details,
                    );
                }

                LicenseUpdateResponse::Ok(Json(Self::license_to_response(pool)))
            }
            Err(e) => LicenseUpdateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete license pool
    #[oai(path = "/:id", method = "delete")]
    async fn delete_license(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> LicenseDeleteResponse {
        let pool_info = self.repository.get_license_pool_by_id(id).await.ok();

        match self.repository.delete_license_pool(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("License with id {} not found", id));
                LicenseDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(pool) = pool_info {
                    let details = serde_json::json!({
                        "license_type": pool.license_type,
                        "total_licenses": pool.total_licenses,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "license",
                        id,
                        details,
                    );
                }
                LicenseDeleteResponse::NoContent
            }
            Err(e) => LicenseDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }
}
