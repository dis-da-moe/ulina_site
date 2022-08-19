use lazy_static::initialize;
use rocket::config::LogLevel;
use rocket::figment::Figment;
use rocket::fs::FileServer;
use rocket::futures::TryFutureExt;
use rocket::Config;
use std::net::IpAddr;
use std::path::Path;

mod auth;
mod directories;
mod get;
pub mod rendering;
mod user_data;
mod post;

use auth::{admin, admin_login, discord_login, login_result, oauth_redirect, logout};
use get::{load_map, nation, nations, page, tools, get_user_data, nation_changes};
use post::edit_nation;
use crate::config::CONFIG;
use crate::site::directories::{CURRENT_DIR, PUBLIC_FOLDER, STATIC_DIR};
pub async fn run() -> Result<(), String> {
    initialize(&CURRENT_DIR);
    initialize(&STATIC_DIR);

    let config = Config {
        log_level: LogLevel::Critical,
        cli_colors: true,
        address: common::LOCAL_IP.parse::<IpAddr>().map_err(|e| format!("error parsing IP address: {}", e))?,
        port: common::PORT,
        ..Default::default()
    };
    let figment = Figment::from(config).merge(("secret_key", &CONFIG.secret_key));

    let rocket = rocket::build()
        .configure(figment)
        .mount(
            "/",
            FileServer::from(CURRENT_DIR.join(Path::new(PUBLIC_FOLDER))),
        )
        .mount(
            "/",
            routes![
                page,
                load_map,
                tools,
                admin,
                oauth_redirect,
                admin_login,
                login_result,
                discord_login,
                nations,
                nation,
                edit_nation,
                get_user_data,
                nation_changes,
                logout
            ],
        )
        .ignite()
        .map_err(|err| err.to_string())
        .await?;

    println!("available on {}", common::LOCAL_URL.as_str());

    rocket.launch().map_err(|x| x.to_string()).await.map(|_| ())
}
