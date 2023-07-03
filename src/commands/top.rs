use std::{path::Path, cmp::min};

use bson::Document;
use serenity::{builder::CreateApplicationCommand, model::prelude::interaction::{application_command::ApplicationCommandInteraction, MessageFlags}, prelude::Context};
use tokio::fs;

use crate::utils::{send_command_response, get_number};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("top").description("top money money")
}

pub(crate) async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context) {
    let data_path = Path::new("data");

    let mut all_money = vec![];
    for file in data_path.read_dir().unwrap() {
        let file_path = file.unwrap().path();
        let data = fs::read(&file_path).await;
        let user_data: Document = bson::from_slice(&data.unwrap()).unwrap_or_default();
        
        let money = get_number(&user_data, "money");
        let bank_money = get_number(&user_data, "bank_money");

        let username = user_data.get("username").unwrap().as_str().unwrap().to_string();
        all_money.push((username, money, bank_money));
    }

    all_money.sort_by_key(|item| item.1 + item.2);
    all_money.reverse();

    let mut top_string: String = String::default();
    for user in all_money[0..min(all_money.len(), 10)].iter() {
        top_string += &format!("{}: `{} total ris` [cash {} / bank {}]\n", user.0, user.1 + user.2, user.1, user.2);
    }

    send_command_response(command, &ctx, &top_string, MessageFlags::default()).await;
}