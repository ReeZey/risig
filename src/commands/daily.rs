use std::time::{SystemTime, UNIX_EPOCH, Duration};

use bson::Document;
use rand::Rng;
use serenity::model::{user::User, prelude::interaction::MessageFlags};
use crate::{utils::{save_userdata_doc, format_duration, get_number, discord_duration}, risig::ReturnMessage};
use serenity::builder::CreateApplicationCommand;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("daily").description("daily money")
}

pub(crate) async fn run(user: User, mut user_data: Document) -> ReturnMessage {
    let time = SystemTime::now();
    let now = time.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;

    let daily = user_data.get("last_daily");
    if daily.is_some() {
        let last = daily.unwrap().as_i64().unwrap();

        if now < last {
            let duration = Duration::from_millis((last - now) as u64);
            return ReturnMessage::new(&format!("you have already claimed your daily you need to wait {} ({})", format_duration(duration), discord_duration(duration)), MessageFlags::EPHEMERAL);
        }
    }
    
    let money = get_number(&user_data, "money");

    let tomorrow = Duration::from_secs(86400);
    let amount = rand::thread_rng().gen_range(5_000..20_000);

    user_data.insert("money", money + amount);
    user_data.insert("last_daily", now + tomorrow.as_millis() as i64);
    
    save_userdata_doc(user.id, &user_data).await;

    return ReturnMessage::new(&format!("you claimed the daily {} ris, you now have {} ris", amount, money + amount), MessageFlags::default());
}