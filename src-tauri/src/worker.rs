use crate::{fortinet, Credentials};
use chrono::Utc;
use log::{error, info};
use tauri::Manager as _;
use tokio::time::{interval, Duration};

// 10800 seconds is actual session timeout
const SLEEP_TIME_SECONDS: u64 = 1 * 60 * 60; // refresh every 1 hour
const CHECK_INTERVAL_SECONDS: u64 = 5; // check every 5 seconds
const MAX_RETRIES: u32 = 5;

pub async fn worker(
    username: String,
    password: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("New worker started!");
    let mut tries = MAX_RETRIES;
    let mut next_login = Utc::now().timestamp() as u64;

    let mut interval = interval(Duration::from_secs(CHECK_INTERVAL_SECONDS));
    loop {
        let now = Utc::now().timestamp() as u64;
        if now >= next_login {
            let res = fortinet::login(&username, &password).await;

            match res {
                Err(e) => {
                    error!("{}", e);
                    if tries == 0 {
                        break Err(e);
                    }
                    tries -= 1;
                    info!(
                        "Retry {}/{}...Retrying in 5 seconds",
                        MAX_RETRIES - tries,
                        MAX_RETRIES
                    );
                }
                Ok(_) => {
                    tries = MAX_RETRIES;
                    next_login = now + SLEEP_TIME_SECONDS;
                }
            }
        }

        interval.tick().await;
    }
}

pub async fn start_and_replace_worker(app_handle: &tauri::AppHandle, username: &String, password: &String) {
    let state: tauri::State<Credentials> = app_handle.state();

    // stop old worker
    let old_worker = state.worker.lock().await.take();
    if let Some(j) = &old_worker {
        info!("Aborting old worker");
        j.abort();
    }

    // start new worker
    info!("Starting new worker");
    let j = tauri::async_runtime::spawn(worker(username.to_string(), password.to_string()));
    state.worker.lock().await.replace(j);
}
