use log::info;
use crate::data::pool::get_pool;
use crate::data::ImageDB;
use crate::logic::ImageManager;
use crate::settings::Settings;

pub mod bot;
pub mod settings;
pub mod logic;
pub mod data;
pub mod objects;
mod error;

#[tokio::main]
async fn main() {
    colog::init(); // setup logging
    info!("Starting PaulBot");

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN environment variable must be set");

    let settings = Settings::new().expect("Failed to load settings");
    
    let pool = get_pool(&settings);
    
    let db = ImageDB::new(&settings, pool);
    
    let image_manager:ImageManager = ImageManager::new(settings.clone(), Box::new(db));

    let mut bot = bot::init(token, &settings, Box::new(image_manager)).await;
    bot.run().await;
}
