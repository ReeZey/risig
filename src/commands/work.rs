use std::time::{SystemTime, UNIX_EPOCH, Duration};

use bson::Document;
use serenity::model::user::User;
use rand::Rng;
use crate::utils::{save_userdata_doc, format_duration, CommandResponse};
use serenity::builder::CreateApplicationCommand;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("work").description("work work")
}

pub(crate) async fn run(user: User, mut user_data: Document) -> CommandResponse {
    let time = SystemTime::now();
    let now = time.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;

    let last_work = user_data.get("last_work");
    if last_work.is_some() {
        let last = last_work.unwrap().as_i64().unwrap();

        if now < last {
            let next_time = Duration::from_millis((last - now) as u64);
            return CommandResponse::new(format!("you are tired, you need to wait {}", format_duration(next_time)), true);
        }
    }
    
    let money: i64 = if let Some(money) = user_data.get("money") {
        money.as_i64().unwrap()
    } else {
        0
    };

    let work_money = rand::thread_rng().gen_range(200..1000);
    let five_minutes = Duration::from_secs(300).as_millis() as i64;

    user_data.insert("money", money + work_money);
    user_data.insert("last_work", now + five_minutes);
    
    save_userdata_doc(user.id, &user_data).await;

    return CommandResponse::new(format!("you earned `{} ris` for working, now you have `{} ris`", work_money, money + work_money), false);
}