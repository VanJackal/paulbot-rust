use log::warn;
use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::async_trait;
use crate::bot::commands::Command;
use crate::logic::{ImageManager, ImageProvider};

pub struct PetCommand {
    im:Box<ImageManager>
}

#[async_trait]
impl Command for PetCommand {
    async fn run(&self, ctx: Context, command: CommandInteraction) {
        let image = self.im.get_image(&None);

        let data = CreateInteractionResponseMessage::new().content(image.unwrap().url);
        let builder = CreateInteractionResponse::Message(data);
        if let Err(e) = command.create_response(&ctx.http, builder).await {
            warn!("Failed to send pet response: {:?}", e);
        }
    }

    async fn register(&self) -> CreateCommand {
        CreateCommand::new("pet")
            .description("Pet a cat")
            .add_option(CreateCommandOption::new(CommandOptionType::String, "cat", "cat to pet"))
    }
}

impl PetCommand {
    pub fn new(image_manager: Box<ImageManager>) -> Self {
        Self {
            im: image_manager
        }
    }
}