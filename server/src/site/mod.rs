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

use crate::config::CONFIG;
use crate::site::directories::{CURRENT_DIR, STATIC_DIR, PUBLIC_DIR};
use auth::{admin, admin_login, discord_login, login_result, logout, oauth_redirect};
use get::{get_user_data, load_map, nation, nation_changes, nations, page, tools};
use post::edit_nation;
pub async fn run() {
    initialize(&CURRENT_DIR);
    initialize(&STATIC_DIR);

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
        .mount(
            "/",
            FileServer::from(PUBLIC_DIR.clone()).rank(-1),
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
        .await
        .expect("Error igniting server");

    println!("available on {}", common::LOCAL_URL.as_str());

    let _rocket = rocket.launch().await.expect("Error launching server");
}
