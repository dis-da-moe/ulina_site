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
    util::{go_to_site, ZERO_WIDTH},
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

    default_data(command, &DATA)
}

pub async fn edit_socials(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let nation = edit_action(&ctx.http, interaction, &DATA).await?;
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
            message.embed(|embed| {
                embed
                    .title("Socials")
                    .description(go_to_site(
                        "create/remove socials",
                        &format!("tools/nation/{}", nation.nationId),
                    ))
                    .field("successfully updated socials", ZERO_WIDTH, false)
            })
        })
        .await;

    Ok(())
}
