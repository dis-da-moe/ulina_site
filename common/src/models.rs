use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct UserData {
    pub is_admin: bool,
    pub owner_discord: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[allow(non_snake_case)]
pub struct Social {
    pub socialsId: i64,
    pub nationId: i64,
    pub link: String,
    pub platform: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct AddSocial {
    pub socials_id: Option<i64>,
    pub link: String,
    pub platform: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct UserAndData<T> {
    pub data: T,
    pub user: UserData,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[allow(unused, non_snake_case)]
pub struct NationContinent {
    pub name: String,
    pub ownerDiscord: String,
    pub continentName: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[allow(unused, non_snake_case)]
pub struct NationContinentId {
    pub nationId: i64,
    pub name: String,
    pub ownerDiscord: String,
    pub continentName: String,
    pub removed: bool
}

pub type LoadNations = UserAndData<Vec<NationContinentId>>;
pub type LoadNation = UserAndData<NationAll>;
pub const CONTINENTS: [&str; 5] = ["Ripiero", "Kanita", "Zapita", "Ailou", "Sivalat"];
