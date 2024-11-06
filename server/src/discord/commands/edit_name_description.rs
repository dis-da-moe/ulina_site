use serenity::{
    builder::CreateInputText, client::Context,
    model::interactions::message_component::InputTextStyle,
};
use sqlx::query;

use crate::{
    database::db,
    discord::{
        helper::Helper,
        ids::{DESCRIPTION_INPUT, NAME_INPUT},
    },
    error::Error,
};

use super::shared::{default_data, edit_action, Category, CommandData, CreateCommand, Interaction};

pub const DATA: CommandData = CommandData {
    admin_only: false,
    name: "edit-name-description",
    category: Category::EditNation,
};

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    default_data(command, &DATA)
}

pub async fn edit_name_description(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let nation = edit_action(&ctx.http, interaction, &DATA).await?;

    let description: Option<String> = query!(
        "SELECT description FROM Nation WHERE nationId = ?",
        nation.nationId
    )
    .fetch_one(db())
    .await?
    .description;

    let mut name_input = CreateInputText::default();

    name_input
        .custom_id(NAME_INPUT)
        .label("Name")
        .required(true)
        .value(&nation.name)
        .style(InputTextStyle::Short);

    let mut description_input = CreateInputText::default();

    description_input
        .custom_id(DESCRIPTION_INPUT)
        .label("Description")
        .required(false)
        .style(InputTextStyle::Paragraph)
        .max_length(250)
        .value(description.unwrap_or("".to_string()));

    interaction
        .modal(&ctx.http, |data| {
            data.custom_id(nation.nationId)
                .title(format!("Edit {}", nation.name))
                .components(|components| {
                    for input in [name_input, description_input] {
                        components.create_action_row(|row| row.add_input_text(input));
                    }

                    components
                })
        })
        .await;

    Ok(())
}
