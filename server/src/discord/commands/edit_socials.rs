use serenity::client::Context;
use sqlx::query;

use crate::{
    database::{db, socials},
    discord::{
        helper::Helper,
        ids::{LINK, PLATFORM},
    },
    error::Error,
    get_options,
    util::go_to_site,
};

use super::shared::{
    default_data, edit_action, Category, CommandData, CreateCommand, Interaction, OptionType,
};

pub const DATA: CommandData = CommandData {
    admin_only: false,
    name: "edit-socials",
    category: Category::EditNation,
};

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    command
        .create_option(|option| {
            option
                .kind(OptionType::String)
                .name(PLATFORM)
                .description("the platform to edit")
                .required(true)
        })
        .create_option(|option| {
            option
                .kind(OptionType::String)
                .name(LINK)
                .description("the new link")
                .required(true)
        });

    default_data(command, &DATA).description(format!(
        "update the links of your socials - {}",
        go_to_site("create/remove socials", "???")
    ))
}

pub async fn edit_socials(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let nation = edit_action(interaction, &DATA).await?;
    let socials = socials(nation.nationId).await?;

    let (platform, link) = get_options!(interaction.data.options, PLATFORM, String, LINK, String);
    let (platform, link) = (platform?, link?);

    let id = socials
        .iter()
        .find(|social| social.platform.as_str() == platform)
        .ok_or(Error::SocialNotFound(platform.clone()))?
        .socialsId;

    query!("UPDATE Social SET link = ? WHERE socialsId = ?", link, id)
        .execute(db())
        .await?;

    interaction
        .message(&ctx.http, |message| {
            message.content("successfully updated socials")
        })
        .await;

    Ok(())
}
