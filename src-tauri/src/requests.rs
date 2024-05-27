use anyhow::Result;
use regex::Regex;
use reqwest::Client;
use serde_json::Value;

pub async fn get_load_user_flow(account: &String) -> Result<Value> {
    let client = Client::new();
    let url = format!(
        "http://202.204.48.66:801/eportal/portal/visitor/loadUserFlow?account={}",
        account
    );
    let response = client.get(url).send().await?.text().await?;
    let re = Regex::new(r"jsonpReturn\((.*)\);").unwrap();
    let json_str = re
        .captures(&response)
        .and_then(|cap| Some(cap.get(1)?.as_str()));
    Ok(serde_json::from_str(json_str.unwrap())?)
}

#[cfg(test)]
mod tests {
    use crate::entities::{GetUserFlowFailed, UserFlow};

    use super::get_load_user_flow;

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
}
