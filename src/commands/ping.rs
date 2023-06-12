use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::MessageFlags;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use crate::utils::send_command_response;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("pong")
}

pub async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context) {
    send_command_response(command, &ctx, "pong", MessageFlags::default()).await;
}


