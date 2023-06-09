use bson::Document;
use rand::Rng;
use crate::utils::save_userdata_doc;
use serenity::{builder::CreateApplicationCommand, model::{user::User, prelude::interaction::application_command::CommandDataOption}};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("gamba").description("GAMBA GAMBA")
}

pub(crate) async fn run(user: User, mut user_data: Document, options: &Vec<CommandDataOption>) -> String {
    let amount = options.get(0).unwrap().value.as_ref().unwrap().as_i64().unwrap();

    if amount < 0 {
        return "you can't gamba negative".to_owned();
    }

    if amount == 0 {
        return "no".to_owned();
    }

    let money: i64 = match user_data.get("money") {
        Some(money) => money.as_i64().unwrap(),
        _ => 0
    };

    if amount > money {
        return format!("you dont have that much money [{} < {}]", money, amount);
    }

    let gamba = rand::thread_rng().gen_range(0..=4);
    if gamba != 4 {
        user_data.insert("money", money - amount);
        save_userdata_doc(user.id, &user_data).await;
        return format!("you lost {}, you now have {}", amount, money - amount);
    }

    let win_amount = amount * 4;
    user_data.insert("money", money + win_amount);
    save_userdata_doc(user.id, &user_data).await;
    return format!("you WON {}, you now have {}", win_amount, money + win_amount);
}