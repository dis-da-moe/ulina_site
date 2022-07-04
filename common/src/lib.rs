use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const LOCAL_IP: &str = "127.0.0.1";
pub const PORT: u16 = 8000;
pub const URL: &str = "https://www.ulinaworld.com";

use lazy_static::lazy_static;

lazy_static! {
    pub static ref LOCAL_URL: String = format!("http://{}:{}", LOCAL_IP, PORT);
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(unused, non_snake_case)]
pub struct Nation {
    pub nationId: i64,
    pub continentName: String,
    pub name: String,
    pub removed: bool,
    pub ownerDiscord: String,
    pub description: Option<String>,
    pub currentFlagId: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_snake_case)]
pub struct Social {
    pub socialsId: i64,
    pub nationId: i64,
    pub link: String,
    pub platform: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NationAll {
    pub core: Nation,
    pub socials: Vec<Social>,
    pub flag_link: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Flag {
    //pub flag_id: i32,
    pub flagPath: String,
    //pub nation_id: i32
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[allow(unused, non_snake_case)]
pub struct NationId {
    pub nationId: i64,
    pub name: String,
    pub ownerDiscord: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LoadMap {
    pub map: Map,
    pub nations: Vec<NationAll>,
}

#[allow(non_snake_case)]
pub struct RawMap {
    pub mapId: i64,
    pub fileName: String,
    pub date: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Map {
    pub date: DateTime<Utc>,
    pub file: String,
}
