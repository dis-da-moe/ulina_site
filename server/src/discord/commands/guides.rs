use rocket::serde::json;
use serde::Deserialize;
use serenity::builder::CreateEmbed;

use crate::discord::commands::shared::{Category, CommandData, CreateCommand, Interaction};
use crate::discord::helper::{embed, Helper};
use crate::discord::ids::GUIDE;
use crate::error::Error;
use crate::util::{capitalise, ZERO_WIDTH};
use crate::{get_options, index_option};
use serenity::client::Context;

use super::shared::create_index_option;

#[derive(Deserialize)]
struct Guide {
    name: String,
    image: String,
    title: String,
    link: String,
    description: String,
    includes: Vec<String>,
}

struct GuideEmbed {
    name: String,
    embed: CreateEmbed,
}

fn get_guides() -> Vec<GuideEmbed> {
    let guides = json::from_str::<Vec<Guide>>(include_str!("../../../config/guide.json")).unwrap();

    guides
        .into_iter()
        .map(|guide| {
            let mut embed = embed();
            embed
                .title(guide.title)
                .url(guide.link.clone())
                .description(format!("{}\n{}", guide.link, guide.description))
                .image(guide.image);

            if !guide.includes.is_empty() {
                embed.field("This includes: ", ZERO_WIDTH, false);
                guide.includes.into_iter().for_each(|include| {
                    embed.field(format!("• **{}**", capitalise(&include)), ZERO_WIDTH, true);
                });
            }

            GuideEmbed {
                name: guide.name,
                embed,
            }
        })
        .collect()
}

lazy_static! {
    static ref GUIDES: Vec<GuideEmbed> = get_guides();
}

pub const DATA: CommandData = CommandData {
    admin_only: false,
    name: "guides",
    category: Category::Help,
};

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    command
        .description("view guides for getting acquainted with Ulina")
        .add_option({
            let mut option =
                create_index_option(GUIDES.iter(), GUIDE, |embed| capitalise(&embed.name));
            option.required(true);
            option
        })
}

pub async fn guides(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let guide = index_option!(interaction.data.options, GUIDE, GUIDES, "guide");

    interaction
        .message(&ctx.http, |data| data.add_embed(guide.embed.clone()))
        .await;

    Ok(())
}
