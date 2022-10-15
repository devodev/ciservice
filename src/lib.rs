use std::io;
use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;
use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{sleep, Duration};
use rocket::State;

#[macro_use]
extern crate rocket;

mod config;
pub use config::Config;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/env")]
pub fn env(config: &State<config::Config>) -> String {
    config.environment.clone()
}

#[get("/delay/<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking_task")]
pub async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[get("/hello/<name>/<age>/<cool>")]
pub fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/<file..>")]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}
