use log::warn;
use serenity::all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction};
use serenity::async_trait;
use crate::bot::commands::Command;

pub struct MeowCommand;

#[async_trait]
impl Command for MeowCommand {
    async fn run(&self, ctx:Context, command:CommandInteraction) {
        let data =
            CreateInteractionResponseMessage::new().content("*Meow!!!*");//todo add more responses
        let builder = CreateInteractionResponse::Message(data);
        if let Err(e)  = command.create_response(&ctx.http, builder).await{
            warn!("Failed to respond to meow command: {}", e)
        }
    }

    async fn register(&self) -> CreateCommand{
        CreateCommand::new("meow").description("Meow!!!")
    }
}