use chrono::NaiveDateTime;
use diesel::Queryable;
use rocket::serde::Serialize;

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

#[derive(AsChangeset)]
#[table_name = "job"]
pub(crate) struct UpdateJob<'a> {
    pub name: &'a str,
}
