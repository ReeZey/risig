use bson::{Bson, Document};
use serenity::builder::CreateApplicationCommand;

use crate::utils::CommandResponse;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("requestmydata").description("request your data, gdpr and stuff")
}

pub(crate) async fn run(user_data: Document) -> CommandResponse {
    let bson_data: Bson = user_data.into();
    let json_data = bson_data.into_relaxed_extjson();
    let pretty_json_data: String = serde_json::to_string_pretty(&json_data).unwrap();

    return CommandResponse::new(format!("```json\n{}```", pretty_json_data), true);
    //send_file(&ctx, &msg, Some(), pretty_json_data.as_bytes().to_vec(), &format!("{}.json", msg.author.id)).await;
}