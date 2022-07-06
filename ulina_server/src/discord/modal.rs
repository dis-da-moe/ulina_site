use serenity::{
    http::Http,
    model::interactions::{
        message_component::{ActionRowComponent},
        modal::ModalSubmitInteraction,
    },
};
use sqlx::{query, query_as};

use crate::{database::db, error::Error};
use common::NationId;

use super::{
    helper::Helper,
    ids::{DESCRIPTION_INPUT, NAME_INPUT},
};

pub async fn handle_modal(http: &Http, interaction: ModalSubmitInteraction) {
    match action(&interaction).await {
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

async fn action(interaction: &ModalSubmitInteraction) -> Result<&str, Error> {
    let nation: NationId = query_as!(
        NationId,
        "SELECT name, ownerDiscord, nationId FROM Nation where nationId = ?",
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
        "UPDATE Nation SET name = ?, description = ? RETURNING name",
        name,
        description
    )
    .fetch_one(db())
    .await?;

    Ok(name)
}
