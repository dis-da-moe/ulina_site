use std::mem::take;

use chrono::Utc;
use common::{AddSocial, CONTINENTS, DATE_TIME_FORMAT};
use rocket::{
    form::{Form, Lenient, Strict},
    fs::TempFile,
    response::content::RawHtml,
    serde::json::{self}, tokio::fs,
};
use sqlx::query;
use sycamore::view;

use super::{rendering::Render, user_data::UserId};
use crate::{database::nation_change, site::user_data::AdminUser, internal};
use crate::{
    database::{add_flag, db, nation_all, validate_flag},
    error::Error,
};
use common::ChangeType;
use common::Id;

macro_rules! none_if_empty {
    ($nation: ident, $($field: tt)+) => {
        $(
            if let Some(string) = &$nation.$field{
                if string.trim().is_empty(){
                    $nation.$field = None;
                }
            }
        )+
    };
}

macro_rules! option_change {
    ($original: expr, $edit: expr, $is_admin: expr, $($field: tt => $change_type: tt),+) => {
        $(
            if $original.$field != $edit.$field{
                nation_change($original.id(), ChangeType::$change_type, $original.$field.clone(), $edit.$field.clone(), $is_admin).await?;
            }
        )+
    };
}

macro_rules! change {
    ($is_admin: expr, $id: expr, $($original: expr => $edit: expr, $change_type: ident)+) => {
        $(
            if $original != $edit{
                nation_change($id, ChangeType::$change_type, Some($original.clone()), Some($edit.clone()), $is_admin).await?;
            }
        )+
    };
}

#[post("/create-nation", data="<create_nation>")]
pub async fn create_nation(
    create_nation: Form<Strict<CreateNation>>,
    _user: AdminUser
) -> Result<RawHtml<String>, Error>{

    if !CONTINENTS.contains(&create_nation.continentName.as_str()){
        return Err(Error::NotContinent(create_nation.continentName.clone()));
    }
    
    let name = create_nation.name.clone();

    query!("INSERT INTO Nation (name, ownerDiscord, continentName) VALUES (?, ?, ?)", name, create_nation.ownerDiscord, create_nation.continentName)
        .execute(db()).await?;
    Ok(view!{
        (format!("Created nation {}", create_nation.name))
    }.render())
}

#[post("/create-map", data="<map>")]
pub async fn create_map(
    mut map: Form<Strict<CreateMap>>,
    _user: AdminUser
) -> Result<RawHtml<String>, Error>{
    let now = Utc::now();
    let file_name = format!("map-{}.svg", now.format(DATE_TIME_FORMAT));
    fs::write(format!("./data/maps/{}", file_name), take(&mut map.svg)).await.map_err(internal!())?;

    query!("INSERT INTO Map (fileName, date) VALUES (?, ?)", file_name, now).execute(db()).await?;

    Ok(view!{
        "saved map"
    }.render())
}

#[post("/edit-nation", data = "<edit>")]
pub async fn edit_nation(
    mut edit: Form<Strict<EditNation<'_>>>,
    user: UserId,
) -> Result<RawHtml<String>, Error> {
    let edit_socials: Vec<AddSocial> = json::from_str(&edit.socials)?;
    let user = query!("SELECT discord, isAdmin FROM User WHERE userId = ?", user.0)
        .fetch_one(db())
        .await?;
    let nation = nation_all(edit.id, user.isAdmin).await?;

    if user.discord.as_ref() != Some(&nation.core.ownerDiscord) && !user.isAdmin {
        return Err(Error::InvalidPermissions(
            "This nation does not belong to you".to_string(),
        ));
    }

    if let Some(flag) = &mut edit.flag {
        if flag.len() != 0 {
            let file_name = flag
                .raw_name()
                .ok_or(Error::JsonError("No file name".to_string()))?
                .dangerous_unsafe_unsanitized_raw()
                .to_string();

            let extension = validate_flag(&file_name, flag.len())?;

            let writer = |path| async move { flag.persist_to(path).await };

            add_flag(
                nation.id(),
                &nation.core.name,
                extension,
                writer,
                user.isAdmin,
            )
            .await?;
        }
    }

    none_if_empty!(edit, description leader capital ideology alliances);

    query!(
        "UPDATE Nation SET name = ?, description = ?, leader = ?, capital = ?, ideology = ?, alliances = ? WHERE nationId = ?",
        edit.name,
        edit.description,
        edit.leader,
        edit.capital,
        edit.ideology,
        edit.alliances,
        nation.core.nationId
    ).execute(db()).await?;

    option_change!(nation.core, edit, user.isAdmin, description => Description, leader => Leader, capital => Capital, ideology => Ideology, alliances => Alliances);

    let mut included_socials = vec![];
    for edit_social in edit_socials {
        match edit_social.socials_id {
            None => {
                query!(
                    "INSERT INTO Social (platform, link, nationId) VALUES (?, ?, ?)",
                    edit_social.platform,
                    edit_social.link,
                    nation.core.nationId
                )
                .execute(db())
                .await?;
            }
            Some(id) => {
                let original = nation
                    .socials
                    .iter()
                    .find(|social| social.socialsId == id)
                    .ok_or(Error::SocialNotFound(edit_social.platform.clone()))?;

                if edit_social.link != original.link || edit_social.platform != original.platform {
                    query!(
                        "UPDATE Social SET platform = ?, link = ? WHERE socialsId = ?",
                        edit_social.platform,
                        edit_social.link,
                        id
                    )
                    .execute(db())
                    .await?;
                }

                included_socials.push(id);
            }
        }
    }

    for delete_social in nation
        .socials
        .iter()
        .filter(|social| !included_socials.contains(&social.socialsId))
    {
        query!(
            "DELETE FROM Social WHERE socialsId = ?",
            delete_social.socialsId
        )
        .execute(db())
        .await?;
    }

    if user.isAdmin {
        if let (Some(removed), Some(discord), Some(continent)) = (
            &edit.removed,
            edit.discord.as_ref(),
            edit.continent.as_ref(),
        ) {
            if !CONTINENTS.contains(&continent.as_str()) {
                return Err(Error::JsonError(format!(
                    "invalid continent: {}",
                    continent
                )));
            }

            let removed: &bool = removed;

            query!("UPDATE Nation SET removed = ?, ownerDiscord = ?, continentName = ? WHERE nationId = ?", removed, discord, continent, nation.core.nationId).execute(db()).await?;

            let removed_changed = |change_type| {
                nation_change(
                    nation.id(),
                    change_type,
                    Some(nation.core.removed.to_string()),
                    Some(removed.to_string()),
                    user.isAdmin,
                )
            };

            match (removed, nation.core.removed) {
                (false, true) => removed_changed(ChangeType::Removed).await?,
                (true, false) => removed_changed(ChangeType::Creation).await?,
                _ => {}
            };

            change!(user.isAdmin, nation.id(),
                &nation.core.ownerDiscord => discord, OwnerDiscord
                &nation.core.continentName => continent, Continent
            );
        } else {
            return Err(Error::JsonError(
                "missing fields required for an admin submission".to_string(),
            ));
        }
    }

    Ok(view!(
        p{(format!("successfully edited {}", nation.core.name))}
        a(href="/tools/nations"){"View nations"}
    )
    .render())
}

#[derive(FromForm)]
pub struct EditNation<'r> {
    id: i64,
    socials: String,
    name: String,
    description: Option<String>,
    leader: Option<String>,
    capital: Option<String>,
    ideology: Option<String>,
    alliances: Option<String>,
    flag: Option<TempFile<'r>>,
    removed: Option<Lenient<bool>>,
    discord: Option<String>,
    continent: Option<String>,
}

#[derive(FromForm)]
#[allow(non_snake_case)]
pub struct CreateNation{
    name: String,
    ownerDiscord: i64,
    continentName: String,
}

#[derive(FromForm)]
pub struct CreateMap{
    svg: String
}