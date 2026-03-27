use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub commands: CommandSettings
}

#[derive(Debug, Deserialize)]
pub struct CommandSettings {
    pub meows:Vec<String>
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name("config"))
            .build()?;
        s.try_deserialize()
    }
}