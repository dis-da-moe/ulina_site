use common::Social;
use sqlx::{SqlitePool, query_as};
use once_cell::sync::OnceCell;

use crate::error::Error;

const DATABASE_URL: &str = "sqlite:data/Ulina.db";

static CONNECTION: OnceCell<SqlitePool> = OnceCell::new();

pub async fn init() {
    CONNECTION
        .set(SqlitePool::connect(DATABASE_URL).await.unwrap())
        .unwrap();
    sqlx::migrate!().run(db()).await.unwrap();
}

pub fn db() -> &'static SqlitePool {
    CONNECTION.get().expect("database uninitialised")
}

pub async fn socials(id: i64) -> Result<Vec<Social>, Error> {
    query_as!(Social, "SELECT * FROM Social WHERE nationId = ?", id)
        .fetch_all(db())
        .await
        .map_err(|err| err.into())
}