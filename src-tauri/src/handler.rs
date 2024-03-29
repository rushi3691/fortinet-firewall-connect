use std::{fs::File, io::Write};

use tauri::Manager;

use crate::{fortinet, worker::worker, Credentials};

#[tauri::command]
pub async fn store_credentials(
    app_handle: tauri::AppHandle,
    username: String,
    password: String,
) -> Result<String, String> {
    let s = format!("{}\n{}", username, password);
    println!("{}", s);
    
    // check credentials valid or not, by logging in
    fortinet::login(&username, &password).await.or(Err("Login failed"))?;

    // write to file
    let app_data_path = app_handle.path_resolver().app_data_dir().unwrap();
    // print if path does not exist
    let exists = app_data_path.exists();
    if !exists {
        println!("Path does not exist: {:?}", app_data_path);
        // create it
        std::fs::create_dir_all(&app_data_path).unwrap();
    }
    
    let app_data_dir = app_data_path.to_str().unwrap();
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

    let response = format!("Stored credentials for user: {}", username);
    Ok(response)
}
