pub mod commands;
pub mod entities;
pub mod onedrive;
mod requests;
pub mod setting;
pub mod utils;

use std::sync::RwLock;

use crate::commands::*;
use crate::entities::AppState;
use crate::setting::Setting;
use onedrive::open_microsoft_login;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::new()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init());
    #[cfg(not(any(target_os = "android", target_os = "linux")))]
    {
        builder = builder.plugin(tauri_plugin_updater::Builder::new().build());
    }
    builder
        .manage(AppState {
            jsessionid: RwLock::new(None),
            setting: RwLock::new(Setting::default()),
            login_via_vpn: RwLock::new(false),
            onedrive_code_verifier: RwLock::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            get_cookie,
            load_refresh_account,
            load_user_flow_by_state,
            load_month_pay,
            load_user_login_log,
            load_mac_address,
            get_current_device_mac,
            do_unbind_macs,
            open_speed_test,
            load_ip_address,
            get_jsessionid,
            set_setting,
            load_setting,
            logout,
            get_cookie_vpn,
            load_monthly_login_log,
            manually_check_update,
            load_ammeter,
            load_user_flow,
            submit_login_ustb_wifi,
            return_os_type,
            set_background_image,
            reset_background_image,
            set_background_transparence,
            set_background_blur,
            set_mac_custom_name,
            collapse,
            open_microsoft_login,
            get_ip_location
        ])
        .setup(|app| {
            #[cfg(not(any(target_os = "android", target_os = "linux")))]
            {
                background_init(app)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
fn background_init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let win = app.get_webview_window("main").unwrap();

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
        use crate::utils::get_windows_build_number;
        if get_windows_build_number() >= 22000 {
            window_vibrancy::apply_mica(&win, None).map_err(|err| format!("启动错误: {}", err))?;
        }
    }

    Ok(())
}
