use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bson::Document;
use rand::Rng;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::MessageFlags;
use serenity::model::user::User;
use strum::IntoEnumIterator;
use rand_distr::{Exp, Distribution};

use crate::risig::{ReturnMessage, InteractionButton};
use crate::structs::fish::{Fish, FishType};
use crate::utils::{format_duration, save_userdata_doc, get_number, discord_duration};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("fish").description("fish fishes")
}

pub async fn run(user: User, mut user_data: Document) -> ReturnMessage {
    let money = get_number(&user_data, "money");

    let cost = 500;

    if money < cost {
        return ReturnMessage::new(&format!("you need atleast {} to fish", cost), MessageFlags::EPHEMERAL);
    }
    
    let time = SystemTime::now();
    let now = time.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;

    let mut fishing: Document = match user_data.get("fishing") {
        Some(doc) => doc.as_document().unwrap().to_owned(),
        None => Document::default()
    };

    let last_fish = fishing.get("cooldown");
    if last_fish.is_some() {
        let last = last_fish.unwrap().as_i64().unwrap();

        if now < last {
            let next_time = Duration::from_millis((last - now) as u64);

            return ReturnMessage::new_with_button(
                &format!("you are tired, you need to wait {} ({})", format_duration(next_time), discord_duration(next_time)), 
                MessageFlags::EPHEMERAL,
                InteractionButton::new("Chill dude, Fish again?", "fish")
            );
        }
    }

    user_data.insert("money", money - cost);

    let mut last_fish = get_number(&fishing, "last_fish");
    
    let success = rand::thread_rng().gen_range(0..20);
    if success != 0 {
        let time_offset = Duration::from_secs(rand::thread_rng().gen_range(30..90));
        fishing.insert("cooldown", now + time_offset.as_millis() as i64);
        last_fish += 1;
        fishing.insert("last_fish", last_fish);
        user_data.insert("fishing", fishing);
        save_userdata_doc(user.id, &user_data).await;

        return ReturnMessage::new_with_button(
            &format!("you lost `{} ris` this is the {}x you missed the fish! hook landed {} meter off! you may fish again in {} ({})", cost, last_fish, success, format_duration(time_offset), discord_duration(time_offset)), 
            MessageFlags::default(),
            InteractionButton::new("Fish again?", "fish")
        );
    }

    let exp = Exp::new(1.0/5.0).unwrap();
    let weight = 1 + exp.sample(&mut rand::thread_rng()) as u8;

    let exp = Exp::new(1.0/8.0).unwrap();
    let length = 1 + exp.sample(&mut rand::thread_rng()) as u8;

    let options = FishType::iter().collect::<Vec<_>>();
    let choice = rand::thread_rng().gen_range(0..options.len());
    let fish_type: FishType = options.get(choice).unwrap().clone();

    let le_fish = Fish {
        weight,
        length,
        fish_type,
    };

    let cloned_fish = le_fish.clone();

    let wait_time = Duration::from_secs((le_fish.length as u64 + le_fish.weight as u64) * 3 * 60);
    
    let mut fish_array = vec![];
    match fishing.get("fishes") {
        Some(fish) => {
            for bson in fish.as_array().unwrap().to_vec() {
                fish_array.push(bson::from_bson::<Fish>(bson).unwrap());
            }
        }
        None => {}
    };

    fishing.remove("last_fish");
    fishing.insert("cooldown", now + wait_time.as_millis() as i64);
    fish_array.push(le_fish);
    fishing.insert("fishes", bson::to_bson(&fish_array).unwrap());
    user_data.insert("fishing", fishing);
    save_userdata_doc(user.id, &user_data).await;

    return ReturnMessage::new(&format!("YOU GOT FISH, an {} worth `{} ris`! now you can chill for {} ({})", cloned_fish.fish_type.to_string(), cloned_fish.length as i64 * cloned_fish.weight as i64 * 1000, format_duration(wait_time), discord_duration(wait_time)), MessageFlags::default());
}


