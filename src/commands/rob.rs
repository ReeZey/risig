use std::cmp::min;

use crate::utils::{get_userdata_doc, save_userdata_doc, CommandResponse};
use bson::Document;
use rand::Rng;
use serenity::{builder::CreateApplicationCommand, model::{prelude::{interaction::application_command::{CommandDataOption, CommandDataOptionValue}}, user::User}};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("rob").description("rob ris from user")
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

    let user_money: i64 = match user_data.get("money") {
        Some(money) => money.as_i64().unwrap(),
        _ => 0
    };

    if user_money < 2000 {
        return CommandResponse::new("you need to have atleast `2000 ris` to rob someone".to_owned(), true);
    }

    let target_money: i64 = match target_data.get("money") {
        Some(money) => money.as_i64().unwrap(),
        _ => 0
    };

    if target_money < 2000 {
        return CommandResponse::new("target does not have enought money, atleast `2000 ris` in cash".to_owned(), true);
    }

    let least_money = min(user_money, target_money);
    let amount = rand::thread_rng().gen_range(0..=least_money);
    
    let successful = rand::thread_rng().gen_range(0..=4) == 4;

    if !successful {
        target_data.insert("money", &target_money + amount);
        user_data.insert("money", &user_money - amount);
        save_userdata_doc(target.id, &target_data).await;
        save_userdata_doc(user.id, &user_data).await;
        return CommandResponse::new(format!("YOU FAILED TO STEAL, YOU PAYED `{} ris` TO <@{}>", amount, target.id), false);
    }

    target_data.insert("money", &target_money - amount);
    user_data.insert("money", &user_money + amount);
    save_userdata_doc(target.id, &target_data).await;
    save_userdata_doc(user.id, &user_data).await;
    return CommandResponse::new(format!("YOU STOLE `{} ris` FROM <@{}>", amount, target.id), false);
}