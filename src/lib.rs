use config::Config;
use rocket::serde::json::{json, Value};
use rocket::{fairing::AdHoc, figment::providers::Serialized};

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_sync_db_pools;

#[macro_use]
extern crate diesel;

mod config;
mod database;
mod errors;
mod guards;
mod models;
mod playground;
mod responders;
mod routes;
mod schema;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Not found"
    })
}

#[catch(500)]
fn internal_server_error() -> Value {
    json!({
        "status": "error",
        "reason": "Internal Server Error"
    })
}

#[launch]
pub fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(Serialized::defaults(Config::default()));

    rocket::custom(figment)
        .attach(AdHoc::config::<Config>())
        .attach(database::stage())
        .attach(routes::stage())
        .attach(playground::stage())
        .register("/", catchers![not_found, internal_server_error])
}
