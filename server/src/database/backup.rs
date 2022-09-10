use chrono::{DateTime, Duration, Utc};
use common::DATE_TIME_FORMAT;
use rocket::tokio::time::sleep;
use rocket::{
    futures::TryFutureExt,
    tokio::fs,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Backup {
    last_back_up: DateTime<Utc>,
}
const BACKUP_PATH: &str = "./data/backup.json";
lazy_static! {
    static ref INTERVAL: Duration = Duration::days(7);
}

pub async fn backup() {
    let now = Utc::now();
    let till_update = fs::read_to_string(BACKUP_PATH)
        .await
        .ok()
        .and_then(|contents| serde_json::from_str::<Backup>(&contents).ok())
        .map(|json| json.last_back_up - now + *INTERVAL)
        .unwrap_or_else(|| Duration::zero());

    println!(
        "next backup scheduled for {}",
        (now + till_update).format(DATE_TIME_FORMAT)
    );

    sleep(till_update.to_std().unwrap()).await;

    loop {
        let now = Utc::now();
        println!("{} - backing up", now.format(DATE_TIME_FORMAT));

        let result = fs::copy(
            "./data/Ulina.db",
            format!("./data/backups/Ulina_{}.db", now.format(DATE_TIME_FORMAT)),
        )
        .await;

        match result {
            Ok(_) => println!("successful backup"),
            Err(e) => println!("error backing up database: {}", e.to_string()),
        };

        let result = fs::write(
            BACKUP_PATH,
            serde_json::to_string(&Backup { last_back_up: now }).unwrap(),
        )
        .await;

        match result {
            Err(e) => println!("error saving backup date: {}", e.to_string()),
            _ => {}
        }

        println!(
            "next backup scheduled for {}",
            (now + *INTERVAL).format(DATE_TIME_FORMAT)
        );

        sleep(INTERVAL.to_std().unwrap()).await;
    }
}
