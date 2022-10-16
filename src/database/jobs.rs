use diesel::prelude::*;
use rocket::response::Debug;

use crate::models::job::Job;
use crate::schema::job;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Insertable)]
#[table_name = "job"]
pub struct NewJob<'a> {
    pub name: &'a str,
}

pub fn create(conn: &mut PgConnection, name: &str) -> Result<Job> {
    let new_job = NewJob { name };

    let job = diesel::insert_into(job::table)
        .values(&new_job)
        .get_result::<Job>(conn)?;
    Ok(job)
}

pub fn list(conn: &mut PgConnection) -> Result<Vec<Job>> {
    let jobs: Vec<Job> = job::table.load(conn)?;
    Ok(jobs)
}
