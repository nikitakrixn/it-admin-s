use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::{ad_group_memberships, ad_groups, ad_sync_history, domain_accounts};

// ============================================================================
// Active Directory groups
// ============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = ad_groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AdGroup {
    pub id: i32,
    pub name: String,
    pub distinguished_name: Option<String>,
    pub description: Option<String>,
    pub group_type: Option<String>,
    pub group_scope: Option<String>,
    pub member_count: Option<i32>,
    pub last_synced: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = ad_groups)]
pub struct NewAdGroup {
    pub name: String,
    pub distinguished_name: Option<String>,
    pub description: Option<String>,
    pub group_type: Option<String>,
    pub group_scope: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = ad_groups)]
pub struct UpdateAdGroup {
    pub name: Option<String>,
    pub distinguished_name: Option<String>,
    pub description: Option<String>,
    pub group_type: Option<String>,
    pub group_scope: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewAdGroupRequest {
    pub name: String,
    pub distinguished_name: Option<String>,
    pub description: Option<String>,
    pub group_type: Option<String>,
    pub group_scope: Option<String>,
}

impl NewAdGroupRequest {
    pub fn into_new_ad_group(self) -> NewAdGroup {
        NewAdGroup {
            name: self.name,
            distinguished_name: self.distinguished_name,
            description: self.description,
            group_type: self.group_type,
            group_scope: self.group_scope,
        }
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateAdGroupRequest {
    pub name: Option<String>,
    pub distinguished_name: Option<String>,
    pub description: Option<String>,
    pub group_type: Option<String>,
    pub group_scope: Option<String>,
}

impl UpdateAdGroupRequest {
    pub fn into_update_ad_group(self) -> UpdateAdGroup {
        UpdateAdGroup {
            name: self.name,
            distinguished_name: self.distinguished_name,
            description: self.description,
            group_type: self.group_type,
            group_scope: self.group_scope,
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct AdGroupResponse {
    pub id: i32,
    pub name: String,
    pub distinguished_name: Option<String>,
    pub description: Option<String>,
    pub group_type: Option<String>,
    pub group_scope: Option<String>,
    pub member_count: Option<i32>,
    pub last_synced: Option<String>,
    pub created_at: String,
}

impl From<AdGroup> for AdGroupResponse {
    fn from(group: AdGroup) -> Self {
        let fmt =
            |dt: Option<NaiveDateTime>| dt.map(|d| d.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string());
        Self {
            id: group.id,
            name: group.name,
            distinguished_name: group.distinguished_name,
            description: group.description,
            group_type: group.group_type,
            group_scope: group.group_scope,
            member_count: group.member_count,
            last_synced: fmt(group.last_synced),
            created_at: group
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct AdGroupListResponse {
    pub groups: Vec<AdGroupResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

// ============================================================================
// AD group memberships
// ============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = ad_group_memberships)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AdGroupMembership {
    pub id: i32,
    pub employee_id: i32,
    pub ad_group_id: i32,
    pub added_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = ad_group_memberships)]
pub struct NewAdGroupMembership {
    pub employee_id: i32,
    pub ad_group_id: i32,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = ad_group_memberships)]
pub struct UpdateAdGroupMembership {
    pub employee_id: Option<i32>,
    pub ad_group_id: Option<i32>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewAdGroupMembershipRequest {
    pub employee_id: i32,
    pub ad_group_id: i32,
}

impl NewAdGroupMembershipRequest {
    pub fn into_new_ad_group_membership(self) -> NewAdGroupMembership {
        NewAdGroupMembership {
            employee_id: self.employee_id,
            ad_group_id: self.ad_group_id,
        }
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateAdGroupMembershipRequest {
    pub employee_id: Option<i32>,
    pub ad_group_id: Option<i32>,
}

impl UpdateAdGroupMembershipRequest {
    pub fn into_update_ad_group_membership(self) -> UpdateAdGroupMembership {
        UpdateAdGroupMembership {
            employee_id: self.employee_id,
            ad_group_id: self.ad_group_id,
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct AdGroupMembershipResponse {
    pub id: i32,
    pub employee_id: i32,
    pub ad_group_id: i32,
    pub added_at: Option<String>,
}

impl From<AdGroupMembership> for AdGroupMembershipResponse {
    fn from(membership: AdGroupMembership) -> Self {
        let fmt =
            |dt: Option<NaiveDateTime>| dt.map(|d| d.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string());
        Self {
            id: membership.id,
            employee_id: membership.employee_id,
            ad_group_id: membership.ad_group_id,
            added_at: fmt(membership.added_at),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct AdGroupMembershipListResponse {
    pub memberships: Vec<AdGroupMembershipResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

// ============================================================================
// Domain accounts
// ============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = domain_accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DomainAccount {
    pub id: i32,
    pub employee_id: Option<i32>,
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub account_type: String,
    pub distinguished_name: Option<String>,
    pub user_principal_name: Option<String>,
    pub sam_account_name: Option<String>,
    pub password_last_set: Option<NaiveDateTime>,
    pub password_expires_at: Option<NaiveDateTime>,
    pub password_never_expires: Option<bool>,
    pub must_change_password: Option<bool>,
    pub cannot_change_password: Option<bool>,
    pub account_expires_at: Option<NaiveDateTime>,
    pub is_enabled: Option<bool>,
    pub is_locked: Option<bool>,
    pub locked_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = domain_accounts)]
pub struct NewDomainAccount {
    pub employee_id: Option<i32>,
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub account_type: String,
    pub distinguished_name: Option<String>,
    pub user_principal_name: Option<String>,
    pub sam_account_name: Option<String>,
    pub password_last_set: Option<NaiveDateTime>,
    pub password_expires_at: Option<NaiveDateTime>,
    pub password_never_expires: Option<bool>,
    pub must_change_password: Option<bool>,
    pub cannot_change_password: Option<bool>,
    pub account_expires_at: Option<NaiveDateTime>,
    pub is_enabled: Option<bool>,
    pub is_locked: Option<bool>,
    pub locked_at: Option<NaiveDateTime>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = domain_accounts)]
pub struct UpdateDomainAccount {
    pub employee_id: Option<i32>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub account_type: Option<String>,
    pub distinguished_name: Option<String>,
    pub user_principal_name: Option<String>,
    pub sam_account_name: Option<String>,
    pub password_last_set: Option<NaiveDateTime>,
    pub password_expires_at: Option<NaiveDateTime>,
    pub password_never_expires: Option<bool>,
    pub must_change_password: Option<bool>,
    pub cannot_change_password: Option<bool>,
    pub account_expires_at: Option<NaiveDateTime>,
    pub is_enabled: Option<bool>,
    pub is_locked: Option<bool>,
    pub locked_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewDomainAccountRequest {
    pub employee_id: Option<i32>,
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub account_type: String,
    pub distinguished_name: Option<String>,
    pub user_principal_name: Option<String>,
    pub sam_account_name: Option<String>,
    pub password_last_set: Option<String>,
    pub password_expires_at: Option<String>,
    pub password_never_expires: Option<bool>,
    pub must_change_password: Option<bool>,
    pub cannot_change_password: Option<bool>,
    pub account_expires_at: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_locked: Option<bool>,
    pub locked_at: Option<String>,
}

impl NewDomainAccountRequest {
    pub fn into_new_domain_account(self) -> Result<NewDomainAccount, String> {
        let parse_datetime = |s: &Option<String>| -> Result<Option<NaiveDateTime>, String> {
            match s {
                Some(v) if !v.is_empty() => {
                    NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S%.3fZ")
                        .or_else(|_| NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%SZ"))
                        .map(Some)
                        .map_err(|e| format!("Invalid datetime format: {}", e))
                }
                _ => Ok(None),
            }
        };

        Ok(NewDomainAccount {
            employee_id: self.employee_id,
            username: self.username,
            display_name: self.display_name,
            email: self.email,
            account_type: self.account_type,
            distinguished_name: self.distinguished_name,
            user_principal_name: self.user_principal_name,
            sam_account_name: self.sam_account_name,
            password_last_set: parse_datetime(&self.password_last_set)?,
            password_expires_at: parse_datetime(&self.password_expires_at)?,
            password_never_expires: self.password_never_expires,
            must_change_password: self.must_change_password,
            cannot_change_password: self.cannot_change_password,
            account_expires_at: parse_datetime(&self.account_expires_at)?,
            is_enabled: self.is_enabled,
            is_locked: self.is_locked,
            locked_at: parse_datetime(&self.locked_at)?,
        })
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateDomainAccountRequest {
    pub employee_id: Option<i32>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub account_type: Option<String>,
    pub distinguished_name: Option<String>,
    pub user_principal_name: Option<String>,
    pub sam_account_name: Option<String>,
    pub password_last_set: Option<String>,
    pub password_expires_at: Option<String>,
    pub password_never_expires: Option<bool>,
    pub must_change_password: Option<bool>,
    pub cannot_change_password: Option<bool>,
    pub account_expires_at: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_locked: Option<bool>,
    pub locked_at: Option<String>,
}

impl UpdateDomainAccountRequest {
    pub fn into_update_domain_account(self) -> Result<UpdateDomainAccount, String> {
        let parse_datetime = |s: &Option<String>| -> Result<Option<NaiveDateTime>, String> {
            match s {
                Some(v) if !v.is_empty() => {
                    NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S%.3fZ")
                        .or_else(|_| NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%SZ"))
                        .map(Some)
                        .map_err(|e| format!("Invalid datetime format: {}", e))
                }
                _ => Ok(None),
            }
        };

        Ok(UpdateDomainAccount {
            employee_id: self.employee_id,
            username: self.username,
            display_name: self.display_name,
            email: self.email,
            account_type: self.account_type,
            distinguished_name: self.distinguished_name,
            user_principal_name: self.user_principal_name,
            sam_account_name: self.sam_account_name,
            password_last_set: parse_datetime(&self.password_last_set)?,
            password_expires_at: parse_datetime(&self.password_expires_at)?,
            password_never_expires: self.password_never_expires,
            must_change_password: self.must_change_password,
            cannot_change_password: self.cannot_change_password,
            account_expires_at: parse_datetime(&self.account_expires_at)?,
            is_enabled: self.is_enabled,
            is_locked: self.is_locked,
            locked_at: parse_datetime(&self.locked_at)?,
        })
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct DomainAccountResponse {
    pub id: i32,
    pub employee_id: Option<i32>,
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub account_type: String,
    pub distinguished_name: Option<String>,
    pub user_principal_name: Option<String>,
    pub sam_account_name: Option<String>,
    pub password_last_set: Option<String>,
    pub password_expires_at: Option<String>,
    pub password_never_expires: Option<bool>,
    pub must_change_password: Option<bool>,
    pub cannot_change_password: Option<bool>,
    pub account_expires_at: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_locked: Option<bool>,
    pub locked_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<DomainAccount> for DomainAccountResponse {
    fn from(account: DomainAccount) -> Self {
        let fmt =
            |dt: Option<NaiveDateTime>| dt.map(|d| d.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string());
        Self {
            id: account.id,
            employee_id: account.employee_id,
            username: account.username,
            display_name: account.display_name,
            email: account.email,
            account_type: account.account_type,
            distinguished_name: account.distinguished_name,
            user_principal_name: account.user_principal_name,
            sam_account_name: account.sam_account_name,
            password_last_set: fmt(account.password_last_set),
            password_expires_at: fmt(account.password_expires_at),
            password_never_expires: account.password_never_expires,
            must_change_password: account.must_change_password,
            cannot_change_password: account.cannot_change_password,
            account_expires_at: fmt(account.account_expires_at),
            is_enabled: account.is_enabled,
            is_locked: account.is_locked,
            locked_at: fmt(account.locked_at),
            created_at: account
                .created_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            updated_at: account
                .updated_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct DomainAccountListResponse {
    pub accounts: Vec<DomainAccountResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

// ============================================================================
// AD sync history
// ============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = ad_sync_history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AdSyncHistory {
    pub id: i32,
    pub sync_type: Option<String>,
    pub status: Option<String>,
    pub records_added: Option<i32>,
    pub records_updated: Option<i32>,
    pub records_removed: Option<i32>,
    pub error_message: Option<String>,
    pub sync_started_at: NaiveDateTime,
    pub sync_completed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = ad_sync_history)]
pub struct NewAdSyncHistory {
    pub sync_type: Option<String>,
    pub status: Option<String>,
    pub records_added: Option<i32>,
    pub records_updated: Option<i32>,
    pub records_removed: Option<i32>,
    pub error_message: Option<String>,
    pub sync_started_at: NaiveDateTime,
    pub sync_completed_at: Option<NaiveDateTime>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = ad_sync_history)]
pub struct UpdateAdSyncHistory {
    pub sync_type: Option<String>,
    pub status: Option<String>,
    pub records_added: Option<i32>,
    pub records_updated: Option<i32>,
    pub records_removed: Option<i32>,
    pub error_message: Option<String>,
    pub sync_started_at: Option<NaiveDateTime>,
    pub sync_completed_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewAdSyncHistoryRequest {
    pub sync_type: Option<String>,
    pub status: Option<String>,
    pub records_added: Option<i32>,
    pub records_updated: Option<i32>,
    pub records_removed: Option<i32>,
    pub error_message: Option<String>,
    pub sync_started_at: Option<String>,
    pub sync_completed_at: Option<String>,
}

impl NewAdSyncHistoryRequest {
    pub fn into_new_ad_sync_history(self) -> Result<NewAdSyncHistory, String> {
        let parse_datetime = |s: &Option<String>| -> Result<Option<NaiveDateTime>, String> {
            match s {
                Some(v) if !v.is_empty() => {
                    NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S%.3fZ")
                        .or_else(|_| NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%SZ"))
                        .map(Some)
                        .map_err(|e| format!("Invalid datetime format: {}", e))
                }
                _ => Ok(None),
            }
        };

        Ok(NewAdSyncHistory {
            sync_type: self.sync_type,
            status: self.status,
            records_added: self.records_added,
            records_updated: self.records_updated,
            records_removed: self.records_removed,
            error_message: self.error_message,
            sync_started_at: parse_datetime(&self.sync_started_at)?.unwrap_or(
                NaiveDate::from_ymd_opt(1970, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
            sync_completed_at: parse_datetime(&self.sync_completed_at)?,
        })
    }
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UpdateAdSyncHistoryRequest {
    pub sync_type: Option<String>,
    pub status: Option<String>,
    pub records_added: Option<i32>,
    pub records_updated: Option<i32>,
    pub records_removed: Option<i32>,
    pub error_message: Option<String>,
    pub sync_started_at: Option<String>,
    pub sync_completed_at: Option<String>,
}

impl UpdateAdSyncHistoryRequest {
    pub fn into_update_ad_sync_history(self) -> Result<UpdateAdSyncHistory, String> {
        let parse_datetime = |s: &Option<String>| -> Result<Option<NaiveDateTime>, String> {
            match s {
                Some(v) if !v.is_empty() => {
                    NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S%.3fZ")
                        .or_else(|_| NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%SZ"))
                        .map(Some)
                        .map_err(|e| format!("Invalid datetime format: {}", e))
                }
                _ => Ok(None),
            }
        };

        Ok(UpdateAdSyncHistory {
            sync_type: self.sync_type,
            status: self.status,
            records_added: self.records_added,
            records_updated: self.records_updated,
            records_removed: self.records_removed,
            error_message: self.error_message,
            sync_started_at: parse_datetime(&self.sync_started_at)?,
            sync_completed_at: parse_datetime(&self.sync_completed_at)?,
        })
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct AdSyncHistoryResponse {
    pub id: i32,
    pub sync_type: Option<String>,
    pub status: Option<String>,
    pub records_added: Option<i32>,
    pub records_updated: Option<i32>,
    pub records_removed: Option<i32>,
    pub error_message: Option<String>,
    pub sync_started_at: String,
    pub sync_completed_at: Option<String>,
    pub created_at: String,
}

impl From<AdSyncHistory> for AdSyncHistoryResponse {
    fn from(sync: AdSyncHistory) -> Self {
        let fmt =
            |dt: Option<NaiveDateTime>| dt.map(|d| d.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string());
        Self {
            id: sync.id,
            sync_type: sync.sync_type,
            status: sync.status,
            records_added: sync.records_added,
            records_updated: sync.records_updated,
            records_removed: sync.records_removed,
            error_message: sync.error_message,
            sync_started_at: sync
                .sync_started_at
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            sync_completed_at: fmt(sync.sync_completed_at),
            created_at: sync.created_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct AdSyncHistoryListResponse {
    pub syncs: Vec<AdSyncHistoryResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
