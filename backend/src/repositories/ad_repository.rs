use crate::config::database::Pool;
use crate::models::ad::{
    AdGroup, AdGroupMembership, AdSyncHistory, DomainAccount, NewAdGroup, NewAdGroupMembership,
    NewAdSyncHistory, NewDomainAccount, UpdateAdGroup, UpdateAdGroupMembership,
    UpdateAdSyncHistory, UpdateDomainAccount,
};
use crate::models::schema::{ad_group_memberships, ad_groups, ad_sync_history, domain_accounts};
use crate::utils::db_helpers::{DbResult, get_connection};
use crate::utils::error::AppError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

// ============================================================================
// AD group operations
// ============================================================================

type AdGroupRow = (
    i32,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<i32>,
    Option<NaiveDateTime>,
    NaiveDateTime,
);

fn map_ad_group(row: AdGroupRow) -> AdGroup {
    let (
        id,
        name,
        distinguished_name,
        description,
        group_type,
        group_scope,
        member_count,
        last_synced,
        created_at,
    ) = row;
    AdGroup {
        id,
        name,
        distinguished_name,
        description,
        group_type,
        group_scope,
        member_count,
        last_synced,
        created_at,
    }
}

pub struct AdGroupRepository {
    pool: Pool,
}

impl AdGroupRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn list_ad_groups(&self, page: i64, per_page: i64) -> DbResult<(Vec<AdGroup>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let total = ad_groups::table
            .count()
            .get_result::<i64>(&mut conn)
            .await?;

        let results = ad_groups::table
            .select((
                ad_groups::id,
                ad_groups::name,
                ad_groups::distinguished_name,
                ad_groups::description,
                ad_groups::group_type,
                ad_groups::group_scope,
                ad_groups::member_count,
                ad_groups::last_synced,
                ad_groups::created_at,
            ))
            .order(ad_groups::name.asc())
            .limit(per_page)
            .offset(offset)
            .load::<AdGroupRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_ad_group)
            .collect();

        Ok((results, total))
    }

    pub async fn get_ad_group_by_id(&self, id: i32) -> DbResult<AdGroup> {
        let mut conn = get_connection(&self.pool).await?;

        let row = ad_groups::table
            .select((
                ad_groups::id,
                ad_groups::name,
                ad_groups::distinguished_name,
                ad_groups::description,
                ad_groups::group_type,
                ad_groups::group_scope,
                ad_groups::member_count,
                ad_groups::last_synced,
                ad_groups::created_at,
            ))
            .filter(ad_groups::id.eq(id))
            .first::<AdGroupRow>(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(map_ad_group(row))
    }

    pub async fn create_ad_group(&self, new_group: NewAdGroup) -> DbResult<AdGroup> {
        let mut conn = get_connection(&self.pool).await?;

        let new_id = diesel::insert_into(ad_groups::table)
            .values(&new_group)
            .returning(ad_groups::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_ad_group_by_id(new_id).await
    }

    pub async fn update_ad_group(&self, id: i32, update_data: UpdateAdGroup) -> DbResult<AdGroup> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(ad_groups::table.find(id))
            .set(&update_data)
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_ad_group_by_id(id).await
    }

    pub async fn delete_ad_group(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(ad_groups::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)
    }
}

// ============================================================================
// AD group membership operations
// ============================================================================

type AdGroupMembershipRow = (i32, i32, i32, Option<NaiveDateTime>);

fn map_ad_group_membership(row: AdGroupMembershipRow) -> AdGroupMembership {
    let (id, employee_id, ad_group_id, added_at) = row;
    AdGroupMembership {
        id,
        employee_id,
        ad_group_id,
        added_at,
    }
}

pub struct AdGroupMembershipRepository {
    pool: Pool,
}

impl AdGroupMembershipRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn list_ad_group_memberships(
        &self,
        page: i64,
        per_page: i64,
        ad_group_id: Option<i32>,
    ) -> DbResult<(Vec<AdGroupMembership>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let mut count_query = ad_group_memberships::table.into_boxed();
        if let Some(gid) = ad_group_id {
            count_query = count_query.filter(ad_group_memberships::ad_group_id.eq(gid));
        }

        let total = count_query.count().get_result::<i64>(&mut conn).await?;

        let mut query = ad_group_memberships::table
            .select((
                ad_group_memberships::id,
                ad_group_memberships::employee_id,
                ad_group_memberships::ad_group_id,
                ad_group_memberships::added_at,
            ))
            .into_boxed();

        if let Some(gid) = ad_group_id {
            query = query.filter(ad_group_memberships::ad_group_id.eq(gid));
        }

        let results = query
            .order(ad_group_memberships::id.asc())
            .limit(per_page)
            .offset(offset)
            .load::<AdGroupMembershipRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_ad_group_membership)
            .collect();

        Ok((results, total))
    }

    pub async fn get_ad_group_membership_by_id(&self, id: i32) -> DbResult<AdGroupMembership> {
        let mut conn = get_connection(&self.pool).await?;

        let row = ad_group_memberships::table
            .select((
                ad_group_memberships::id,
                ad_group_memberships::employee_id,
                ad_group_memberships::ad_group_id,
                ad_group_memberships::added_at,
            ))
            .filter(ad_group_memberships::id.eq(id))
            .first::<AdGroupMembershipRow>(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(map_ad_group_membership(row))
    }

    pub async fn create_ad_group_membership(
        &self,
        new_membership: NewAdGroupMembership,
    ) -> DbResult<AdGroupMembership> {
        let mut conn = get_connection(&self.pool).await?;

        let new_id = diesel::insert_into(ad_group_memberships::table)
            .values(&new_membership)
            .returning(ad_group_memberships::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_ad_group_membership_by_id(new_id).await
    }

    pub async fn update_ad_group_membership(
        &self,
        id: i32,
        update_data: UpdateAdGroupMembership,
    ) -> DbResult<AdGroupMembership> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(ad_group_memberships::table.find(id))
            .set(&update_data)
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_ad_group_membership_by_id(id).await
    }

    pub async fn delete_ad_group_membership(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(ad_group_memberships::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)
    }
}

// ============================================================================
// Domain account operations
// ============================================================================

type DomainAccountRow = (
    i32,
    Option<i32>,
    String,
    Option<String>,
    Option<String>,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<NaiveDateTime>,
    Option<NaiveDateTime>,
    Option<bool>,
    Option<bool>,
    Option<bool>,
    Option<NaiveDateTime>,
    Option<bool>,
    Option<bool>,
    Option<NaiveDateTime>,
    NaiveDateTime,
    NaiveDateTime,
);

fn map_domain_account(row: DomainAccountRow) -> DomainAccount {
    let (
        id,
        employee_id,
        username,
        display_name,
        email,
        account_type,
        distinguished_name,
        user_principal_name,
        sam_account_name,
        password_last_set,
        password_expires_at,
        password_never_expires,
        must_change_password,
        cannot_change_password,
        account_expires_at,
        is_enabled,
        is_locked,
        locked_at,
        created_at,
        updated_at,
    ) = row;
    DomainAccount {
        id,
        employee_id,
        username,
        display_name,
        email,
        account_type,
        distinguished_name,
        user_principal_name,
        sam_account_name,
        password_last_set,
        password_expires_at,
        password_never_expires,
        must_change_password,
        cannot_change_password,
        account_expires_at,
        is_enabled,
        is_locked,
        locked_at,
        created_at,
        updated_at,
    }
}

pub struct DomainAccountRepository {
    pool: Pool,
}

impl DomainAccountRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn list_domain_accounts(
        &self,
        page: i64,
        per_page: i64,
        account_status: Option<String>,
    ) -> DbResult<(Vec<DomainAccount>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let mut count_query = domain_accounts::table.into_boxed();
        if let Some(ref s) = account_status {
            count_query = count_query.filter(domain_accounts::account_type.eq(s));
        }

        let total = count_query.count().get_result::<i64>(&mut conn).await?;

        let mut query = domain_accounts::table
            .select((
                domain_accounts::id,
                domain_accounts::employee_id,
                domain_accounts::username,
                domain_accounts::display_name,
                domain_accounts::email,
                domain_accounts::account_type,
                domain_accounts::distinguished_name,
                domain_accounts::user_principal_name,
                domain_accounts::sam_account_name,
                domain_accounts::password_last_set,
                domain_accounts::password_expires_at,
                domain_accounts::password_never_expires,
                domain_accounts::must_change_password,
                domain_accounts::cannot_change_password,
                domain_accounts::account_expires_at,
                domain_accounts::is_enabled,
                domain_accounts::is_locked,
                domain_accounts::locked_at,
                domain_accounts::created_at,
                domain_accounts::updated_at,
            ))
            .into_boxed();

        if let Some(ref s) = account_status {
            query = query.filter(domain_accounts::account_type.eq(s));
        }

        let results = query
            .order(domain_accounts::username.asc().nulls_last())
            .limit(per_page)
            .offset(offset)
            .load::<DomainAccountRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_domain_account)
            .collect();

        Ok((results, total))
    }

    pub async fn get_domain_account_by_id(&self, id: i32) -> DbResult<DomainAccount> {
        let mut conn = get_connection(&self.pool).await?;

        let row = domain_accounts::table
            .select((
                domain_accounts::id,
                domain_accounts::employee_id,
                domain_accounts::username,
                domain_accounts::display_name,
                domain_accounts::email,
                domain_accounts::account_type,
                domain_accounts::distinguished_name,
                domain_accounts::user_principal_name,
                domain_accounts::sam_account_name,
                domain_accounts::password_last_set,
                domain_accounts::password_expires_at,
                domain_accounts::password_never_expires,
                domain_accounts::must_change_password,
                domain_accounts::cannot_change_password,
                domain_accounts::account_expires_at,
                domain_accounts::is_enabled,
                domain_accounts::is_locked,
                domain_accounts::locked_at,
                domain_accounts::created_at,
                domain_accounts::updated_at,
            ))
            .filter(domain_accounts::id.eq(id))
            .first::<DomainAccountRow>(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(map_domain_account(row))
    }

    pub async fn create_domain_account(
        &self,
        new_account: NewDomainAccount,
    ) -> DbResult<DomainAccount> {
        let mut conn = get_connection(&self.pool).await?;

        let new_id = diesel::insert_into(domain_accounts::table)
            .values(&new_account)
            .returning(domain_accounts::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_domain_account_by_id(new_id).await
    }

    pub async fn update_domain_account(
        &self,
        id: i32,
        update_data: UpdateDomainAccount,
    ) -> DbResult<DomainAccount> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(domain_accounts::table.find(id))
            .set(&update_data)
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_domain_account_by_id(id).await
    }

    pub async fn delete_domain_account(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(domain_accounts::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)
    }
}

// ============================================================================
// AD sync history operations
// ============================================================================

type AdSyncHistoryRow = (
    i32,
    Option<String>,
    Option<String>,
    Option<i32>,
    Option<i32>,
    Option<i32>,
    Option<String>,
    NaiveDateTime,
    Option<NaiveDateTime>,
    NaiveDateTime,
);

fn map_ad_sync_history(row: AdSyncHistoryRow) -> AdSyncHistory {
    let (
        id,
        sync_type,
        status,
        records_added,
        records_updated,
        records_removed,
        error_message,
        sync_started_at,
        sync_completed_at,
        created_at,
    ) = row;
    AdSyncHistory {
        id,
        sync_type,
        status,
        records_added,
        records_updated,
        records_removed,
        error_message,
        sync_started_at,
        sync_completed_at,
        created_at,
    }
}

pub struct AdSyncHistoryRepository {
    pool: Pool,
}

impl AdSyncHistoryRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn list_ad_sync_history(
        &self,
        page: i64,
        per_page: i64,
    ) -> DbResult<(Vec<AdSyncHistory>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let total = ad_sync_history::table
            .count()
            .get_result::<i64>(&mut conn)
            .await?;

        let results = ad_sync_history::table
            .select((
                ad_sync_history::id,
                ad_sync_history::sync_type,
                ad_sync_history::status,
                ad_sync_history::records_added,
                ad_sync_history::records_updated,
                ad_sync_history::records_removed,
                ad_sync_history::error_message,
                ad_sync_history::sync_started_at,
                ad_sync_history::sync_completed_at,
                ad_sync_history::created_at,
            ))
            .order(ad_sync_history::created_at.desc())
            .limit(per_page)
            .offset(offset)
            .load::<AdSyncHistoryRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_ad_sync_history)
            .collect();

        Ok((results, total))
    }

    pub async fn get_ad_sync_history_by_id(&self, id: i32) -> DbResult<AdSyncHistory> {
        let mut conn = get_connection(&self.pool).await?;

        let row = ad_sync_history::table
            .select((
                ad_sync_history::id,
                ad_sync_history::sync_type,
                ad_sync_history::status,
                ad_sync_history::records_added,
                ad_sync_history::records_updated,
                ad_sync_history::records_removed,
                ad_sync_history::error_message,
                ad_sync_history::sync_started_at,
                ad_sync_history::sync_completed_at,
                ad_sync_history::created_at,
            ))
            .filter(ad_sync_history::id.eq(id))
            .first::<AdSyncHistoryRow>(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(map_ad_sync_history(row))
    }

    pub async fn create_ad_sync_history(
        &self,
        new_sync: NewAdSyncHistory,
    ) -> DbResult<AdSyncHistory> {
        let mut conn = get_connection(&self.pool).await?;

        let new_id = diesel::insert_into(ad_sync_history::table)
            .values(&new_sync)
            .returning(ad_sync_history::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_ad_sync_history_by_id(new_id).await
    }

    pub async fn update_ad_sync_history(
        &self,
        id: i32,
        update_data: UpdateAdSyncHistory,
    ) -> DbResult<AdSyncHistory> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(ad_sync_history::table.find(id))
            .set(&update_data)
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_ad_sync_history_by_id(id).await
    }

    pub async fn delete_ad_sync_history(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(ad_sync_history::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)
    }
}
