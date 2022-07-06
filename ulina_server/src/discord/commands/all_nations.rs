use std::collections::HashMap;

use serenity::{http::Http, builder::CreateEmbed, collector::{MessageCollector, MessageCollectorBuilder, ComponentInteractionCollectorBuilder}, client::Context};
use sqlx::query_as;

use crate::{error::Error, get_options, discord::{ids::CONTINENT, helper::{display_user, Helper}}, database::db};

use common::NationContinent;

use super::shared::{CommandData, Category, default_data, CreateCommand, Interaction, continent_option, get_continent};

pub const DATA: CommandData = CommandData{
    admin_only: false,
    name: "all-nations",
    category: Category::ViewNation,
};

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    default_data(command, &DATA).description("view a list of nations")
        .create_option(|option| continent_option(option).description("limit nations to a continent"))
}

fn nations_embed(nations: Vec<NationContinent>, continent: &str) -> CreateEmbed{
    let mut embed = CreateEmbed::default();
    
    embed.title(continent);

    for nation in nations{
        embed.field(&nation.name, display_user(&nation.ownerDiscord), true);
    }

    embed
}

pub async fn all_nations(ctx: &Context, interaction: &Interaction) -> Result<(), Error>{
    let continent = get_options!(interaction.data.options, CONTINENT, Integer);

    if let Ok(index) = continent{
        let continent = get_continent(*index)?;
        let nations = query_as!(NationContinent, "SELECT name, ownerDiscord, continentName FROM Nation").fetch_all(db()).await?;

        interaction.message(&ctx.http, |message| message.add_embed(nations_embed(nations, continent))).await;
    }
    else{
        let nations: Vec<NationContinent> = query_as!(NationContinent, "SELECT name, ownerDiscord, continentName FROM Nation").fetch_all(db()).await?;
        let mut nations_map = HashMap::new();

        for nation in nations{
            nations_map.entry(nation.continentName.clone()).or_insert(vec![]).push(nation);
        }

        let mut nations = nations_map
            .into_iter().collect::<Vec<_>>();

        nations.sort_by(|(_, a), (_, b)| a.len().cmp(&b.len()));

        let nations = nations.into_iter().map(|(continent, nations)| nations_embed(nations, &continent)).collect::<Vec<_>>();
        
        let reply = interaction.get_interaction_response(&ctx.http).await.map_err(|err| Error::InternalError(format!("{:?}", err)))?;
        
        let collector = reply.await_component_interaction(&ctx);
        
        todo!()
    }
    Ok(())
}
