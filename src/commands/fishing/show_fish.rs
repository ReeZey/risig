use bson::Document;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use crate::structs::fish::Fish;
use serenity::model::prelude::interaction::InteractionResponseType::ChannelMessageWithSource;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("showfish").description("show fishes")
}

pub async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context, user_data: Document) {
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

    let mut embed_fields: Vec<(String, String, bool)> = vec![];

    for fish in fish_array {
        embed_fields.push((fish.fish_type.to_string(), format!("{} weight {} length", fish.weight, fish.length), false));
    }
    
    command.create_interaction_response(&ctx.http, |response| {
        response.kind(ChannelMessageWithSource)
            .interaction_response_data(|message| 
                message.embed(|e| {
                    e.title(&format!("{}'s fishes", user_data.get("username").unwrap().as_str().unwrap()))
                        .fields(embed_fields)
                })
            )
    }).await.unwrap();
}


