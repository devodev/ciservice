use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use serde::Serialize;
use validator::Validate;

use crate::database;
use crate::database::Database;
use crate::errors::{Errors, FieldValidator};
use crate::models::job::{Job, JobJson};

type Result<T, E = Errors> = std::result::Result<T, E>;

#[derive(Deserialize, Validate)]
pub(crate) struct CreateRequest {
    #[validate(length(min = 1))]
    name: Option<String>,
}

#[post("/jobs", format = "json", data = "<new_job>")]
pub(crate) async fn create<'a>(
    new_job: Json<CreateRequest>,
    db: Database,
) -> Result<Created<Json<JobJson>>> {
    let new_job = new_job.into_inner();

    let mut validator = FieldValidator::validate(&new_job);
    let name = validator.extract("name", new_job.name);
    validator.check()?;

    let job = db
        .run(move |conn| database::jobs::create(conn, &name))
        .await
        .map_err(|e| Errors::DatabaseError(e.0))?;

    let created_location = format!("/jobs/{}", job.id);
    Ok(Created::new(created_location).body(Json(job.attach())))
}

#[derive(Serialize)]
pub(crate) struct ListResponse {
    jobs: Vec<JobJson>,
    count: usize,
}

impl From<Vec<Job>> for ListResponse {
    fn from(value: Vec<Job>) -> Self {
        let jobs: Vec<JobJson> = value.into_iter().map(|job| job.attach()).collect();
        let count = jobs.len();
        Self { jobs, count }
    }
}

#[get("/jobs", format = "json")]
pub(crate) async fn list(db: Database) -> Result<Json<ListResponse>> {
    let jobs = db
        .run(database::jobs::list)
        .await
        .map_err(|e| Errors::DatabaseError(e.0))?;

    Ok(Json(jobs.into()))
}
