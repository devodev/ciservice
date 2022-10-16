use diesel::prelude::*;
use diesel::result::Error;
use rocket::response::Debug;

use crate::models::job::{Job, NewJob, UpdateJob};
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

pub(crate) fn update(conn: &mut PgConnection, id: i32, name: &str) -> Result<Job> {
    let update_job = UpdateJob { name };
    let job = diesel::update(job::table)
        .filter(job::id.eq(id))
        .set(update_job)
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
    let job = job::table.filter(job::id.eq(id)).first::<Job>(conn)?;

    Ok(job)
}

pub(crate) fn delete(conn: &mut PgConnection, id: i32) -> Result<()> {
    let affected = diesel::delete(job::table)
        .filter(job::id.eq(id))
        .execute(conn)?;

    match affected != 0 {
        true => Ok(()),
        false => Err(Debug(Error::NotFound)),
    }
}
