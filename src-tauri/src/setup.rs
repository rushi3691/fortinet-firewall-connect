use std::
    io::ErrorKind
;

use log::{debug, info, warn};
use std::io::Error;
use tauri::{App, Manager};

use crate::{
    creds_window::get_stored_creds_with_app,
    worker::start_and_replace_worker,
};

// pub fn verify_creds_on_start(app: &mut App) {
//     let state: tauri::State<Credentials> = app.state();

//     let app_data_path_buf = app.path_resolver().app_data_dir().unwrap();

//     info!("Path: {:?}", app_data_path_buf);
//     info!(
//         "Logging path: {:?}",
//         app.path_resolver().app_log_dir().unwrap()
//     );
//     // std::fs::create_dir_all(&app_data_path_buf).unwrap();
//     let app_data_path = app_data_path_buf.to_str().unwrap();

//     let file_path = format!("{}/creds.txt", app_data_path);
//     let file = File::open(file_path);
//     match file {
//         Ok(mut file) => {
//             let mut contents = String::new();
//             file.read_to_string(&mut contents).unwrap();
//             let creds_vec: Vec<&str> = contents.split("\n").collect();
//             let username = creds_vec.get(0);
//             let password = creds_vec.get(1);

//             if let (Some(username), Some(password)) = (username, password) {
//                 let j =
//                     tauri::async_runtime::spawn(worker(username.to_string(), password.to_string()));
//                 state.worker.blocking_lock().replace(j);
//                 // start_and_replace_worker(
//                 //     &app.app_handle(),
//                 //     &username.to_string(),
//                 //     &password.to_string(),
//                 // );
//             } else {
//                 warn!("No creds found");
//             }
//         }
//         Err(e) => {
//             error!("{:?}", e);
//             creds_window::open_window(&app.app_handle());
//         }
//     }
// }

pub fn verify_creds_on_start_v2(app: &mut App) {
    let path = check_or_create_app_data_path_with_app(app).unwrap();
    if let Some((username, password)) = get_stored_creds_with_app(app, &path) {
        tauri::async_runtime::block_on(start_and_replace_worker(
            &app.app_handle(),
            &username,
            &password,
        ));
    } else {
        warn!("No creds found");
    }
}

fn check_or_create_app_data_path_with_app(app: &App) -> Result<String, Error> {
    let app_data_path_buf = app.path_resolver().app_data_dir();
    match app_data_path_buf {
        Some(path) => {
            info!("Path: {:?}", path);
            // info!(
            //     "Logging path: {:?}",
            //     app.path_resolver().app_log_dir().unwrap()
            // );
            let exists = path.exists();
            if !exists {
                warn!("Path does not exist: {:?}", path);
                // create the path
                debug!("Creating path: {:?}", path);
                std::fs::create_dir_all(&path)?;
            }
            let app_data_path = path.to_str();
            match app_data_path {
                Some(path) => Ok(path.to_string()),
                None => Err(Error::new(
                    ErrorKind::Other,
                    "Failed to convert path to string",
                )),
            }
        }
        None => Err(Error::new(ErrorKind::Other, "Failed to get path")),
    }
}

// fn check_or_create_app_data_path_with_app_handle(app: &AppHandle) -> Result<String, Error> {
//     let app_data_path_buf = app.path_resolver().app_data_dir();
//     match app_data_path_buf {
//         Some(path) => {
//             info!("Path: {:?}", path);
//             // info!(
//             //     "Logging path: {:?}",
//             //     app.path_resolver().app_log_dir().unwrap()
//             // );
//             let exists = path.exists();
//             if !exists {
//                 warn!("Path does not exist: {:?}", path);
//                 // create the path
//                 debug!("Creating path: {:?}", path);
//                 std::fs::create_dir_all(&path)?;
//             }
//             let app_data_path = path.to_str();
//             match app_data_path {
//                 Some(path) => Ok(path.to_string()),
//                 None => Err(Error::new(
//                     ErrorKind::Other,
//                     "Failed to convert path to string",
//                 )),
//             }
//         }
//         None => Err(Error::new(ErrorKind::Other, "Failed to get path")),
//     }
// }
