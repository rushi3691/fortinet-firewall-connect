// fn setup: Box<dyn FnOnce(&mut App<R>) -> Result<(), Box<dyn Error, Global>> + Send, Global>
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};


// fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error, Global>> {
//     let state: tauri::State<Credentials> = app.state();

//     let path = app.handle().path_resolver().app_data_dir().unwrap();
//     println!("Path: {:?}", path);
//     let file_path = format!("{}/creds.txt", path.to_str().unwrap());
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
//             } else {
//                 println!("No creds found");
//             }
//         }
//         Err(e) => {
//             println!("Error: {:?}", e);
//         }
//     }

//     Ok(())
// }



// generate tray
pub fn generate_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let add = CustomMenuItem::new("add_creds".to_string(), "Add Credentials");
    let tray_menu = SystemTrayMenu::new().add_item(quit).add_item(add);
    SystemTray::new().with_menu(tray_menu)
}