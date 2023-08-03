use bson::Document;
use serenity::model::{user::User, prelude::interaction::{MessageFlags, application_command::CommandDataOption}};

use crate::{utils::{get_userdata_doc, save_userdata_doc}, commands::{ping, work, daily, top, balance, fishing, requestmydata, deposit, withdraw, donate, checkup, rob, gamba}};

pub async fn handle_message(user: User, action: String, args: Option<Vec<CommandDataOption>>) -> ReturnMessage {
    let mut user_data = get_userdata_doc(user.id).await;

    if user_data.is_none() {
        let mut new_user = Document::default();
        new_user.insert("username", &user.name);
        new_user.insert("userid", i64::from(user.id));
        save_userdata_doc(user.id, &new_user).await;

        user_data = Some(new_user);
    }

    let mut user_data = user_data.unwrap();

    if user_data.get("username").unwrap().as_str().unwrap() != user.name {
        user_data.insert("username", &user.name);
        save_userdata_doc(user.id, &user_data).await;
    }

    println!("{}: {} ({:?})", user.name, action, args);

    return match action.as_str() {
        "ping" => ping::run(),
        "work" => work::run(user, user_data).await,
        "daily" => daily::run(user, user_data).await,
        "top"  => top::run().await,
        "balance" => balance::run(user_data).await,
        "deposit" => deposit::run(user, user_data, args.unwrap()).await,
        "fish" => fishing::fish::run(user, user_data).await,
        "showfish" => fishing::show_fish::run(user_data).await,
        "sellfish" => fishing::sell_fish::run(user, user_data).await,
        "requestmydata" => requestmydata::run(user_data),
        "withdraw" => withdraw::run(user, user_data, args.unwrap()).await,
        "donate" => donate::run(user, user_data, args.unwrap()).await,
        "checkup" => checkup::run(args.unwrap()).await,
        "rob" => rob::run(user, user_data, args.unwrap()).await,
        "gamba" => gamba::run(user, user_data, args.unwrap()).await,
        _ => {
            return ReturnMessage::new("command not found", MessageFlags::default());
        }
    };
}

pub struct InteractionButton {
    pub label: String,
    pub command: String,
}

impl InteractionButton {
    pub fn new(label: &str, command: &str) -> Self {
        Self { label: label.to_owned(), command: command.to_owned()  }
    }
}

pub struct Embed {
    pub title: String,
    pub fields: Vec<(String, String, bool)>
}

pub struct ReturnMessage {
    pub message: String,
    pub message_flags: MessageFlags,
    pub button: Option<InteractionButton>,
    pub embed: Option<Embed>
}

impl ReturnMessage {
    pub fn new(message: &str, message_flags: MessageFlags) -> Self {
        Self { message: message.to_owned(), message_flags, button: None, embed: None }
    }

    pub fn new_with_button(message: &str, message_flags: MessageFlags, button: InteractionButton) -> Self {
        Self { message: message.to_owned(), message_flags, button: Some(button), embed: None }
    }

    pub fn embed(message: &str, message_flags: MessageFlags, title: &str, fields: Vec<(String, String, bool)>) -> Self {
        Self { message: message.to_owned(), message_flags, button: None, embed: Some(Embed { title: title.to_owned(), fields }) }
    }
}