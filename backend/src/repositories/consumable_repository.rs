use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::BigInt;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::config::database::Pool;
use crate::models::consumable::{
    Consumable, ConsumableMovement, NewConsumable, NewConsumableMovement, UpdateConsumable,
};
use crate::models::schema::{consumable_movements, consumables, hardware_types, users};
use crate::utils::db_helpers::{DbResult, get_connection};
use crate::utils::error::AppError;
use chrono::NaiveDateTime;

type ConsumableRow = (
    i32,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<i32>,
    Option<String>,
    i32,
    i32,
    Option<String>,
    Option<String>,
    NaiveDateTime,
    NaiveDateTime,
);

type MovementRow = (
    i32,
    i32,
    Option<i32>,
    Option<i32>,
    i32,
    String,
    Option<i32>,
    Option<Uuid>,
    Option<String>,
    Option<String>,
    NaiveDateTime,
);

fn map_consumable(row: ConsumableRow) -> Consumable {
    let (
        id,
        name,
        manufacturer,
        model,
        category,
        compatible_with,
        compatible_with_name,
        quantity,
        min_quantity,
        unit,
        notes,
        created_at,
        updated_at,
    ) = row;
    Consumable {
        id,
        name,
        manufacturer,
        model,
        category,
        compatible_with,
        compatible_with_name,
        quantity,
        min_quantity,
        unit,
        notes,
        created_at,
        updated_at,
    }
}

fn map_movement(row: MovementRow) -> ConsumableMovement {
    let (
        id,
        consumable_id,
        equipment_id,
        computer_id,
        quantity,
        movement_type,
        employee_id,
        performed_by,
        performed_by_email,
        notes,
        created_at,
    ) = row;
    ConsumableMovement {
        id,
        consumable_id,
        equipment_id,
        computer_id,
        quantity,
        movement_type,
        employee_id,
        performed_by,
        performed_by_email,
        notes,
        created_at,
    }
}

pub struct ConsumableRepository {
    pool: Pool,
}

impl ConsumableRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Consumable operations
    // ========================================================================

    pub async fn list_consumables(
        &self,
        page: i64,
        per_page: i64,
        category: Option<String>,
    ) -> DbResult<(Vec<Consumable>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let mut count_query = consumables::table.into_boxed();
        if let Some(ref c) = category {
            count_query = count_query.filter(consumables::category.eq(c));
        }

        let total = count_query.count().get_result::<i64>(&mut conn).await?;

        let mut query = consumables::table
            .left_join(
                hardware_types::table
                    .on(consumables::compatible_with.eq(hardware_types::id.nullable())),
            )
            .select((
                consumables::id,
                consumables::name,
                consumables::manufacturer,
                consumables::model,
                consumables::category,
                consumables::compatible_with,
                hardware_types::name.nullable(),
                consumables::quantity,
                consumables::min_quantity,
                consumables::unit,
                consumables::notes,
                consumables::created_at,
                consumables::updated_at,
            ))
            .into_boxed();

        if let Some(ref c) = category {
            query = query.filter(consumables::category.eq(c));
        }

        let results = query
            .order(consumables::name.asc())
            .limit(per_page)
            .offset(offset)
            .load::<ConsumableRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_consumable)
            .collect();

        Ok((results, total))
    }

    pub async fn get_consumable_by_id(&self, id: i32) -> DbResult<Consumable> {
        let mut conn = get_connection(&self.pool).await?;

        let row = consumables::table
            .left_join(
                hardware_types::table
                    .on(consumables::compatible_with.eq(hardware_types::id.nullable())),
            )
            .select((
                consumables::id,
                consumables::name,
                consumables::manufacturer,
                consumables::model,
                consumables::category,
                consumables::compatible_with,
                hardware_types::name.nullable(),
                consumables::quantity,
                consumables::min_quantity,
                consumables::unit,
                consumables::notes,
                consumables::created_at,
                consumables::updated_at,
            ))
            .filter(consumables::id.eq(id))
            .first::<ConsumableRow>(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(map_consumable(row))
    }

    pub async fn create_consumable(&self, new_consumable: NewConsumable) -> DbResult<Consumable> {
        let mut conn = get_connection(&self.pool).await?;

        let new_id = diesel::insert_into(consumables::table)
            .values(&new_consumable)
            .returning(consumables::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_consumable_by_id(new_id).await
    }

    pub async fn update_consumable(
        &self,
        id: i32,
        update_data: UpdateConsumable,
    ) -> DbResult<Consumable> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(consumables::table.find(id))
            .set(&update_data)
            .execute(&mut conn)
            .await
            .map_err(AppError::from)?;

        self.get_consumable_by_id(id).await
    }

    pub async fn delete_consumable(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(consumables::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(AppError::from)
    }

    // ========================================================================
    // Movement operations
    // ========================================================================

    pub async fn list_movements(
        &self,
        consumable_id: i32,
        page: i64,
        per_page: i64,
    ) -> DbResult<(Vec<ConsumableMovement>, i64, i64, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let total = consumable_movements::table
            .filter(consumable_movements::consumable_id.eq(consumable_id))
            .count()
            .get_result::<i64>(&mut conn)
            .await?;

        let (total_in, total_out) = consumable_movements::table
            .filter(consumable_movements::consumable_id.eq(consumable_id))
            .select((
                sql::<BigInt>("COALESCE(SUM(CASE WHEN quantity > 0 THEN quantity ELSE 0 END), 0)"),
                sql::<BigInt>("COALESCE(SUM(CASE WHEN quantity < 0 THEN -quantity ELSE 0 END), 0)"),
            ))
            .first::<(i64, i64)>(&mut conn)
            .await?;

        let movements = consumable_movements::table
            .left_join(users::table.on(consumable_movements::performed_by.eq(users::id.nullable())))
            .select((
                consumable_movements::id,
                consumable_movements::consumable_id,
                consumable_movements::equipment_id,
                consumable_movements::computer_id,
                consumable_movements::quantity,
                consumable_movements::movement_type,
                consumable_movements::employee_id,
                consumable_movements::performed_by,
                users::email.nullable(),
                consumable_movements::notes,
                consumable_movements::created_at,
            ))
            .filter(consumable_movements::consumable_id.eq(consumable_id))
            .order((
                consumable_movements::created_at.desc(),
                consumable_movements::id.desc(),
            ))
            .limit(per_page)
            .offset(offset)
            .load::<MovementRow>(&mut conn)
            .await?
            .into_iter()
            .map(map_movement)
            .collect();

        Ok((movements, total, total_in, total_out))
    }

    pub async fn get_movement_by_id(&self, id: i32) -> DbResult<ConsumableMovement> {
        let mut conn = get_connection(&self.pool).await?;

        let row = consumable_movements::table
            .left_join(users::table.on(consumable_movements::performed_by.eq(users::id.nullable())))
            .select((
                consumable_movements::id,
                consumable_movements::consumable_id,
                consumable_movements::equipment_id,
                consumable_movements::computer_id,
                consumable_movements::quantity,
                consumable_movements::movement_type,
                consumable_movements::employee_id,
                consumable_movements::performed_by,
                users::email.nullable(),
                consumable_movements::notes,
                consumable_movements::created_at,
            ))
            .filter(consumable_movements::id.eq(id))
            .first::<MovementRow>(&mut conn)
            .await
            .map_err(AppError::from)?;

        Ok(map_movement(row))
    }

    pub async fn create_movement(
        &self,
        consumable_id: i32,
        movement: NewConsumableMovement,
    ) -> DbResult<(ConsumableMovement, Consumable)> {
        // The database trigger `update_consumable_quantity_trigger` keeps
        // `consumables.quantity` in sync with the movement history, so the
        // repository only has to validate the stock and insert the movement.
        let current = self.get_consumable_by_id(consumable_id).await?;

        if current.quantity + movement.quantity < 0 {
            return Err(AppError::ValidationError(
                "Not enough stock for this movement".to_string(),
            ));
        }

        let mut conn = get_connection(&self.pool).await?;

        let movement_id = diesel::insert_into(consumable_movements::table)
            .values(&movement)
            .returning(consumable_movements::id)
            .get_result::<i32>(&mut conn)
            .await
            .map_err(|err| match err {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::CheckViolation,
                    _,
                ) => AppError::ValidationError("Not enough stock for this movement".to_string()),
                other => AppError::from(other),
            })?;

        let saved_movement = self.get_movement_by_id(movement_id).await?;
        let consumable = self.get_consumable_by_id(consumable_id).await?;

        Ok((saved_movement, consumable))
    }
}
