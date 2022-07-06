pub use crate::database::models::*;
use crate::error::Error;
use common::{Flag, FlagId, Map, RawMap, Social};
use once_cell::sync::OnceCell;

use sqlx::types::chrono::{self, TimeZone, Utc};
use sqlx::{query, query_as, SqlitePool};
use std::fs;

pub mod models;

const DATABASE_URL: &str = "sqlite:data/Ulina.db";

static CONNECTION: OnceCell<SqlitePool> = OnceCell::new();

pub async fn init() {
    CONNECTION
        .set(SqlitePool::connect(DATABASE_URL).await.unwrap())
        .unwrap();
    sqlx::migrate!().run(db()).await.unwrap();
}

pub fn db() -> &'static SqlitePool {
    CONNECTION.get().expect("database uninitialised")
}

#[macro_export]
macro_rules! find_nation {
    ($model: ident, $select: tt, $predicate: tt $(,$bind: expr)*) => {
        sqlx::query_as!(common::$model, "SELECT " + $select + " FROM Nation WHERE " + $predicate $(,$bind)*)
    };
}

pub async fn socials(id: i64) -> Result<Vec<Social>, Error> {
    query_as!(Social, "SELECT * FROM Social WHERE nationId = ?", id)
        .fetch_all(db())
        .await
        .map_err(|err| err.into())
}
const MAP_DIR: &str = "data/maps";

pub async fn latest_map() -> Result<Map, Error> {
    let map: RawMap = query_as!(RawMap, "SELECT * FROM Map ORDER BY date DESC LIMIT 1")
        .fetch_one(db())
        .await?;

    let file = fs::read_to_string(format!("{}/{}", MAP_DIR, &map.fileName))
        .map_err(|err| Error::InternalError(format!("Error reading map file: {:?}", err)))?;
    let date = Utc.from_utc_datetime(&map.date);

    Ok(Map { date, file })
}

pub async fn flag_link(id: i64) -> String {
    query_as!(Flag, "SELECT flagPath FROM Flag WHERE flagId = ?", id)
        .fetch_one(db())
        .await
        .map(|flag: Flag| format!("https://www.ulinaworld.com{}", flag.flagPath))
        .unwrap()
}

const FLAG_DIR: &str = "./public/flags";

pub async fn add_flag(
    nation_id: i64,
    name: &str,
    extension: &str,
    buffer: Vec<u8>,
) -> Result<(), Error> {
    let date = chrono::Utc::now().naive_utc();

    let file_name = format!(
        "{}-{}.{}",
        name.replace(" ", ""),
        date.format("%Y_%m_%d_%H_%M_%S"),
        extension
    );

    fs::write(format!("{}/{}", FLAG_DIR, file_name), buffer)
        .map_err(|e| Error::InternalError(format!("{:?}", e)))?;

    let path = format!("/flags/{}", file_name);

    let flag: FlagId = query_as!(
        FlagId,
        "INSERT INTO Flag (flagPath, nationId) VALUES (?, ?) RETURNING flagId",
        path,
        nation_id
    )
    .fetch_one(db())
    .await?;

    query!(
        "UPDATE Nation SET currentFlagId = ? WHERE nationId = ?",
        flag.flagId,
        nation_id
    )
    .execute(db())
    .await?;

    Ok(())
}

pub const CONTINENTS: [&str; 5] = ["Ripiero", "Kanita", "Zapita", "Ailou", "Sivalat"];
