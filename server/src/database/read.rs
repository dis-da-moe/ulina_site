pub use crate::database::models::*;
use crate::error::Error;
use common::{Map, Social};

use sqlx::types::chrono::{self, TimeZone, Utc};
use sqlx::{query, query_as, SqlitePool};
use std::fs;

use super::{db, FlagId};
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
        .map(|flag| format!("https://www.ulinaworld.com{}", flag.flagPath))
        .map_err(|e| e.into())
}

pub const CONTINENTS: [&str; 5] = ["Ripiero", "Kanita", "Zapita", "Ailou", "Sivalat"];