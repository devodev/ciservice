use rocket::serde::{Deserialize, Serialize};

pub const DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub environment: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            environment: "dev".into(),
        }
    }
}
