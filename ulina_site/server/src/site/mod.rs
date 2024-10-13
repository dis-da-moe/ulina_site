use lazy_static::initialize;
use rocket::config::LogLevel;
use rocket::figment::Figment;
use rocket::fs::FileServer;
use rocket::Config;
use std::net::IpAddr;

mod auth;
mod directories;
mod get;
mod post;
pub mod rendering;
mod user_data;
mod cors;

use crate::config::CONFIG;
use crate::site::cors::CORS;
use crate::site::directories::{CURRENT_DIR, PUBLIC_DIR};
use auth::{admin, admin_login, discord_login, login_result, logout, oauth_redirect};
use get::{get_user_data, load_map, nation, nation_changes, nations, tools, tools_dir};
use post::{edit_nation, create_nation, create_map};
pub async fn run() {
    initialize(&CURRENT_DIR);

    let config = Config {
        log_level: LogLevel::Critical,
        cli_colors: true,
        address: common::LOCAL_IP
            .parse::<IpAddr>()
            .expect("error parsing IP address"),
        port: common::PORT,
        ..Default::default()
    };
    let figment = Figment::from(config).merge(("secret_key", &CONFIG.secret_key));

    let rocket = rocket::build()
        .configure(figment)
        .mount("/", FileServer::from(PUBLIC_DIR.clone()).rank(-1))
        .mount(
            "/",
            routes![
                load_map,
                tools,
                tools_dir,
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
                logout,
                create_nation,
                create_map
            ],
        )
        .attach(CORS)
        .ignite()
        .await
        .expect("Error igniting server");

    println!("available on {}", common::LOCAL_URL.as_str());

    let _rocket = rocket.launch().await.expect("Error launching server");
}
