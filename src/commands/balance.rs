use bson::Document;
use serenity::{builder::CreateApplicationCommand, prelude::Context, model::prelude::interaction::{application_command::ApplicationCommandInteraction, MessageFlags}};

use crate::utils::send_command_response;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("balance").description("shows balance")
}

pub(crate) async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context, user_data: Document) {
    let money: i64 = match user_data.get("money") {
        Some(money) => money.as_i64().unwrap(),
        None => 0
    };

    let bank_money: i64 = match user_data.get("bank_money") {
        Some(bank_money) => bank_money.as_i64().unwrap(),
        None => 0
    };

    send_command_response(command, &ctx, &format!("you have `{} ris`\nand `{} ris` in the bank", money, bank_money), MessageFlags::default()).await;
}