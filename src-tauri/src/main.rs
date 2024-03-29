// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod creds_window;
mod fortinet;
mod handler;
mod setup;
mod worker;

use std::{error::Error, fs::File, io::Read};
use tauri::{Manager, SystemTrayEvent};
use tokio::sync::Mutex;

use handler::store_credentials;
use setup::generate_tray;
use worker::worker;

struct Credentials {
    worker:
        Mutex<Option<tauri::async_runtime::JoinHandle<Result<(), Box<dyn Error + Send + Sync>>>>>,
}

fn main() {
    let tray = generate_tray();

    tauri::Builder::default()
        .manage(Credentials {
            worker: Mutex::new(None),
        })
        .setup(|app| {
            let state: tauri::State<Credentials> = app.state();

            let app_data_path_buf = app.path_resolver().app_data_dir().unwrap();

            println!("Path: {:?}", app_data_path_buf);
            println!("Logging path: {:?}", app.path_resolver().app_log_dir().unwrap());
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
                        let j = tauri::async_runtime::spawn(worker(
                            username.to_string(),
                            password.to_string(),
                        ));
                        state.worker.blocking_lock().replace(j);
                    } else {
                        println!("No creds found");
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    creds_window::open_window(&app.app_handle());
                }
            }

            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(|app_handle, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "add_creds" => {
                    creds_window::open_window(app_handle);
                }

                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![store_credentials])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
