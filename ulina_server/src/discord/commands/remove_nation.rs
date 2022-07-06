use crate::database::db;
use crate::discord::commands::shared::{default_data, edit_action};
use crate::discord::commands::shared::{Category, CommandData, CreateCommand, Interaction};
use crate::discord::helper::Helper;
use crate::error::Error;
use serenity::client::Context;
use serenity::http::Http;
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
    let nation = edit_action(interaction, &DATA).await?;

    query!(
        "UPDATE Nation SET removed = true WHERE nationId = ?",
        nation.nationId
    )
    .execute(db())
    .await?;

    interaction
        .message(&ctx.http, |message| {
            message.content(format!("Successfully removed {}", nation.name))
        })
        .await;

    Ok(())
}
