use bson::{Bson, Document};
use serenity::{builder::CreateApplicationCommand, model::{prelude::interaction::{application_command::ApplicationCommandInteraction, MessageFlags}, user::User}, prelude::Context};

use crate::utils::send_file_command_response;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("requestmydata").description("request your data, gdpr and stuff")
}

pub(crate) async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context, user: User, user_data: Document) {
    let bson_data: Bson = user_data.into();
    let json_data = bson_data.into_relaxed_extjson();
    let pretty_json_data: String = serde_json::to_string_pretty(&json_data).unwrap();
    
    //send_command_response(command, &ctx, &format!("```json\n{}```", pretty_json_data), MessageFlags::EPHEMERAL).await;
    send_file_command_response(command, &ctx, &pretty_json_data[..2000], (pretty_json_data.as_bytes().to_vec(), &format!("{}.json", user.name)), MessageFlags::EPHEMERAL).await;
    //send_file(&ctx, &msg, Some(), pretty_json_data.as_bytes().to_vec(), &format!("{}.json", msg.author.id)).await;
}