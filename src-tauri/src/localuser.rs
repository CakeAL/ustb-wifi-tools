use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use std::fs::{create_dir, File};
use tauri::Manager;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use crate::entities::{AppState, UserLoginLog};
use crate::requests::get_user_login_log;
use crate::utils::{get_session_id, get_store_path};

pub struct Localuser {
    pub username: String,
}

impl Localuser {
    fn get_local_data_path(&self, app: &tauri::AppHandle) -> Result<PathBuf> {
        let mut path = get_store_path(app)?;
        path.push(format!("local_{}", self.username));
        if !path.exists() {
            create_dir(&path)?;
        }
        Ok(path)
    }

    // 获取输入年月至现在的数据
    pub async fn get_historical_data(
        &self,
        app: &tauri::AppHandle,
        start_date: i64,
    ) -> Result<String> {
        let app_state = app.state::<AppState>();
        let session_id = Arc::new(
            get_session_id(&app_state)
                .await
                .map_err(|err| anyhow!(err))?,
        );
        let via_vpn = *app_state.login_via_vpn.read().await;

        let current_date = Utc::now().date_naive();
        let mut start_date = DateTime::from_timestamp(start_date, 0)
            .unwrap()
            .date_naive();
        let path = Arc::new(self.get_local_data_path(app)?);

        while start_date <= current_date {
            let start_date_string = start_date.format("%Y-%m-%d").to_string();
            let end_date_string = get_last_day_of_month(&start_date)
                .format("%Y-%m-%d")
                .to_string();
            println!("{start_date_string} -> {end_date_string}");
            let session_id = session_id.clone();
            let path = path.clone();
            tokio::spawn(async move {
                let res =
                    get_user_login_log(&session_id, &start_date_string, &end_date_string, via_vpn)
                        .await;
                if let Ok(Some(data)) = res {
                    let mut file_path = (*path).clone();
                    file_path.push(format!("{}.json", start_date.format("%Y-%m")));
                    let json_str = serde_json::to_string(&data).unwrap_or_default();
                    let file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(file_path)
                        .await;
                    if let Ok(mut file) = file {
                        let _ = file.write_all(json_str.as_bytes()).await;
                    }
                }
            });
            start_date = get_first_day_next_month(&start_date);
        }
        Ok(path.to_string_lossy().to_string())
    }

    pub fn get_local_data(
        &self,
        app: &tauri::AppHandle,
        start_date: i64,
    ) -> Result<Option<UserLoginLog>> {
        let start_date_string = DateTime::from_timestamp(start_date, 0)
            .unwrap()
            .date_naive()
            .format("%Y-%m");
        let mut path = self.get_local_data_path(app)?;
        path.push(format!("{}.json", start_date_string));
        let mut json_file = File::open(path)?;
        let mut buf = String::new();
        json_file.read_to_string(&mut buf)?;
        Ok(serde_json::from_str(&buf)?)
    }
}

fn get_last_day_of_month(date: &NaiveDate) -> NaiveDate {
    let first_day_next_month = get_first_day_next_month(date);
    first_day_next_month.pred_opt().unwrap()
}

fn get_first_day_next_month(date: &NaiveDate) -> NaiveDate {
    let (year, month) = (date.year(), date.month());
    let first_day_next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    };
    first_day_next_month.unwrap()
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[tokio::test]
    // async fn test_gen_dates() {
    //     let timestamp = 1627776000;
    //     let res = Localuser::get_historical_data(timestamp).await;
    //     dbg!(res.unwrap());
    // }
}
