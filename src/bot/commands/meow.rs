use log::warn;
use serenity::all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::async_trait;
use crate::bot::commands::Command;
use crate::settings::Settings;

pub struct MeowCommand {
    meows:Vec<String>
}

#[async_trait]
impl Command for MeowCommand {
    async fn run(&self, ctx:Context, command:CommandInteraction) {
        let meow = self.meows.get(rand::random_range(0..self.meows.len())).unwrap();
        let data =
            CreateInteractionResponseMessage::new().content(meow);
        let builder = CreateInteractionResponse::Message(data);
        if let Err(e)  = command.create_response(&ctx.http, builder).await{
            warn!("Failed to respond to meow command: {}", e)
        }
    }

    async fn register(&self) -> CreateCommand{
        CreateCommand::new("meow").description("Meow!!!")
    }
}
impl MeowCommand {
    pub fn new(settings:&Settings) -> Self {
        let meows = settings.commands.meows.clone();
        Self {
            meows
        }
    }
}