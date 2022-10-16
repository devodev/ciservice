use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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
