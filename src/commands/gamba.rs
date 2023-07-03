use std::time::Duration;

use bson::Document;
use rand::Rng;
use tokio::time;
use crate::{utils::{save_userdata_doc, send_command_response, send_message, get_number}, translator::translate};
use serenity::{builder::CreateApplicationCommand, model::{user::User, prelude::interaction::{application_command::ApplicationCommandInteraction, MessageFlags}}, prelude::Context};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("gamba").description("GAMBA GAMBA")
}

pub(crate) async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context, user: User, mut user_data: Document) {
    let amount = command.data.options.get(0).unwrap().value.as_ref().unwrap().as_i64().unwrap();

    if amount < 1 {
        send_command_response(command, &ctx, translate("invalid-amount"), MessageFlags::EPHEMERAL).await;
        return
    }

    let money = get_number(&user_data, "money");

    if amount > money {
        send_command_response(command, &ctx, &format!("{} `{} ris`", translate("too-poor"), amount - money), MessageFlags::EPHEMERAL).await;
        return
    }

    
    send_command_response(command, &ctx, "<a:GAMBA:1118516455110099016>", MessageFlags::default()).await;

    let gamba = rand::thread_rng().gen_range(0..=4);
    if gamba != 4 {
        user_data.insert("money", money - amount);
        save_userdata_doc(user.id, &user_data).await;

        time::sleep(Duration::new(3, 0)).await;
        
        send_message(&ctx, &command.channel_id, &format!("<@{}> lost `{} ris` <:disbelief:1037738451493203998> you now have `{} ris`", user.id, amount, money - amount)).await;
        return
    }

    let win_amount = amount * 4;
    user_data.insert("money", money + win_amount);
    save_userdata_doc(user.id, &user_data).await;

    time::sleep(Duration::new(3, 0)).await;
    
    send_message(&ctx, &command.channel_id, &format!("💰💰💰 <@{}> WON `{} ris` 💰💰💰, you now have `{} ris`", user.id, win_amount, money + win_amount)).await;
}