use super::shared::name_option;
use crate::config::CONFIG;
use crate::database::NationDiscord;
use crate::discord::commands::shared::{Category, CommandData, CreateCommand, Interaction};
use crate::discord::helper::{embed, Helper};
use crate::error::Error;
use crate::get_nation;
use serde::Deserialize;
use serenity::client::Context;

pub const DATA: CommandData = CommandData {
    admin_only: false,
    name: "coin",
    category: Category::ViewNation,
};
const SPREADSHEET_URL: &str = "https://docs.google.com/spreadsheets/d/1WIvqURKF8xik6ysmd1WqJip8WlX05ksUfu1HUZIx7bk/edit#gid=0";
const SPREADSHEET_ID: &str = "1WIvqURKF8xik6ysmd1WqJip8WlX05ksUfu1HUZIx7bk";
const RANGE: &str = "A:B";

lazy_static! {
    static ref URL: String = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?key={}",
        SPREADSHEET_ID, RANGE, CONFIG.google_key
    );
}

#[derive(Deserialize)]
struct ValueRange {
    values: Vec<Vec<String>>,
}

macro_rules! spreadsheet {
    () => {
        |err| Error::NetworkError(format!("could not get spreadsheet data: {}", err))
    };
}

pub fn create(command: &mut CreateCommand) -> &mut CreateCommand {
    command
        .description("view the Ulinacoin of your nation or a nation specified by name")
        .create_option(|option| {
            name_option(option)
                .description("the name of the ulina nation")
                .required(false)
        })
}

pub async fn coin(ctx: &Context, interaction: &Interaction) -> Result<(), Error> {
    let nation_name = get_nation!(interaction, NationDiscord, "ownerDiscord, nationId, name")?.name;

    let request = reqwest::get(URL.clone())
        .await
        .map_err(spreadsheet!())?
        .json::<ValueRange>()
        .await
        .map_err(spreadsheet!())?;
    let mut balance = request.values.into_iter().find_map(|mut row|{
        match (row.get(0)?, row.get(1)?){
            (name, _) if name == &nation_name => Some(row.pop()?),
            _ => None
        }
    })
        .ok_or_else(|| Error::NetworkError(format!("nation by name of {} not found in spreadsheet - please check the spreadsheet for yourself: {}", &nation_name, SPREADSHEET_URL)))?;

    if &balance == "0" {
        balance = "broke".to_string()
    }

    let mut embed = embed();

    embed
        .title("Ulinacoin balance")
        .url(SPREADSHEET_URL)
        .description(SPREADSHEET_URL)
        .field(nation_name, balance, false);

    interaction
        .message(&ctx.http, |data| data.add_embed(embed))
        .await;

    Ok(())
}
