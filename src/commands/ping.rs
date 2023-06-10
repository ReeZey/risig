use serenity::builder::CreateApplicationCommand;

use crate::utils::CommandResponse;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("pong")
}

pub fn run() -> CommandResponse {
    return CommandResponse::new("pong".to_owned(), true);
}


