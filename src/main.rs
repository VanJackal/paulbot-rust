use log::info;

pub mod bot;

#[tokio::main]
async fn main() {
    colog::init(); // setup logging
    info!("Starting PaulBot");

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN environment variable must be set");

    let mut bot = bot::init(token).await;
    bot.run().await;
}
