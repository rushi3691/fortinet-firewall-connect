use tokio::time::{sleep, Duration};

use crate::fortinet;

const SLEEP_TIME: u64 = 10; // 10750

pub async fn worker(
    username: String,
    password: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("New worker started!");
    let mut tries = 3;
    loop {
        let res = fortinet::login(&username, &password).await;

        match res {
            Ok(_) => println!("Logged in!"),
            Err(e) => {
                println!("Error: {}", e);
                if tries == 1 {
                    break Ok(());
                }
                tries -= 1;
                sleep(Duration::from_secs(1)).await;
                continue;
            }
        }

        sleep(Duration::from_secs(SLEEP_TIME)).await;
    }
}

