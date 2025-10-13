use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use poem_openapi::{param::Path, param::Query, payload::Json, OpenApi};

use crate::config::database::Pool;
use crate::middleware;
use crate::routes::employees::ErrorResponse;
use crate::models::schema::software_catalog;
use crate::models::software::{
    NewSoftware, Software, SoftwareListResponse, SoftwareResponse, UpdateSoftware,
};
use crate::services::activity_log_service::ActivityLogService;

pub struct SoftwareApi {
    db_pool: Pool,
    activity_log: ActivityLogService,
}

impl SoftwareApi {
    pub fn new(db_pool: Pool, activity_log: ActivityLogService) -> Self {
        Self {
            db_pool,
            activity_log,
        }
    }
}

#[derive(poem_openapi::ApiResponse)]
enum SoftwareListApiResponse {
    #[oai(status = 200)]
    Ok(Json<SoftwareListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(poem_openapi::ApiResponse)]
enum SoftwareDetailResponse {
    #[oai(status = 200)]
    Ok(Json<SoftwareResponse>),
    #[oai(status = 201)]
    Created(Json<SoftwareResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(poem_openapi::ApiResponse)]
enum SoftwareDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[OpenApi(prefix_path = "/software", tag = "crate::routes::api::ApiTags::Software")]
impl SoftwareApi {
    /// Get list of software
    #[oai(path = "/", method = "get")]
    async fn list_software(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(category): Query<Option<String>>,
        Query(search): Query<Option<String>>,
    ) -> SoftwareListApiResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).clamp(1, 100);
        let offset = (page - 1) * per_page;

        let mut conn = match self.db_pool.get().await {
            Ok(conn) => conn,
            Err(e) => {
                return SoftwareListApiResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }));
            }
        };

        // Build count query
        let mut count_query = software_catalog::table.into_boxed();
        
        if let Some(ref cat) = category {
            count_query = count_query.filter(software_catalog::category.eq(cat));
        }

        if let Some(ref search_term) = search {
            let pattern = format!("%{}%", search_term);
            count_query = count_query.filter(
                software_catalog::name
                    .ilike(pattern.clone())
                    .or(software_catalog::manufacturer.ilike(pattern)),
            );
        }

        // Get total count
        let total = match count_query
            .count()
            .get_result::<i64>(&mut conn)
            .await
        {
            Ok(count) => count,
            Err(e) => {
                return SoftwareListApiResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to count software: {}", e),
                }));
            }
        };

        // Build data query
        let mut data_query = software_catalog::table.into_boxed();
        
        if let Some(cat) = category {
            data_query = data_query.filter(software_catalog::category.eq(cat));
        }

        if let Some(search_term) = search {
            let pattern = format!("%{}%", search_term);
            data_query = data_query.filter(
                software_catalog::name
                    .ilike(pattern.clone())
                    .or(software_catalog::manufacturer.ilike(pattern)),
            );
        }

        // Get paginated results
        let software_list = match data_query
            .order(software_catalog::name.asc())
            .limit(per_page)
            .offset(offset)
            .load::<Software>(&mut conn)
            .await
        {
            Ok(list) => list,
            Err(e) => {
                return SoftwareListApiResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to fetch software: {}", e),
                }));
            }
        };

        let software: Vec<SoftwareResponse> =
            software_list.into_iter().map(|s| s.into()).collect();

        SoftwareListApiResponse::Ok(Json(SoftwareListResponse {
            software,
            total,
            page,
            per_page,
        }))
    }

    /// Get software by ID
    #[oai(path = "/:id", method = "get")]
    async fn get_software(&self, Path(id): Path<i32>) -> SoftwareDetailResponse {
        let mut conn = match self.db_pool.get().await {
            Ok(conn) => conn,
            Err(e) => {
                return SoftwareDetailResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }));
            }
        };

        match software_catalog::table
            .find(id)
            .first::<Software>(&mut conn)
            .await
        {
            Ok(software) => SoftwareDetailResponse::Ok(Json(software.into())),
            Err(diesel::result::Error::NotFound) => {
                SoftwareDetailResponse::NotFound(Json(ErrorResponse {
                    error: "not_found".to_string(),
                    message: format!("Software with id {} not found", id),
                }))
            }
            Err(e) => SoftwareDetailResponse::InternalError(Json(ErrorResponse {
                error: "database_error".to_string(),
                message: format!("Failed to fetch software: {}", e),
            })),
        }
    }

    /// Create new software
    #[oai(path = "/", method = "post")]
    async fn create_software(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(new_software): Json<NewSoftware>,
    ) -> SoftwareDetailResponse {
        let mut conn = match self.db_pool.get().await {
            Ok(conn) => conn,
            Err(e) => {
                return SoftwareDetailResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }));
            }
        };

        match diesel::insert_into(software_catalog::table)
            .values(&new_software)
            .get_result::<Software>(&mut conn)
            .await
        {
            Ok(software) => {
                // Log activity
                let details = serde_json::json!({
                    "software_name": software.name,
                    "manufacturer": software.manufacturer,
                    "category": software.category,
                });
                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "software",
                    software.id,
                    details,
                );

                SoftwareDetailResponse::Created(Json(software.into()))
            }
            Err(e) => SoftwareDetailResponse::InternalError(Json(ErrorResponse {
                error: "database_error".to_string(),
                message: format!("Failed to create software: {}", e),
            })),
        }
    }

    /// Update software
    #[oai(path = "/:id", method = "put")]
    async fn update_software(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(update_data): Json<UpdateSoftware>,
    ) -> SoftwareDetailResponse {
        let mut conn = match self.db_pool.get().await {
            Ok(conn) => conn,
            Err(e) => {
                return SoftwareDetailResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }));
            }
        };

        // Get old data for logging
        let old_software = match software_catalog::table
            .find(id)
            .first::<Software>(&mut conn)
            .await
        {
            Ok(s) => s,
            Err(diesel::result::Error::NotFound) => {
                return SoftwareDetailResponse::NotFound(Json(ErrorResponse {
                    error: "not_found".to_string(),
                    message: format!("Software with id {} not found", id),
                }));
            }
            Err(e) => {
                return SoftwareDetailResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to find software: {}", e),
                }));
            }
        };

        match diesel::update(software_catalog::table.find(id))
            .set(&update_data)
            .get_result::<Software>(&mut conn)
            .await
        {
            Ok(software) => {
                // Log changes
                let mut changes = serde_json::Map::new();
                
                if let Some(ref new_name) = update_data.name {
                    if &old_software.name != new_name {
                        changes.insert(
                            "name".to_string(),
                            serde_json::json!({
                                "old": old_software.name,
                                "new": new_name
                            }),
                        );
                    }
                }
                
                if let Some(ref new_manufacturer) = update_data.manufacturer {
                    if old_software.manufacturer.as_ref() != Some(new_manufacturer) {
                        changes.insert(
                            "manufacturer".to_string(),
                            serde_json::json!({
                                "old": old_software.manufacturer,
                                "new": new_manufacturer
                            }),
                        );
                    }
                }
                
                if let Some(ref new_category) = update_data.category {
                    if old_software.category.as_ref() != Some(new_category) {
                        changes.insert(
                            "category".to_string(),
                            serde_json::json!({
                                "old": old_software.category,
                                "new": new_category
                            }),
                        );
                    }
                }

                // Always log if there are any changes
                if !changes.is_empty() {
                    let details = serde_json::json!({
                        "software_name": software.name,
                        "changes": changes,
                    });
                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "software",
                        software.id,
                        details,
                    );
                }

                SoftwareDetailResponse::Ok(Json(software.into()))
            }
            Err(e) => SoftwareDetailResponse::InternalError(Json(ErrorResponse {
                error: "database_error".to_string(),
                message: format!("Failed to update software: {}", e),
            })),
        }
    }

    /// Delete software
    #[oai(path = "/:id", method = "delete")]
    async fn delete_software(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> SoftwareDeleteResponse {
        let mut conn = match self.db_pool.get().await {
            Ok(conn) => conn,
            Err(e) => {
                return SoftwareDeleteResponse::InternalError(Json(ErrorResponse {
                    error: "database_error".to_string(),
                    message: format!("Failed to get database connection: {}", e),
                }));
            }
        };

        // Get software name before deletion
        let software_name = software_catalog::table
            .find(id)
            .select(software_catalog::name)
            .first::<String>(&mut conn)
            .await
            .ok();

        let result = diesel::delete(software_catalog::table.find(id))
            .execute(&mut conn)
            .await;

        match result {
            Ok(0) => SoftwareDeleteResponse::NotFound(Json(ErrorResponse {
                error: "not_found".to_string(),
                message: format!("Software with id {} not found", id),
            })),
            Ok(_) => {
                // Log activity
                if let Some(name) = software_name {
                    let details = serde_json::json!({ "software_name": name });
                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "software",
                        id,
                        details,
                    );
                }
                SoftwareDeleteResponse::NoContent
            }
            Err(e) => SoftwareDeleteResponse::InternalError(Json(ErrorResponse {
                error: "database_error".to_string(),
                message: format!("Failed to delete software: {}", e),
            })),
        }
    }
}
