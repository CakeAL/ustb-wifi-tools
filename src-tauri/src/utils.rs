use anyhow::{anyhow, Result};
use chrono::{Datelike, Local};
use std::{fs::create_dir, path::PathBuf};
use tauri::{ipc::Channel, Manager};

use crate::{
    entities::{AppState, DownloadEvent, MonthPayInfo, MonthlyData, UserLoginLog, UserType},
    requests::get_user_login_log,
};

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

#[inline]
pub async fn get_session_id(app_state: &tauri::State<'_, AppState>) -> Result<String, String> {
    app_state
        .jsessionid
        .read()
        .await
        .clone()
        .ok_or("是否已经点击登录校园网后台按钮？".to_string())
}

// 用来补全获取校园网后台年度使用数据缺失的数据
pub async fn complete_month_pay_data(
    month_pay_info: &mut MonthPayInfo,
    year: u16,
    session_id: &str,
    user_type: UserType,
) {
    // 如果 year 大于 今年，手动加载去年 12 月的数据进来
    let this_year = Local::now().year() as u16;
    if this_year > year {
        let start_date = format!("{}-12-01", year);
        let end_date = format!("{}-12-31", year);
        let dec_data = match get_user_login_log(session_id, &start_date, &end_date, user_type).await
        {
            Ok(Some(v)) => v,
            _ => UserLoginLog::default(),
        };
        month_pay_info.year_cost += dec_data.cost;
        month_pay_info.year_used_duration += dec_data.used_duration;
        month_pay_info.year_used_flow += dec_data.used_flow;
        month_pay_info.monthly_data.push(MonthlyData {
            month: 12,
            month_cost: dec_data.cost,
            month_used_flow: dec_data.used_flow,
            month_used_duration: dec_data.used_duration,
        });
        // 前端应该改成，超过当年 1 月之后，才显示今年数据，否则是去年数据
    }

    // 如果是2023年，手动获取前8个月的数据
    if year == 2023 {
        month_pay_info.monthly_data.drain(0..8);
        let mut handles = vec![];
        for i in 0..8 {
            let session_id = session_id.to_string();
            let month = i + 1;
            let handle = tokio::spawn(async move {
                let start_date = format!("2023-{:02}-01", month);
                let end_date = format!("2023-{:02}-31", month);
                (
                    month,
                    get_user_login_log(&session_id, &start_date, &end_date, user_type).await,
                )
            });
            handles.push(handle);
        }
        for handle in handles {
            let (month, data) = handle.await.unwrap();
            let data = match data {
                Ok(Some(v)) => v,
                _ => UserLoginLog::default(),
            };
            month_pay_info.monthly_data.insert(
                month - 1,
                MonthlyData {
                    month: month as u8,
                    month_cost: data.cost,
                    month_used_flow: data.used_flow,
                    month_used_duration: data.used_duration,
                },
            );
            month_pay_info.year_cost += data.cost;
            month_pay_info.year_used_flow += data.used_flow;
        }
    }
    // 如果是 2022 年，手动获取 6 ～ 11 月数据（前面12月已经获取完了）
    else if year == 2022 {
        month_pay_info.monthly_data.drain(5..11);
        let mut handles = vec![];
        for i in 5..11 {
            let session_id = session_id.to_string();
            let month = i + 1;
            let handle = tokio::spawn(async move {
                let start_date = format!("2022-{:02}-01", month);
                let end_date = format!("2022-{:02}-31", month);
                (
                    month,
                    get_user_login_log(&session_id, &start_date, &end_date, user_type).await,
                )
            });
            handles.push(handle);
        }
        for handle in handles {
            let (month, data) = handle.await.unwrap();
            let data = match data {
                Ok(Some(v)) => v,
                _ => UserLoginLog::default(),
            };
            month_pay_info.monthly_data.insert(
                month - 1,
                MonthlyData {
                    month: month as u8,
                    month_cost: data.cost,
                    month_used_flow: data.used_flow,
                    month_used_duration: data.used_duration,
                },
            );
            month_pay_info.year_cost += data.cost;
            month_pay_info.year_used_flow += data.used_flow;
        }
    }
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
