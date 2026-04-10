use serenity::all::{CommandInteraction, Context, CreateCommand};
use serenity::async_trait;
use crate::bot::commands::Command;
use crate::logic::ImageManager;

pub struct PetCommand <'a>{
    im:&'a ImageManager<'a>
}

#[async_trait]
impl Command for PetCommand<'_> {
    async fn run(&self, ctx: Context, command: CommandInteraction) {

    }

    async fn register(&self) -> CreateCommand {
        todo!()
    }
}

impl <'a> PetCommand<'a> {
    pub fn new(image_manager: &'a ImageManager) -> Self {
        Self {
            im: image_manager
        }
    }
}