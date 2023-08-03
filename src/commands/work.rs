use std::time::{SystemTime, UNIX_EPOCH, Duration};

use bson::Document;
use serenity::model::{user::User, prelude::interaction::MessageFlags};
use rand::Rng;
use crate::{utils::{save_userdata_doc, get_number, discord_duration, format_duration}, risig::{ReturnMessage, InteractionButton}};
use serenity::builder::CreateApplicationCommand;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("work").description("work work")
}

pub(crate) async fn run(user: User, mut user_data: Document) -> ReturnMessage {
    let time = SystemTime::now();
    let now = time.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;

    let last_work = user_data.get("last_work");
    if last_work.is_some() {
        let last = last_work.unwrap().as_i64().unwrap();

        if now < last {
            let next_time = Duration::from_millis((last - now) as u64);
            return ReturnMessage::new_with_button(
                &format!("you are tired, you need to wait {}, ({})", format_duration(next_time), discord_duration(next_time)), 
                MessageFlags::EPHEMERAL,
                InteractionButton::new("mimimi work mimimi work mimimi work", "work")
            );
        }
    }
    
    let money = get_number(&user_data, "money");

    let work_money = rand::thread_rng().gen_range(500..2500);
    let time_offset = Duration::from_secs(rand::thread_rng().gen_range(60..300));

    user_data.insert("money", money + work_money);
    user_data.insert("last_work", now + time_offset.as_millis() as i64);
    
    save_userdata_doc(user.id, &user_data).await;

    return ReturnMessage::new_with_button(
        &format!("you earned `{} ris` for working, you may work again in {} ({})", work_money, format_duration(time_offset), discord_duration(time_offset)), 
        MessageFlags::default(),
        InteractionButton::new("WORK WORK WORK", "work")
    );
}