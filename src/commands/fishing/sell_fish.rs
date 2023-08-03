use bson::Document;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::MessageFlags;
use serenity::model::user::User;


use crate::risig::ReturnMessage;
use crate::structs::fish::Fish;
use crate::utils::{save_userdata_doc, get_number};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("sellfish").description("sell all fishes")
}

pub async fn run(user: User, mut user_data: Document) -> ReturnMessage {
    let mut fishing: Document = match user_data.get("fishing") {
        Some(doc) => doc.as_document().unwrap().to_owned(),
        None => Document::default()
    };
    
    let mut fish_array = vec![];
    match fishing.get("fishes") {
        Some(fish) => {
            for bson in fish.as_array().unwrap().to_vec() {
                fish_array.push(bson::from_bson::<Fish>(bson).unwrap());
            }
        }
        None => {}
    };

    if fish_array.len() == 0 {
        return ReturnMessage::new("yo, you have no fishes", MessageFlags::EPHEMERAL);
    }

    let mut total_money = 0;
    for fish in fish_array {
        total_money += fish.length as i64 * fish.weight as i64 * 1000;
    }

    let money = get_number(&user_data, "money");

    user_data.insert("money", money + total_money);
    fishing.remove("fishes");
    user_data.insert("fishing", fishing);
    save_userdata_doc(user.id, &user_data).await;
    
    return ReturnMessage::new(&format!("you selled all you fish for `{} ris`", total_money), MessageFlags::default());
}


