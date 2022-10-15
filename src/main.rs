use std::io;
use std::path::{Path, PathBuf};

use rocket::fairing::AdHoc;
use rocket::figment::providers::Serialized;
use rocket::fs::NamedFile;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{sleep, Duration};
use rocket::State;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/env")]
fn env(config: &State<Config>) -> String {
    config.environment.clone()
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    environment: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            environment: "dev".into(),
        }
    }
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(Serialized::defaults(Config::default()));

    rocket::custom(figment)
        .attach(AdHoc::config::<Config>())
        .mount("/", routes![index, env, delay, blocking_task, hello])
        .mount("/static", routes![files])
}
