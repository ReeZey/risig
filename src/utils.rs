use std::{path::Path, time::{Duration, SystemTime, UNIX_EPOCH}, ops::Add};
use bson::Document;
use tokio::{fs::File, io::AsyncWriteExt};
use serenity::{client::Context, model::prelude::{UserId, interaction::{application_command::ApplicationCommandInteraction, MessageFlags}, ChannelId}};
use tokio::fs;
use serenity::model::prelude::interaction::InteractionResponseType::ChannelMessageWithSource;

#[allow(dead_code)]
pub(crate) async fn send_message(ctx: &Context, channel_id: &ChannelId, response: &str) {
    if let Err(why) = channel_id.send_message(&ctx.http, |m| m.content(response)).await {
        println!("Error sending message: {:?}", why);
    }
}

#[allow(dead_code)]
pub(crate) async fn send_file(ctx: &Context, channel_id: &ChannelId, response: Option<&str>, data: Vec<u8>, filename: &str) {
    let files = vec![(data.as_slice(), filename)];

    let response = if response == None {
        ""
    }else {
        response.unwrap()
    };

    if let Err(why) = channel_id.send_files(&ctx.http, files, |m| m.content(response)).await {
        println!("Error sending message: {:?}", why);
    }
}

pub(crate) async fn get_userdata(user: UserId) -> Option<Vec<u8>> {
    let data_path = Path::new("data");
    let user_path = data_path.join(format!("{}.bin", user));
    if !user_path.exists() {
        return None;
    }
    return Some(fs::read(user_path).await.expect("could not read user data"));
}

pub(crate) async fn get_userdata_doc(user: UserId) -> Option<Document> {
    let user_data = get_userdata(user).await;

    if user_data.is_none() {
        return None;
    }

    return Some(bson::from_slice(&user_data.unwrap()).unwrap_or_default());
}

pub(crate) async fn save_userdata(user: UserId, content: Vec<u8>) {
    let data_path = Path::new("data");
    let user_path = data_path.join(format!("{}.bin", user));
    
    let mut file_handle = File::create(user_path).await.expect("could not create userdata");
    file_handle.write_all(&content).await.unwrap();
}

pub(crate) async fn save_userdata_doc(user: UserId, bson_doc: &Document){
    let mut out_buffer: Vec<u8> = vec![];
    bson_doc.to_writer(&mut out_buffer).unwrap();
    save_userdata(user, out_buffer).await;
}

pub(crate) fn get_number(bson_doc: &Document, key: &str) -> i64 {
    match bson_doc.get(key) {
        Some(key) => key.as_i64().unwrap(),
        None => 0
    }
}

pub(crate) fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    let mut time_builder: Vec<String> = vec![];
    if hours > 0 {
        time_builder.push(format!("{}h", hours));
    }

    if minutes > 0 {
        time_builder.push(format!("{}m", minutes));
    }

    if seconds > 0 {
        time_builder.push(format!("{}s", seconds));
    }

    if time_builder.len() == 0 {
        return format!("{}ms", duration.as_millis());
    }

    return time_builder.join(" ");
}

pub(crate) fn discord_duration(duration: Duration) -> String {
    let start = SystemTime::now();
    let relative_time = start.duration_since(UNIX_EPOCH).unwrap().add(duration).as_secs();

    return format!("<t:{}:R>", relative_time);
}

pub(crate) async fn send_command_response(command: &mut ApplicationCommandInteraction, ctx: &Context, content: &str, flags: MessageFlags) {
    command.create_interaction_response(&ctx.http, |response| {
        response.kind(ChannelMessageWithSource)
            .interaction_response_data(|message| 
                message.content(content).flags(flags)
            )
    }).await.unwrap();
}

pub(crate) async fn send_file_command_response(command: &mut ApplicationCommandInteraction, ctx: &Context, content: &str, file: (Vec<u8>, &str), flags: MessageFlags) {
    command.create_interaction_response(&ctx.http, |response| {
        response.kind(ChannelMessageWithSource)
            .interaction_response_data(|message| 
                message.content(content).add_file((file.0.as_slice(), file.1)).flags(flags)
            )
    }).await.unwrap();
}