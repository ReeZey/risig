use bson::Document;
use serenity::{builder::CreateApplicationCommand, model::{prelude::interaction::{application_command::{CommandDataOptionValue, CommandDataOption}, MessageFlags}, user::User}};
use crate::{utils::{save_userdata_doc, get_userdata_doc, get_number}, translator::translate, risig::ReturnMessage};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("donate").description("donate moni")
}

pub(crate) async fn run(user: User, mut user_data: Document, args: Vec<CommandDataOption>) -> ReturnMessage {
    let target = if let CommandDataOptionValue::User(target, _member) = args.get(0).unwrap().resolved.as_ref().unwrap() {
        target
    } else {
        return ReturnMessage::new("how did you do this?", MessageFlags::EPHEMERAL);
    };

    let target_data = get_userdata_doc(target.id).await;
    if target_data.is_none() {
        return ReturnMessage::new("user not found, the user must have used <@568163802907148307> atleast once", MessageFlags::EPHEMERAL);
    }
    let mut target_data = target_data.unwrap();

    let amount = args.get(1).unwrap().value.as_ref().unwrap().as_i64().unwrap();
    
    if amount < 1 {
        return ReturnMessage::new(translate("invalid-amount"), MessageFlags::EPHEMERAL);
    }

    let money = get_number(&user_data, "money");
    let target_money = get_number(&target_data, "money");

    if amount > money {
        return ReturnMessage::new(&format!("{} `{} ris`", translate("too-poor"), amount - money), MessageFlags::EPHEMERAL);
    }

    user_data.insert("money", money - amount);
    target_data.insert("money", target_money + amount);

    save_userdata_doc(user.id, &user_data).await;
    save_userdata_doc(target.id, &target_data).await;

    return ReturnMessage::new(&format!("you donated `{} ris` to <@{}>", amount, target.id), MessageFlags::default());
}