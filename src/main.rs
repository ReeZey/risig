use std::env;
use std::path::Path;

use bson::Document;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::command::{Command, CommandOptionType};
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;

mod commands;
use commands::{work, ping, top, balance};
use tokio::fs;
use utils::{get_userdata_doc, save_userdata_doc};

use crate::commands::{requestmydata, deposit, withdraw};

mod utils;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /*
    async fn message(&self, ctx: Context, msg: Message) {

    }
    */

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            //println!("Received command interaction: {:#?}", command);

            let user = command.member.clone().unwrap().user;
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

            let content = match command.data.name.as_str() {
                "ping" => ping::run().await,
                "work" => work::run(user, user_data).await,
                "top"  => top::run().await,
                "balance" => balance::run(user_data).await,
                "requestmydata" => requestmydata::run(user_data).await,
                "deposit" => deposit::run(user, user_data, &command.data.options).await,
                "withdraw" => withdraw::run(user, user_data, &command.data.options).await,
                _ => "mitä 🇫🇮".to_string(),
            };

            if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
                    response.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
            }).await {
                println!("Cannot respond to slash command: {}", why);
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| ping::register(command))
                    .create_application_command(|command| requestmydata::register(command))
                    .create_application_command(|command| top::register(command))
                    .create_application_command(|command| work::register(command))
                    .create_application_command(|command| balance::register(command))
                    .create_application_command(|command| deposit::register(command)
                        .create_option(|option| {
                            option.name("amount").description("amount").kind(CommandOptionType::Integer).required(true)
                        })
                    )
                    .create_application_command(|command| withdraw::register(command)
                        .create_option(|option| {
                            option.name("amount").description("amount").kind(CommandOptionType::Integer).required(true)
                        })
                    )
        }).await.unwrap();

        //println!("I now have the following guild slash commands: {:#?}", commands);
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Could not find .env file, did you forget to create one? err");

    let data_path = Path::new("data");
    if !data_path.exists() {
        fs::create_dir(data_path).await.expect("could not create data folder");
    }

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");
    client.start().await.expect("discord client could not start");
}