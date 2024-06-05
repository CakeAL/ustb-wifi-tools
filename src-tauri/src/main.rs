// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ustb_wifi_tools::commands::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_user_flow, get_cookie])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
