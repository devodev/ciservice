use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use validator::Validate;

use crate::database;
use crate::database::Database;
use crate::errors::{Errors, FieldValidator};
use crate::models::job::Job;
use crate::routes::{ListResponse, Pagination};

type Result<T, E = Errors> = std::result::Result<T, E>;

#[derive(Deserialize, Validate)]
pub(crate) struct CreateRequest {
    #[validate(length(min = 1))]
    name: Option<String>,
}

#[post("/jobs", format = "json", data = "<new_job>")]
pub(crate) async fn create<'a>(
    db: Database,
    new_job: Json<CreateRequest>,
) -> Result<Created<Json<Job>>> {
    let new_job = new_job.into_inner();

    let mut validator = FieldValidator::validate(&new_job);
    let name = validator.extract("name", new_job.name);
    validator.check()?;

    let job = db
        .run(move |conn| database::jobs::create(conn, &name))
        .await
        .map_err(|e| Errors::DatabaseError(e.0))?;

    let created_location = format!("/jobs/{}", job.id);
    Ok(Created::new(created_location).tagged_body(Json(job)))
}

#[get("/jobs?<params..>", format = "json")]
pub(crate) async fn list(db: Database, params: Pagination) -> Result<Json<ListResponse<Job>>> {
    let paginated_jobs = db
        .run(move |c| database::jobs::list(c, &params.into()))
        .await
        .map_err(|e| Errors::DatabaseError(e.0))?;

    let results = paginated_jobs.data;
    let count = results.len();

    Ok(Json(ListResponse {
        results,
        count,
        limit: paginated_jobs.limit,
        page: paginated_jobs.page,
        total: paginated_jobs.total,
    }))
}

#[get("/jobs/<id>", format = "json")]
pub(crate) async fn get(db: Database, id: i32) -> Result<Json<Job>> {
    let job = db
        .run(move |c| database::jobs::get(c, id))
        .await
        .map_err(|e| Errors::DatabaseError(e.0))?;

    Ok(Json(job))
}

#[derive(Deserialize, Validate)]
pub(crate) struct UpdateRequest {
    #[validate(length(min = 1))]
    name: Option<String>,
}

#[put("/jobs/<id>", format = "json", data = "<update_job>")]
pub(crate) async fn update(
    db: Database,
    update_job: Json<UpdateRequest>,
    id: i32,
) -> Result<Json<Job>> {
    let update_job = update_job.into_inner();

    let mut validator = FieldValidator::validate(&update_job);
    let name = validator.extract("name", update_job.name);
    validator.check()?;

    let job = db
        .run(move |c| database::jobs::update(c, id, &name))
        .await
        .map_err(|e| Errors::DatabaseError(e.0))?;

    Ok(Json(job))
}

#[delete("/jobs/<id>")]
pub(crate) async fn delete(db: Database, id: i32) -> Result<NoContent> {
    db.run(move |c| database::jobs::delete(c, id))
        .await
        .map_err(|e| Errors::DatabaseError(e.0))?;

    Ok(NoContent)
}
