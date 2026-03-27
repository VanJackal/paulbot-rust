mod meow;

use std::collections::HashMap;
use std::sync::Arc;
use log::info;
use serenity::all::{CommandInteraction, Context, CreateCommand, GuildId};
use serenity::async_trait;
use crate::settings::Settings;

#[async_trait]
pub trait Command {
    async fn run(&self, ctx:Context, command:CommandInteraction );
    async fn register(&self) -> CreateCommand;
}

pub fn init_commands(settings: &Settings) -> HashMap<String, Arc<dyn Command+Send+Sync>> {
    let mut commands :HashMap<String, Arc<dyn Command+Send+Sync>> = HashMap::new();
    // INIT
    commands.insert("meow".into(), Arc::new(meow::MeowCommand::new(settings)));
    
    commands
}
pub async fn register_commands(ctx:&Context,commands:&HashMap<String,Arc<dyn Command+Send+Sync>>){
    
    // REGISTRATION
    let mut command_list:Vec<CreateCommand> = Vec::new();
    
    for (name, command) in commands.iter() {
        command_list.push(command.register().await);
        info!("Registered command: {}", name)
    }
    
    let guild = GuildId::new(667029987479650324);//todo switch to global
    guild.set_commands(&ctx.http, command_list).await.expect("Error registering commands");
    
}