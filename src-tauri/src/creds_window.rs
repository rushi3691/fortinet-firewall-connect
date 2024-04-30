use std::{fs::File, io::Read};

use log::{error, warn};
use tauri::{App, AppHandle, Manager as _};

pub fn open_window(app_handle: &AppHandle) {
    let creds_window = tauri::WindowBuilder::new(
        app_handle,
        "addcreds", /* the unique window label */
        tauri::WindowUrl::App("index.html".into()),
    )
    .title("fortinet-connect")
    .center()
    .content_protected(true)
    .build()
    .ok();

    if let Some(creds_window) = creds_window {
        creds_window.show().ok();
    }
}

pub fn get_stored_creds_with_app_handle(app: &AppHandle) -> Option<(String, String)> {
    let app_data_path_buf = app.path_resolver().app_data_dir().unwrap();
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
                return Some((username.to_string(), password.to_string()));
            } else {
                warn!("No creds found");
            }
        }
        Err(e) => {
            error!("{:?}", e);
            open_window(&app.app_handle());
        }
    }
    None
}

pub fn get_stored_creds_with_app(app: &App, app_data_path: &str) -> Option<(String, String)> {
    // let app_data_path_buf = app.path_resolver().app_data_dir().unwrap();
    // let app_data_path = app_data_path_buf.to_str().unwrap();
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
                return Some((username.to_string(), password.to_string()));
            } else {
                warn!("No creds found");
            }
        }
        Err(e) => {
            error!("{:?}", e);
            open_window(&app.app_handle());
        }
    }
    None
}
