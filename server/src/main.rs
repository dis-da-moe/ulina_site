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

use rocket::tokio::{self};

#[tokio::main]
async fn main() -> Result<(), String> {
    std::env::set_current_dir("./server").unwrap();

    dotenv::dotenv().unwrap();

    database::init().await;

    /*
    let rocket = tokio::spawn(site::run());
    let discord = tokio::spawn(discord::run());

    let (discord, rocket) = try_join!(discord, rocket).unwrap();

    discord?;
    rocket*/

    site::run().await
}
