use std::time::{SystemTime, UNIX_EPOCH, Duration};

use bson::Document;
use serenity::{model::{user::User, prelude::interaction::{MessageFlags, application_command::ApplicationCommandInteraction}}, prelude::Context};
use rand::Rng;
use crate::utils::{save_userdata_doc, format_duration, send_command_response, get_number};
use serenity::builder::CreateApplicationCommand;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("work").description("work work")
}

pub(crate) async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context, user: User, mut user_data: Document) {
    let time = SystemTime::now();
    let now = time.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;

    let last_work = user_data.get("last_work");
    if last_work.is_some() {
        let last = last_work.unwrap().as_i64().unwrap();

        if now < last {
            let next_time = Duration::from_millis((last - now) as u64);
            send_command_response(command, &ctx, &format!("you are tired, you need to wait {}", format_duration(next_time)), MessageFlags::EPHEMERAL).await;
            return
        }
    }
    
    let money = get_number(&user_data, "money");

    let work_money = rand::thread_rng().gen_range(500..2500);
    let time_offset = Duration::from_secs(rand::thread_rng().gen_range(60..300));

    user_data.insert("money", money + work_money);
    user_data.insert("last_work", now + time_offset.as_millis() as i64);
    
    save_userdata_doc(user.id, &user_data).await;

    send_command_response(command, &ctx, &format!("you earned `{} ris` for working, you may work again in {}", work_money, &format_duration(time_offset)), MessageFlags::default()).await;
}