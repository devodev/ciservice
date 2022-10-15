use rocket::fairing::AdHoc;
use rocket::figment::providers::Serialized;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let figment =
        rocket::Config::figment().merge(Serialized::defaults(ciservice::Config::default()));

    rocket::custom(figment)
        .attach(AdHoc::config::<ciservice::Config>())
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
