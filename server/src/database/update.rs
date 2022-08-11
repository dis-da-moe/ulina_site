pub use crate::database::models::*;
use crate::error::Error;

use sqlx::query;
use sqlx::types::chrono::{self, Utc};
use std::fs;

use super::{db, NationId};
const FLAG_DIR: &str = "./public/flags";

pub async fn add_flag(
    nation_id: NationId,
    name: &str,
    extension: &str,
    buffer: Vec<u8>,
    is_admin: bool,
) -> Result<(), Error> {
    let date = chrono::Utc::now().naive_utc();

    let file_name = format!(
        "{}-{}.{}",
        name.replace(" ", ""),
        date.format("%Y_%m_%d_%H_%M_%S"),
        extension
    );

    fs::write(format!("{}/{}", FLAG_DIR, file_name), buffer)
        .map_err(|e| Error::InternalError(format!("{:?}", e)))?;

    let path = format!("/flags/{}", file_name);

    let new_flag = query!(
        "INSERT INTO Flag (flagPath, nationId) VALUES (?, ?) RETURNING flagId",
        path,
        nation_id.0
    )
    .fetch_one(db())
    .await?
    .flagId;

    let old_flag = query!(
        "SELECT currentFlagId FROM Nation WHERE nationId = ?",
        nation_id.0
    )
    .fetch_one(db())
    .await
    .unwrap()
    .currentFlagId
    .map(|x| x.to_string());

    query!(
        "UPDATE Nation SET currentFlagId = ? WHERE nationId = ?",
        new_flag,
        nation_id.0
    )
    .execute(db())
    .await?;

    nation_change(
        nation_id,
        ChangeType::Flag,
        old_flag,
        Some(new_flag.to_string()),
        is_admin,
    )
    .await?;

    Ok(())
}

pub enum ChangeType {
    Creation,
    Removed,
    Continent,
    Flag,
    OwnerDiscord,
    Description,
    Name,
}

impl ToString for ChangeType {
    fn to_string(&self) -> String {
        match self {
            ChangeType::Creation => "Creation",
            ChangeType::Removed => "Removed",
            ChangeType::Continent => "Continent",
            ChangeType::Flag => "Flag",
            ChangeType::OwnerDiscord => "OwnerDiscord",
            ChangeType::Description => "Description",
            ChangeType::Name => "Name",
        }
        .to_string()
    }
}

pub async fn nation_change(
    nation_id: NationId,
    change_type: ChangeType,
    old_value: Option<String>,
    new_value: Option<String>,
    admin: bool,
) -> Result<(), Error> {
    let now = Utc::now();
    let change_type = change_type.to_string();

    query!("INSERT INTO NationChange (nationId, type, oldValue, newValue, admin, timeStamp) VALUES (?, ?, ?, ?, ?, ?)",
        nation_id.0,
        change_type,
        old_value,
        new_value,
        admin,
        now
    ).execute(db()).await?;

    Ok(())
}
