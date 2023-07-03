use bson::Document;
use crate::{utils::{save_userdata_doc, send_command_response, get_number}, translator::translate};
use serenity::{builder::CreateApplicationCommand, model::{user::User, prelude::interaction::{application_command::ApplicationCommandInteraction, MessageFlags}}, prelude::Context};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("withdraw").description("withdraw moni")
}

pub(crate) async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context, user: User, mut user_data: Document) {
    let amount = command.data.options.get(0).unwrap().value.as_ref().unwrap().as_i64().unwrap();

    if amount < 1 {
        send_command_response(command, &ctx, translate("invalid-amount"), MessageFlags::EPHEMERAL).await;
        return
    }

    let money = get_number(&user_data, "money");
    let bank_money = get_number(&user_data, "bank_money");

    if amount > bank_money {
        send_command_response(command, &ctx, &format!("you are missing `{} ris`", amount - money), MessageFlags::EPHEMERAL).await;
        return
    }

    user_data.insert("money", money + amount);
    user_data.insert("bank_money", bank_money - amount);
    save_userdata_doc(user.id, &user_data).await;

    send_command_response(command, &ctx, &format!("you withdrew `{} ris`, you now have `{} ris` in cash", amount, money + amount), MessageFlags::default()).await;
}