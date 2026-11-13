use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::config::database::Pool;
use crate::models::computer::{Computer, NewComputer, UpdateComputer};
use crate::models::schema::computers;
use crate::utils::db_helpers::{DbResult, get_connection};

pub struct ComputerRepository {
    pool: Pool,
}

impl ComputerRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Computer operations
    // ========================================================================

    pub async fn list_computers(
        &self,
        page: i64,
        per_page: i64,
        status: Option<String>,
        computer_type: Option<String>,
    ) -> DbResult<(Vec<Computer>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let mut count_query = computers::table.into_boxed();
        if let Some(ref s) = status {
            count_query = count_query.filter(computers::status.eq(s));
        }
        if let Some(ref t) = computer_type {
            count_query = count_query.filter(computers::computer_type.eq(t));
        }

        let total = count_query.count().get_result::<i64>(&mut conn).await?;

        let mut query = computers::table.into_boxed();

        if let Some(s) = status {
            query = query.filter(computers::status.eq(s));
        }
        if let Some(ref t) = computer_type {
            query = query.filter(computers::computer_type.eq(t));
        }

        let results = query
            .order(computers::hostname.asc().nulls_last())
            .limit(per_page)
            .offset(offset)
            .load::<Computer>(&mut conn)
            .await?;

        Ok((results, total))
    }

    pub async fn get_computer_by_id(&self, id: i32) -> DbResult<Computer> {
        let mut conn = get_connection(&self.pool).await?;

        computers::table
            .find(id)
            .first::<Computer>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn create_computer(&self, new_computer: NewComputer) -> DbResult<Computer> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::insert_into(computers::table)
            .values(&new_computer)
            .get_result::<Computer>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn update_computer(
        &self,
        id: i32,
        update_data: UpdateComputer,
    ) -> DbResult<Computer> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(computers::table.find(id))
            .set(&update_data)
            .get_result::<Computer>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn delete_computer(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(computers::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(Into::into)
    }
}
