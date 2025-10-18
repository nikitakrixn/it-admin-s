use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::config::database::Pool;
use crate::models::department::{
    Department, DepartmentWithCounts, NewDepartment, UpdateDepartment,
};
use crate::models::schema::departments;
use crate::utils::db_helpers::{DbResult, get_connection};

pub struct DepartmentRepository {
    pool: Pool,
}

impl DepartmentRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn list_departments(&self) -> DbResult<Vec<DepartmentWithCounts>> {
        let mut conn = get_connection(&self.pool).await?;

        let query = "SELECT * FROM v_departments_with_counts ORDER BY name";
        diesel::sql_query(query)
            .load::<DepartmentWithCounts>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn get_department_by_id(&self, id: i32) -> DbResult<Department> {
        let mut conn = get_connection(&self.pool).await?;

        departments::table
            .find(id)
            .first::<Department>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn create_department(&self, new_dept: NewDepartment) -> DbResult<Department> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::insert_into(departments::table)
            .values(&new_dept)
            .get_result::<Department>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn update_department(
        &self,
        id: i32,
        update_data: UpdateDepartment,
    ) -> DbResult<Department> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(departments::table.find(id))
            .set(&update_data)
            .get_result::<Department>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn delete_department(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(departments::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(Into::into)
    }
}
