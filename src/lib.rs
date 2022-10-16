use chrono::Utc;
use config::Config;
use rocket::http::Status;
use rocket::serde::json::{json, Value};
use rocket::Request;
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

#[catch(default)]
fn default_error_catcher(status: Status, req: &Request) -> Value {
    let code = status.code;
    let reason = status.reason();
    let uri = req.uri().to_string();
    let timestamp = Utc::now();
    json!({
        "status": code,
        "reason": reason,
        "uri": uri,
        "timestamp": timestamp
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
        .register("/", catchers![default_error_catcher])
}
