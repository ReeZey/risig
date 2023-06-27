use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bson::Document;
use rand::Rng;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::MessageFlags;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::user::User;
use serenity::prelude::Context;
use strum::IntoEnumIterator;

use crate::structs::fish::{Fish, FishType};
use crate::utils::{send_command_response, format_duration, save_userdata_doc};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("fish").description("fish fishes")
}

pub async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context, user: User, mut user_data: Document) {
    let money = match user_data.get("money") {
        Some(money) => money.as_i64().unwrap(),
        None => 0
    };

    let cost = 500;

    if money < cost {
        send_command_response(command, &ctx, &format!("you need atleast {} to fish", cost), MessageFlags::EPHEMERAL).await;
        return;
    }
    
    let time = SystemTime::now();
    let now = time.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
    
    let last_fish = user_data.get("last_fish");
    if last_fish.is_some() {
        let last = last_fish.unwrap().as_i64().unwrap();

        if now < last {
            let next_time = Duration::from_millis((last - now) as u64);
            send_command_response(command, &ctx, &format!("you are tired, you need to wait {}", format_duration(next_time)), MessageFlags::EPHEMERAL).await;
            return
        }
    }

    user_data.insert("money", money - cost);

    let success = rand::thread_rng().gen_range(0..20);
    if success != 0 {
        let time_offset = Duration::from_secs(rand::thread_rng().gen_range(30..90));
        user_data.insert("last_fish", now + time_offset.as_millis() as i64);
        save_userdata_doc(user.id, &user_data).await;

        send_command_response(command, &ctx, &format!("you lost `{} ris` and missed the fish! hook landed {} meter off! you may fish again in {}", cost, success, format_duration(time_offset)), MessageFlags::default()).await;
        return;
    }

    let options = FishType::iter().collect::<Vec<_>>();
    let choice = rand::thread_rng().gen_range(0..options.len());
    let fish_type: FishType = options.get(choice).unwrap().clone();

    let le_fish = Fish {
        weight: rand::thread_rng().gen_range(1..10),
        length: rand::thread_rng().gen_range(1..25),
        fish_type,
    };

    send_command_response(command, &ctx, &format!("you lost `{} ris` BUT you got an {} worth {}! now you can chill for 30 minutes", cost, le_fish.fish_type.to_string(), le_fish.length as i64 * le_fish.weight as i64 * 2000), MessageFlags::default()).await;

    let mut fish_array = vec![];
    match user_data.get("fishes") {
        Some(fish) => {
            for bson in fish.as_array().unwrap().to_vec() {
                fish_array.push(bson::from_bson::<Fish>(bson).unwrap());
            }
        }
        None => {}
    };

    fish_array.push(le_fish);

    let thirty_minutes = Duration::from_secs(1800);
    user_data.insert("last_fish", now + thirty_minutes.as_millis() as i64);
    user_data.insert("fishes", bson::to_bson(&fish_array).unwrap());
    save_userdata_doc(user.id, &user_data).await;
}


