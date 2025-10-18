use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::config::database::Pool;
use crate::models::employee::{Employee, NewEmployee, UpdateEmployee};
use crate::models::schema::{departments, employees, positions};
use crate::utils::db_helpers::{DbResult, get_connection};

pub struct EmployeeRepository {
    pool: Pool,
}

impl EmployeeRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Employee operations
    // ========================================================================

    pub async fn list_employees(
        &self,
        page: i64,
        per_page: i64,
        status: Option<String>,
        department_id: Option<i32>,
    ) -> DbResult<(Vec<(Employee, Option<String>, Option<String>)>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let mut count_query = employees::table.into_boxed();
        if let Some(ref s) = status {
            count_query = count_query.filter(employees::status.eq(s));
        }
        if let Some(dept_id) = department_id {
            count_query = count_query.filter(employees::department_id.eq(dept_id));
        }

        let total = count_query.count().get_result::<i64>(&mut conn).await?;

        let mut query = employees::table
            .left_join(positions::table)
            .left_join(departments::table)
            .into_boxed();

        if let Some(s) = status {
            query = query.filter(employees::status.eq(s));
        }
        if let Some(dept_id) = department_id {
            query = query.filter(employees::department_id.eq(dept_id));
        }

        let results = query
            .select((
                Employee::as_select(),
                positions::name.nullable(),
                departments::name.nullable(),
            ))
            .order(employees::last_name.asc())
            .limit(per_page)
            .offset(offset)
            .load::<(Employee, Option<String>, Option<String>)>(&mut conn)
            .await?;

        Ok((results, total))
    }

    pub async fn get_employee_by_id(
        &self,
        id: i32,
    ) -> DbResult<(Employee, Option<String>, Option<String>)> {
        let mut conn = get_connection(&self.pool).await?;

        employees::table
            .find(id)
            .left_join(positions::table)
            .left_join(departments::table)
            .select((
                Employee::as_select(),
                positions::name.nullable(),
                departments::name.nullable(),
            ))
            .first::<(Employee, Option<String>, Option<String>)>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn create_employee(&self, new_employee: NewEmployee) -> DbResult<Employee> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::insert_into(employees::table)
            .values(&new_employee)
            .get_result::<Employee>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn update_employee(
        &self,
        id: i32,
        update_data: UpdateEmployee,
    ) -> DbResult<Employee> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(employees::table.find(id))
            .set(&update_data)
            .get_result::<Employee>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn delete_employee(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(employees::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn get_employee_info_for_log(
        &self,
        id: i32,
    ) -> DbResult<
        Option<(
            String,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            String,
            Option<String>,
            Option<String>,
        )>,
    > {
        let mut conn = get_connection(&self.pool).await?;

        employees::table
            .find(id)
            .left_join(positions::table)
            .left_join(departments::table)
            .select((
                employees::first_name,
                employees::last_name,
                employees::middle_name,
                employees::email,
                employees::phone,
                employees::status,
                positions::name.nullable(),
                departments::name.nullable(),
            ))
            .first(&mut conn)
            .await
            .optional()
            .map_err(Into::into)
    }

    pub async fn get_position_and_department_names(
        &self,
        position_id: Option<i32>,
        department_id: Option<i32>,
    ) -> DbResult<(Option<String>, Option<String>)> {
        let mut conn = get_connection(&self.pool).await?;

        let pos = if let Some(pid) = position_id {
            positions::table
                .find(pid)
                .select(positions::name)
                .first::<String>(&mut conn)
                .await
                .ok()
        } else {
            None
        };

        let dept = if let Some(did) = department_id {
            departments::table
                .find(did)
                .select(departments::name)
                .first::<String>(&mut conn)
                .await
                .ok()
        } else {
            None
        };

        Ok((pos, dept))
    }
}
