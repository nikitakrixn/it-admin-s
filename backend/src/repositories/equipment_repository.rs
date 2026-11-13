use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::config::database::Pool;
use crate::models::equipment::{Equipment, NewEquipment, UpdateEquipment};
use crate::models::schema::equipment;
use crate::utils::db_helpers::{DbResult, get_connection};

pub struct EquipmentRepository {
    pool: Pool,
}

impl EquipmentRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Equipment operations
    // ========================================================================

    pub async fn list_equipment(
        &self,
        page: i64,
        per_page: i64,
        status: Option<String>,
        location: Option<String>,
    ) -> DbResult<(Vec<Equipment>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let mut count_query = equipment::table.into_boxed();
        if let Some(ref s) = status {
            count_query = count_query.filter(equipment::status.eq(s));
        }
        if let Some(ref l) = location {
            count_query = count_query.filter(equipment::location.eq(l));
        }

        let total = count_query.count().get_result::<i64>(&mut conn).await?;

        let mut query = equipment::table.into_boxed();

        if let Some(s) = status {
            query = query.filter(equipment::status.eq(s));
        }
        if let Some(ref l) = location {
            query = query.filter(equipment::location.eq(l));
        }

        let results = query
            .order(equipment::name.asc())
            .limit(per_page)
            .offset(offset)
            .load::<Equipment>(&mut conn)
            .await?;

        Ok((results, total))
    }

    pub async fn get_equipment_by_id(&self, id: i32) -> DbResult<Equipment> {
        let mut conn = get_connection(&self.pool).await?;

        equipment::table
            .find(id)
            .first::<Equipment>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn create_equipment(&self, new_equipment: NewEquipment) -> DbResult<Equipment> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::insert_into(equipment::table)
            .values(&new_equipment)
            .get_result::<Equipment>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn update_equipment(
        &self,
        id: i32,
        update_data: UpdateEquipment,
    ) -> DbResult<Equipment> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(equipment::table.find(id))
            .set(&update_data)
            .get_result::<Equipment>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn delete_equipment(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(equipment::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(Into::into)
    }
}
