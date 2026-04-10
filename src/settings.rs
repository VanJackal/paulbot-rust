use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub commands: CommandSettings,
    pub database: DatabaseSettings
}

#[derive(Debug, Deserialize, Clone)]
pub struct CommandSettings {
    pub meows:Vec<String>,
    pub default_cat_id:i64
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name("config"))
            .build()?;
        s.try_deserialize()
    }
}