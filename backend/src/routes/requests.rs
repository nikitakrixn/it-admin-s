use poem_openapi::{ApiResponse, OpenApi, param::Path, param::Query, payload::Json};

use crate::config::database::Pool;
use crate::middleware;
use crate::models::request::{
    NewRequestComment, NewRequestCommentRequest, NewRequestRequest, Request,
    RequestCommentListResponse, RequestCommentResponse, RequestListResponse, RequestResponse,
    UpdateRequestRequest,
};
use crate::repositories::request_repository::RequestRepository;
use crate::routes::api::ApiTags;
use crate::services::activity_log_service::ActivityLogService;
use crate::utils::change_tracker::ChangeTracker;
use crate::utils::error::{AppError, ErrorResponse};

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum RequestsListResponse {
    #[oai(status = 200)]
    Ok(Json<RequestListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum RequestDetailResponse {
    #[oai(status = 200)]
    Ok(Json<RequestResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum RequestCreateResponse {
    #[oai(status = 201)]
    Created(Json<RequestResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum RequestUpdateResponse {
    #[oai(status = 200)]
    Ok(Json<RequestResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum RequestDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum RequestCommentsListResponse {
    #[oai(status = 200)]
    Ok(Json<RequestCommentListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum RequestCommentCreateResponse {
    #[oai(status = 201)]
    Created(Json<RequestCommentResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum RequestCommentDeleteResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

pub struct RequestsApi {
    repository: RequestRepository,
    activity_log: ActivityLogService,
}

impl RequestsApi {
    pub fn new(db_pool: Pool) -> Self {
        Self {
            repository: RequestRepository::new(db_pool.clone()),
            activity_log: ActivityLogService::new(db_pool),
        }
    }

    fn request_to_response(request: Request) -> RequestResponse {
        RequestResponse::from(request)
    }
}

#[OpenApi(prefix_path = "/requests", tag = "ApiTags::Requests")]
impl RequestsApi {
    /// Get all requests with pagination
    #[oai(path = "/", method = "get")]
    async fn list_requests(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(status): Query<Option<String>>,
        Query(priority): Query<Option<String>>,
        Query(assignee_id): Query<Option<i32>>,
        Query(requester_id): Query<Option<i32>>,
    ) -> RequestsListResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self
            .repository
            .list_requests(page, per_page, status, priority, assignee_id, requester_id)
            .await
        {
            Ok((requests_list, total)) => {
                let responses: Vec<RequestResponse> = requests_list
                    .into_iter()
                    .map(Self::request_to_response)
                    .collect();

                RequestsListResponse::Ok(Json(RequestListResponse {
                    requests: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => RequestsListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get request by ID
    #[oai(path = "/:id", method = "get")]
    async fn get_request(&self, Path(id): Path<i32>) -> RequestDetailResponse {
        match self.repository.get_request_by_id(id).await {
            Ok(request) => RequestDetailResponse::Ok(Json(Self::request_to_response(request))),
            Err(e) => match e {
                AppError::NotFound(_) => {
                    RequestDetailResponse::NotFound(Json(e.to_error_response()))
                }
                _ => RequestDetailResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new request
    #[oai(path = "/", method = "post")]
    async fn create_request(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewRequestRequest>,
    ) -> RequestCreateResponse {
        let new_request = match req.into_new_request() {
            Ok(r) => r,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return RequestCreateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.repository.create_request(new_request).await {
            Ok(request) => {
                let details = serde_json::json!({
                    "title": request.title,
                    "priority": request.priority,
                    "status": request.status,
                    "category": request.category,
                    "requester_id": request.requester_id,
                    "assignee_id": request.assignee_id,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "request",
                    request.id,
                    details,
                );

                RequestCreateResponse::Created(Json(Self::request_to_response(request)))
            }
            Err(e) => RequestCreateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update request
    #[oai(path = "/:id", method = "put")]
    async fn update_request(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateRequestRequest>,
    ) -> RequestUpdateResponse {
        let old_request = match self.repository.get_request_by_id(id).await {
            Ok(r) => r,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        RequestUpdateResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => RequestUpdateResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let update_data = match req.into_update_request() {
            Ok(data) => data,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return RequestUpdateResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.repository.update_request(id, update_data).await {
            Ok(request) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("title", &old_request.title, &request.title);
                tracker.track_string(
                    "description",
                    &old_request.description,
                    &request.description,
                );
                tracker.track_string("priority", &old_request.priority, &request.priority);
                tracker.track_string("status", &old_request.status, &request.status);
                tracker.track_option_string("category", &old_request.category, &request.category);
                tracker.track_option_i32(
                    "assignee_id",
                    &old_request.assignee_id,
                    &request.assignee_id,
                );
                tracker.track_option_i32(
                    "computer_id",
                    &old_request.computer_id,
                    &request.computer_id,
                );
                tracker.track_option_i32(
                    "equipment_id",
                    &old_request.equipment_id,
                    &request.equipment_id,
                );

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "title": request.title,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "request",
                        request.id,
                        details,
                    );
                }

                RequestUpdateResponse::Ok(Json(Self::request_to_response(request)))
            }
            Err(e) => RequestUpdateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete request
    #[oai(path = "/:id", method = "delete")]
    async fn delete_request(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> RequestDeleteResponse {
        let request_info = self.repository.get_request_by_id(id).await.ok();

        match self.repository.delete_request(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("Request with id {} not found", id));
                RequestDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(request) = request_info {
                    let details = serde_json::json!({
                        "title": request.title,
                        "priority": request.priority,
                        "status": request.status,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "request",
                        id,
                        details,
                    );
                }
                RequestDeleteResponse::NoContent
            }
            Err(e) => RequestDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// List comments of a request
    #[oai(path = "/:id/comments", method = "get")]
    async fn list_request_comments(&self, Path(id): Path<i32>) -> RequestCommentsListResponse {
        match self.repository.list_comments(id).await {
            Ok(comments) => {
                let responses: Vec<RequestCommentResponse> = comments
                    .into_iter()
                    .map(|(comment, email)| RequestCommentResponse::new(comment, email))
                    .collect();

                RequestCommentsListResponse::Ok(Json(RequestCommentListResponse {
                    total: responses.len() as i64,
                    comments: responses,
                }))
            }
            Err(e) => RequestCommentsListResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Add a comment to a request
    #[oai(path = "/:id/comments", method = "post")]
    async fn create_request_comment(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<NewRequestCommentRequest>,
    ) -> RequestCommentCreateResponse {
        let comment = req.comment.trim().to_string();
        if comment.is_empty() {
            let err = AppError::ValidationError("Comment cannot be empty".to_string());
            return RequestCommentCreateResponse::BadRequest(Json(err.to_error_response()));
        }

        if let Err(e) = self.repository.get_request_by_id(id).await {
            return match e {
                AppError::NotFound(_) => {
                    RequestCommentCreateResponse::NotFound(Json(e.to_error_response()))
                }
                _ => RequestCommentCreateResponse::InternalError(Json(e.to_error_response())),
            };
        }

        use crate::middleware::auth::ClaimsExt;
        let Some(user_id) = auth.0.user_id() else {
            let err = AppError::ValidationError("Invalid user id in token".to_string());
            return RequestCommentCreateResponse::BadRequest(Json(err.to_error_response()));
        };

        let new_comment = NewRequestComment {
            request_id: id,
            user_id,
            comment,
            is_internal: req.is_internal,
        };

        match self.repository.create_comment(new_comment).await {
            Ok(comment) => RequestCommentCreateResponse::Created(Json(
                RequestCommentResponse::new(comment, Some(auth.0.email.clone())),
            )),
            Err(e) => RequestCommentCreateResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete a request comment
    #[oai(path = "/comments/:comment_id", method = "delete")]
    async fn delete_request_comment(
        &self,
        _auth: middleware::auth::AdminAuth,
        Path(comment_id): Path<i32>,
    ) -> RequestCommentDeleteResponse {
        match self.repository.delete_comment(comment_id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("Comment with id {} not found", comment_id));
                RequestCommentDeleteResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => RequestCommentDeleteResponse::NoContent,
            Err(e) => RequestCommentDeleteResponse::InternalError(Json(e.to_error_response())),
        }
    }
}
