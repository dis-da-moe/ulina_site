use crate::database::{db, flag_link};
use crate::database::{latest_map, nation_all, nations_all};
use crate::error::Error;
use crate::site::directories::PUBLIC_DIR;
use crate::site::user_data::AdminUser;
use chrono::{TimeZone, Utc};
use common::{FlagId, LoadMap, LoadResult};
use common::{
    ChangeType, LoadChanges, LoadNation, LoadNations, NationChange, NationContinentId,
    UserAndData, UserData,
};
use rocket::fs::NamedFile;
use rocket::serde::json::Json;

use sqlx::{query, query_as};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use super::user_data::UserId;

lazy_static!{
    static ref ALIASES: HashMap<&'static str, &'static str> = HashMap::from([
        ("tools_bg.wasm", "tools/tools_bg.wasm"),
        ("tools.js", "tools/tools.js"),
        ("index", "index.html"),
    ]);
}

#[get("/<path..>", rank = 11)]
pub async fn tools(path: PathBuf) -> Option<NamedFile> {
    let path = path.file_name().and_then(|file| file.to_str())?;
    let alias = *ALIASES.get(path)?;
    NamedFile::open(PUBLIC_DIR.join(alias))
            .await
            .ok()
}

#[get("/tools/<_path..>", rank = 10)]
pub async fn tools_dir(_path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(PUBLIC_DIR.join(Path::new("tools/index.html")))
        .await
        .ok()
}

// a replacement for the currently unstable try block feature to remove boiler plate
macro_rules! json_try {
    ($block: expr) => {
        Json({|| async move {
            $block
        }}().await.map_err(|e: Error| e.to_string()))
    };
}
type Load<T> = Json<LoadResult<T>>;

#[get("/load-map")]
pub async fn load_map() -> Load<LoadMap> {
    json_try!({
        Ok(LoadMap {
            nations: nations_all().await?,
            map: latest_map().await?,
        })
    })
}

#[get("/nation/<id>")]
pub async fn nation(user: UserId, id: i64) -> Load<LoadNation> {
    json_try!({
        let is_admin = query!("SELECT isAdmin FROM User WHERE userId = ?", user.0)
            .fetch_one(db())
            .await?
            .isAdmin;
        
        match nation_all(id, is_admin).await {
            Ok(nation) => Ok(user_and_data(user, nation).await?),
            Err(e) => Err(e),
        }
    })
}



#[get("/nations")]
pub async fn nations(user: UserId) -> Load<LoadNations> {
    json_try!({
        user_and_data(
            user,
            query_as!(
                NationContinentId,
                "SELECT nationId, name, continentName, ownerDiscord, removed FROM Nation"
            )
            .fetch_all(db())
            .await?,
        )
        .await
    })
}

#[get("/user-data")]
pub async fn get_user_data(user: UserId) -> Load<UserData> {
    Json(user_data(user).await.map_err(|e| e.to_string()))
}

#[get("/nation-changes")]
pub async fn nation_changes(user: AdminUser) -> Load<LoadChanges> {
    json_try!{{let user = user_data(user.into()).await?;

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

    Ok(UserAndData {
        data: changes,
        user,
    })}}
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
