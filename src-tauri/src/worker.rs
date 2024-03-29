use tokio::time::{sleep, Duration};

use crate::fortinet;

const SLEEP_TIME: u64 = 10700; // 10800 seconds is actual session timeout

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

        sleep(Duration::from_secs(SLEEP_TIME)).await;
    }
}

