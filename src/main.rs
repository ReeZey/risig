use std::env;
use std::path::Path;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::GuildId;
use serenity::model::prelude::command::{Command, CommandOptionType};
use serenity::model::prelude::interaction::{MessageFlags, Interaction};
use serenity::model::prelude::interaction::InteractionResponseType::ChannelMessageWithSource;
use serenity::prelude::*;

mod risig;
mod commands;
use commands::{work, ping, top, balance, daily};
use tokio::fs;

use crate::commands::{requestmydata, deposit, withdraw, donate, checkup, gamba, rob, fishing, captcha};
use crate::risig::InteractionButton;

mod utils;
mod translator;
mod structs;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /*
    async fn message(&self, ctx: Context, msg: Message) {

    }
    */
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::MessageComponent(component) = interaction {
            //println!("{:#?}", component);
            //let options = component.data;

            let user = component.user.clone();
            let action = component.data.custom_id.clone();

            let risig_response = risig::handle_message(user.clone(), action.to_string(), None).await;

            let content = if risig_response.message_flags == MessageFlags::EPHEMERAL {
                risig_response.message
            } else {
                format!("<@{}> used {}:\n{}", user.id, action, risig_response.message)
            };

            component.create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|message| {
                    message.content(content);
                    if risig_response.button.is_some() {
                        let interact_button: InteractionButton = risig_response.button.unwrap();

                        message.components(|comp| {
                            comp.create_action_row(|ar| {
                                ar.create_button(|button| {
                                    button.custom_id(interact_button.command).label(interact_button.label)
                                })
                            })
                        });
                    }
                    if risig_response.embed.is_some() {
                        let embed = risig_response.embed.unwrap();

                        message.embed(|e| {
                            e.title(embed.title).fields(embed.fields)
                        });
                    }
                    message.flags(risig_response.message_flags);

                    return message;
                })
            }).await.unwrap();
            return;
        }
        if let Interaction::ApplicationCommand(command) = interaction {
            let guild_id = env::var("GUILD_ID").expect("Expected a token in the environment");
            if command.guild_id.unwrap().to_string() != guild_id {
                return;
            }

            let channel_id = env::var("MESSAGE_CHANNEL").expect("Expected a token in the environment");
            if command.channel_id.to_string() != channel_id {
                command.create_interaction_response(&ctx.http, |response| {
                    response.kind(ChannelMessageWithSource)
                        .interaction_response_data(|message| 
                            message.content(format!("you can only use the bot in <#{}>", channel_id)).flags(MessageFlags::EPHEMERAL)
                        )
                }).await.unwrap();
                return;
            }

            let user = command.user.clone();
            let action = command.data.name.clone();

            let options = if command.data.options.len() > 0 {
                Some(command.data.options.clone())
            } else {
                None
            };

            let risig_response = risig::handle_message(user, action, options).await;
            
            command.create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|message| {
                    message.content(risig_response.message);
                    if risig_response.button.is_some() {
                        let interact_button: InteractionButton = risig_response.button.unwrap();

                        message.components(|comp| {
                            comp.create_action_row(|ar| {
                                ar.create_button(|button| {
                                    button.custom_id(interact_button.command).label(interact_button.label)
                                })
                            })
                        });
                    }
                    if risig_response.embed.is_some() {
                        let embed = risig_response.embed.unwrap();

                        message.embed(|e| {
                            e.title(embed.title).fields(embed.fields)
                        });
                    }
                    message.flags(risig_response.message_flags);

                    return message;
                })
            }).await.unwrap();
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
                    .create_application_command(|command| fishing::fish::register(command))
                    .create_application_command(|command| fishing::show_fish::register(command))
                    .create_application_command(|command| fishing::sell_fish::register(command))
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
                    .create_application_command(|command| captcha::register(command)
                        .create_option(|option| {
                            option.name("captcha").description("verification text").kind(CommandOptionType::String).required(true)
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