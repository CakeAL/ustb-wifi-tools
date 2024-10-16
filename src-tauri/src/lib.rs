pub mod commands;
pub mod entities;
mod requests;
pub mod setting;
pub mod utils;

use std::sync::RwLock;

use tauri::Manager;
use crate::commands::*;
use crate::entities::AppState;
use crate::setting::Setting;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            jsessionid: RwLock::new(None),
            setting: RwLock::new(Setting::default()),
            login_via_vpn: RwLock::new(false),
        })
        .invoke_handler(tauri::generate_handler![
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
            logout,
            get_cookie_vpn,
        ])
        .setup(background_init)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn background_init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let win = app.get_webview_window("main").unwrap();
    win.set_decorations(true).unwrap();

    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(
        &win,
        window_vibrancy::NSVisualEffectMaterial::Sidebar,
        Some(window_vibrancy::NSVisualEffectState::Active),
        None,
    )
    .map_err(|err| format!("启动错误: {}", err))?;

    #[cfg(target_os = "windows")]
    {
        use ustb_wifi_tools::utils::get_windows_build_number;
        if get_windows_build_number()? >= 22000 {
            window_vibrancy::apply_mica(&win, None).map_err(|err| format!("启动错误: {}", err))?;
        } else {
            window_vibrancy::apply_blur(&win, Some((18, 18, 18, 125)))
                .map_err(|err| format!("启动错误: {}", err))?;
        }
    }

    Ok(())
}
