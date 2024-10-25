pub mod commands;
pub mod entities;
mod requests;
pub mod setting;
pub mod utils;

use std::sync::RwLock;

use crate::commands::*;
use crate::entities::AppState;
use crate::setting::Setting;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_updater::UpdaterExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(AppState {
            jsessionid: RwLock::new(None),
            setting: RwLock::new(Setting::default()),
            login_via_vpn: RwLock::new(false),
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
            manually_check_update
        ])
        .setup(|app| {
            background_init(app)?;
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let _ = update(handle, false).await;
            });
            Ok(())
        })
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
        use crate::utils::get_windows_build_number;
        if get_windows_build_number()? >= 22000 {
            window_vibrancy::apply_mica(&win, None).map_err(|err| format!("启动错误: {}", err))?;
        } else {
            window_vibrancy::apply_blur(&win, Some((18, 18, 18, 125)))
                .map_err(|err| format!("启动错误: {}", err))?;
        }
    }

    Ok(())
}

async fn update(app: tauri::AppHandle, manually: bool) -> anyhow::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        // 对话框
        let answer = app
            .dialog()
            .message(format!(
                "有新版本！{}->{}\n是否更新？",
                update.current_version, update.version
            ))
            .title("貌似有版本更新？")
            .buttons(tauri_plugin_dialog::MessageDialogButtons::OkCancel)
            .blocking_show();

        if answer {
            let mut downloaded = 0;
            update
                .download_and_install(
                    |chunk_length, content_length| {
                        downloaded += chunk_length;
                        println!("downloaded {downloaded} from {content_length:?}");
                    },
                    || {
                        println!("download finished");
                    },
                )
                .await?;
            app.dialog()
                .message("下载完成，点击重启")
                .kind(tauri_plugin_dialog::MessageDialogKind::Info)
                .title("这是个提示框")
                .buttons(tauri_plugin_dialog::MessageDialogButtons::Ok)
                .blocking_show();
            println!("update installed");
            app.restart();
        }
    } else if manually {
        app.dialog()
            .message("没有更新😭")
            .kind(tauri_plugin_dialog::MessageDialogKind::Info)
            .title("这是个提示框")
            .buttons(tauri_plugin_dialog::MessageDialogButtons::Ok)
            .blocking_show();
    }

    Ok(())
}
