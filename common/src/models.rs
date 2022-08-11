use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub leader: Option<String>,
    pub capital: Option<String>,
    pub ideology: Option<String>,
    pub alliances: Option<String>,
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
    pub flag_link: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LoadMap {
    pub map: Map,
    pub nations: Vec<NationAll>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Map {
    pub date: DateTime<Utc>,
    pub file: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[allow(unused, non_snake_case)]
pub struct User {
    pub userId: i64,
    pub isAdmin: bool,
    pub discord: Option<String>,
    pub pendingAuth: Option<String>,
}