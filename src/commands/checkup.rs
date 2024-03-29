use crate::{utils::{get_userdata_doc, get_number}, risig::ReturnMessage};
use serenity::{builder::CreateApplicationCommand, model::prelude::interaction::{application_command::{CommandDataOptionValue, CommandDataOption}, MessageFlags}};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("checkup").description("check moni on person")
}

pub(crate) async fn run(args: Vec<CommandDataOption>) -> ReturnMessage {
    let target = if let CommandDataOptionValue::User(target, _member) = args.get(0).unwrap().resolved.as_ref().unwrap() {
        target
    } else {
        return ReturnMessage::new("how did you do this?", MessageFlags::default());
    };

    let target_data = get_userdata_doc(target.id).await;
    if target_data.is_none() {
        return ReturnMessage::new("user not found, the user must have used <@568163802907148307> atleast once", MessageFlags::EPHEMERAL);
    }
    let target_data = target_data.unwrap();

    let target_money = get_number(&target_data, "money");
    let target_bank_money = get_number(&target_data, "bank_money");
    
    return ReturnMessage::new(&format!("{} has [cash {} / bank {}]", target.name, target_money, target_bank_money), MessageFlags::default());
}