use bson::Document;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::MessageFlags;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::user::User;
use serenity::prelude::Context;

use crate::structs::fish::Fish;
use crate::send_command_response;
use crate::utils::{save_userdata_doc, get_number};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("sellfish").description("sell all fishes")
}

pub async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context, user: User, mut user_data: Document) {
    let mut fish_array = vec![];
    match user_data.get("fishes") {
        Some(fish) => {
            for bson in fish.as_array().unwrap().to_vec() {
                fish_array.push(bson::from_bson::<Fish>(bson).unwrap());
            }
        }
        None => {}
    };

    if fish_array.len() == 0 {
        send_command_response(command, ctx, "yo, you have no fishes", MessageFlags::EPHEMERAL).await;
        return;
    }

    let mut total_money = 0;
    for fish in fish_array {
        total_money += fish.length as i64 * fish.weight as i64 * 1000;
    }

    let money = get_number(&user_data, "money");

    user_data.insert("money", money + total_money);
    user_data.remove("fishes");
    save_userdata_doc(user.id, &user_data).await;
    
    send_command_response(command, ctx, &format!("you selled all you fish for `{} ris`", total_money), MessageFlags::default()).await;
}


