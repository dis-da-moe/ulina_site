use rocket::tokio::fs;
use serenity::client::Context;

use crate::{
    database::{add_flag, validate_flag},
    discord::{
        helper::{is_admin, Helper},
        ids::FLAG,
    },
    error::Error,
    get_options,
};
use common::Id;

use super::shared::{
    default_data, edit_action, Category, CommandData, CreateCommand, Interaction, OptionType,
};

pub const DATA: CommandData = CommandData {
    admin_only: false,
    name: "edit-flag",
    category: Category::EditNation,
};

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
    let nation = edit_action(&ctx.http, interaction, &DATA).await?;
    let flag = get_options!(interaction.data.options, FLAG, Attachment)?;

    let extension = validate_flag(&flag.filename, flag.size)?;

    interaction
        .message(&ctx.http, |message| {
            message.content("updating...").ephemeral(true)
        })
        .await;

    let buffer = flag
        .download()
        .await
        .map_err(|err| Error::InternalError(format!("{:?}", err)))?;

    let writer = |path| async move { fs::write(path, buffer).await };

    add_flag(
        nation.id(),
        &nation.name,
        extension,
        writer,
        is_admin(&ctx.http,&interaction.user).await?,
    )
    .await?;

    interaction
        .follow_up(&ctx.http, |message| {
            message.content(format!("successfully added flag for {}", nation.name))
        })
        .await;

    Ok(())
}
