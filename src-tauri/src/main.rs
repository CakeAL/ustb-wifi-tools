// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::RwLock;

use ustb_wifi_tools::commands::*;
use ustb_wifi_tools::entities::AppState;
use ustb_wifi_tools::setting::Setting;

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            jsessionid: RwLock::new(None),
            setting: RwLock::new(Setting::default()),
        })
        .invoke_handler(tauri::generate_handler![
            load_user_flow,
            get_cookie,
            load_refresh_account,
            open_nav_login,
            load_user_flow_by_state,
            load_month_pay,
            load_user_login_log,
            load_mac_address,
            get_current_device_mac,
            do_unbind_macs,
            open_speed_test,
            set_browser_path,
            check_has_browser,
            load_ip_address,
            get_jsessionid,
            set_setting,
            load_setting,
            logout
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
