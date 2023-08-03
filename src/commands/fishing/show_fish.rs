use bson::Document;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::MessageFlags;

use crate::risig::ReturnMessage;
use crate::structs::fish::Fish;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("showfish").description("show fishes")
}

pub async fn run(user_data: Document) -> ReturnMessage {
    let fishing: Document = match user_data.get("fishing") {
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
        return ReturnMessage::new("you have no fishes bitch", MessageFlags::default());
    }

    let mut embed_fields: Vec<(String, String, bool)> = vec![];

    for fish in fish_array {
        embed_fields.push((fish.fish_type.to_string(), format!("{} kg {} cm - price: {}", fish.weight, fish.length, fish.length as i64 * fish.weight as i64 * 1000), false));
    }

    return ReturnMessage::embed("", MessageFlags::default(), &format!("{}'s fishes", user_data.get("username").unwrap().as_str().unwrap()), embed_fields)
}


