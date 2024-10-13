pub use crate::database::models::*;
use crate::error::Error;

use common::{ChangeType, DATE_TIME_FORMAT};
use sqlx::query;
use sqlx::types::chrono::{self, Utc};
use std::fmt::Debug;

use std::future::Future;
use std::path::Path;

use super::db;
use common::NationId;
const FLAG_DIR: &str = "./public/flags";

pub const ACCEPTED_EXTENSIONS: [&str; 3] = ["jpg", "jpeg", "png"];

pub const MAX_SIZE: u64 = 8_000_000; //max size of an image in bytes

pub fn validate_flag(file_name: &str, size: u64) -> Result<&str, Error> {
    //accepts the full file-name of an image and its size in bytes
    //validates the size and extension and returns the extension of the file
    let extension = Path::new(file_name)
        .extension()
        .and_then(|e| e.to_str())
        .ok_or(Error::ExpectedImage("unknown".to_string()))?;

    if !ACCEPTED_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
        return Err(Error::ExpectedImage(extension.to_string()));
    }

    if size > MAX_SIZE {
        return Err(Error::TooLarge("8MB".to_string()));
    }

    Ok(extension)
}

pub async fn add_flag<F, E>(
    nation_id: NationId,
    nation_name: &str,
    extension: &str,
    //async closure that accepts a file path and writes the flag to it
    writer: impl FnOnce(String) -> F,
    is_admin: bool,
) -> Result<(), Error>
where
    F: Future<Output = Result<(), E>>,
    E: Debug,
{
    //adds a flag and assigns it to the nation as its current flag

    let date = chrono::Utc::now().naive_utc();

    let file_name = format!(
        "{}-{}.{}",
        nation_name.replace(" ", ""),
        date.format(DATE_TIME_FORMAT),
        extension
    );

    writer(format!("{}/{}", FLAG_DIR, file_name))
        .await
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
    .await?
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
