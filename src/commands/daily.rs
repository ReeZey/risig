use std::time::{SystemTime, UNIX_EPOCH, Duration};

use bson::Document;
use rand::Rng;
use serenity::{model::{user::User, prelude::interaction::{application_command::ApplicationCommandInteraction, MessageFlags}}, prelude::Context};
use crate::utils::{save_userdata_doc, format_duration, send_command_response};
use serenity::builder::CreateApplicationCommand;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("daily").description("daily money")
}

pub(crate) async fn run(command: &mut ApplicationCommandInteraction, ctx: &Context, user: User, mut user_data: Document) {
    let time = SystemTime::now();
    let now = time.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;

    let daily = user_data.get("last_daily");
    if daily.is_some() {
        let last = daily.unwrap().as_i64().unwrap();

        if now < last {
            let duration = Duration::from_millis((last - now) as u64);
            send_command_response(command, &ctx, &format!("you have already claimed your daily you need to wait {}", format_duration(duration)), MessageFlags::EPHEMERAL).await;
            return
        }
    }
    
    let money: i64 = if let Some(money) = user_data.get("money") {
        money.as_i64().unwrap()
    } else {
        0
    };

    let tomorrow = Duration::from_secs(86400);
    let amount = rand::thread_rng().gen_range(5_000..20_000);

    user_data.insert("money", money + amount);
    user_data.insert("last_daily", now + tomorrow.as_millis() as i64);
    
    save_userdata_doc(user.id, &user_data).await;

    send_command_response(command, &ctx, &format!("you claimed the daily {}, you now have {}", amount, money + amount), MessageFlags::default()).await;
}