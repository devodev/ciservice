use diesel::prelude::*;
use rocket::response::Debug;

use crate::models::job::{Job, NewJob};
use crate::schema::job;

use super::pagination::{Paginate, PaginatedQueryResult};
use super::PaginatedParams;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

pub(crate) fn create(conn: &mut PgConnection, name: &str) -> Result<Job> {
    let new_job = NewJob { name };

    let job = diesel::insert_into(job::table)
        .values(&new_job)
        .get_result::<Job>(conn)?;
    Ok(job)
}

pub(crate) fn list(
    conn: &mut PgConnection,
    params: &PaginatedParams,
) -> Result<PaginatedQueryResult<Job>> {
    let jobs = job::table
        .order_by(job::id)
        .paginate(params.page, params.limit)
        .load_and_count(conn)?;

    Ok(jobs)
}

pub(crate) fn get(conn: &mut PgConnection, id: i32) -> Result<Job> {
    let job = job::table.filter(job::id.eq_all(id)).first::<Job>(conn)?;

    Ok(job)
}
