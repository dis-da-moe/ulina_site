use crate::discord::commands::shared::{Category, CommandData, CreateCommand, Interaction};
use crate::discord::helper::Helper;
use crate::error::Error;
use serenity::client::Context;

pub const DATA: CommandData = CommandData {
    admin_only: false,
    name: "ping",
    category: Category::Misc,
};

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    command.description("replies with pong")
}

pub async fn ping(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    interaction
        .message(&ctx.http, |data| data.content("pong!"))
        .await;

    Ok(())
}
