use std::time::Duration;

use bson::Document;
use rand::Rng;
use tokio::time;
use crate::{utils::{save_userdata_doc, get_number}, translator::translate, risig::ReturnMessage};
use serenity::{builder::CreateApplicationCommand, model::{user::User, prelude::interaction::{application_command::CommandDataOption, MessageFlags}}};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("gamba").description("GAMBA GAMBA")
}

pub(crate) async fn run(user: User, mut user_data: Document, args: Vec<CommandDataOption>) -> ReturnMessage {
    let amount = args.get(0).unwrap().value.as_ref().unwrap().as_i64().unwrap();

    if amount < 1 {
        return ReturnMessage::new(translate("invalid-amount"), MessageFlags::EPHEMERAL);
    }

    let money = get_number(&user_data, "money");

    if amount > money {
        return ReturnMessage::new(&format!("{} `{} ris`", translate("too-poor"), amount - money), MessageFlags::EPHEMERAL);
    }

    let gamba = rand::thread_rng().gen_range(0..=4);
    if gamba != 4 {
        user_data.insert("money", money - amount);
        save_userdata_doc(user.id, &user_data).await;
        
        return ReturnMessage::new(&format!("<@{}> lost `{} ris` <:disbelief:1037738451493203998> you now have `{} ris`", user.id, amount, money - amount), MessageFlags::default());
    }

    let win_amount = amount * 4;
    user_data.insert("money", money + win_amount);
    save_userdata_doc(user.id, &user_data).await;

    //fix timing sometime
    //time::sleep(Duration::new(3, 0)).await;
    
    return ReturnMessage::new(&format!("💰💰💰 <@{}> WON `{} ris` 💰💰💰, you now have `{} ris`", user.id, win_amount, money + win_amount), MessageFlags::default());
}