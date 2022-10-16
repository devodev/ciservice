use rocket::fairing::AdHoc;

pub(crate) mod jobs;

#[database("postgres_db")]
pub(crate) struct Database(diesel::PgConnection);

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Postgres pool initialization", |rocket| async {
        rocket.attach(Database::fairing())
    })
}
