use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_validation::{Validate, Validated};

use crate::database;
use crate::database::Database;
use crate::errors::Error;
use crate::models::job::Job;
use crate::routes::{ListResponse, Pagination};

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub(crate) struct CreateRequest {
    #[validate(length(min = 1))]
    name: String,
}

#[post("/jobs", format = "json", data = "<new_job>")]
pub(crate) async fn create<'a>(
    db: Database,
    new_job: Validated<Json<CreateRequest>>,
) -> Result<Created<Json<Job>>> {
    let new_job = new_job.into_inner();

    let job = db
        .run(move |conn| database::jobs::create(conn, &new_job.name))
        .await
        .map_err(|e| Error::DatabaseError(e.0))?;

    let created_location = format!("/jobs/{}", job.id);
    Ok(Created::new(created_location).tagged_body(Json(job)))
}

#[get("/jobs?<params..>", format = "json")]
pub(crate) async fn list(db: Database, params: Pagination) -> Result<Json<ListResponse<Job>>> {
    let paginated_jobs = db
        .run(move |c| database::jobs::list(c, &params.into()))
        .await
        .map_err(|e| Error::DatabaseError(e.0))?;

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
        .map_err(|e| Error::DatabaseError(e.0))?;

    Ok(Json(job))
}

#[derive(Deserialize, Validate)]
pub(crate) struct UpdateRequest {
    #[validate(length(min = 1))]
    name: String,
}

#[put("/jobs/<id>", format = "json", data = "<update_job>")]
pub(crate) async fn update(
    db: Database,
    update_job: Validated<Json<UpdateRequest>>,
    id: i32,
) -> Result<Json<Job>> {
    let update_job = update_job.into_inner();

    let job = db
        .run(move |c| database::jobs::update(c, id, &update_job.name))
        .await
        .map_err(|e| Error::DatabaseError(e.0))?;

    Ok(Json(job))
}

#[delete("/jobs/<id>")]
pub(crate) async fn delete(db: Database, id: i32) -> Result<NoContent> {
    db.run(move |c| database::jobs::delete(c, id))
        .await
        .map_err(|e| Error::DatabaseError(e.0))?;

    Ok(NoContent)
}
