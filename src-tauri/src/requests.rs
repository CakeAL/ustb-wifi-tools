use std::f64;

use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::{header::SET_COOKIE, Client};
use scraper::{Html, Selector};
use serde_json::Value;

use crate::entities::{MonthPayInfo, MonthlyData};

// Ciallo～(∠・ω< )⌒☆
pub async fn get_load_user_flow(account: &str) -> Result<Value> {
    let url = format!(
        "http://202.204.48.66:801/eportal/portal/visitor/loadUserFlow?account={}",
        account
    );
    let response = Client::new().get(url).send().await?.text().await?;
    let re = Regex::new(r"jsonpReturn\((.*)\);")?;
    let json_str = re
        .captures(&response)
        .and_then(|cap| Some(cap.get(1)?.as_str()));
    Ok(serde_json::from_str(json_str.unwrap())?)
}

// 该函数可能用不到了
#[allow(dead_code)]
pub async fn get_jsessionid(account: &str, password: &str) -> Result<String> {
    let client = Client::new();
    let check_url = "http://202.204.60.7:8080/nav_login";
    let res_check = client.get(check_url).send().await?.text().await?;
    let re = Regex::new(r#"var checkcode="([^"]*)";"#)?;
    let check_code = re
        .captures(&res_check)
        .and_then(|cap| cap.get(1))
        .unwrap()
        .as_str();
    println!("{check_code}");
    let client = Client::new();
    let url = "http://202.204.60.7:8080/LoginAction.action";
    let params = [
        ("account", account),
        ("password", password),
        ("code", ""),
        ("check_code", check_code),
        ("Submit", "登 陆"),
    ];
    let response = client.post(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36")
        .header("Referer", "http://202.204.60.7:8080/nav_login")
        .form(&params).send().await?;
    // println!("{}", response.headers().get_all(SET_COOKIE).iter().next().unwrap().to_str()?);
    let jessionid = response.headers().get_all(SET_COOKIE).iter().next();
    if let Some(jessionid) = jessionid {
        let re = Regex::new(r#"JSESSIONID=([^;]*)"#)?;
        let res = re
            .captures(jessionid.to_str()?)
            .and_then(|cap| cap.get(1))
            .unwrap()
            .as_str();
        Ok(res.to_string())
    } else {
        Err(anyhow!("No session_id found!"))
    }
}

pub async fn get_refresh_account(session_id: &str) -> Result<Option<String>> {
    let url = "http://202.204.60.7:8080/refreshaccount";
    let response = Client::new()
        .get(url)
        .header("Cookie", format!("JSESSIONID={}", session_id))
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36")
        .header("Referer", "http://202.204.60.7:8080/LoginAction.action")
        .header("Host", "202.204.60.7:8080")
        .send()
        .await?
        .text()
        .await?;
    // println!("{response}");
    if response.contains("nav_login") {
        return Ok(None); // Cookie无效，没有获取到account信息
    }
    Ok(Some(response))
}

pub async fn get_month_pay(session_id: &str, year: u16) -> Result<Option<Value>> {
    let url = "http://202.204.60.7:8080/MonthPayAction.action";
    let response = Client::new()
        .post(url)
        .header("Cookie", format!("JSESSIONID={}", session_id))
        .form(&[("type", 1), ("year", year)])
        .send()
        .await?
        .text()
        .await?;
    // println!("{response}");
    if response.contains("nav_login") {
        return Ok(None); // Cookie无效，没有获取到account信息
    }
    let parsed_html = Html::parse_document(&response);
    let redtext_selector = Selector::parse(".redtext").unwrap();
    let redtexts = parsed_html
        .select(&redtext_selector)
        .flat_map(|ele| ele.text().collect::<Vec<&str>>())
        .collect::<Vec<&str>>();
    // println!("{:?}", redtexts); // ["0.00", "0.95", "616323", "330284.323"]
    let monthly_data_selector = Selector::parse(".table4 > tbody > tr > td").unwrap();
    let monthly_data_text = parsed_html
        .select(&monthly_data_selector)
        .flat_map(|ele| ele.text().collect::<Vec<&str>>());
    // 不是，哥们，你这网页数据存这么多\n, \t干什么啊
    let mut data_index = 0;
    let mut monthly_datas = vec![];
    let mut month_data = MonthlyData {
        month: 0,
        month_cost: 0.0,
        month_used_flow: 0.0,
        month_used_duration: 0,
    };
    for data in monthly_data_text {
        // println!("{:?}", data);
        // 依次匹配每行字符串，到7的时候设置为-1，然后+1变成0，说明再读取就到下一行了
        match data_index {
            0 => month_data.month = data[5..7].parse::<u8>()?,
            4 => month_data.month_cost = data.trim().parse::<f64>()?,
            5 => month_data.month_used_duration = data.trim().parse::<u32>()?,
            6 => month_data.month_used_flow = data.trim().parse::<f64>()?,
            7 => {
                data_index = -1;
                // println!("{:?}", month_data);
                monthly_datas.push(month_data);
            }
            _ => (),
        }
        data_index += 1;
    }
    // println!("{:?}", monthly_datas);

    Ok(Some(serde_json::json!(MonthPayInfo {
        year_cost: redtexts.get(1).unwrap().parse()?,
        year_used_duration: redtexts.get(2).unwrap().parse()?,
        year_used_flow: redtexts.get(3).unwrap().parse()?,
        monthly_data: monthly_datas,
    })))
}

#[cfg(test)]
mod tests {
    // use crate::entities::{GetUserFlowFailed, UserFlow};
    use super::*;

    #[tokio::test]
    async fn test_get_load_user_flow() {
        let account = "U202141234".to_string();
        let res = get_load_user_flow(&account).await.unwrap();
        println!("{:?}", res);
        // if let Ok(user_flow) = serde_json::from_value::<UserFlow>(res.clone()) {
        //     println!("{:?}", user_flow);
        // } else if let Ok(get_failed) = serde_json::from_value::<GetUserFlowFailed>(res) {
        //     println!("Error: {}", get_failed.msg);
        // } else {
        //     println!("Error: 其他未知原因");
        // }
    }

    #[tokio::test]
    async fn test_get_jsessionid() {
        let account = "stu_id";
        let password = "md5_password";
        let res = get_jsessionid(account, password).await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_get_refresh_account() {
        let session_id = "session_id";
        let res = get_refresh_account(session_id).await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_get_month_pay() {
        let session_id = "session_id";
        let year = 2024u16;
        let res = get_month_pay(session_id, year).await;
        dbg!(res);
    }
}
