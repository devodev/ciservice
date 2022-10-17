use chrono::Utc;
use config::Config;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::{json, Value};
use rocket::Request;

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
mod responders;
mod routes;
mod schema;

#[catch(default)]
fn default_error_catcher(status: Status, req: &Request) -> Value {
    let code = status.code;
    let reason = status.reason();
    let method = req.method();
    let uri = req.uri().to_string();
    let timestamp = Utc::now();
    json!({
        "status": code,
        "reason": reason,
        "method": method,
        "uri": uri,
        "timestamp": timestamp
    })
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::config::<Config>())
        .attach(database::stage())
        .attach(routes::stage())
        .attach(AdHoc::on_response(
            "Set Server header on all responses",
            |_, resp| {
                Box::pin(async move {
                    resp.set_raw_header("Server", "ciservice");
                })
            },
        ))
        .register("/", catchers![rocket_validation::validation_catcher])
        .register("/", catchers![default_error_catcher])
}
