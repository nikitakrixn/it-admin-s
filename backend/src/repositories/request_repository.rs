use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::config::database::Pool;
use crate::models::request::{
    NewRequest, NewRequestComment, Request, RequestComment, UpdateRequest,
};
use crate::models::schema::{request_comments, requests, users};
use crate::utils::db_helpers::{DbResult, get_connection};

pub struct RequestRepository {
    pool: Pool,
}

impl RequestRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Request operations
    // ========================================================================

    pub async fn list_requests(
        &self,
        page: i64,
        per_page: i64,
        status: Option<String>,
        priority: Option<String>,
        assignee_id: Option<i32>,
        requester_id: Option<i32>,
    ) -> DbResult<(Vec<Request>, i64)> {
        let mut conn = get_connection(&self.pool).await?;
        let offset = (page - 1) * per_page;

        let mut count_query = requests::table.into_boxed();
        if let Some(ref s) = status {
            count_query = count_query.filter(requests::status.eq(s));
        }
        if let Some(ref p) = priority {
            count_query = count_query.filter(requests::priority.eq(p));
        }
        if let Some(aid) = assignee_id {
            count_query = count_query.filter(requests::assignee_id.eq(aid));
        }
        if let Some(rid) = requester_id {
            count_query = count_query.filter(requests::requester_id.eq(rid));
        }

        let total = count_query.count().get_result::<i64>(&mut conn).await?;

        let mut query = requests::table.into_boxed();

        if let Some(s) = status {
            query = query.filter(requests::status.eq(s));
        }
        if let Some(ref p) = priority {
            query = query.filter(requests::priority.eq(p));
        }
        if let Some(aid) = assignee_id {
            query = query.filter(requests::assignee_id.eq(aid));
        }
        if let Some(rid) = requester_id {
            query = query.filter(requests::requester_id.eq(rid));
        }

        let results = query
            .order(requests::created_at.desc())
            .limit(per_page)
            .offset(offset)
            .load::<Request>(&mut conn)
            .await?;

        Ok((results, total))
    }

    pub async fn get_request_by_id(&self, id: i32) -> DbResult<Request> {
        let mut conn = get_connection(&self.pool).await?;

        requests::table
            .find(id)
            .first::<Request>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn create_request(&self, new_request: NewRequest) -> DbResult<Request> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::insert_into(requests::table)
            .values(&new_request)
            .get_result::<Request>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn update_request(&self, id: i32, update_data: UpdateRequest) -> DbResult<Request> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(requests::table.find(id))
            .set(&update_data)
            .get_result::<Request>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn delete_request(&self, id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(requests::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(Into::into)
    }

    // ========================================================================
    // Request comments
    // ========================================================================

    pub async fn list_comments(
        &self,
        request_id: i32,
    ) -> DbResult<Vec<(RequestComment, Option<String>)>> {
        let mut conn = get_connection(&self.pool).await?;

        request_comments::table
            .left_join(users::table.on(users::id.eq(request_comments::user_id)))
            .filter(request_comments::request_id.eq(request_id))
            .select((RequestComment::as_select(), users::email.nullable()))
            .order(request_comments::created_at.asc())
            .load::<(RequestComment, Option<String>)>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn create_comment(&self, new_comment: NewRequestComment) -> DbResult<RequestComment> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::insert_into(request_comments::table)
            .values(&new_comment)
            .get_result::<RequestComment>(&mut conn)
            .await
            .map_err(Into::into)
    }

    pub async fn delete_comment(&self, comment_id: i32) -> DbResult<usize> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::delete(request_comments::table.find(comment_id))
            .execute(&mut conn)
            .await
            .map_err(Into::into)
    }
}
