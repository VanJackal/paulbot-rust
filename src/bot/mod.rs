use log::info;
use serenity::all::{Context, EventHandler, GatewayIntents, Ready};
use serenity::{async_trait, Client};

pub struct PaulBot {
    client: Client,
}

impl PaulBot {
    pub async fn run(&mut self) {
        self.client.start().await.expect("Error starting client");
    }
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready( &self, _ctx: Context, data_about_bot: Ready) {
        info!("PaulBot logged in as {}", data_about_bot.user.id)
    }
}

pub async fn init (token: String) -> PaulBot {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_MESSAGE_TYPING
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::DIRECT_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGE_TYPING
        | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents).event_handler(Handler).await.expect("Error creating client");
    PaulBot { client }
}



