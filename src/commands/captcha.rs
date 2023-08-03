use bson::Document;
use serenity::model::{user::User, prelude::interaction::{application_command::CommandDataOption, MessageFlags}};
use crate::{risig::ReturnMessage, utils::save_userdata_doc};
use serenity::builder::CreateApplicationCommand;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("captcha").description("solve captcha if you are required")
}

pub(crate) async fn run(user: User, mut user_data: Document, args: Vec<CommandDataOption>) -> ReturnMessage {
    if !user_data.contains_key("captcha") {
        return ReturnMessage::new("you dont have a captcha?", MessageFlags::default());
    }
    
    let mut captcha = user_data.get("captcha").unwrap().as_document().unwrap().to_owned();
    let error_count = match captcha.get("error-count") {
        Some(bson) => bson.as_i64().unwrap(),
        None => 0
    };

    let input_phrase = args.get(0).unwrap().value.as_ref().unwrap().as_str().unwrap();
    let captcha_phrase = captcha.get("phrase").unwrap().as_str().unwrap();

    if input_phrase != captcha_phrase {
        captcha.insert("error-count", error_count + 1);
        user_data.insert("captcha", captcha);
        save_userdata_doc(user.id, &user_data).await;
        return ReturnMessage::new("wrong captcha!!!!!", MessageFlags::default());
    }

    user_data.remove("captcha");
    user_data.remove("pending-captcha");
    save_userdata_doc(user.id, &user_data).await;
    return ReturnMessage::new("ok thank, continue", MessageFlags::default());
}