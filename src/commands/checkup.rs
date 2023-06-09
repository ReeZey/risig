use bson::Document;
use crate::utils::{save_userdata_doc, get_userdata_doc};
use serenity::{builder::CreateApplicationCommand, model::{user::User, prelude::{interaction::application_command::{CommandDataOption, CommandDataOptionValue}, PartialMember}}};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("checkup").description("check moni on person")
}

pub(crate) async fn run(options: &Vec<CommandDataOption>) -> String {
    let target = if let CommandDataOptionValue::User(target, _member) = options.get(0).unwrap().resolved.as_ref().unwrap() {
        target
    } else {
        return "what?".to_owned();
    };

    let target_data = get_userdata_doc(target.id).await;
    if target_data.is_none() {
        return "user not found, the user must have used risig atleast once".to_owned();
    }
    let target_data = target_data.unwrap();

    let target_money: i64 = match target_data.get("money") {
        Some(target_money) => target_money.as_i64().unwrap(),
        _ => 0
    };

    let target_bank_money: i64 = match target_data.get("bank_money") {
        Some(target_bank_money) => target_bank_money.as_i64().unwrap(),
        _ => 0
    };

    return format!("{} has [cash {} / bank {}]", target.name, target_money, target_bank_money);
}