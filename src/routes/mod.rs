use rocket::fairing::AdHoc;

pub(crate) mod job;

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("Api routes", |rocket| async {
        rocket.mount("/api", routes![job::create, job::list])
    })
}
