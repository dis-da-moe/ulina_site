use crate::discord::commands::shared::{Category, CommandData, CreateCommand, Interaction};
use crate::discord::helper::{embed, is_admin, Helper};
use crate::error::Error;
use crate::util::ZERO_WIDTH;
use once_cell::sync::OnceCell;
use serenity::builder::CreateEmbed;
use serenity::client::Context;

use std::collections::HashMap;

static NORMAL_COMMANDS: OnceCell<CreateEmbed> = OnceCell::new();
static ADMINS_COMMANDS: OnceCell<CreateEmbed> = OnceCell::new();

pub fn build_commands_embed(categorised_commands: HashMap<Category, Vec<(&CommandData, String)>>) {
    let mut normal_embed = embed();

    normal_embed
        .title("Moley Commands")
        .description("commands and their descriptions");

    let mut admin_embed = normal_embed.clone();

    for (category, commands) in categorised_commands.iter() {
        for embed in [&mut admin_embed, &mut normal_embed].iter_mut() {
            embed.field(format!("__{}__", category.to_string()), ZERO_WIDTH, false);
        }

        for (data, description) in commands {
            if !data.admin_only {
                normal_embed.field(data.name, description, true);
            }

            admin_embed.field(data.name, description, true);
        }
    }

    NORMAL_COMMANDS.set(normal_embed).unwrap();
    ADMINS_COMMANDS.set(admin_embed).unwrap();
}

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    command.name(DATA.name).description("list all commands")
}

pub const DATA: CommandData = CommandData {
    admin_only: false,
    name: "commands",
    category: Category::Help,
};

pub async fn commands(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let embed = if is_admin(&ctx.http, &interaction.user).await? {
        &ADMINS_COMMANDS
    } else {
        &NORMAL_COMMANDS
    }
    .get()
    .ok_or_else(|| Error::InternalError("can not get embeds from `OnceCell`".to_string()))?
    .clone();

    interaction
        .message(&ctx.http, |message| message.add_embed(embed))
        .await;

    Ok(())
}
