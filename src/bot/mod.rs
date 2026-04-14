pub mod commands;

use std::collections::HashMap;
use std::sync::Arc;
use log::{debug, info};
use serenity::all::{Context, EventHandler, GatewayIntents, Interaction, Ready};
use serenity::{async_trait, Client};
use crate::bot::commands::{register_commands, Command};
use crate::logic::ImageManager;
use crate::settings::Settings;

pub struct PaulBot {
    client: Client,
}

impl PaulBot {
    pub async fn run(&mut self) {
        self.client.start().await.expect("Error starting client");
    }
}

pub struct Handler{

    commands: HashMap<String, Arc<dyn Command>>
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready( &self, ctx: Context, data_about_bot: Ready) {
        info!("PaulBot logged in as {}", data_about_bot.user.id);

        register_commands(&ctx, &self.commands).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            debug!("Received command interaction -> {command:#?}");

            let cmd = self.commands.get(command.data.name.as_str()).unwrap();

            cmd.run(ctx, command).await;
        }
    }
}

pub async fn init<'a> (token:String, settings: &Settings, image_manager: Box<ImageManager>) -> PaulBot {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_MESSAGE_TYPING
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::DIRECT_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGE_TYPING
        | GatewayIntents::MESSAGE_CONTENT;

    let commands = commands::init_commands(&settings, image_manager);

    let handler = Handler { commands };
    let client = Client::builder(&token, intents).event_handler(handler).await.expect("Error creating client");

    PaulBot { client }
}



