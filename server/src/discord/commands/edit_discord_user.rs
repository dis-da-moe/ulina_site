use crate::database::{db, nation_change};
use crate::discord::commands::shared::{Category, CommandData, CreateCommand, Interaction};
use crate::discord::helper::Helper;
use crate::discord::ids::USER;
use crate::error::Error;
use crate::get_options;
use common::ChangeType;
use common::Id;
use serenity::client::Context;
use sqlx::query;

use super::shared::{default_data, edit_action, OptionType};

pub const DATA: CommandData = CommandData {
    admin_only: true,
    name: "edit-discord-user",
    category: Category::EditNation,
};

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    default_data(command, &DATA).create_option(|option| {
        option
            .kind(OptionType::User)
            .name(USER)
            .description("the user that owns this nation")
            .required(true)
    })
}

pub async fn edit_discord_user(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let nation = edit_action(&ctx.http, interaction, &DATA).await?;
    let user = get_options!(interaction.data.options, USER, User)?
        .id
        .0
        .to_string();

    query!(
        "UPDATE Nation SET ownerDiscord = ? WHERE nationId = ?",
        user,
        nation.nationId
    )
    .execute(db())
    .await?;

    nation_change(
        nation.id(),
        ChangeType::OwnerDiscord,
        Some(nation.ownerDiscord),
        Some(user),
        true,
    )
    .await?;

    interaction
        .message(&ctx.http, |data| data.content("successfully updated owner"))
        .await;

    Ok(())
}
