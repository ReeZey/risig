use std::time::{SystemTime, UNIX_EPOCH};

use bson::Document;
use serenity::model::user::User;
use rand::Rng;
use crate::utils::save_userdata_doc;
use serenity::builder::CreateApplicationCommand;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("work").description("work work")
}

pub(crate) async fn run(user: User, mut user_data: Document) -> String {
    let time = SystemTime::now();
    let now = time.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;

    let last_work = user_data.get("last_work");
    if last_work.is_some() {
        let last = last_work.unwrap().as_i64().unwrap();

        if now < last {
            return format!("no work, next time work: {:.3}s", (last - now) as f32 / 1000.0);
        }
    }
    
    let money: i64 = if let Some(money) = user_data.get("money") {
        money.as_i64().unwrap()
    } else {
        0
    };

    let work_money = rand::thread_rng().gen_range(100..500);

    user_data.insert("money", money + work_money);
    user_data.insert("last_work", now + 10_000);
    
    save_userdata_doc(user.id, &user_data).await;

    return format!("you earned `{} ris` for working, now you have `{} ris`", work_money, money + work_money);
}