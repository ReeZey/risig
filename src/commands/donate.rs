use bson::Document;
use crate::utils::{save_userdata_doc, get_userdata_doc, CommandResponse};
use serenity::{builder::CreateApplicationCommand, model::{user::User, prelude::{interaction::application_command::{CommandDataOption, CommandDataOptionValue}}}};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("donate").description("donate moni")
}

pub(crate) async fn run(user: User, mut user_data: Document, options: &Vec<CommandDataOption>) -> CommandResponse {
    let target = if let CommandDataOptionValue::User(target, _member) = options.get(0).unwrap().resolved.as_ref().unwrap() {
        target
    } else {
        return CommandResponse::new("what?".to_owned(), true);
    };

    let target_data = get_userdata_doc(target.id).await;
    if target_data.is_none() {
        return CommandResponse::new("user not found, the user must have used <@568163802907148307> atleast once".to_owned(), true);
    }
    let mut target_data = target_data.unwrap();

    let amount = options.get(1).unwrap().value.as_ref().unwrap().as_i64().unwrap();
    if amount < 1 {
        return CommandResponse::new("invalid amount".to_owned(), false);
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
        return CommandResponse::new(format!("not enough money [{} < {}]", money, amount), true);
    }

    user_data.insert("money", money - amount);
    target_data.insert("money", target_money + amount);

    save_userdata_doc(user.id, &user_data).await;
    save_userdata_doc(target.id, &target_data).await;

    return CommandResponse::new(format!("you donated `{} ris` to <@{}>", amount, target.id), false);
}