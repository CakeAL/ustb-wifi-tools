use std::{io::SeekFrom, path::PathBuf};

use crate::{entities::RemainingElectricity, requests::get_ammeter};
use anyhow::{anyhow, Result};
use chrono::{Days, Local, TimeZone};
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};

pub async fn update_ammeter(
    ammeter_number: u32,
    file_path: PathBuf,
) -> Result<(Vec<RemainingElectricity>, String)> {
    let mut buf = vec![];
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&file_path)
        .await?;
    let _ = file.read_to_end(&mut buf).await;
    let mut remain_elec = vec![];
    if !buf.is_empty() {
        remain_elec = serde_json::from_slice::<Vec<RemainingElectricity>>(&buf)?;
        // dbg!(&remain_elec);
    }
    let now = Local::now();
    let mut last_data_day = now.date_naive().checked_sub_days(Days::new(1)).unwrap();
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
            return Ok((
                remain_elec,
                "今日已经获取过电表数据，明天再来吧".to_string(),
            ));
        }
    }

    let new_data = {
        let remain = get_ammeter(ammeter_number)
            .await?
            .ok_or(anyhow!("获取今日数据时返回为 None"))?;
        RemainingElectricity {
            date: now.timestamp(),
            remain: remain,
            average: (last_remain - remain) as f64
                / (now.date_naive() - last_data_day).num_days() as f64,
        }
    };
    remain_elec.push(new_data);

    // 写回
    file.set_len(0).await?;
    file.seek(SeekFrom::Start(0)).await?;
    let _ = file.write_all(&serde_json::to_vec(&remain_elec)?).await;
    Ok((remain_elec, "已更新今日数据".to_string()))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::electric_bill::update_ammeter;

    #[tokio::test]
    async fn test_update_ammeter() {
        let ammeter_number = 00000000;
        let path = PathBuf::from("/Users/cakeal/Downloads/00000000.json");
        let res = update_ammeter(ammeter_number, path).await;
        dbg!(res.unwrap());
    }
}
