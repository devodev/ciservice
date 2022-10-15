use rocket::fairing::AdHoc;
use rocket::figment::providers::Serialized;
use rocket_sync_db_pools::{database, diesel};

#[macro_use]
extern crate rocket;

#[database("postgres_db")]
struct Database(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    let figment =
        rocket::Config::figment().merge(Serialized::defaults(ciservice::Config::default()));

    rocket::custom(figment)
        .attach(AdHoc::config::<ciservice::Config>())
        .attach(Database::fairing())
        .mount(
            "/",
            routes![
                ciservice::index,
                ciservice::env,
                ciservice::delay,
                ciservice::blocking_task,
                ciservice::hello
            ],
        )
        .mount("/static", routes![ciservice::files])
}
