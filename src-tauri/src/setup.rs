use std::{fs::File, io::Read};

use log::{error, info, warn};
use tauri::{App, Manager as _};

use crate::{creds_window, worker::worker, Credentials};

pub fn verify_creds_on_start(app: &mut App) {
    let state: tauri::State<Credentials> = app.state();

    let app_data_path_buf = app.path_resolver().app_data_dir().unwrap();

    info!("Path: {:?}", app_data_path_buf);
    info!(
        "Logging path: {:?}",
        app.path_resolver().app_log_dir().unwrap()
    );
    // std::fs::create_dir_all(&app_data_path_buf).unwrap();
    let app_data_path = app_data_path_buf.to_str().unwrap();

    let file_path = format!("{}/creds.txt", app_data_path);
    let file = File::open(file_path);
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let creds_vec: Vec<&str> = contents.split("\n").collect();
            let username = creds_vec.get(0);
            let password = creds_vec.get(1);

            if let (Some(username), Some(password)) = (username, password) {
                let j =
                    tauri::async_runtime::spawn(worker(username.to_string(), password.to_string()));
                state.worker.blocking_lock().replace(j);
            } else {
                warn!("No creds found");
            }
        }
        Err(e) => {
            error!("{:?}", e);
            creds_window::open_window(&app.app_handle());
        }
    }
}
