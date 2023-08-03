use bson::Document;
use serenity::{builder::CreateApplicationCommand, model::prelude::interaction::MessageFlags};

use crate::{utils::get_number, risig::ReturnMessage};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("balance").description("shows balance")
}

pub(crate) async fn run(user_data: Document) -> ReturnMessage {
    let money = get_number(&user_data, "money");
    let bank_money = get_number(&user_data, "bank_money");

    return ReturnMessage::new(&format!("you have `{} ris`\nand `{} ris` in the bank", money, bank_money), MessageFlags::default());
}