use std::path::PathBuf;

use crate::{entities::RemainingElectricity, requests::get_ammeter};
use anyhow::{anyhow, Result};
use chrono::{Local, TimeZone};
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt},
};

pub async fn update_ammeter(
    ammeter_number: u32,
    file_path: PathBuf,
) -> Result<Vec<RemainingElectricity>> {
    let mut buf = vec![];
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .open(&file_path)
        .await?;
    let _ = file.read_to_end(&mut buf);
    let mut remain_elec = serde_json::from_slice::<Vec<RemainingElectricity>>(&buf)?;
    let now = Local::now();
    let mut last_data_day = now.date_naive();
    let mut last_remain = 0;
    // 判断最后获取日期是否是今天，如果是，不用再获取了
    if let Some(last_data) = remain_elec.last() {
        let today = now.date_naive();
        last_remain = last_data.remain;
        last_data_day = now
            .offset()
            .timestamp_opt(last_data.date, 0)
            .unwrap()
            .date_naive();
        if today == last_data_day {
            return Err(anyhow!("今日已经获取过电表数据，明天再来吧"));
        }
    }

    let new_data = RemainingElectricity {
        date: now.timestamp(),
        remain: get_ammeter(ammeter_number)
            .await?
            .ok_or(anyhow!("返回数据为 None"))?,
        average: last_remain as f64 / (now.date_naive() - last_data_day).num_days() as f64,
    };
    remain_elec.push(new_data);

    // 写回
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .await?;
    let _ = file.write_all(&serde_json::to_vec(&remain_elec)?);
    Ok(remain_elec)
}
