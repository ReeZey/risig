use serenity::{builder::CreateApplicationCommand, model::prelude::interaction::MessageFlags};

use crate::risig::ReturnMessage;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("pong")
}

pub fn run() -> ReturnMessage {
    return ReturnMessage::new("pong", MessageFlags::default());
}


