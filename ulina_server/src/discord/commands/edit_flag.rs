use std::{path::Path};

use serenity::{client::Context};

use crate::{
    database::add_flag,
    discord::{helper::Helper, ids::FLAG},
    error::Error,
    get_options,
};

use super::shared::{
    default_data, edit_action, Category, CommandData, CreateCommand, Interaction, OptionType,
};

pub const DATA: CommandData = CommandData {
    admin_only: false,
    name: "edit-flag",
    category: Category::EditNation,
};

pub const ACCEPTED_EXTENSIONS: [&str; 3] = ["jpg", "jpeg", "png"];

pub const MAX_SIZE: u64 = 8_000_000;

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    command.create_option(|option| {
        option
            .kind(OptionType::Attachment)
            .name(FLAG)
            .description("the new flag of the nation")
            .required(true)
    });
    default_data(command, &DATA)
}

pub async fn edit_flag(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let nation = edit_action(interaction, &DATA).await?;
    let flag = get_options!(interaction.data.options, FLAG, Attachment)?;

    let extension = Path::new(&flag.filename)
        .extension()
        .and_then(|e| e.to_str())
        .ok_or(Error::ExpectedImage("unknown".to_string()))?;

    if !ACCEPTED_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
        return Err(Error::ExpectedImage(extension.to_string()));
    }

    if flag.size > MAX_SIZE {
        return Err(Error::TooLarge("8MB".to_string()));
    }

    interaction
        .message(&ctx.http, |message| {
            message.content("updating...").ephemeral(true)
        })
        .await;

    let buffer = flag
        .download()
        .await
        .map_err(|err| Error::InternalError(format!("{:?}", err)))?;

    add_flag(nation.nationId, &nation.name, extension, buffer).await?;

    interaction
        .follow_up(&ctx.http, |message| {
            message.content(format!("successfully added flag for {}", nation.name))
        })
        .await;

    Ok(())
}
