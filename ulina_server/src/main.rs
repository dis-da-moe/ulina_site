mod error;

#[macro_use]
mod database;
mod discord;
mod site;
mod util;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

use crate::tokio::try_join;
use database::{db, socials};
use rocket::{tokio, serde::{json::Json, self}};
use sqlx::query_as;
use common::{Nation, NationAll};

#[tokio::main]
async fn main() -> Result<(), String> {
    std::env::set_current_dir("./ulina_server").unwrap();

    dotenv::dotenv().unwrap();

    database::init().await;

    /*
    let rocket = tokio::spawn(site::run());
    let discord = tokio::spawn(discord::run());

    let (discord, rocket) = try_join!(discord, rocket).unwrap();

    discord?;
    rocket
    */
    site::run().await
}
