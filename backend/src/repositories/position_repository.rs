use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::config::database::Pool;
use crate::models::position::{NewPosition, Position, PositionWithDetails, UpdatePosition};
use crate::models::schema::positions;
use crate::utils::db_helpers::{get_connection, DbResult};

pub struct PositionRepository {
    pool: Pool,
}

impl PositionRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn list_positions(&self) -> DbResult<Vec<PositionWithDetails>> {
        let mut conn = get_connection(&self.pool).await?;

        let query = "SELECT * FROM v_positions_with_details ORDER BY name";
        diesel::sql_query(query)
            .load::<PositionWithDetails>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn get_position_by_id(&self, id: i32) -> DbResult<Position> {
        let mut conn = get_connection(&self.pool).await?;

        positions::table
            .find(id)
            .first::<Position>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn create_position(&self, new_pos: NewPosition) -> DbResult<Position> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::insert_into(positions::table)
            .values(&new_pos)
            .get_result::<Position>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn update_position(&self, id: i32, update_data: UpdatePosition) -> DbResult<Position> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(positions::table.find(id))
            .set(&update_data)
            .get_result::<Position>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn delete_position(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(positions::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(Into::into)
    }
}
