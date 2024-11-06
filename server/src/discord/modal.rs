use common::ChangeType;
use serenity::{
    http::Http,
    model::interactions::{message_component::ActionRowComponent, modal::ModalSubmitInteraction},
};
use sqlx::query;

use super::{
    helper::{is_admin, Helper},
    ids::{DESCRIPTION_INPUT, NAME_INPUT},
};
use crate::{
    database::{db, nation_change},
    error::Error,
};
use common::NationId;

pub async fn handle_modal(http: &Http, interaction: ModalSubmitInteraction) {
    match action(http,&interaction).await {
        Ok(name) => {
            interaction
                .message(http, |message| {
                    message.content(format!("{} successfully edited", name))
                })
                .await;
        }
        Err(err) => {
            interaction
                .message(http, |message| {
                    message.content(format!("{}", err.to_string()))
                })
                .await;
        }
    }
}

async fn action(http: &Http, interaction: &ModalSubmitInteraction) -> Result<String, Error> {
    let nation = query!(
        "SELECT ownerDiscord, nationId, name, description FROM Nation where nationId = ?",
        interaction.data.custom_id
    )
    .fetch_one(db())
    .await?;

    if interaction.user.id.0.to_string() != nation.ownerDiscord {
        return Err(Error::InvalidPermissions(
            "this nation does not belong to you".to_string(),
        ));
    };

    let mut name = None;
    let mut description = None;

    for row in interaction.data.components.iter() {
        for component in row.components.iter() {
            match component {
                ActionRowComponent::InputText(field) => {
                    if field.custom_id == NAME_INPUT {
                        name = Some(&field.value);
                    } else if field.custom_id == DESCRIPTION_INPUT && !field.value.is_empty() {
                        description = Some(&field.value);
                    }
                }
                _ => {}
            }
        }
    }

    let name = name.ok_or(Error::InternalError("expected name field".to_string()))?;

    query!(
        "UPDATE Nation SET name = ?, description = ? WHERE nationId = ? RETURNING name",
        name,
        description,
        nation.nationId
    )
    .fetch_one(db())
    .await?;

    let id = NationId(nation.nationId);
    let admin = is_admin(http, &interaction.user).await?;

    let change = |change_type, old, new| nation_change(id, change_type, old, new, admin);

    if &nation.name != name {
        change(ChangeType::Name, Some(nation.name), Some(name.clone())).await?;
    }

    if nation.description.as_ref() != description {
        change(
            ChangeType::Description,
            nation.description,
            description.cloned(),
        )
        .await?;
    }

    Ok(name.clone())
}
