use crate::database::db;
use crate::database::{latest_map, nation_all, nations_all};
use crate::error::Error;
use common::{LoadMap, LoadNation, LoadNations, NationContinentId, UserAndData, UserData};
use rocket::fs::NamedFile;

use rocket::serde::json::Json;

use sqlx::{query, query_as};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

use super::directories::STATIC_DIR;
use super::user_data::UserId;

#[get("/<path..>", rank = 12)]
pub async fn page(path: PathBuf) -> Option<NamedFile> {
    let mut page = path
        .file_name()
        .unwrap_or_else(|| OsStr::new("index"))
        .to_owned();

    page.push(".html");

    NamedFile::open(STATIC_DIR.join(Path::new(&page)))
        .await
        .ok()
}

#[get("/tools/<_path..>")]
pub async fn tools(_path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(STATIC_DIR.join(Path::new("tools.html")))
        .await
        .ok()
}

#[get("/load-map")]
pub async fn load_map() -> Result<Json<LoadMap>, Error> {
    Ok(Json(LoadMap {
        nations: nations_all().await?,
        map: latest_map().await?,
    }))
}

#[get("/nation/<id>")]
pub async fn nation(user: UserId, id: i64) -> Result<Json<Option<LoadNation>>, Error> {
    let is_admin = query!("SELECT isAdmin FROM User WHERE userId = ?", user.0).fetch_one(db()).await?.isAdmin;
    
    Ok(Json(match nation_all(id, is_admin).await.ok() {
        Some(nation) => Some(user_and_data(user, nation).await?),
        _ => None,
    }))
}

#[get("/nations")]
pub async fn nations(user: UserId) -> Result<Json<LoadNations>, Error> {
    Ok(Json(
        user_and_data(
            user,
            query_as!(
                NationContinentId,
                "SELECT nationId, name, continentName, ownerDiscord, removed FROM Nation"
            )
            .fetch_all(db())
            .await?,
        )
        .await?,
    ))
}

async fn user_and_data<T>(user: UserId, data: T) -> Result<UserAndData<T>, Error> {
    let user = query!("SELECT discord, isAdmin FROM User WHERE userId = ?", user.0)
        .fetch_one(db())
        .await?;
    let user = UserData {
        is_admin: user.isAdmin,
        owner_discord: user.discord,
    };
    Ok(UserAndData { data, user })
}
