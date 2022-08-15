use crate::database::{self, latest_map, nation_all, nations_all, FlagId};
use crate::database::{db, socials};
use common::{
    LoadMap, LoadNation, LoadNations, Nation, NationAll, NationContinentId, UserAndData, UserData,
};
use rocket::fs::NamedFile;
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use rocket::serde::DeserializeOwned;
use serde::{Deserialize, Serialize};
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
pub async fn load_map() -> Json<LoadMap> {
    Json(LoadMap {
        nations: nations_all().await.unwrap(),
        map: latest_map().await.unwrap(),
    })
}

#[get("/nation/<id>")]
pub async fn nation(user: UserId, id: i64) -> Json<Option<LoadNation>> {
    Json(match nation_all(id).await.ok() {
        Some(nation) => Some(user_and_data(user, nation).await),
        _ => None,
    })
}

#[get("/nations")]
pub async fn nations(user: UserId) -> Json<LoadNations> {
    Json(
        user_and_data(
            user,
            query_as!(
                NationContinentId,
                "SELECT nationId, name, continentName, ownerDiscord FROM Nation WHERE removed = false"
            )
            .fetch_all(db())
            .await
            .unwrap(),
        )
        .await,
    )
}

async fn user_and_data<T>(user: UserId, data: T) -> UserAndData<T> {
    let user = query!("SELECT discord, isAdmin FROM User WHERE userId = ?", user.0)
        .fetch_one(db())
        .await
        .unwrap();
    let user = UserData {
        is_admin: user.isAdmin,
        owner_discord: user.discord,
    };
    UserAndData { data, user }
}
