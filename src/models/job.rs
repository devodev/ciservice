use chrono::NaiveDateTime;
use diesel::Queryable;
use rocket::serde::Serialize;

use crate::schema::job;
use crate::schema::job_parameter;

#[derive(Identifiable, Queryable, Serialize, Clone, Hash, PartialEq, Debug)]
#[table_name = "job"]
pub(crate) struct Job {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Clone, Hash, PartialEq, Debug)]
#[table_name = "job"]
pub(crate) struct NewJob<'a> {
    pub name: &'a str,
}

#[derive(AsChangeset, Clone, Hash, PartialEq, Debug)]
#[table_name = "job"]
pub(crate) struct UpdateJob<'a> {
    pub name: &'a str,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Clone, Hash, PartialEq, Debug)]
#[belongs_to(Job)]
#[table_name = "job_parameter"]
pub(crate) struct JobParameter {
    pub id: i32,
    pub job_id: i32,
    pub name: String,
    pub value: String,
    pub r#type: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Clone, Hash, PartialEq, Debug)]
#[table_name = "job_parameter"]
pub(crate) struct NewJobParameter<'a> {
    pub job_id: i32,
    pub name: &'a str,
    pub value: &'a str,
    #[column_name = "type_"]
    pub r#type: &'a str,
}
