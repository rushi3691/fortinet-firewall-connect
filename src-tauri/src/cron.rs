use cron::Schedule;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

use crate::fortinet;

// cron = "0.5.1"
async fn worker(username: String, password: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut tries = 3;
    let res = fortinet::login(&username, &password).await;

    match res {
        Ok(_) => println!("Logged in!"),
        Err(e) => {
            println!("Error: {}", e);
            if tries == 1 {
                return Ok(());
            }
            tries -= 1;
            sleep(Duration::from_secs(1)).await;
        }
    }

    Ok(())
}

async fn cron() {
    // let tray = get_tray();
    let username = "your_username".to_string();
    let password = "your_password".to_string();

    // Parse a cron schedule
    let schedule = Schedule::from_str("0 0 * * * *").unwrap(); // This will run the job every hour

    let mut now = chrono::Utc::now();
    for t in schedule.upcoming(chrono::Utc) {
        if let Some(difference) = t.signed_duration_since(now).to_std().ok() {
            sleep(Duration::from_secs(difference.as_secs())).await;
            let _ = worker(username.clone(), password.clone()).await;
        }
        now = chrono::Utc::now();
    }
}