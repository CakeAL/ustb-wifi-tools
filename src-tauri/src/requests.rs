use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::{header::SET_COOKIE, Client};
use serde_json::Value;
//  Ciallo～(∠・ω< )⌒☆
pub async fn get_load_user_flow(account: &str) -> Result<Value> {
    let client = Client::new();
    let url = format!(
        "http://202.204.48.66:801/eportal/portal/visitor/loadUserFlow?account={}",
        account
    );
    let response = client.get(url).send().await?.text().await?;
    let re = Regex::new(r"jsonpReturn\((.*)\);")?;
    let json_str = re
        .captures(&response)
        .and_then(|cap| Some(cap.get(1)?.as_str()));
    Ok(serde_json::from_str(json_str.unwrap())?)
}

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

pub async fn get_refresh_account(
    account: &str,
    password: &str,
    session_id: &str,
) -> Result<String> {
    let url = "http://202.204.60.7:8080/LoginAction.action";
    let params = [
        ("account", account),
        ("password", password),
        // ("code", ""),
        // ("check_code", check_code),
        // ("Submit", "登 陆"),
    ];
    let client = Client::new();
    let _ = client.post(url)
    .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36")
    .header("Cookie", format!("JSESSIONID={}", session_id))
    .form(&params).send().await?;

    let url = "http://202.204.60.7:8080/refreshaccount";
    let client = Client::new();
    let response = client
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
    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::entities::{GetUserFlowFailed, UserFlow};

    #[warn(unused_imports)]
    use super::get_jsessionid;
    use super::get_load_user_flow;
    #[warn(unused_imports)]
    use super::get_refresh_account;

    #[tokio::test]
    async fn test_get_load_user_flow() {
        let account = "U202141234".to_string();
        let res = get_load_user_flow(&account).await.unwrap();
        println!("{:?}", res);
        if let Ok(user_flow) = serde_json::from_value::<UserFlow>(res.clone()) {
            println!("{:?}", user_flow);
        } else if let Ok(get_failed) = serde_json::from_value::<GetUserFlowFailed>(res) {
            println!("Error: {}", get_failed.msg);
        } else {
            println!("Error: 其他未知原因");
        }
    }

    #[tokio::test]
    async fn test_get_jsessionid() {
        let account = "stu_id".to_string();
        let password = "md5_password".to_string();
        let res = get_jsessionid(&account, &password).await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_get_refresh_account() {
        let account = "stu_id".to_string();
        let password = "md5_password".to_string();
        let session_id = "session_id".to_string();
        let res = get_refresh_account(&account, &password, &session_id).await;
        println!("{:?}", res);
    }
}
