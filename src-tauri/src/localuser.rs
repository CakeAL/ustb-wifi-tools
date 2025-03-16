use anyhow::{anyhow, Result};
use chrono::{DateTime, Datelike, NaiveDate, Utc};

use crate::entities::AppState;
use crate::utils::get_session_id;

pub struct Localuser {
    pub username: String,
}

impl Localuser {
    // 获取输入年月至现在的数据
    pub async fn get_historical_data(
        // app_state: tauri::State<'_, AppState>,
        start_date: i64,
    ) -> Result<String> {
        // let session_id = get_session_id(&app_state)
        //     .await
        //     .map_err(|err| anyhow!(err))?;
        // let via_vpn = *app_state.login_via_vpn.read().await;
        let current_date = Utc::now().date_naive();
        let mut start_date = DateTime::from_timestamp(start_date, 0)
            .unwrap()
            .date_naive();
        while start_date <= current_date {
            let start_date_string = start_date.format("%Y-%m-%d").to_string();
            let end_date_string = get_last_day_of_month(&start_date)
                .format("%Y-%m-%d")
                .to_string();
            println!("{start_date_string} -> {end_date_string}");
            start_date = get_first_day_next_month(&start_date);
        }
        Ok("".to_string())
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
    use super::*;

    #[tokio::test] 
    async fn test_gen_dates() {
        let timestamp = 1627776000;
        let res = Localuser::get_historical_data(timestamp).await; 
        dbg!(res.unwrap());
    }
}