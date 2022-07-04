use crate::{database, nation_id};
use common::{LoadMap, NationId, NationAll, Nation};
use lazy_static::initialize;
use rocket::config::LogLevel;
use rocket::fs::{FileServer, NamedFile};
use rocket::futures::TryFutureExt;
use rocket::serde::json::Json;
use rocket::Config;
use sqlx::query_as;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::database::{db, socials, flag_link};

const PUBLIC_FOLDER: &str = "public";
const STATIC_FOLDER: &str = "static";

lazy_static! {
    static ref CURRENT_DIR: PathBuf = env::current_dir().expect("could not get current directory");
    static ref STATIC_DIR: PathBuf = CURRENT_DIR.join(Path::new(STATIC_FOLDER));
}

#[get("/<path..>", rank = 12)]
async fn page(path: PathBuf) -> Option<NamedFile> {
    let mut page = path
        .file_name()
        .unwrap_or_else(|| OsStr::new("index"))
        .to_owned();

    page.push(".html");

    NamedFile::open(STATIC_DIR.join(Path::new(&page)))
        .await
        .ok()
}

#[get("/load-map")]
async fn load_map() -> Json<LoadMap> {
    let result: Vec<Nation> = query_as!(Nation, "SELECT * FROM Nation WHERE removed = false")
        .fetch_all(db()).await.unwrap();

    let mut nations = vec![];
    
    for nation in result{
        let socials = socials(nation.nationId).await.unwrap();

        let flag_link = match nation.currentFlagId{
            Some(id) => {
                Some(database::flag_link(id).await)
            },
            None => None
        };
        nations.push(NationAll{
            core: nation,
            socials,
            flag_link
        });
    }


    let map = database::latest_map().await.unwrap();

    Json(LoadMap { nations, map })
}

/* 
#[get("/nation-all?<id>")]
async fn nation_all(id: i64) -> Option<Json<NationAll>>{
    let nation: Nation = 
        find_nation!(Nation, "*", "removed = false AND nationId = ?", id).fetch_one(db()).await.ok()?;
    
    let socials = socials(nation.nationId).await.unwrap();

    let flag_link = match nation.currentFlagId{
        Some(id) => {
            Some(database::flag_link(id).await)
        },
        None => None
    };

    Some(Json(NationAll{
        core: nation,
        socials,
        flag_link
    }))
}
*/

pub async fn run() -> Result<(), String> {
    initialize(&CURRENT_DIR);
    initialize(&STATIC_DIR);

    let rocket = rocket::build()
        .configure(Config {
            log_level: LogLevel::Critical,
            cli_colors: true,
            address: common::LOCAL_IP.parse().unwrap(),
            port: common::PORT,
            ..Default::default()
        })
        .mount(
            "/",
            FileServer::from(CURRENT_DIR.join(Path::new(PUBLIC_FOLDER))),
        )
        .mount("/", routes![page, load_map])
        .ignite()
        .map_err(|err| err.to_string())
        .await?;

    println!("available on {}", common::LOCAL_URL.as_str());

    rocket.launch().map_err(|x| x.to_string()).await.map(|_| ())
}
