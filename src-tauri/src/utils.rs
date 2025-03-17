use anyhow::{anyhow, Result};
use std::{fs::create_dir, path::PathBuf};
use tauri::Manager;

use crate::entities::AppState;

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
