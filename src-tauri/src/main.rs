// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::RwLock;

use ustb_wifi_tools::commands::*;
use ustb_wifi_tools::entities::AppState;

fn main() {
    tauri::Builder::default()
        .manage(AppState{
            jsessionid: RwLock::new(None),
            account: RwLock::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            load_user_flow,
            get_cookie,
            load_refresh_account,
            open_nav_login,
            load_user_flow_by_state,
            load_month_pay,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
