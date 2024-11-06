use crate::discord::commands::shared::{Category, CommandData, CreateCommand, Interaction};
use crate::discord::helper::Helper;
use crate::discord::ids::PASTA;
use crate::error::Error;
use crate::{get_options, index_option};
use rocket::serde::json;
use serde::Deserialize;
use serenity::client::Context;

use super::shared::create_index_option;

pub const DATA: CommandData = CommandData {
    admin_only: false,
    name: "pasta",
    category: Category::Misc,
};

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    command
        .description("sends the ulina copy-pasta of your choice")
        .add_option({
            let mut option = create_index_option(PASTAS.iter(), PASTA, |pasta| pasta.name.clone());
            option.required(true);
            option
        })
}

lazy_static! {
    static ref PASTAS: Vec<Pasta> =
        json::from_str(include_str!("../../../config/pasta.json")).unwrap();
}

#[derive(Deserialize)]
struct Pasta {
    name: String,
    content: String,
}

pub async fn pasta(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let pasta = index_option!(interaction.data.options, PASTA, PASTAS, "pasta");

    interaction
        .message(&ctx.http, |data| data.content(&pasta.content))
        .await;

    Ok(())
}
