use bson::Document;
use serenity::builder::CreateApplicationCommand;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("balance").description("shows balance")
}

pub(crate) async fn run(user_data: Document) -> String {
    let money: i64 = if let Some(money) = user_data.get("money") {
        money.as_i64().unwrap()
    } else {
        return "you down have any money, HAHAHA POOOR <a:xqcL:920758314026008576>".to_owned();
    };

    let bank_money: i64 = if let Some(bank_money) = user_data.get("bank_money") {
        bank_money.as_i64().unwrap()
    } else {
        0
    };

    return format!("you have {} dollares\nand {} in the bank", money, bank_money);
}