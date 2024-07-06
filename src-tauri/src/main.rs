// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::RwLock;

use tauri::Manager;
use ustb_wifi_tools::commands::*;
use ustb_wifi_tools::entities::AppState;
use ustb_wifi_tools::utils::{get_browser_path, open_headless_browser};

fn main() {
    let app_state = AppState {
        jsessionid: RwLock::new(None),
        account: RwLock::new(None),
        browser: RwLock::new(None),
        tab: RwLock::new(None),
        browser_state: RwLock::new(true), // 找到浏览器
    };
    match get_browser_path() {
        Some(path) => {
            let (b, t) = open_headless_browser(path).unwrap();
            *app_state.browser.write().unwrap() = Some(b);
            *app_state.tab.write().unwrap() = Some(t);
        }
        None => *app_state.browser_state.write().unwrap() = false, // 没找到浏览器
    }

    tauri::Builder::default()
        .manage(app_state)
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
            check_browser_state,
            set_browser_path
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            if let tauri::RunEvent::WindowEvent { event, .. } = event {
                if let tauri::WindowEvent::CloseRequested { .. } = event {
                    // 点击关闭按钮时，关闭 headless 浏览器
                    let app_state = app.state::<AppState>();
                    if app_state.browser.read().unwrap().is_some() {
                        let b = app_state.browser.write().unwrap().take();
                        if b.is_some() {
                            drop(b);
                        }
                    }
                }
            }
        });
}
