use tauri::AppHandle;

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
