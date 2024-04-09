// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod creds_window;
mod fortinet;
mod handler;
mod setup;
mod system_tray;
mod worker;

use std::error::Error;
use tokio::sync::Mutex;
// use sys_info::{PowerStatus, SystemExt};
use tauri_plugin_autostart::MacosLauncher;

use handler::store_credentials;
// use setup::{generate_tray, setup_tray_with_autostart};

struct Credentials {
    worker:
        Mutex<Option<tauri::async_runtime::JoinHandle<Result<(), Box<dyn Error + Send + Sync>>>>>,
}

fn main() {
    let tray = system_tray::generate_tray();

    let mut _app = tauri::Builder::default()
        .manage(Credentials {
            worker: Mutex::new(None),
        })
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .system_tray(tray)
        .on_system_tray_event(system_tray::handle_system_tray_events)
        .setup(|app| {
            system_tray::configure_tray_with_autostart(app);
            setup::verify_creds_on_start(app);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![store_credentials])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    #[cfg(target_os = "macos")]
    _app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    _app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
