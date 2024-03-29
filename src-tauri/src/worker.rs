use tokio::time::{sleep, Duration};

use crate::fortinet;

// 10800 seconds is actual session timeout
const SLEEP_TIME_SECONDS: u64 = 10*60; // refresh every 10 minutes    

pub async fn worker(
    username: String,
    password: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("New worker started!");
    let mut tries = 3;
    loop {
        let res = fortinet::login(&username, &password).await;

        match res {
            Err(e) => {
                println!("Error: {}", e);
                if tries == 1 {
                    break Err(e);
                }
                tries -= 1;
                sleep(Duration::from_secs(1)).await;
                continue;
            }
            Ok(_) => {}
        }

        sleep(Duration::from_secs(SLEEP_TIME_SECONDS)).await;
    }
}

