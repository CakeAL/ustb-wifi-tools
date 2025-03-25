pub mod commands;
pub mod entities;
pub mod onedrive;
mod requests;
pub mod setting;
pub mod utils;
pub mod localuser;

use crate::commands::*;
use crate::entities::AppState;
use onedrive::open_microsoft_login;
#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;
use tauri::{WebviewUrl, WebviewWindow, WebviewWindowBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::new()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init());
    #[cfg(not(any(target_os = "android", target_os = "linux")))]
    {
        builder = builder.plugin(tauri_plugin_updater::Builder::new().build());
    }
    builder
        .manage(AppState::default())
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
            load_setting,
            logout,
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
            get_ip_location,
            switch_login_ustb_wifi,
            get_current_user_name,
            set_current_user_name,
            create_local_user,
            down_historical_data
        ])
        .setup(|app| {
            // {
            //     "label": "main",
            //     "title": "USTB Wifi Tools",
            //     "width": 800,
            //     "height": 600,
            //     "minHeight": 600,
            //     "minWidth": 800,
            //     "transparent": true
            // }
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("USTB Wifi Tools")
                .inner_size(800.0, 600.0)
                .min_inner_size(800.0, 600.0)
                .transparent(true);
            #[cfg(target_os = "macos")]
            let win_builder = win_builder.title_bar_style(TitleBarStyle::Overlay);
            let window = win_builder.build().unwrap();
            #[cfg(not(any(target_os = "android", target_os = "linux")))]
            {
                background_init(&window)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
fn background_init(win: &WebviewWindow) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        // use cocoa::appkit::{NSColor, NSWindow};
        // use cocoa::base::{id, nil};

        // let ns_window = win.ns_window().unwrap() as id;
        // unsafe {
        //     let bg_color = NSColor::colorWithRed_green_blue_alpha_(
        //         nil,
        //         50.0 / 255.0,
        //         158.0 / 255.0,
        //         163.5 / 255.0,
        //         0.0,
        //     );
        //     ns_window.setBackgroundColor_(bg_color);
        // }
        let _ = win.set_title("");
        window_vibrancy::apply_vibrancy(
            win,
            window_vibrancy::NSVisualEffectMaterial::Sidebar,
            Some(window_vibrancy::NSVisualEffectState::Active),
            None,
        )
        .map_err(|err| format!("启动错误: {}", err))?;
    }

    #[cfg(target_os = "windows")]
    {
        use crate::utils::get_windows_build_number;
        if get_windows_build_number() >= 22000 {
            window_vibrancy::apply_mica(&win, None).map_err(|err| format!("启动错误: {}", err))?;
        }
    }

    Ok(())
}
