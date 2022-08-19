mod error;

#[macro_use]
mod database;
mod config;
mod discord;
mod site;
mod util;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

use lazy_static::initialize;
use rocket::tokio::{self, try_join};

#[tokio::main]
async fn main() -> Result<(), String> {
    std::env::set_current_dir("./server").unwrap();

    initialize(&config::CONFIG);

    database::init().await;
    tokio::spawn(site::run());
    discord::run().await
}
