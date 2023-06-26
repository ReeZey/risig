use std::env;
use std::path::Path;

use bson::Document;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::GuildId;
use serenity::model::prelude::command::{Command, CommandOptionType};
use serenity::model::prelude::interaction::{Interaction, MessageFlags};
use serenity::prelude::*;

mod commands;
use commands::{work, ping, top, balance, daily};
use tokio::fs;
use utils::{get_userdata_doc, save_userdata_doc, send_command_response};

use crate::commands::{requestmydata, deposit, withdraw, donate, checkup, gamba, rob};

mod utils;
mod translator;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /*
    async fn message(&self, ctx: Context, msg: Message) {

    }
    */

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(mut command) = interaction {
            //println!("Received command interaction: {:#?}", command);

            let token = env::var("MESSAGE_CHANNEL").expect("Expected a token in the environment");
            if command.channel_id.to_string() != token {
                send_command_response(&mut command, &ctx, &format!("you can only use the bot in <#{}>", token), MessageFlags::EPHEMERAL).await;
                return
            }

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

            //println!("{:#?}", command);

            println!("{}: {}", user.name, command.data.name.as_str());

            match command.data.name.as_str() {
                "ping" => ping::run(&mut command, &ctx).await,
                "work" => work::run(&mut command, &ctx, user, user_data).await,
                "daily" => daily::run(&mut command, &ctx, user, user_data).await,
                "top"  => top::run(&mut command, &ctx).await,
                "balance" => balance::run(&mut command, &ctx, user_data).await,
                "requestmydata" => requestmydata::run(&mut command, &ctx, user, user_data).await,
                "deposit" => deposit::run(&mut command, &ctx, user, user_data).await,
                "withdraw" => withdraw::run(&mut command, &ctx, user, user_data).await,
                "donate" => donate::run(&mut command, &ctx, user, user_data).await,
                "checkup" => checkup::run(&mut command, &ctx).await,
                "rob" => rob::run(&mut command, &ctx, user, user_data).await,
                "gamba" => gamba::run(&mut command, &ctx, user, user_data).await,
                _ => {
                    send_command_response(&mut command, &ctx, "command not found", MessageFlags::default()).await
                },
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| ping::register(command))
                    .create_application_command(|command| requestmydata::register(command))
                    .create_application_command(|command| top::register(command))
                    .create_application_command(|command| work::register(command))
                    .create_application_command(|command| daily::register(command))
                    .create_application_command(|command| balance::register(command))
                    .create_application_command(|command| deposit::register(command)
                        .create_option(|option| {
                            option.name("amount").description("amount of deposit").kind(CommandOptionType::Integer).required(true)
                        })
                    )
                    .create_application_command(|command| withdraw::register(command)
                        .create_option(|option| {
                            option.name("amount").description("amount of withdraw").kind(CommandOptionType::Integer).required(true)
                        })
                    )
                    .create_application_command(|command| donate::register(command)
                        .create_option(|option| {
                            option.name("who").description("user to give money").kind(CommandOptionType::User).required(true)
                        })
                        .create_option(|option| {
                            option.name("amount").description("amount of money").kind(CommandOptionType::Integer).required(true)
                        })
                    )
                    .create_application_command(|command| checkup::register(command)
                        .create_option(|option| {
                            option.name("who").description("user to check money").kind(CommandOptionType::User).required(true)
                        })
                    )
                    .create_application_command(|command| rob::register(command)
                        .create_option(|option| {
                            option.name("who").description("user to check money").kind(CommandOptionType::User).required(true)
                        })
                    )
                    .create_application_command(|command| gamba::register(command)
                        .create_option(|option| {
                            option.name("amount").description("amount to GAMBA").kind(CommandOptionType::Integer).required(true)
                        })
                    )
        }).await.unwrap();

        Command::set_global_application_commands(&ctx.http, |commands| {
            commands
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