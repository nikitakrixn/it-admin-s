use poem_openapi::{ApiResponse, OpenApi, param::Path, param::Query, payload::Json};

use crate::config::database::Pool;
use crate::middleware;
use crate::models::ad::{
    AdGroupListResponse, AdGroupMembershipListResponse, AdGroupMembershipResponse, AdGroupResponse,
    AdSyncHistoryListResponse, AdSyncHistoryResponse, DomainAccountListResponse,
    DomainAccountResponse, NewAdGroupMembershipRequest, NewAdGroupRequest, NewAdSyncHistoryRequest,
    NewDomainAccountRequest, UpdateAdGroupMembershipRequest, UpdateAdGroupRequest,
    UpdateAdSyncHistoryRequest, UpdateDomainAccountRequest,
};
use crate::repositories::ad_repository::{
    AdGroupMembershipRepository, AdGroupRepository, AdSyncHistoryRepository,
    DomainAccountRepository,
};
use crate::routes::api::ApiTags;
use crate::services::activity_log_service::ActivityLogService;
use crate::utils::change_tracker::ChangeTracker;
use crate::utils::error::{AppError, ErrorResponse};

// ============================================================================
// AD group response types
// ============================================================================

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum AdGroupsListApiResponse {
    #[oai(status = 200)]
    Ok(Json<AdGroupListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum AdGroupDetailApiResponse {
    #[oai(status = 200)]
    Ok(Json<AdGroupResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum AdGroupCreateApiResponse {
    #[oai(status = 201)]
    Created(Json<AdGroupResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum AdGroupUpdateApiResponse {
    #[oai(status = 200)]
    Ok(Json<AdGroupResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum AdGroupDeleteApiResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// AD group membership response types
// ============================================================================

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum AdGroupMembershipsListApiResponse {
    #[oai(status = 200)]
    Ok(Json<AdGroupMembershipListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum AdGroupMembershipDetailApiResponse {
    #[oai(status = 200)]
    Ok(Json<AdGroupMembershipResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum AdGroupMembershipCreateApiResponse {
    #[oai(status = 201)]
    Created(Json<AdGroupMembershipResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum AdGroupMembershipUpdateApiResponse {
    #[oai(status = 200)]
    Ok(Json<AdGroupMembershipResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum AdGroupMembershipDeleteApiResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// Domain account response types
// ============================================================================

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum DomainAccountsListApiResponse {
    #[oai(status = 200)]
    Ok(Json<DomainAccountListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum DomainAccountDetailApiResponse {
    #[oai(status = 200)]
    Ok(Json<DomainAccountResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum DomainAccountCreateApiResponse {
    #[oai(status = 201)]
    Created(Json<DomainAccountResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum DomainAccountUpdateApiResponse {
    #[oai(status = 200)]
    Ok(Json<DomainAccountResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum DomainAccountDeleteApiResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// AD sync history response types
// ============================================================================

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum AdSyncHistoryListApiResponse {
    #[oai(status = 200)]
    Ok(Json<AdSyncHistoryListResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum AdSyncHistoryDetailApiResponse {
    #[oai(status = 200)]
    Ok(Json<AdSyncHistoryResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum AdSyncHistoryCreateApiResponse {
    #[oai(status = 201)]
    Created(Json<AdSyncHistoryResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
pub enum AdSyncHistoryUpdateApiResponse {
    #[oai(status = 200)]
    Ok(Json<AdSyncHistoryResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
pub enum AdSyncHistoryDeleteApiResponse {
    #[oai(status = 204)]
    NoContent,
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
}

// ============================================================================
// API implementation РІР‚вЂќ single OpenApi block with prefix /ad
// ============================================================================

pub struct AdApi {
    group_repo: AdGroupRepository,
    membership_repo: AdGroupMembershipRepository,
    account_repo: DomainAccountRepository,
    sync_repo: AdSyncHistoryRepository,
    activity_log: ActivityLogService,
}

impl AdApi {
    pub fn new(db_pool: Pool) -> Self {
        Self {
            group_repo: AdGroupRepository::new(db_pool.clone()),
            membership_repo: AdGroupMembershipRepository::new(db_pool.clone()),
            account_repo: DomainAccountRepository::new(db_pool.clone()),
            sync_repo: AdSyncHistoryRepository::new(db_pool.clone()),
            activity_log: ActivityLogService::new(db_pool),
        }
    }
}

#[OpenApi(prefix_path = "/ad", tag = "ApiTags::ActiveDirectory")]
impl AdApi {
    // ------------------------------------------------------------------------
    // AD groups РІР‚вЂќ GET /ad/groups
    // ------------------------------------------------------------------------
    /// Get all AD groups with pagination
    #[oai(path = "/groups", method = "get")]
    async fn list_ad_groups(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
    ) -> AdGroupsListApiResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self.group_repo.list_ad_groups(page, per_page).await {
            Ok((groups, total)) => {
                let responses: Vec<AdGroupResponse> =
                    groups.into_iter().map(AdGroupResponse::from).collect();

                AdGroupsListApiResponse::Ok(Json(AdGroupListResponse {
                    groups: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => AdGroupsListApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get AD group by ID
    #[oai(path = "/groups/:id", method = "get")]
    async fn get_ad_group(&self, Path(id): Path<i32>) -> AdGroupDetailApiResponse {
        match self.group_repo.get_ad_group_by_id(id).await {
            Ok(group) => AdGroupDetailApiResponse::Ok(Json(AdGroupResponse::from(group))),
            Err(e) => match e {
                AppError::NotFound(_) => {
                    AdGroupDetailApiResponse::NotFound(Json(e.to_error_response()))
                }
                _ => AdGroupDetailApiResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new AD group
    #[oai(path = "/groups", method = "post")]
    async fn create_ad_group(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewAdGroupRequest>,
    ) -> AdGroupCreateApiResponse {
        let new_group = req.into_new_ad_group();

        match self.group_repo.create_ad_group(new_group).await {
            Ok(group) => {
                let details = serde_json::json!({
                    "name": group.name,
                    "group_type": group.group_type,
                    "group_scope": group.group_scope,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "ad_group",
                    group.id,
                    details,
                );

                AdGroupCreateApiResponse::Created(Json(AdGroupResponse::from(group)))
            }
            Err(e) => AdGroupCreateApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update AD group
    #[oai(path = "/groups/:id", method = "put")]
    async fn update_ad_group(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateAdGroupRequest>,
    ) -> AdGroupUpdateApiResponse {
        let old_group = match self.group_repo.get_ad_group_by_id(id).await {
            Ok(g) => g,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        AdGroupUpdateApiResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => AdGroupUpdateApiResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let update_data = req.into_update_ad_group();

        match self.group_repo.update_ad_group(id, update_data).await {
            Ok(group) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("name", &old_group.name, &group.name);
                tracker.track_option_string(
                    "distinguished_name",
                    &old_group.distinguished_name,
                    &group.distinguished_name,
                );
                tracker.track_option_string(
                    "description",
                    &old_group.description,
                    &group.description,
                );
                tracker.track_option_string("group_type", &old_group.group_type, &group.group_type);
                tracker.track_option_string(
                    "group_scope",
                    &old_group.group_scope,
                    &group.group_scope,
                );

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "group_name": group.name,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "ad_group",
                        group.id,
                        details,
                    );
                }

                AdGroupUpdateApiResponse::Ok(Json(AdGroupResponse::from(group)))
            }
            Err(e) => AdGroupUpdateApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete AD group
    #[oai(path = "/groups/:id", method = "delete")]
    async fn delete_ad_group(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> AdGroupDeleteApiResponse {
        let group_info = self.group_repo.get_ad_group_by_id(id).await.ok();

        match self.group_repo.delete_ad_group(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("AD group with id {} not found", id));
                AdGroupDeleteApiResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(group) = group_info {
                    let details = serde_json::json!({
                        "name": group.name,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "ad_group",
                        id,
                        details,
                    );
                }
                AdGroupDeleteApiResponse::NoContent
            }
            Err(e) => AdGroupDeleteApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    // ------------------------------------------------------------------------
    // AD group memberships РІР‚вЂќ GET /ad/group-memberships
    // ------------------------------------------------------------------------
    /// Get all AD group memberships with pagination
    #[oai(path = "/group-memberships", method = "get")]
    async fn list_ad_group_memberships(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(ad_group_id): Query<Option<i32>>,
    ) -> AdGroupMembershipsListApiResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self
            .membership_repo
            .list_ad_group_memberships(page, per_page, ad_group_id)
            .await
        {
            Ok((memberships, total)) => {
                let responses: Vec<AdGroupMembershipResponse> = memberships
                    .into_iter()
                    .map(AdGroupMembershipResponse::from)
                    .collect();

                AdGroupMembershipsListApiResponse::Ok(Json(AdGroupMembershipListResponse {
                    memberships: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => AdGroupMembershipsListApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get AD group membership by ID
    #[oai(path = "/group-memberships/:id", method = "get")]
    async fn get_ad_group_membership(
        &self,
        Path(id): Path<i32>,
    ) -> AdGroupMembershipDetailApiResponse {
        match self.membership_repo.get_ad_group_membership_by_id(id).await {
            Ok(membership) => AdGroupMembershipDetailApiResponse::Ok(Json(
                AdGroupMembershipResponse::from(membership),
            )),
            Err(e) => match e {
                AppError::NotFound(_) => {
                    AdGroupMembershipDetailApiResponse::NotFound(Json(e.to_error_response()))
                }
                _ => AdGroupMembershipDetailApiResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new AD group membership
    #[oai(path = "/group-memberships", method = "post")]
    async fn create_ad_group_membership(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewAdGroupMembershipRequest>,
    ) -> AdGroupMembershipCreateApiResponse {
        let new_membership = req.into_new_ad_group_membership();

        match self
            .membership_repo
            .create_ad_group_membership(new_membership)
            .await
        {
            Ok(membership) => {
                let details = serde_json::json!({
                    "ad_group_id": membership.ad_group_id,
                    "employee_id": membership.employee_id,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "ad_group_membership",
                    membership.id,
                    details,
                );

                AdGroupMembershipCreateApiResponse::Created(Json(AdGroupMembershipResponse::from(
                    membership,
                )))
            }
            Err(e) => {
                AdGroupMembershipCreateApiResponse::InternalError(Json(e.to_error_response()))
            }
        }
    }

    /// Update AD group membership
    #[oai(path = "/group-memberships/:id", method = "put")]
    async fn update_ad_group_membership(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateAdGroupMembershipRequest>,
    ) -> AdGroupMembershipUpdateApiResponse {
        let old_membership = match self.membership_repo.get_ad_group_membership_by_id(id).await {
            Ok(m) => m,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        AdGroupMembershipUpdateApiResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => AdGroupMembershipUpdateApiResponse::InternalError(Json(
                        e.to_error_response(),
                    )),
                };
            }
        };

        let update_data = req.into_update_ad_group_membership();

        match self
            .membership_repo
            .update_ad_group_membership(id, update_data)
            .await
        {
            Ok(membership) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string(
                    "ad_group_id",
                    &old_membership.ad_group_id.to_string(),
                    &membership.ad_group_id.to_string(),
                );
                tracker.track_string(
                    "employee_id",
                    &old_membership.employee_id.to_string(),
                    &membership.employee_id.to_string(),
                );

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "membership_id": membership.id,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "ad_group_membership",
                        membership.id,
                        details,
                    );
                }

                AdGroupMembershipUpdateApiResponse::Ok(Json(AdGroupMembershipResponse::from(
                    membership,
                )))
            }
            Err(e) => {
                AdGroupMembershipUpdateApiResponse::InternalError(Json(e.to_error_response()))
            }
        }
    }

    /// Delete AD group membership
    #[oai(path = "/group-memberships/:id", method = "delete")]
    async fn delete_ad_group_membership(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> AdGroupMembershipDeleteApiResponse {
        let membership_info = self
            .membership_repo
            .get_ad_group_membership_by_id(id)
            .await
            .ok();

        match self.membership_repo.delete_ad_group_membership(id).await {
            Ok(0) => {
                let err =
                    AppError::NotFound(format!("AD group membership with id {} not found", id));
                AdGroupMembershipDeleteApiResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(membership) = membership_info {
                    let details = serde_json::json!({
                        "ad_group_id": membership.ad_group_id,
                        "employee_id": membership.employee_id,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "ad_group_membership",
                        id,
                        details,
                    );
                }
                AdGroupMembershipDeleteApiResponse::NoContent
            }
            Err(e) => {
                AdGroupMembershipDeleteApiResponse::InternalError(Json(e.to_error_response()))
            }
        }
    }

    // ------------------------------------------------------------------------
    // Domain accounts РІР‚вЂќ GET /ad/accounts
    // ------------------------------------------------------------------------
    /// Get all domain accounts with pagination
    #[oai(path = "/accounts", method = "get")]
    async fn list_domain_accounts(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
        Query(account_status): Query<Option<String>>,
    ) -> DomainAccountsListApiResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self
            .account_repo
            .list_domain_accounts(page, per_page, account_status)
            .await
        {
            Ok((accounts, total)) => {
                let responses: Vec<DomainAccountResponse> = accounts
                    .into_iter()
                    .map(DomainAccountResponse::from)
                    .collect();

                DomainAccountsListApiResponse::Ok(Json(DomainAccountListResponse {
                    accounts: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => DomainAccountsListApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get domain account by ID
    #[oai(path = "/accounts/:id", method = "get")]
    async fn get_domain_account(&self, Path(id): Path<i32>) -> DomainAccountDetailApiResponse {
        match self.account_repo.get_domain_account_by_id(id).await {
            Ok(account) => {
                DomainAccountDetailApiResponse::Ok(Json(DomainAccountResponse::from(account)))
            }
            Err(e) => match e {
                AppError::NotFound(_) => {
                    DomainAccountDetailApiResponse::NotFound(Json(e.to_error_response()))
                }
                _ => DomainAccountDetailApiResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new domain account
    #[oai(path = "/accounts", method = "post")]
    async fn create_domain_account(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewDomainAccountRequest>,
    ) -> DomainAccountCreateApiResponse {
        let new_account = match req.into_new_domain_account() {
            Ok(a) => a,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return DomainAccountCreateApiResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.account_repo.create_domain_account(new_account).await {
            Ok(account) => {
                let details = serde_json::json!({
                    "username": account.username,
                    "user_principal_name": account.user_principal_name,
                    "account_type": account.account_type,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "domain_account",
                    account.id,
                    details,
                );

                DomainAccountCreateApiResponse::Created(Json(DomainAccountResponse::from(account)))
            }
            Err(e) => DomainAccountCreateApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update domain account
    #[oai(path = "/accounts/:id", method = "put")]
    async fn update_domain_account(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateDomainAccountRequest>,
    ) -> DomainAccountUpdateApiResponse {
        let old_account = match self.account_repo.get_domain_account_by_id(id).await {
            Ok(a) => a,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        DomainAccountUpdateApiResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => DomainAccountUpdateApiResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let update_data = match req.into_update_domain_account() {
            Ok(d) => d,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return DomainAccountUpdateApiResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self
            .account_repo
            .update_domain_account(id, update_data)
            .await
        {
            Ok(account) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_string("username", &old_account.username, &account.username);
                tracker.track_string(
                    "account_type",
                    &old_account.account_type,
                    &account.account_type,
                );

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "username": account.username,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "domain_account",
                        account.id,
                        details,
                    );
                }

                DomainAccountUpdateApiResponse::Ok(Json(DomainAccountResponse::from(account)))
            }
            Err(e) => DomainAccountUpdateApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete domain account
    #[oai(path = "/accounts/:id", method = "delete")]
    async fn delete_domain_account(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> DomainAccountDeleteApiResponse {
        let account_info = self.account_repo.get_domain_account_by_id(id).await.ok();

        match self.account_repo.delete_domain_account(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("Domain account with id {} not found", id));
                DomainAccountDeleteApiResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(account) = account_info {
                    let details = serde_json::json!({
                        "username": account.username,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "domain_account",
                        id,
                        details,
                    );
                }
                DomainAccountDeleteApiResponse::NoContent
            }
            Err(e) => DomainAccountDeleteApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    // ------------------------------------------------------------------------
    // AD sync history РІР‚вЂќ GET /ad/sync-history
    // ------------------------------------------------------------------------
    /// Get all AD sync history with pagination
    #[oai(path = "/sync-history", method = "get")]
    async fn list_ad_sync_history(
        &self,
        Query(page): Query<Option<i64>>,
        Query(per_page): Query<Option<i64>>,
    ) -> AdSyncHistoryListApiResponse {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).min(100);

        match self.sync_repo.list_ad_sync_history(page, per_page).await {
            Ok((syncs, total)) => {
                let responses: Vec<AdSyncHistoryResponse> =
                    syncs.into_iter().map(AdSyncHistoryResponse::from).collect();

                AdSyncHistoryListApiResponse::Ok(Json(AdSyncHistoryListResponse {
                    syncs: responses,
                    total,
                    page,
                    per_page,
                }))
            }
            Err(e) => AdSyncHistoryListApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Get AD sync history by ID
    #[oai(path = "/sync-history/:id", method = "get")]
    async fn get_ad_sync_history(&self, Path(id): Path<i32>) -> AdSyncHistoryDetailApiResponse {
        match self.sync_repo.get_ad_sync_history_by_id(id).await {
            Ok(sync) => AdSyncHistoryDetailApiResponse::Ok(Json(AdSyncHistoryResponse::from(sync))),
            Err(e) => match e {
                AppError::NotFound(_) => {
                    AdSyncHistoryDetailApiResponse::NotFound(Json(e.to_error_response()))
                }
                _ => AdSyncHistoryDetailApiResponse::InternalError(Json(e.to_error_response())),
            },
        }
    }

    /// Create new AD sync history record
    #[oai(path = "/sync-history", method = "post")]
    async fn create_ad_sync_history(
        &self,
        auth: middleware::auth::AdminAuth,
        Json(req): Json<NewAdSyncHistoryRequest>,
    ) -> AdSyncHistoryCreateApiResponse {
        let new_sync = match req.into_new_ad_sync_history() {
            Ok(s) => s,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return AdSyncHistoryCreateApiResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.sync_repo.create_ad_sync_history(new_sync).await {
            Ok(sync) => {
                let details = serde_json::json!({
                    "sync_type": sync.sync_type,
                    "records_added": sync.records_added,
                    "status": sync.status,
                });

                use crate::middleware::auth::ClaimsExt;
                self.activity_log.log_with_details_async(
                    auth.0.user_id(),
                    "created",
                    "ad_sync_history",
                    sync.id,
                    details,
                );

                AdSyncHistoryCreateApiResponse::Created(Json(AdSyncHistoryResponse::from(sync)))
            }
            Err(e) => AdSyncHistoryCreateApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Update AD sync history record
    #[oai(path = "/sync-history/:id", method = "put")]
    async fn update_ad_sync_history(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
        Json(req): Json<UpdateAdSyncHistoryRequest>,
    ) -> AdSyncHistoryUpdateApiResponse {
        let old_sync = match self.sync_repo.get_ad_sync_history_by_id(id).await {
            Ok(s) => s,
            Err(e) => {
                return match e {
                    AppError::NotFound(_) => {
                        AdSyncHistoryUpdateApiResponse::NotFound(Json(e.to_error_response()))
                    }
                    _ => AdSyncHistoryUpdateApiResponse::InternalError(Json(e.to_error_response())),
                };
            }
        };

        let update_data = match req.into_update_ad_sync_history() {
            Ok(d) => d,
            Err(e) => {
                let err = AppError::ValidationError(e);
                return AdSyncHistoryUpdateApiResponse::BadRequest(Json(err.to_error_response()));
            }
        };

        match self.sync_repo.update_ad_sync_history(id, update_data).await {
            Ok(sync) => {
                let mut tracker = ChangeTracker::new();
                tracker.track_option_string("sync_type", &old_sync.sync_type, &sync.sync_type);
                tracker.track_option_i32(
                    "records_added",
                    &old_sync.records_added,
                    &sync.records_added,
                );
                tracker.track_option_i32(
                    "records_updated",
                    &old_sync.records_updated,
                    &sync.records_updated,
                );
                tracker.track_option_string("status", &old_sync.status, &sync.status);

                if tracker.has_changes() {
                    let details = serde_json::json!({
                        "sync_type": sync.sync_type,
                        "changes": tracker.into_json(),
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "updated",
                        "ad_sync_history",
                        sync.id,
                        details,
                    );
                }

                AdSyncHistoryUpdateApiResponse::Ok(Json(AdSyncHistoryResponse::from(sync)))
            }
            Err(e) => AdSyncHistoryUpdateApiResponse::InternalError(Json(e.to_error_response())),
        }
    }

    /// Delete AD sync history record
    #[oai(path = "/sync-history/:id", method = "delete")]
    async fn delete_ad_sync_history(
        &self,
        auth: middleware::auth::AdminAuth,
        Path(id): Path<i32>,
    ) -> AdSyncHistoryDeleteApiResponse {
        let sync_info = self.sync_repo.get_ad_sync_history_by_id(id).await.ok();

        match self.sync_repo.delete_ad_sync_history(id).await {
            Ok(0) => {
                let err = AppError::NotFound(format!("AD sync history with id {} not found", id));
                AdSyncHistoryDeleteApiResponse::NotFound(Json(err.to_error_response()))
            }
            Ok(_) => {
                if let Some(sync) = sync_info {
                    let details = serde_json::json!({
                        "sync_type": sync.sync_type,
                        "status": sync.status,
                    });

                    use crate::middleware::auth::ClaimsExt;
                    self.activity_log.log_with_details_async(
                        auth.0.user_id(),
                        "deleted",
                        "ad_sync_history",
                        id,
                        details,
                    );
                }
                AdSyncHistoryDeleteApiResponse::NoContent
            }
            Err(e) => AdSyncHistoryDeleteApiResponse::InternalError(Json(e.to_error_response())),
        }
    }
}
