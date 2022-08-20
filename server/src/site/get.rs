use crate::database::{db, flag_link, FlagId};
use crate::database::{latest_map, nation_all, nations_all};
use crate::error::Error;
use crate::site::user_data::AdminUser;
use chrono::{TimeZone, Utc};
use common::{
    ChangeType, LoadChanges, LoadMap, LoadNation, LoadNations, NationChange, NationContinentId,
    UserAndData, UserData,
};
use rocket::fs::NamedFile;
use rocket::serde::json::Json;

use sqlx::{query, query_as};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::str::FromStr;

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
    let is_admin = query!("SELECT isAdmin FROM User WHERE userId = ?", user.0)
        .fetch_one(db())
        .await?
        .isAdmin;

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

#[get("/user-data")]
pub async fn get_user_data(user: UserId) -> Result<Json<UserData>, Error> {
    Ok(Json(user_data(user).await?))
}

#[get("/nation-changes")]
pub async fn nation_changes(user: AdminUser) -> Result<Json<LoadChanges>, Error> {
    let user = user_data(user.into()).await?;

    let queried_changes = query!("SELECT * FROM NationChange").fetch_all(db()).await?;
    let mut changes = vec![];

    let nations: HashMap<_, _> = query!("SELECT nationId, name FROM Nation")
        .fetch_all(db())
        .await?
        .into_iter()
        .map(|nation| (nation.nationId, nation.name))
        .collect();

    let get_flag = |flag_id: Option<String>| async move {
        match flag_id {
            Some(id) => {
                let id = i64::from_str(&id).map_err(|e| {
                    Error::InternalError(format!("{} is not a valid flagId: {}", id, e))
                })?;

                flag_link(FlagId(id)).await.map(|value| Some(value))
            }
            None => Ok(None),
        }
    };

    for change in queried_changes {
        let change_type = ChangeType::from_str(&change.r#type)?;
        let name = nations
            .get(&change.nationId)
            .ok_or_else(|| {
                Error::InternalError(format!(
                    "NationChange of id {} has a nationId that does not exist: {}",
                    change.changeId, change.nationId
                ))
            })?
            .clone();

        let (old_value, new_value) = match change_type {
            ChangeType::Flag => (
                get_flag(change.oldValue).await?,
                get_flag(change.newValue).await?,
            ),
            _ => (change.oldValue, change.newValue),
        };

        changes.push(NationChange {
            nation_name: name,
            change_type,
            old_value,
            new_value,
            date: Utc.from_utc_datetime(&change.timeStamp),
            admin: change.admin,
        });
    }

    Ok(Json(UserAndData {
        data: changes,
        user,
    }))
}

async fn user_data(user: UserId) -> Result<UserData, Error> {
    query_as!(
        UserData,
        "SELECT discord, isAdmin FROM User WHERE userId = ?",
        user.0
    )
    .fetch_one(db())
    .await
    .map_err(|e| e.into())
}

async fn user_and_data<T>(user: UserId, data: T) -> Result<UserAndData<T>, Error> {
    Ok(UserAndData {
        data,
        user: user_data(user).await?,
    })
}
