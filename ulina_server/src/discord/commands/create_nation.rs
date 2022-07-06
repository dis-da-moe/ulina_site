use crate::database::db;
use crate::discord::commands::shared::{
    continent_option, get_continent, Category, CommandData, CreateCommand, Interaction, OptionType,
};
use crate::discord::helper::Helper;
use crate::discord::ids::{CONTINENT, NAME, USER};
use crate::error::Error;
use crate::get_options;
use serenity::client::Context;

use sqlx::query;

pub const DATA: CommandData = CommandData {
    admin_only: true,
    name: "create-nation",
    category: Category::EditNation,
};

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    command
        .name(DATA.name)
        .description("admin only - create a nation")
        .create_option(|option| {
            option
                .name(NAME)
                .description("name of the new nation")
                .kind(OptionType::String)
                .required(true)
        })
        .create_option(|option| {
            continent_option(option)
                .description("continent of the new nation")
                .required(true)
        })
        .create_option(|option| {
            option
                .name(USER)
                .description("user of the new nation")
                .kind(OptionType::User)
                .required(true)
        })
}

pub async fn create_nation(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let (name, continent, user) = get_options!(
        interaction.data.options,
        NAME,
        String,
        CONTINENT,
        Integer,
        USER,
        User
    );
    let (name, continent, user) = (name?, get_continent(*continent?)?, user?);
    let user = user.id.0.to_string();

    query!(
        "INSERT INTO Nation (continentName, name, removed, ownerDiscord) VALUES (?, ?, false, ?)",
        continent,
        name,
        user
    )
    .execute(db())
    .await?;

    interaction
        .message(&ctx.http, |message| {
            message.content(format!("Successfully created {}", name))
        })
        .await;

    Ok(())
}
