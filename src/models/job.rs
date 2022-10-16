use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;

use crate::config::DATE_FORMAT;

#[derive(Queryable)]
pub struct Job {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}

impl Job {
    pub fn attach(self) -> JobJson {
        JobJson {
            id: self.id,
            name: self.name,
            created_at: self.created_at.format(DATE_FORMAT).to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct JobJson {
    pub id: i32,
    pub name: String,
    pub created_at: String,
}
