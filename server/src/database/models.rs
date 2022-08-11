use serde::{Deserialize, Serialize};

#[allow(unused, non_snake_case)]
pub struct NationContinent {
    pub name: String,
    pub ownerDiscord: String,
    pub continentName: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[allow(unused, non_snake_case)]
pub struct NationDiscord {
    pub nationId: i64,
    pub name: String,
    pub ownerDiscord: String,
}
