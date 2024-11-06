use crate::database::NationDiscord;
use crate::discord::commands::commands::build_commands_embed;
use crate::discord::commands::COMMANDS;
use crate::discord::helper::is_admin;
use crate::discord::ids::CONTINENT;
use crate::discord::ids::NAME;
use crate::error::Error;
use common::CONTINENTS;
use serenity::builder::{CreateApplicationCommand, CreateApplicationCommandOption};
use serenity::client::Context;
use serenity::http::Http;
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::ApplicationCommandOptionType;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::interactions::application_command::ApplicationCommand;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::slice::Iter;
use std::usize;

pub struct UlinaCommand {
    pub data: CommandData,
    pub create: fn(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
    pub action: for<'a> fn(
        &'a Context,
        &'a ApplicationCommandInteraction,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a + Send>>,
}

pub struct CommandData {
    pub admin_only: bool,
    pub name: &'static str,
    pub category: Category,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum Category {
    ViewNation,
    Help,
    EditNation,
    Misc,
}

impl ToString for Category {
    fn to_string(&self) -> String {
        match self {
            Category::ViewNation => "View Nation",
            Category::Help => "Help",
            Category::EditNation => "Edit Nation",
            Category::Misc => "Misc",
        }
        .to_string()
    }
}

pub type OptionType = ApplicationCommandOptionType;
pub type CreateCommand = CreateApplicationCommand;
pub type CreateOption = CreateApplicationCommandOption;
pub type Interaction = ApplicationCommandInteraction;

#[macro_export]
macro_rules! get_nation {
    ($interaction: expr, $model: ident, $select: tt) => {{
        let name_option =
            crate::get_options!($interaction.data.options, crate::discord::ids::NAME, String);
        let id = $interaction.user.id.0.to_string();

        match name_option {
            Ok(name) => {
                sqlx::query_as!(
                    $model,
                    "SELECT " + $select + " FROM Nation WHERE name LIKE ? AND removed = false",
                    name
                )
                .fetch_one(crate::database::db())
                .await
            }
            _ => {
                {
                    sqlx::query_as!(
                        $model,
                        "SELECT "
                            + $select
                            + " FROM Nation WHERE ownerDiscord = ? AND removed = false",
                        id
                    )
                }
                .fetch_one(crate::database::db())
                .await
            }
        }
    }};
}

pub fn default_data<'a>(
    command: &'a mut CreateCommand,
    data: &CommandData,
) -> &'a mut CreateCommand {
    if matches!(data.category, Category::EditNation) {
        let editing = data.name.replace("edit-", "").replace("-", " and ");

        let description = if data.admin_only {
            format!("admin only - change the {} of a nation", editing)
        } else {
            format!("change the {} of your nation", editing)
        };

        command.description(description);
        command.create_option(|option| {
            name_option(option)
                .description(if data.admin_only {
                    "the name of the ulina nation"
                } else {
                    "admin only - the name of an ulina nation"
                })
                .required(data.admin_only)
        });
    }

    command.name(data.name);

    command
}

pub fn name_option(option: &mut CreateOption) -> &mut CreateOption {
    option.name(NAME).kind(OptionType::String)
}

pub async fn edit_action(
    http: &Http,
    interaction: &Interaction,
    data: &CommandData,
) -> Result<NationDiscord, Error> {
    let is_admin = is_admin(http, &interaction.user).await?;
    if data.admin_only && !is_admin {
        return Err(Error::InvalidPermissions(
            "only an admin can do this - contact one if this is desired".to_string(),
        ));
    }

    let nation = get_nation!(interaction, NationDiscord, "nationId, name, ownerDiscord")?;

    if nation.ownerDiscord != interaction.user.id.0.to_string() && !is_admin {
        Err(Error::InvalidPermissions(
            "this nation does not belong to you - contact an admin if this is a mistake"
                .to_string(),
        ))
    } else {
        Ok(nation)
    }
}

pub async fn create_commands(guild_id: &GuildId, http: &Http) {
    let mut categorised_commands: HashMap<Category, Vec<(&CommandData, String)>> = HashMap::new();
    
    //remove all global commands (these were from the previous version of the bot)
    ApplicationCommand::set_global_application_commands(http, |commands| commands).await.expect("Error overriding global commands");

    let commands = GuildId::set_application_commands(guild_id, http, |commands| {
        for (name, command) in COMMANDS.iter() {
            commands.create_application_command(|builder| {
                let data = (command.create)(builder.name(name));

                let description = data
                    .0
                    .get("description")
                    .expect(&format!("no description set for command {}", name))
                    .as_str()
                    .expect(&format!("description for command {} is not a string", name));

                let entry = match categorised_commands.entry(command.data.category) {
                    Entry::Occupied(entry) => entry.into_mut(),
                    Entry::Vacant(entry) => entry.insert(Vec::new()),
                };

                entry.push((&command.data, description.to_string()));

                data
            });
        }

        commands
    })
    .await
    .expect("error creating slash commands");

    build_commands_embed(categorised_commands);

    println!(
        "slash commands created for guild {}:\n{}",
        guild_id.0,
        commands
            .iter()
            .map(|command| format!("\"{}\": {}", command.name, command.description))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

pub fn get_continent(index: i64) -> Result<&'static str, Error> {
    CONTINENTS
        .get(index as usize)
        .map(|string| *string)
        .ok_or(Error::InternalError(format!(
            "continent of index {} not found",
            index
        )))
}

#[macro_export]
macro_rules! index_option {
    ($options: expr, $name: ident, $array: ident, $string: expr) => {{
        let option = get_options!($options, $name, Integer)?;
        let option = usize::try_from(*option).map_err(crate::internal!())?;
        $array.get(option).ok_or(Error::InternalError(format!(
            "{} of index {} not found",
            $string, option
        )))?
    }};
}

pub fn create_index_option<T>(
    iter: Iter<T>,
    name: &str,
    choice: fn(&T) -> String,
) -> CreateApplicationCommandOption {
    let mut option = CreateApplicationCommandOption::default();

    iter.enumerate().for_each(|(index, x)| {
        option.add_int_choice(choice(x), index as i32);
    });

    option
        .name(name)
        .description(format!("the selected {}", name))
        .kind(OptionType::Integer);

    option
}

pub fn create_continent_option() -> CreateOption {
    create_index_option(CONTINENTS.iter(), CONTINENT, |continent| {
        continent.to_string()
    })
}
