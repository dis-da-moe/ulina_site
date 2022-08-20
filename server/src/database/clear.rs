use chrono::Duration as LongDuration;
use chrono::{DateTime, Utc};
use rocket::tokio::time::sleep;
use sqlx::query;
use std::time::Duration;

use crate::error::Error;

use super::db;

pub async fn clear() {
    let week = LongDuration::days(7);
    let pause_duration = Duration::from_secs(LongDuration::days(1).num_seconds() as u64);

    loop {
        sleep(pause_duration).await;
        let now = Utc::now();
        let week_ago = now - week;

        let message = match delete(week_ago).await {
            Ok(count) => format!("successfully cleared {} users", count),
            Err(e) => format!("an error occurred while clearing users: {}", e.to_string()),
        };

        println!("{}: {}", now, message);
    }
}

async fn delete(week_ago: DateTime<Utc>) -> Result<i32, Error> {
    let count = query!(
        "SELECT COUNT(*) as count FROM User WHERE lastVisit < ?",
        week_ago
    )
    .fetch_one(db())
    .await?
    .count;

    query!("DELETE FROM User WHERE lastVisit < ?", week_ago)
        .execute(db())
        .await?;

    Ok(count)
}
