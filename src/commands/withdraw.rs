use bson::Document;
use crate::utils::save_userdata_doc;
use serenity::{builder::CreateApplicationCommand, model::{user::User, prelude::interaction::application_command::CommandDataOption}};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("withdraw").description("withdraw moni")
}

pub(crate) async fn run(user: User, mut user_data: Document, options: &Vec<CommandDataOption>) -> String {
    //let amount: i64 = options.get(0).unwrap().value.unwrap().as_i64().unwrap();
    let amount = options.get(0).unwrap().value.as_ref().unwrap().as_i64().unwrap();

    if amount < 0 {
        return "you can't withdraw negative".to_owned();
    }
    
    if amount == 0 {
        return "no".to_owned();
    }

    let money: i64 = match user_data.get("money") {
        Some(money) => money.as_i64().unwrap(),
        _ => 0
    };

    let bank_money: i64 = match user_data.get("bank_money") {
        Some(bank_money) => bank_money.as_i64().unwrap(),
        _ => 0
    };

    if amount > bank_money {
        return format!("you dont have that much money in the bank [{} < {}]", bank_money, amount);
    }

    user_data.insert("money", money + amount);
    user_data.insert("bank_money", bank_money - amount);
    save_userdata_doc(user.id, &user_data).await;

    return format!("you withdrew `{} ris`, you now have `{} ris` in cash", amount, money + amount);
}