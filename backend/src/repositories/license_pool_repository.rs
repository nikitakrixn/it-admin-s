use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::config::database::Pool;
use crate::models::license_pool::{LicensePool, NewLicensePool, UpdateLicensePool};
use crate::models::schema::license_pools;
use crate::utils::db_helpers::{DbResult, get_connection};
use crate::utils::error::AppError;
use chrono::{NaiveDate, NaiveDateTime};

type LicensePoolRow = (
    i32,
    i32,
    String,
    i32,
    Option<i32>,
    Option<i32>,
    Option<String>,
    Option<NaiveDate>,
    Option<NaiveDate>,
    Option<i32>,
    Option<i32>,
    Option<bool>,
    Option<String>,
    NaiveDateTime,
    NaiveDateTime,
);

fn map_license_pool(row: LicensePoolRow) -> LicensePool {
    let (
        id,
        software_catalog_id,
        license_type,
        total_licenses,
        used_licenses,
        available_licenses,
        license_key,
        purchase_date,
        expiration_date,
        vendor_id,
        contract_id,
        auto_renewal,
        notes,
        created_at,
        updated_at,
    ) = row;
    LicensePool {
        id,
        software_catalog_id,
        license_type,
        total_licenses,
        used_licenses,
        available_licenses,
        license_key,
        purchase_date,
        expiration_date,
        vendor_id,
        contract_id,
        auto_renewal,
        notes,
        created_at,
        updated_at,
    }
}

pub struct LicensePoolRepository {
    pool: Pool,
}

impl LicensePoolRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // License pool operations
    // ========================================================================

    pub async fn list_license_pools(
        &self,
        page: i64,
        per_page: i64,
        license_type: Option<String>,
        software_catalog_id: Option<i32>,
    ) -> DbResult<(Vec<LicensePool>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let mut count_query = license_pools::table.into_boxed();
        if let Some(ref t) = license_type {
            count_query = count_query.filter(license_pools::license_type.eq(t));
        }
        if let Some(sid) = software_catalog_id {
            count_query = count_query.filter(license_pools::software_catalog_id.eq(sid));
        }

        let total = count_query.count().get_result::<i64>(&mut conn).await?;

        let mut query = license_pools::table
            .select((
                license_pools::id,
                license_pools::software_catalog_id,
                license_pools::license_type,
                license_pools::total_licenses,
                license_pools::used_licenses,
                license_pools::available_licenses,
                license_pools::license_key,
                license_pools::purchase_date,
                license_pools::expiration_date,
                license_pools::vendor_id,
                license_pools::contract_id,
                license_pools::auto_renewal,
                license_pools::notes,
                license_pools::created_at,
                license_pools::updated_at,
            ))
            .into_boxed();

        if let Some(ref t) = license_type {
            query = query.filter(license_pools::license_type.eq(t));
        }
        if let Some(sid) = software_catalog_id {
            query = query.filter(license_pools::software_catalog_id.eq(sid));
        }

        let results = query
            .order(license_pools::license_type.asc())
            .limit(per_page)
            .offset(offset)
            .load::<LicensePoolRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_license_pool)
            .collect();

        Ok((results, total))
    }

    pub async fn get_license_pool_by_id(&self, id: i32) -> DbResult<LicensePool> {
        let mut conn = get_connection(&self.pool).await?;

        let row = license_pools::table
            .select((
                license_pools::id,
                license_pools::software_catalog_id,
                license_pools::license_type,
                license_pools::total_licenses,
                license_pools::used_licenses,
                license_pools::available_licenses,
                license_pools::license_key,
                license_pools::purchase_date,
                license_pools::expiration_date,
                license_pools::vendor_id,
                license_pools::contract_id,
                license_pools::auto_renewal,
                license_pools::notes,
                license_pools::created_at,
                license_pools::updated_at,
            ))
            .filter(license_pools::id.eq(id))
            .first::<LicensePoolRow>(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(map_license_pool(row))
    }

    pub async fn create_license_pool(&self, new_pool: NewLicensePool) -> DbResult<LicensePool> {
        let mut conn = get_connection(&self.pool).await?;

        let new_id = diesel::insert_into(license_pools::table)
            .values(&new_pool)
            .returning(license_pools::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_license_pool_by_id(new_id).await
    }

    pub async fn update_license_pool(
        &self,
        id: i32,
        update_data: UpdateLicensePool,
    ) -> DbResult<LicensePool> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(license_pools::table.find(id))
            .set(&update_data)
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_license_pool_by_id(id).await
    }

    pub async fn delete_license_pool(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(license_pools::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)
    }
}
