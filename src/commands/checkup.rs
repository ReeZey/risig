use crate::utils::{get_userdata_doc, send_command_response, get_number};
use serenity::{builder::CreateApplicationCommand, model::prelude::{interaction::{application_command::{ApplicationCommandInteraction, CommandDataOptionValue}, MessageFlags}}, prelude::Context};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("checkup").description("check moni on person")
}

pub(crate) async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context) {
    let target = if let CommandDataOptionValue::User(target, _member) = command.data.options.get(0).unwrap().resolved.as_ref().unwrap() {
        target
    } else {
        send_command_response(command, &ctx, "how did you do this?", MessageFlags::default()).await;
        return
    };

    let target_data = get_userdata_doc(target.id).await;
    if target_data.is_none() {
        send_command_response(command, &ctx, "user not found, the user must have used <@568163802907148307> atleast once", MessageFlags::EPHEMERAL).await;
        return
    }
    let target_data = target_data.unwrap();

    let target_money = get_number(&target_data, "money");
    let target_bank_money = get_number(&target_data, "bank_money");
    
    send_command_response(command, &ctx, &format!("{} has [cash {} / bank {}]", target.name, target_money, target_bank_money), MessageFlags::default()).await;
}