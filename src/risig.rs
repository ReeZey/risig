use bson::Document;
use rand_distr::Alphanumeric;
use serenity::model::{user::User, prelude::interaction::{MessageFlags, application_command::CommandDataOption}};
use rand::Rng;

use crate::{utils::{get_userdata_doc, save_userdata_doc}, commands::{ping, work, daily, top, balance, fishing, requestmydata, deposit, withdraw, donate, checkup, rob, gamba, captcha}};

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

    if action != "captcha" {
        if let Some(captcha) = user_data.get("captcha") {
            let captcha = captcha.as_document().unwrap();
            let phrase = captcha.get("phrase").unwrap().as_str().unwrap();
            return ReturnMessage::new(&format!("⚠⚠⚠ CAPTCHA TIME, <@{}> HAVE BEEN DEEMED TO BE A ROBOT!!!! ⚠⚠⚠\nYOU MAY NOT CONTINUE BEFORE YOU TYPE `/captcha {}` THIS WILL RESOLVE THE ISSUE", user.id, phrase), MessageFlags::default());
        }
    }

    println!("{}: {} ({:?})", user.name, action, args);

    let mut user_data_hidden = user_data.clone();
    let user_hidden = user.clone();

    let response = match action.as_str() {
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
        "captcha" => captcha::run(user, user_data, args.unwrap()).await,
        _ => {
            return ReturnMessage::new("command not found", MessageFlags::default());
        }
    };

    if response.message_flags == MessageFlags::EPHEMERAL {
        let mut pending_captcha = match user_data_hidden.get("pending-captcha") {
            Some(bson) => bson.as_i64().unwrap(),
            None => 0
        };

        pending_captcha += 1;

        if pending_captcha > 5 {
            let mut captcha = Document::default();
            let phrase: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect();
            captcha.insert("phrase", phrase);
            user_data_hidden.insert("captcha", captcha);
        }

        user_data_hidden.insert("pending-captcha", pending_captcha);
        save_userdata_doc(user_hidden.id, &user_data_hidden).await;
    }

    return response;
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