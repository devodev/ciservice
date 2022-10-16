use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;

use crate::schema::job;

#[derive(Hash, Queryable, Serialize)]
pub struct Job {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "job"]
pub(crate) struct NewJob<'a> {
    pub name: &'a str,
}
