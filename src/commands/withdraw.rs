use bson::Document;
use crate::{utils::{save_userdata_doc, get_number}, translator::translate, risig::ReturnMessage};
use serenity::{builder::CreateApplicationCommand, model::{user::User, prelude::interaction::{application_command::CommandDataOption, MessageFlags}}};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("withdraw").description("withdraw moni")
}

pub(crate) async fn run(user: User, mut user_data: Document, args: Vec<CommandDataOption>) -> ReturnMessage {
    let amount = args.get(0).unwrap().value.as_ref().unwrap().as_i64().unwrap();

    if amount < 1 {
        return ReturnMessage::new(translate("invalid-amount"), MessageFlags::EPHEMERAL);
    }

    let money = get_number(&user_data, "money");
    let bank_money = get_number(&user_data, "bank_money");

    if amount > bank_money {
        return ReturnMessage::new(&format!("you are missing `{} ris`", amount - money), MessageFlags::EPHEMERAL);
    }

    user_data.insert("money", money + amount);
    user_data.insert("bank_money", bank_money - amount);
    save_userdata_doc(user.id, &user_data).await;

    return ReturnMessage::new(&format!("you withdrew `{} ris`, you now have `{} ris` in cash", amount, money + amount), MessageFlags::default());
}