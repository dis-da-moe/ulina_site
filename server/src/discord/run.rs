use crate::config::CONFIG;
use crate::discord::helper::Helper;
use serenity::async_trait;
use serenity::http::error::DiscordJsonError;
use serenity::http::error::ErrorResponse;
use serenity::http::Http;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::Error;

use crate::discord::commands::create_commands;
use crate::discord::commands::COMMANDS;

use super::modal::handle_modal;

struct Handler;

async fn handle_err_reply(
    original: &Error,
    reply_err: serenity::Error,
    http: &Http,
    command: &ApplicationCommandInteraction,
) {
    let debug = format!("{:?}", reply_err);

    let mut handled = false;
    if let serenity::Error::Http(http_err) = reply_err {
        if let HttpError::UnsuccessfulRequest(ErrorResponse {
            error: DiscordJsonError { code: 40060, .. },
            ..
        }) = *http_err
        {
            command
                .follow_up(http, |message| {
                    message.content(original.clone()).ephemeral(true)
                })
                .await;
            handled = true;
        }
    }

    if !handled {
        println!("{}", debug);
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("logged in as {}", ready.user.name);

        let guild_id = GuildId(CONFIG.guild_id);

        create_commands(&guild_id, &ctx.http).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                println!("command called: {}", command.data.name);

                if let Some(ulina_command) = COMMANDS.get(command.data.name.as_str()) {
                    if let Err(command_err) = (ulina_command.action)(&ctx, &command).await {
                        if let Err(reply_err) = command
                            .message_unhandled(&ctx.http, |message| {
                                message.content(command_err.to_string()).ephemeral(true)
                            })
                            .await
                        {
                            handle_err_reply(&command_err, reply_err, &ctx.http, &command).await;
                        }
                    };
                } else {
                    command
                        .message(&ctx.http, |response| {
                            response.content("could not find command").ephemeral(true)
                        })
                        .await;
                }
            }
            Interaction::ModalSubmit(modal) => {
                handle_modal(&ctx.http, modal).await;
            }
            _ => {}
        }
    }
}

pub async fn run() {
    lazy_static::initialize(&COMMANDS);

    // Login with a bot token from the environment
    let intents =
        GatewayIntents::non_privileged() | GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES;
    let mut client = Client::builder(&CONFIG.discord_token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    client.start().await.expect("Error starting client");
}
