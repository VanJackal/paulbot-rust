use paulbot_rust::settings::{CommandSettings, DatabaseSettings, Settings};

/**
* Returns a settings object for testing.
*/
pub fn settings() -> Settings {
    Settings{
        commands: CommandSettings {
            meows: vec!["meow".into()],
            default_cat_id: 1
        },
        database: DatabaseSettings {
            url: ":memory:".into()
        }
    }
}