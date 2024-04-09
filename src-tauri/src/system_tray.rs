use log::info;
use tauri::{App, AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_autostart::ManagerExt as _;

use crate::creds_window;

pub fn generate_tray() -> SystemTray {
    let add = CustomMenuItem::new("add_creds".to_string(), "Add Credentials");
    let enable_auto_start =
        CustomMenuItem::new("enable_auto_start".to_string(), "Enable Auto Start");
    let disable_auto_start =
        CustomMenuItem::new("disable_auto_start".to_string(), "Disable Auto Start");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(add)
        .add_item(enable_auto_start)
        .add_item(disable_auto_start)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

pub fn handle_system_tray_events(app_handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app_handle.tray_handle().get_item(&id);
            match id.as_str() {
                "add_creds" => {
                    creds_window::open_window(app_handle);
                }
                "enable_auto_start" => {
                    info!("enable_auto_start");
                    let autostart_manager = app_handle.autolaunch();
                    let _ = autostart_manager.enable();
                    info!(
                        "registered for autostart? {:?}",
                        autostart_manager.is_enabled()
                    );
                    // item_handle.set_title("Disable Auto Start").unwrap();
                    item_handle.set_enabled(false).unwrap();

                    let disable_auto_start_item =
                        app_handle.tray_handle().get_item("disable_auto_start");
                    disable_auto_start_item.set_enabled(true).unwrap();
                }
                "disable_auto_start" => {
                    info!("disable_auto_start");
                    let autostart_manager = app_handle.autolaunch();
                    let _ = autostart_manager.disable();
                    info!(
                        "registered for autostart? {:?}",
                        autostart_manager.is_enabled()
                    );
                    item_handle.set_enabled(false).unwrap();
                    let enable_auto_start_item =
                        app_handle.tray_handle().get_item("enable_auto_start");
                    enable_auto_start_item.set_enabled(true).unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }

                _ => {}
            }
        }
        _ => {}
    }
}


pub fn configure_tray_with_autostart(app: &mut App) {
    let autostart_manager = app.autolaunch();
    let is_enabled = autostart_manager.is_enabled().unwrap();

    let tray = app.tray_handle();
    let enable_auto_start_item = tray.get_item("enable_auto_start");
    let disable_auto_start_item = tray.get_item("disable_auto_start");
    if is_enabled {
        enable_auto_start_item.set_enabled(false).unwrap();
        disable_auto_start_item.set_enabled(true).unwrap();
    } else {
        enable_auto_start_item.set_enabled(true).unwrap();
        disable_auto_start_item.set_enabled(false).unwrap();
    }
}