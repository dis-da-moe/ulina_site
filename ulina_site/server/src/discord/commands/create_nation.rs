use crate::database::{db, nation_change};
use crate::discord::commands::shared::{
    create_continent_option, get_continent, Category, CommandData, CreateCommand, Interaction,
    OptionType,
};
use crate::discord::helper::Helper;
use crate::discord::ids::{CONTINENT, NAME, USER};
use crate::error::Error;
use crate::get_options;
use common::ChangeType;
use common::NationId;
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
        .add_option({
            let mut option = create_continent_option();
            option
                .description("continent of the new nation")
                .required(true);
            option
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

    let id = query!(
        "INSERT INTO Nation (continentName, name, removed, ownerDiscord) VALUES (?, ?, false, ?) RETURNING nationId",
        continent,
        name,
        user
    )
    .fetch_one(db())
    .await?.nationId;

    nation_change(NationId(id), ChangeType::Creation, None, None, true).await?;

    interaction
        .message(&ctx.http, |message| {
            message.content(format!("Successfully created {}", name))
        })
        .await;

    Ok(())
}
