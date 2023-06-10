use bson::Document;
use crate::utils::{save_userdata_doc, get_userdata_doc};
use serenity::{builder::CreateApplicationCommand, model::{user::User, prelude::{interaction::application_command::{CommandDataOption, CommandDataOptionValue}}}};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("donate").description("donate moni")
}

pub(crate) async fn run(user: User, mut user_data: Document, options: &Vec<CommandDataOption>) -> String {
    let target = if let CommandDataOptionValue::User(target, _member) = options.get(0).unwrap().resolved.as_ref().unwrap() {
        target
    } else {
        return "what?".to_owned();
    };

    let target_data = get_userdata_doc(target.id).await;
    if target_data.is_none() {
        return "user not found, the user must have used <@568163802907148307> atleast once".to_owned();
    }
    let mut target_data = target_data.unwrap();

    let amount = options.get(1).unwrap().value.as_ref().unwrap().as_i64().unwrap();
    if amount < 0 {
        return "you can't donate negative amount".to_owned();
    }

    if amount == 0 {
        return "no".to_owned();
    }

    let money: i64 = match user_data.get("money") {
        Some(money) => money.as_i64().unwrap(),
        _ => 0
    };

    let target_money: i64 = match target_data.get("money") {
        Some(target_money) => target_money.as_i64().unwrap(),
        _ => 0
    };

    if amount > money {
        return format!("not enough money [{} < {}]", money, amount);
    }

    user_data.insert("money", money - amount);
    target_data.insert("money", target_money + amount);

    save_userdata_doc(user.id, &user_data).await;
    save_userdata_doc(target.id, &target_data).await;

    return format!("you donated `{} ris` to <@{}>", amount, target.id);
}