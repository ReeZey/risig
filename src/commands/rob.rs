use std::cmp::min;

use crate::{utils::{get_userdata_doc, save_userdata_doc, send_command_response, get_number}, translator::translate};
use bson::Document;
use rand::Rng;
use serenity::{builder::CreateApplicationCommand, model::{prelude::{interaction::{application_command::{CommandDataOptionValue, ApplicationCommandInteraction}, MessageFlags}}, user::User}, prelude::Context};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("rob").description("rob ris from user")
}

pub(crate) async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context, user: User, mut user_data: Document) {
    let target = if let CommandDataOptionValue::User(target, _member) = command.data.options.get(0).unwrap().resolved.as_ref().unwrap() {
        target
    } else {
        send_command_response(command, &ctx, "how did you do this?", MessageFlags::default()).await;
        return
    };

    if target.id == user.id {
        send_command_response(command, &ctx, "you cannot rob yourself", MessageFlags::default()).await;
        return;
    }

    let target_data = get_userdata_doc(target.id).await;
    if target_data.is_none() {
        send_command_response(command, &ctx, translate("user-not-found"), MessageFlags::EPHEMERAL).await;
        return
    }
    let mut target_data = target_data.unwrap();

    let money = get_number(&user_data, "money");

    if money < 2000 {
        send_command_response(command, &ctx, "you need to have atleast `2000 ris` to rob someone", MessageFlags::EPHEMERAL).await;
        return
    }

    let target_money = get_number(&target_data, "money");

    if target_money < 2000 {
        send_command_response(command, &ctx, "target does not have enought money, atleast `2000 ris` in cash", MessageFlags::EPHEMERAL).await;
        return
    }

    let least_money = min(money, target_money);
    let amount = rand::thread_rng().gen_range(0..=least_money);
    
    let successful = rand::thread_rng().gen_range(0..2) == 0;

    if !successful {
        target_data.insert("money", &target_money + amount);
        user_data.insert("money", &money - amount);
        save_userdata_doc(target.id, &target_data).await;
        save_userdata_doc(user.id, &user_data).await;

        send_command_response(command, &ctx, &format!("YOU FAILED TO STEAL, YOU PAYED `{} ris` TO <@{}>", amount, target.id), MessageFlags::default()).await;
        return
    }

    user_data.insert("money", &money + amount);
    target_data.insert("money", &target_money - amount);
    save_userdata_doc(target.id, &target_data).await;
    save_userdata_doc(user.id, &user_data).await;

    send_command_response(command, &ctx, &format!("YOU STOLE `{} ris` FROM <@{}>", amount, target.id), MessageFlags::default()).await;
}