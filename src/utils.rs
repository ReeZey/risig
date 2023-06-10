use std::{path::Path, time::Duration};
use bson::Document;
use tokio::{fs::File, io::AsyncWriteExt};
use serenity::{client::Context, model::prelude::{Message, UserId, command::Command}};
use tokio::fs;

#[allow(dead_code)]
pub(crate) async fn send_message(ctx: &Context, msg: &Message, response: &str) {
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| m.content(response)).await {
        println!("Error sending message: {:?}", why);
    }
}

#[allow(dead_code)]
pub(crate) async fn send_file(ctx: &Context, msg: &Message, response: Option<&str>, data: Vec<u8>, filename: &str) {
    let files = vec![(data.as_slice(), filename)];

    let response = if response == None {
        ""
    }else {
        response.unwrap()
    };

    if let Err(why) = msg.channel_id.send_files(&ctx.http, files, |m| m.content(response)).await {
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

pub(crate) fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    if hours > 0 {
        return format!("{}h {}m {}s", hours, minutes, seconds);
    } else if minutes > 0 {
        return format!("{}m {}s", minutes, seconds);
    } else {
        return format!("{}s", seconds);
    }
}

pub struct CommandResponse {
    pub content: String,
    pub hidden: bool
}

impl CommandResponse {
    pub fn new(content: String, hidden: bool) -> CommandResponse {
        return CommandResponse { content, hidden }
    }
}