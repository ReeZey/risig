use bson::{Bson, Document};
use serenity::{builder::CreateApplicationCommand, model::prelude::interaction::MessageFlags};

use crate::risig::ReturnMessage;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("requestmydata").description("request your data, gdpr and stuff")
}

pub(crate) fn run(user_data: Document) -> ReturnMessage {
    let bson_data: Bson = user_data.into();
    let json_data = bson_data.into_relaxed_extjson();
    let mut pretty_json_data: String = serde_json::to_string_pretty(&json_data).unwrap();
    
    if pretty_json_data.len() > 1980 {
        pretty_json_data = format!("{}{}", &pretty_json_data[..1980], "...");
    }
    
    //send_command_response(command, &ctx, &format!("```json\n{}```", pretty_json_data), MessageFlags::EPHEMERAL).await;
    return ReturnMessage::new(&format!("```json\n{}```", &pretty_json_data), MessageFlags::EPHEMERAL);
    //send_file(&ctx, &msg, Some(), pretty_json_data.as_bytes().to_vec(), &format!("{}.json", msg.author.id)).await;
}