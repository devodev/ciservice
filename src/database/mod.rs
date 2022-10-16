use rocket::fairing::AdHoc;

pub(crate) mod jobs;
mod pagination;

const DEFAULT_LIMIT: i64 = 20;

#[database("postgres_db")]
pub(crate) struct Database(diesel::PgConnection);

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Postgres pool initialization", |rocket| async {
        rocket.attach(Database::fairing())
    })
}

pub struct PaginatedParams {
    pub limit: i64,
    pub page: i64,
}

impl Default for PaginatedParams {
    fn default() -> Self {
        Self {
            limit: DEFAULT_LIMIT,
            page: 0,
        }
    }
}
