use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[allow(unused, non_snake_case)]
pub struct NationDiscord {
    pub nationId: i64,
    pub name: String,
    pub ownerDiscord: String,
}
