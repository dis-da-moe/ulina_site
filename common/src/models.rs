use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[allow(non_snake_case)]
pub struct UserData {
    pub isAdmin: bool,
    pub discord: Option<String>,
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
    pub removed: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct NationChange {
    pub nation_name: String,
    pub change_type: ChangeType,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub date: DateTime<Utc>,
    pub admin: bool,
}

macro_rules! change_type {
    ($($variant: tt),+) => {
        #[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
        pub enum ChangeType {
            $($variant),+
        }

        impl FromStr for ChangeType{
            type Err = ChangeTypeParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        stringify!($variant) => Ok(ChangeType::$variant),
                    )+
                    _ => Err(ChangeTypeParseError(format!("{} is not a Change Type", s)))
                }
            }
        }

        impl ToString for ChangeType {
            fn to_string(&self) -> String {
                match self {
                    $(
                        ChangeType::$variant => stringify!($variant),
                    )+
                }
                .to_string()
            }
        }
    };
}

pub struct ChangeTypeParseError(pub String);

change_type! {
    Creation,
    Removed,
    Continent,
    Flag,
    OwnerDiscord,
    Description,
    Name,
    Leader,
    Capital,
    Ideology,
    Alliances
}

pub type LoadResult<T> = Result<T, String>;
type LoadType<T> = UserAndData<T>;

pub type LoadNations = LoadType<Vec<NationContinentId>>;
pub type LoadNation = LoadType<NationAll>;
pub type LoadChanges = LoadType<Vec<NationChange>>;

pub const CONTINENTS: [&str; 5] = ["Ripiero", "Kanita", "Zapita", "Ailou", "Sivalat"];

#[macro_export]
macro_rules! id_type {
    ($(($name: tt, $field_name: tt) $(, $model: ident)*),+) => {
    $(  #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name(pub i64);

    $(
        impl Id<$name> for $model{
            fn id(&self) -> $name{
                $name(self.$field_name)
            }
        }
    )*
    )+
    };
}

pub trait Id<T> {
    fn id(&self) -> T;
}

id_type!(
    (SocialsId, socialsId),
    Social,
    (NationId, nationId),
    Nation,
    NationContinentId,
    (FlagId, flagId)
);

impl Id<NationId> for NationAll {
    fn id(&self) -> NationId {
        self.core.id()
    }
}
