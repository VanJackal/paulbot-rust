use log::info;
use crate::settings::Settings;

pub mod bot;
pub mod settings;

#[tokio::main]
async fn main() {
    colog::init(); // setup logging
    info!("Starting PaulBot");

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN environment variable must be set");
    
    let settings = Settings::new().expect("Failed to load settings");

    let mut bot = bot::init(token, &settings).await;
    bot.run().await;
}
