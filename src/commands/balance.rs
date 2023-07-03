use bson::Document;
use serenity::{builder::CreateApplicationCommand, prelude::Context, model::prelude::interaction::{application_command::ApplicationCommandInteraction, MessageFlags}};

use crate::utils::{send_command_response, get_number};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("balance").description("shows balance")
}

pub(crate) async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context, user_data: Document) {
    let money = get_number(&user_data, "money");
    let bank_money = get_number(&user_data, "bank_money");

    send_command_response(command, &ctx, &format!("you have `{} ris`\nand `{} ris` in the bank", money, bank_money), MessageFlags::default()).await;
}