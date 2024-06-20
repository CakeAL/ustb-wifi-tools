use std::f64;

use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use regex::Regex;
use reqwest::{header::SET_COOKIE, Client};
use scraper::{Html, Selector};
use serde_json::Value;

use crate::entities::{EveryLoginData, MacAddress, MonthPayInfo, MonthlyData, UserLoginLog};

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

// 已废弃->这里的year_month应该是类似于 202203 或者 202312 这样的格式
// start_date 2024-05-01 end_date 2024-05-31
// 校园网的API并不能返回全部数据，有条数限制。
pub async fn get_user_login_log(
    session_id: &str,
    start_date: &str,
    end_date: &str,
) -> Result<Option<Value>> {
    let url = "http://202.204.60.7:8080/UserLoginLogAction.action";
    // let month = format!("CHECKER.TBLUSERLOGIN{}", year_month); // 按月份已经废了，现在只能按照开始结束日期查
    let response = Client::new()
        .post(url)
        .header("Cookie", format!("JSESSIONID={}", session_id))
        .form(&[
            ("type", "4"),
            ("month", "CHECKER.TBLUSERLOGIN202304"),
            ("startDate", start_date),
            ("endDate", end_date),
        ])
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
    // println!("{:?}", redtexts);
    // ["18170.41", "181351.69", "33287.1", "39530.03", "181351.69", "0.0", "85922"]
    let every_login_data_selector = Selector::parse(".table4 > tbody > tr > td").unwrap();
    let every_login_data_text = parsed_html
        .select(&every_login_data_selector)
        .flat_map(|ele| ele.text().collect::<Vec<&str>>());
    let mut data_index = 0;
    let mut every_login_data = EveryLoginData {
        online_time: 0,
        offline_time: 0,
        used_duration: 0,
        used_flow: 0.0,
        cost: 0.0,
        ipv4_up: 0.0,
        ipv4_down: 0.0,
        ipv6_up: 0.0,
        ipv6_down: 0.0,
        ipv4_addr: String::new(),
        ipv6_addr: String::new(),
    };
    let mut every_login_datas = vec![];
    //0 "2023-04-01 00:00:04"
    //1 "2023-04-01 00:12:53"
    //2 "\n\t\t\t\t\t\t\t13\n\t\t\t\t\t\t"
    //3 "\n\t\t\t\t\t\t\t28.997\n\t\t\t\t\t\t"
    //4 "\n\t\t\t\t\t\t\t28.997\n\t\t\t\t\t\t" // hide
    //5 "\n\t\t\t\t\t\t\t0.00\n\t\t\t\t\t\t"
    //6 "\n\t\t\t\t\t\t\t2.315\n\t\t\t\t\t\t"
    //7 "\n\t\t\t\t\t\t\t28.997\n\t\t\t\t\t\t"
    //8 "\n\t\t\t\t\t\t\t1.800\n\t\t\t\t\t\t"
    //9 "\n\t\t\t\t\t\t\t23.113\n\t\t\t\t\t\t"
    //10"ipv4"
    //11"ipv6"
    for data in every_login_data_text {
        // println!("{:?}", data);
        match data_index {
            0 => {
                every_login_data.online_time =
                    NaiveDateTime::parse_from_str(data, "%Y-%m-%d %H:%M:%S")?
                        .and_utc()
                        .timestamp()
            }
            1 => {
                every_login_data.offline_time =
                    NaiveDateTime::parse_from_str(data, "%Y-%m-%d %H:%M:%S")?
                        .and_utc()
                        .timestamp()
            }
            2 => every_login_data.used_duration = data.trim().parse()?,
            3 => every_login_data.used_flow = data.trim().parse()?,
            4 => (),
            5 => every_login_data.cost = data.trim().parse()?,
            6 => every_login_data.ipv4_up = data.trim().parse()?,
            7 => every_login_data.ipv4_down = data.trim().parse()?,
            8 => every_login_data.ipv6_up = data.trim().parse()?,
            9 => every_login_data.ipv6_down = data.trim().parse()?,
            10 => every_login_data.ipv4_addr = data.to_string(),
            11 => {
                // 如果 11 的字符串内容包含'-'，说明已经到下一行了，该行没有ipv6地址
                if data.contains("-") {
                    every_login_data.ipv6_addr = "".to_string();
                    every_login_datas.push(every_login_data.clone());
                    // 别把这个data数据直接丢了，得把这一行的数据存进去
                    every_login_data.online_time =
                        NaiveDateTime::parse_from_str(data, "%Y-%m-%d %H:%M:%S")?
                            .and_utc()
                            .timestamp();
                    data_index = 1; // 所以index直接变成1了
                    continue; // 开始新一行，从1开始
                } else {
                    every_login_data.ipv6_addr = data.to_string();
                    data_index = 0; // 开始新一行
                    every_login_datas.push(every_login_data.clone());
                    continue;
                }
            }
            _ => (),
        }
        data_index += 1;
    }
    // dbg!(every_login_datas);
    Ok(Some(serde_json::json!(UserLoginLog {
        ipv4_up: redtexts.get(0).unwrap().trim().parse()?,
        ipv4_down: redtexts.get(1).unwrap().trim().parse()?,
        ipv6_up: redtexts.get(2).unwrap().trim().parse()?,
        ipv6_down: redtexts.get(3).unwrap().trim().parse()?,
        used_flow: redtexts.get(4).unwrap().trim().parse()?,
        cost: redtexts.get(5).unwrap().trim().parse()?,
        used_duration: redtexts.get(6).unwrap().trim().parse()?,
        every_login_data: every_login_datas,
    })))
}

pub async fn get_mac_address(session_id: &str) -> Result<Option<Value>> {
    let url = "http://202.204.60.7:8080/nav_unBandMacJsp";
    let response = Client::new()
        .get(url)
        .header("Cookie", format!("JSESSIONID={}", session_id))
        .send()
        .await?
        .text()
        .await?;
    // println!("{response}");
    if response.contains("nav_login") {
        return Ok(None); // Cookie无效，没有获取到account信息
    }
    let parsed_html = Html::parse_document(&response);
    let device_name_selector =
        Selector::parse(".row > .v-col:first-of-type input[type=\"text\"]").unwrap();
    let device_names_value = parsed_html
        .select(&device_name_selector)
        .flat_map(|ele| ele.value().attr("value"))
        .collect::<Vec<&str>>();
    let mac_address_selector =
        Selector::parse(".row > .v-col:nth-of-type(2) input[type=\"text\"][name=\"macs\"]")
            .unwrap();
    let mac_address_value = parsed_html
        .select(&mac_address_selector)
        .flat_map(|ele| ele.value().attr("value"))
        .collect::<Vec<&str>>();
    // dbg!(device_names);
    // dbg!(mac_address);
    let mac_address = device_names_value
        .iter()
        .zip(mac_address_value.iter())
        .map(|(device_name, mac_address)| MacAddress {
            device_name: device_name.to_string(),
            mac_address: mac_address.to_string(),
        })
        .collect::<Vec<_>>();

    Ok(Some(serde_json::json!(mac_address)))
}

// 这里传进来的是 **不需要** 解绑的macs
pub async fn unbind_macs(session_id: &str, macs: &Vec<String>) -> Result<Option<()>> {
    let url = "http://202.204.60.7:8080/nav_unbindMACAction.action";
    let mut mac_str = String::new();
    for mac in macs {
        mac_str = format!("{};{}", mac, mac_str);
    }
    let _ = mac_str.pop(); // 删末尾分号
    dbg!(&mac_str);
    let response = Client::new()
        .post(url)
        .header("Cookie", format!("JSESSIONID={}", session_id))
        .form(&[("macStr", mac_str), ("Submit", "解绑".to_string())])
        .send()
        .await?
        .text()
        .await?;
    if response.contains("nav_login") {
        return Ok(None); // Cookie无效，没有获取到account信息
    }

    Ok(Some(()))
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
        dbg!(res.unwrap());
    }

    #[tokio::test]
    async fn test_get_user_login_log() {
        let session_id = "session_id";
        // let year_month = "202405";
        let start_date = "2024-05-01";
        let end_date = "2024-05-31";
        let res = get_user_login_log(session_id, start_date, end_date).await;
        dbg!(res.unwrap());
    }

    #[tokio::test]
    async fn test_get_mac_address() {
        let session_id = "session_id";
        let res = get_mac_address(session_id).await;
        dbg!(res.unwrap());
    }

    #[tokio::test]
    async fn test_unbind_macs() {
        let session_id = "session_id";
        let macs = vec![]; // such as "ABCD12345678".to_string()
                           // macs 为空执行此 test 会导致退出全部你的校园网账号
        let res = unbind_macs(&session_id, &macs).await;
        dbg!(res.unwrap());
    }
}
