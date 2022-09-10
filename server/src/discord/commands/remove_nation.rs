use crate::database::{db, nation_change};
use crate::discord::commands::shared::{default_data, edit_action};
use crate::discord::commands::shared::{Category, CommandData, CreateCommand, Interaction};
use crate::discord::helper::{is_admin, Helper};
use crate::error::Error;
use common::ChangeType;
use common::Id;
use serenity::client::Context;

use sqlx::query;

pub const DATA: CommandData = CommandData {
    admin_only: true,
    name: "edit-remove-nation",
    category: Category::EditNation,
};

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    default_data(command, &DATA).description("admin only - remove a nation")
}

pub async fn remove_nation(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let nation = edit_action(&ctx.http, interaction, &DATA).await?;

    query!(
        "UPDATE Nation SET removed = true WHERE nationId = ? AND removed = false",
        nation.nationId
    )
    .execute(db())
    .await?;

    nation_change(
        nation.id(),
        ChangeType::Removed,
        Some("false".into()),
        Some("true".into()),
        is_admin(&ctx.http,&interaction.user).await?,
    )
    .await?;

    interaction
        .message(&ctx.http, |message| {
            message.content(format!("Successfully removed {}", nation.name))
        })
        .await;

    Ok(())
}
