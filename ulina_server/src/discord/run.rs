use std::env;

use crate::discord::helper::Helper;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::result::Result;

use crate::discord::commands::create_commands;
use crate::discord::commands::COMMANDS;

const DISCORD_TOKEN: &str = "DISCORD_TOKEN";
const GUILD_ID: &str = "GUILD_ID";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("logged in as {}", ready.user.name);

        let guild_id = GuildId(
            env::var(GUILD_ID)
                .expect("no guild id")
                .parse()
                .expect("guild id must be number"),
        );

        create_commands(&guild_id, &ctx.http).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("command called: {}", command.data.name);

            if let Some(ulina_command) = COMMANDS.get(command.data.name.as_str()) {
                if let Err(err) = (ulina_command.action)(&ctx.http, &command).await {
                    command
                        .message(&ctx.http, |response| {
                            response.content(err.to_string()).ephemeral(true)
                        })
                        .await;

                    match err {
                        crate::error::Error::InternalError(_) => println!("{:?}", err),
                        _ => {}
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
    }
}

pub async fn run() -> Result<(), String> {
    lazy_static::initialize(&COMMANDS);

    // Login with a bot token from the environment
    let token = env::var(DISCORD_TOKEN).expect("token");
    let intents =
        GatewayIntents::non_privileged() | GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    client
        .start()
        .await
        .map_err(|err| format!("Error creating client: {}", err))
}
