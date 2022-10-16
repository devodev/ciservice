use rocket::serde::{Deserialize, Serialize};

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
