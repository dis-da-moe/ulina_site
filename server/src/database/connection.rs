use once_cell::sync::OnceCell;
use rocket::tokio::spawn;
use sqlx::SqlitePool;

use super::{backup::backup, clear::clear};

const DATABASE_URL: &str = "sqlite:data/Ulina.db";

static CONNECTION: OnceCell<SqlitePool> = OnceCell::new();

pub async fn init() {
    spawn(clear());
    spawn(backup());
    CONNECTION
        .set(SqlitePool::connect(DATABASE_URL).await.unwrap())
        .unwrap();
    sqlx::migrate!().run(db()).await.unwrap();
}

pub fn db() -> &'static SqlitePool {
    CONNECTION.get().expect("database uninitialised")
}
