use crate::database::{self, FlagId};
use crate::database::{db, socials};
use common::{LoadMap, Nation, NationAll};
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use sqlx::query_as;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use super::directories::STATIC_DIR;

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
    let result: Vec<Nation> = query_as!(Nation, "SELECT * FROM Nation WHERE removed = false")
        .fetch_all(db())
        .await
        .unwrap();

    let mut nations = vec![];

    for nation in result {
        let socials = socials(nation.nationId).await.unwrap();

        let flag_link = match nation.currentFlagId {
            Some(id) => Some(database::flag_link(FlagId(id)).await.unwrap()),
            None => None,
        };
        nations.push(NationAll {
            core: nation,
            socials,
            flag_link,
        });
    }

    let map = database::latest_map().await.unwrap();

    Json(LoadMap { nations, map })
}
