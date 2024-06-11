// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use ustb_wifi_tools::commands::*;
use ustb_wifi_tools::entities::JsessionId;

fn main() {
    tauri::Builder::default()
        .manage(JsessionId(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            load_user_flow,
            get_cookie,
            load_refresh_account
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
