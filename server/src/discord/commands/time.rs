use crate::discord::commands::shared::{Category, CommandData, CreateCommand, Interaction};
use crate::discord::helper::{embed, Helper};
use crate::error::Error;
use crate::util::{go_to_site, ZERO_WIDTH};
use chrono::NaiveDate;
use common::{to_ulina, DATE_FORMAT};
use serenity::client::Context;

pub const DATA: CommandData = CommandData {
    admin_only: false,
    name: "time",
    category: Category::Misc,
};

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    command.description("gives current ulina time")
}

pub async fn time(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let now = chrono::Utc::now();
    let ulina_time = to_ulina::<NaiveDate>(now.timestamp()).map_err(|e| Error::TimeError(e))?;
    let mut embed = embed();

    embed
        .title("Ulina Time")
        .field("Real Time", now.format(DATE_FORMAT), false)
        .field("Ulina Time", ulina_time.format(DATE_FORMAT), false)
        .field(
            ZERO_WIDTH,
            go_to_site("convert a custom time", "tools/time"),
            false,
        );

    interaction
        .message(&ctx.http, |data| data.add_embed(embed))
        .await;

    Ok(())
}
