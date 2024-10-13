use std::{collections::HashMap, time::Duration};

use common::NationContinent;
use rocket::futures::StreamExt;
use serenity::{
    builder::{CreateActionRow, CreateEmbed},
    client::Context,
    collector::ComponentInteractionCollectorBuilder,
    model::interactions::message_component::ButtonStyle,
};
use sqlx::query_as;

use crate::{
    database::db,
    discord::{
        helper::{display_user, embed, Helper},
        ids::{CONTINENT, NEXT, PREVIOUS},
    },
    error::Error,
    get_options, internal,
};

use super::shared::{
    create_continent_option, default_data, get_continent, Category, CommandData, CreateCommand,
    Interaction,
};

pub const DATA: CommandData = CommandData {
    admin_only: false,
    name: "all-nations",
    category: Category::ViewNation,
};

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    default_data(command, &DATA)
        .description("view a list of nations")
        .add_option({
            let mut option = create_continent_option();
            option.description("limit nations to a continent");
            option
        })
}

fn nations_embed(nations: Vec<NationContinent>, continent: &str) -> CreateEmbed {
    let mut embed = embed();

    embed.title(continent);

    for nation in nations {
        embed.field(&nation.name, display_user(&nation.ownerDiscord), true);
    }

    embed
}

macro_rules! button {
    ($row: expr, $($id: ident, $emoji: expr, $disabled: expr),+) => {
        $row$(.create_button(|button| {
            button
                .custom_id($id)
                .style(ButtonStyle::Secondary)
                .label($emoji)
                .disabled($disabled)
        }))+;
    };
}

fn content<'a, 'b>(embeds: &Vec<CreateEmbed>, page: usize) -> (CreateEmbed, CreateActionRow) {
    let mut row = CreateActionRow::default();

    button!(
        row,
        PREVIOUS,
        "⬅️",
        page == 0,
        NEXT,
        "➡️",
        page == embeds.len() - 1
    );

    (embeds[page].clone(), row)
}

macro_rules! set_content {
    ($embeds: expr, $page: expr) => {
        |message| {
            let (embed, row) = content($embeds, $page);
            message
                .set_embed(embed)
                .components(|components| components.set_action_row(row))
        }
    };
}

pub async fn all_nations(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let continent = get_options!(interaction.data.options, CONTINENT, Integer);

    if let Ok(index) = continent {
        let continent = get_continent(*index)?;
        let nations = query_as!(
            NationContinent,
            "SELECT name, ownerDiscord, continentName FROM Nation WHERE removed = false AND continentName = ?",
            continent
        )
        .fetch_all(db())
        .await?;

        interaction
            .message(&ctx.http, |message| {
                message.add_embed(nations_embed(nations, continent))
            })
            .await;

        return Ok(());
    }

    let nations: Vec<NationContinent> = query_as!(
        NationContinent,
        "SELECT name, ownerDiscord, continentName FROM Nation WHERE removed = false"
    )
    .fetch_all(db())
    .await?;
    let mut nations_map = HashMap::new();

    for nation in nations {
        nations_map
            .entry(nation.continentName.clone())
            .or_insert(vec![])
            .push(nation);
    }

    let mut nations = nations_map.into_iter().collect::<Vec<_>>();

    nations.sort_by(|(_, a), (_, b)| b.len().cmp(&a.len()));

    let embeds = nations
        .into_iter()
        .map(|(continent, nations)| nations_embed(nations, &continent))
        .collect::<Vec<_>>();

    let mut page = 0;

    interaction
        .message(&ctx.http, set_content!(&embeds, page))
        .await;

    let reply = interaction
        .get_interaction_response(&ctx.http)
        .await
        .map_err(internal!())?;

    let mut collector = ComponentInteractionCollectorBuilder::new(&ctx)
        .message_id(reply.id)
        .author_id(interaction.user.id)
        .timeout(Duration::from_secs(30))
        .build();

    while let Some(button_click) = collector.next().await {
        match button_click.data.custom_id.as_str() {
            PREVIOUS => {
                page = page.checked_sub(1).unwrap_or(0);
            }
            NEXT => {
                page += 1;
                page = page.min(embeds.len());
            }
            _ => {
                continue;
            }
        }

        interaction
            .edit_original_interaction_response(&ctx.http, set_content!(&embeds, page))
            .await
            .map_err(internal!())?;

        button_click.defer(&ctx.http).await.map_err(internal!())?;
    }
    interaction
        .edit_original_interaction_response(&ctx.http, |message| {
            message.components(|components| components)
        })
        .await
        .map_err(internal!())?;

    Ok(())
}
