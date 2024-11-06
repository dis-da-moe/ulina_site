use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub admin: String,
    pub client_id: u64,
    pub discord_token: String,
    pub client_secret: String,
    pub guild_id: u64,
    pub admin_id: u64,
    pub admin_role_id: u64,
    pub secret_key: String,
    pub redirect: String,
    pub database_url: String,
    pub google_key: String,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        dotenv::dotenv().unwrap();
        envy::from_env().unwrap()
    };
}
