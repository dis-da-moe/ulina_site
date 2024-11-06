pub use crate::database::models::*;
use crate::error::Error;
use common::{Map, Nation, NationAll, Social, current_url};

use sqlx::types::chrono::{TimeZone, Utc};
use sqlx::{query, query_as};
use std::fs;
use super::db;
use common::FlagId;
const MAP_DIR: &str = "data/maps";

pub async fn latest_map() -> Result<Map, Error> {
    let map = query!("SELECT * FROM Map ORDER BY date DESC LIMIT 1")
        .fetch_one(db())
        .await?;

    let file = fs::read_to_string(format!("{}/{}", MAP_DIR, &map.fileName))
        .map_err(|err| Error::InternalError(format!("Error reading map file: {:?}", err)))?;
    let date = Utc.from_utc_datetime(&map.date);

    Ok(Map { date, file })
}

pub async fn flag_link(id: FlagId) -> Result<String, Error> {
    query!("SELECT flagPath FROM Flag WHERE flagId = ?", id.0)
        .fetch_one(db())
        .await
        .map(|flag| format!("{}{}", current_url(), flag.flagPath))
        .map_err(|e| e.into())
}

pub async fn socials(id: i64) -> Result<Vec<Social>, Error> {
    query_as!(Social, "SELECT * FROM Social WHERE nationId = ?", id)
        .fetch_all(db())
        .await
        .map_err(|err| err.into())
}

async fn more_nation_info(nation: Nation) -> Result<NationAll, Error> {
    let socials = socials(nation.nationId).await?;

    let flag_link = match nation.currentFlagId {
        Some(id) => Some(flag_link(FlagId(id)).await?),
        None => None,
    };

    Ok(NationAll {
        core: nation,
        socials,
        flag_link,
    })
}

pub async fn nations_all() -> Result<Vec<NationAll>, Error> {
    let result: Vec<Nation> = query_as!(Nation, "SELECT * FROM Nation WHERE removed = false")
        .fetch_all(db())
        .await?;

    let mut nations = vec![];

    for nation in result {
        nations.push(more_nation_info(nation).await?);
    }

    Ok(nations)
}

pub async fn nation_all(id: i64, include_removed: bool) -> Result<NationAll, Error> {
    let result: Nation = if include_removed {
        query_as!(Nation, "SELECT * FROM Nation WHERE nationId = ?", id)
            .fetch_one(db())
            .await
    } else {
        query_as!(
            Nation,
            "SELECT * FROM Nation WHERE removed = false AND nationId = ?",
            id
        )
        .fetch_one(db())
        .await
    }?;

    more_nation_info(result).await
}
