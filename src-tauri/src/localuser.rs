use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use std::fs::{create_dir, File};
use tauri::Manager;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use crate::entities::{AppState, UserLoginLog, UserType};
use crate::requests::get_user_login_log;
use crate::utils::{get_session_id, get_store_path};

#[derive(Debug, Clone)]
pub enum CurrentUser {
    OnlineUser(String),
    LocalUser(String),
}

impl Default for CurrentUser {
    fn default() -> Self {
        CurrentUser::OnlineUser("".to_string())
    }
}

impl CurrentUser {
    pub async fn new_local_user(app: &tauri::AppHandle) -> Result<String> {
        let app_state = app.state::<AppState>();
        let cur_account = match app_state.cur_account.read().await.clone() {
            CurrentUser::OnlineUser(username) => username,
            CurrentUser::LocalUser(_) => return Err(anyhow!("请登陆在线账户再创建本地账户")),
        };
        let local_username = format!("local_{cur_account}");
        if app_state
            .setting
            .read()
            .await
            .has_local_account(&local_username)
        {
            return Err(anyhow!("该本地账户已存在"));
        }
        app_state
            .setting
            .write()
            .await
            .set_account(local_username, "".to_string());
        let _ = app_state.setting.write().await.write_setting(app);
        Ok("创建本地账户成功".to_string())
    }

    fn get_local_data_path(&self, app: &tauri::AppHandle) -> Result<PathBuf> {
        let mut path = get_store_path(app)?;
        match self {
            CurrentUser::LocalUser(username) => path.push(username),
            CurrentUser::OnlineUser(username) => path.push(format!("local_{username}")),
        }
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
    ) -> Result<Vec<String>> {
        let app_state = app.state::<AppState>();
        let session_id = Arc::new(
            get_session_id(&app_state)
                .await
                .map_err(|err| anyhow!(err))?,
        );
        let user_type = *app_state.user_type.read().await;
        if let UserType::LocalUser = user_type {
            return Err(anyhow!("本地存储不适用此功能"));
        }

        let current_date = Utc::now().date_naive();
        let mut start_date = DateTime::from_timestamp(start_date, 0)
            .unwrap()
            .date_naive();
        let path = Arc::new(self.get_local_data_path(app)?);
        let mut tasks = vec![];

        while start_date <= current_date {
            let start_date_string = start_date.format("%Y-%m-%d").to_string();
            let end_date_string = get_last_day_of_month(&start_date)
                .format("%Y-%m-%d")
                .to_string();
            // println!("{start_date_string} -> {end_date_string}");
            let session_id = session_id.clone();
            let path = path.clone();
            let task = tokio::spawn(async move {
                let res = get_user_login_log(
                    &session_id,
                    &start_date_string,
                    &end_date_string,
                    user_type,
                )
                .await;

                match res {
                    Ok(Some(data)) => {
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
                        Ok(())
                    }
                    Ok(None) => Ok(()),
                    Err(e) => Err(anyhow!(
                        "{} 获取失败，原因：{}",
                        start_date.format("%Y-%m"),
                        e.to_string()
                    )),
                }
            });
            tasks.push(task);
            start_date = get_first_day_next_month(&start_date);
        }
        let mut res = vec![];
        res.push(format!("存储于：{}", path.to_string_lossy()));
        for task in tasks {
            if let Err(e) = task.await? {
                // println!("{}", e);
                res.push(e.to_string());
            }
        }
        println!("finished");
        Ok(res)
    }

    pub fn get_local_data(
        &self,
        app: &tauri::AppHandle,
        start_date: i64,
        end_date: Option<i64>,
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
        let value = serde_json::from_str::<UserLoginLog>(&buf)?;
        if let Some(mut end_date) = end_date {
            if end_date == start_date {
                end_date += 24 * 3600;
            }
            let (ipv4_up, ipv4_down, ipv6_up, ipv6_down, used_flow, cost, used_duration, every) =
                value
                    .every_login_data
                    .into_iter()
                    .filter(|data| data.offline_time >= start_date && data.offline_time < end_date)
                    .fold(
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0, Vec::new()), // 初始值
                        |(
                            ipv4_up,
                            ipv4_down,
                            ipv6_up,
                            ipv6_down,
                            used_flow,
                            cost,
                            used_duration,
                            mut every,
                        ),
                         data| {
                            // 将每个 `data` 累加到对应的变量
                            every.push(data.clone());
                            (
                                ipv4_up + data.ipv4_up,
                                ipv4_down + data.ipv4_down,
                                ipv6_up + data.ipv6_up,
                                ipv6_down + data.ipv6_down,
                                used_flow + data.used_flow,
                                cost + data.cost,
                                used_duration + data.used_duration,
                                every,
                            )
                        },
                    );
            Ok(Some(UserLoginLog {
                ipv4_up,
                ipv4_down,
                ipv6_up,
                ipv6_down,
                used_flow,
                cost,
                used_duration,
                every_login_data: every,
            }))
        } else {
            Ok(Some(value))
        }
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
