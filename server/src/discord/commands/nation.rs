use crate::discord::commands::shared::{
    name_option, Category, CommandData, CreateCommand, Interaction,
};
use crate::discord::helper::Helper;
use crate::error::Error;
use crate::{database, get_nation};
use common::FlagId;
use common::Nation;
use serenity::client::Context;

pub const DATA: CommandData = CommandData {
    admin_only: false,
    name: "nation",
    category: Category::ViewNation,
};

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    command
        .description("view your nation or a nation specified by name")
        .create_option(|option| {
            name_option(option)
                .description("the name of the ulina nation")
                .required(false)
        })
}

pub async fn nation(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let nation = get_nation!(interaction, Nation, "*")?;

    let socials = database::socials(nation.nationId).await?;

    let flag_link = match nation.currentFlagId {
        Some(flag) => Some(database::flag_link(FlagId(flag)).await?),
        _ => None,
    };
    //TODO: add trivia fields
    interaction
        .message(&ctx.http, |message| {
            message.embed(|embed| {
                embed.title(nation.name);

                if let Some(description) = nation.description {
                    embed.description(description);
                }

                embed.field("Continent", nation.continentName, false);

                for social in socials {
                    embed.field(social.platform, social.link, false);
                }

                embed.field("Owner", format!("<@{}>", nation.ownerDiscord), false);

                if let Some(link) = flag_link {
                    embed.image(link);
                }

                embed
            })
        })
        .await;

    Ok(())
}
