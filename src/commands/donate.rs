use bson::Document;
use serenity::{builder::CreateApplicationCommand, model::{prelude::interaction::{application_command::{ApplicationCommandInteraction, CommandDataOptionValue}, MessageFlags}, user::User}, prelude::Context};
use crate::utils::{save_userdata_doc, get_userdata_doc, send_command_response};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("donate").description("donate moni")
}

pub(crate) async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context, user: User, mut user_data: Document) {
    let target = if let CommandDataOptionValue::User(target, _member) = command.data.options.get(0).unwrap().resolved.as_ref().unwrap() {
        target
    } else {
        send_command_response(command, &ctx, "how did you do this?", MessageFlags::EPHEMERAL).await;
        return
    };

    let target_data = get_userdata_doc(target.id).await;
    if target_data.is_none() {
        send_command_response(command, &ctx, "user not found, the user must have used <@568163802907148307> atleast once", MessageFlags::EPHEMERAL).await;
        return
    }
    let mut target_data = target_data.unwrap();

    let amount = command.data.options.get(1).unwrap().value.as_ref().unwrap().as_i64().unwrap();
    if amount < 1 {
        send_command_response(command, &ctx, "invalid amount", MessageFlags::EPHEMERAL).await;
        return
    }

    let money: i64 = match user_data.get("money") {
        Some(money) => money.as_i64().unwrap(),
        _ => 0
    };

    let target_money: i64 = match target_data.get("money") {
        Some(target_money) => target_money.as_i64().unwrap(),
        _ => 0
    };

    if amount > money {
        send_command_response(command, &ctx, &format!("you are missing `{} ris`", amount - money), MessageFlags::EPHEMERAL).await;
        return
    }

    user_data.insert("money", money - amount);
    target_data.insert("money", target_money + amount);

    save_userdata_doc(user.id, &user_data).await;
    save_userdata_doc(target.id, &target_data).await;

    send_command_response(command, &ctx, &format!("you donated `{} ris` to <@{}>", amount, target.id), MessageFlags::default()).await;
}