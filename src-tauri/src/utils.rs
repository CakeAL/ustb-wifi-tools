use anyhow::{anyhow, Result};
use std::{fs::create_dir, path::PathBuf};
use tauri::{ipc::Channel, Manager};

use crate::entities::{AppState, DownloadEvent};

#[cfg(target_os = "windows")]
pub fn get_windows_build_number() -> u32 {
    let version = windows_version::OsVersion::current();
    dbg!(version.build);
    version.build
}

pub fn get_config_path(app: &tauri::AppHandle) -> Result<PathBuf> {
    let mut path = get_store_path(app)?;
    path.push("config.json");
    Ok(path)
}

pub fn get_store_path(app: &tauri::AppHandle) -> Result<PathBuf> {
    match app.path().data_dir() {
        Ok(mut p) => {
            p.push("ustb-wifi-tools");
            if !p.exists() {
                // 如果不存在这个文件夹先创建
                create_dir(&p)?
            }
            Ok(p)
        }
        Err(e) => Err(anyhow!("There is no such app data dir: {e}")),
    }
}

pub async fn get_session_id(app_state: &tauri::State<'_, AppState>) -> Result<String, String> {
    app_state
        .jsessionid
        .read()
        .await
        .clone()
        .ok_or("是否已经点击登录校园网后台按钮？".to_string())
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
pub async fn update(
    app: tauri::AppHandle,
    manually: bool,
    on_event: Channel<DownloadEvent>,
) -> Result<(), String> {
    use tauri_plugin_dialog::DialogExt;
    use tauri_plugin_opener::OpenerExt;
    #[cfg(not(any(target_os = "android", target_os = "linux")))]
    use tauri_plugin_updater::UpdaterExt;

    if let Some(update) = app
        .updater()
        .map_err(|e| e.to_string())?
        .check()
        .await
        .map_err(|e| e.to_string())?
    {
        // 对话框
        let answer: bool = app
            .dialog()
            .message(format!(
                "{}->{}\n更新内容：{}",
                update.current_version,
                update.version,
                update.body.clone().unwrap_or_default()
            ))
            .title("有新版本！")
            .buttons(tauri_plugin_dialog::MessageDialogButtons::OkCancel)
            .blocking_show();

        if answer {
            on_event
                .send(DownloadEvent::Started { new_version: true })
                .unwrap();

            let mut downloaded = 0;
            update
                .download_and_install(
                    |chunk_length, content_length| {
                        downloaded += chunk_length;
                        // println!("downloaded {downloaded} from {content_length:?}");
                        on_event
                            .send(DownloadEvent::Progress {
                                downloaded,
                                content_length: content_length.unwrap_or_default(),
                            })
                            .unwrap();
                    },
                    || {
                        // println!("download finished");
                        on_event
                            .send(DownloadEvent::Finished { finished: true })
                            .unwrap();
                    },
                )
                .await
                .map_err(|e| e.to_string())?;
            // 添加查看 CHANGELOG
            let answer = app
                .dialog()
                .message("是否查看更新记录")
                .title("更新完成")
                .buttons(tauri_plugin_dialog::MessageDialogButtons::YesNo)
                .blocking_show();

            if answer {
                let _ = app.opener().open_url(
                    "https://github.com/CakeAL/ustb-wifi-tools/blob/main/CHANGELOG.md",
                    None::<&str>,
                );
            }
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

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_get_windows_build_number() {
        let res = get_windows_build_number();
        dbg!(res);
    }
}
