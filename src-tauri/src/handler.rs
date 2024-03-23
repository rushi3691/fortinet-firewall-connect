use std::{fs::File, io::Write};

use tauri::Manager;

use crate::{worker::worker, Credentials};



#[tauri::command]
pub async fn store_credentials(
    app_handle: tauri::AppHandle,
    username: String,
    password: String,
) -> Result<String, ()> {
    let s = format!("{}\n{}", username, password);
    println!("{}", s);

    // write to file
    let binding = app_handle.path_resolver().app_data_dir().unwrap();
    let app_data_dir = binding.to_str().unwrap();
    // println!("App data dir: {:?}", app_data_dir);
    let file_path = format!("{}/creds.txt", app_data_dir);
    let mut file = File::create(file_path).unwrap();
    file.write_all(s.as_bytes()).unwrap();

    // update state
    let state: tauri::State<Credentials> = app_handle.state();

    // stop old worker
    let old_worker = state.worker.lock().await.take();
    if let Some(j) = &old_worker {
        println!("Aborting old worker");
        j.abort();
    }

    // start new worker
    let j = tauri::async_runtime::spawn(worker(username.to_string(), password.to_string()));
    state.worker.lock().await.replace(j);

    Ok(s)
}
