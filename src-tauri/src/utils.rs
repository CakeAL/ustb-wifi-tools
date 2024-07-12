use anyhow::Result;
use headless_chrome::{Browser, LaunchOptions};
use std::{path::PathBuf, thread, time::Duration};

use crate::entities::Account;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
}

pub fn get_browser_path() -> Option<PathBuf> {
    match std::env::consts::OS {
        "windows" => {
            let edge =
                PathBuf::from("C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe");
            let chrome =
                PathBuf::from("C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe");
            if edge.exists() {
                Some(edge.clone())
            } else if chrome.exists() {
                Some(chrome.clone())
            } else {
                None
            }
        }
        "macos" => {
            let edge =
                PathBuf::from("/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge");
            let chromium = PathBuf::from("/Applications/Chromium.app/Contents/MacOS/Chromium");
            let chrome =
                PathBuf::from("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome");
            if chrome.exists() {
                Some(chrome.clone())
            } else if chromium.exists() {
                Some(chromium.clone())
            } else if edge.exists() {
                Some(edge.clone())
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn try_open_headless_browser(browser_path: PathBuf) -> Result<()> {
    let _ = Browser::new(LaunchOptions {
        path: Some(browser_path),
        ..Default::default()
    })?;
    Ok(())
}

pub fn login_via_headless_browser(browser_path: PathBuf, account: &Account) -> Result<Vec<Cookie>> {
    let browser = Browser::new(LaunchOptions {
        headless: false,
        // window_size: Some((1600, 900)),
        path: Some(browser_path),
        ..Default::default()
    })?;

    let tab = browser.new_tab()?;
    tab.set_default_timeout(std::time::Duration::from_secs(5))
        .navigate_to("http://202.204.60.7:8080/nav_login")?
        .wait_until_navigated()?;

    let user_name_ele =
        tab.find_element_by_xpath(r#"/html/body/div/div/div[3]/div/div/form/div[3]/input"#)?;
    let password_ele =
        tab.find_element_by_xpath(r#"/html/body/div/div/div[3]/div/div/form/div[4]/input"#)?;
    let check_code_ele =
        tab.find_element_by_xpath(r#"/html/body/div/div/div[3]/div/div/form/div[5]/input[1]"#)?;

    user_name_ele.call_js_fn(
        "function(str) { this.value = str }",
        vec![serde_json::json!(account.user_name)],
        false,
    )?;
    password_ele.call_js_fn(
        "function(str) { this.value = str }",
        vec![serde_json::json!(account.password)],
        false,
    )?;

    if account.check_code.is_some() {
        check_code_ele.call_js_fn(
            "function(str) { this.value = str }",
            vec![serde_json::json!(account.check_code.clone().unwrap())],
            false,
        )?;
    }

    let submit_button_ele =
        tab.find_element_by_xpath(r#"/html/body/div/div/div[3]/div/div/form/div[6]/input"#)?;
    submit_button_ele.click()?;

    loop {
        thread::sleep(Duration::from_millis(20));
        // 等待网址变更
        if tab.get_url() == "http://202.204.60.7:8080/LoginAction.action" {
            tab.wait_until_navigated()?;
            break;
        }
    }

    let mut res = vec![];
    match tab.find_element_by_xpath(
        r#"/html/body/div/div[1]/div[3]/div/div[1]/div[1]/div[1]/div/div/div[1]"#,
    ) {
        Ok(_) => {
            let c = tab.get_cookies()?.first().unwrap().clone();
            res.push(Cookie {
                name: c.name,
                value: c.value,
                domain: c.domain,
                path: c.path,
            });
        }
        Err(_) => {
            let ele = tab.find_element_by_xpath(r#"/html/body/div/div/div[3]/div/div/form/div[2]/div[1]/div"#).unwrap();
            return Err(anyhow::anyhow!(format!("{}", ele.get_inner_text().unwrap_or_default())));
        }
    }

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_browser_path() {
        let path = get_browser_path();
        dbg!(path.unwrap().to_str());
    }

    #[test]
    fn test_login_via_headless_browser() {
        let account: Account = Account {
            user_name: "user_name".to_string(),
            password: "password".to_string(),
            check_code: None,
        };
        let browser_path = get_browser_path().unwrap();
        let res = login_via_headless_browser(browser_path, &account);
        dbg!(res.unwrap());
    }
}
