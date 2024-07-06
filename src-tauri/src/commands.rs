use std::borrow::Borrow;

use chrono::DateTime;
use regex::Regex;
use tauri::Manager;

use crate::{
    entities::{Account, AppState},
    requests::{
        get_load_user_flow, get_mac_address, get_month_pay, get_refresh_account,
        get_user_login_log, unbind_macs,
    },
    utils::login_via_headless_browser,
};

#[tauri::command(async)]
pub fn open_nav_login(app_handle: tauri::AppHandle) -> Result<(), String> {
    // 判断该窗口是否已存在
    if let Some(window) = app_handle.get_window("nav_login") {
        window.close().map_err(|e| e.to_string())?;
    }

    tauri::WindowBuilder::new(
        &app_handle,
        "nav_login",
        tauri::WindowUrl::App("http://202.204.60.7:8080/nav_login".into()),
    )
    .build()
    .map_err(|e| {
        format!(
            "Error when building the nav_login window, 可能需要再点一下: {}",
            e
        )
    })?;
    // // 什么Golang😡
    // if nav_login_window.is_ok() != true {
    //     return Err("Error when building the nav_login window".into());
    // };

    Ok(())
}

// 该函数可能用不到
#[tauri::command(async)]
pub async fn load_user_flow(account: String) -> Result<String, String> {
    get_load_user_flow(&account)
        .await
        .map_err(|e| format!("Error while loading user flow: {}", e))
        .map(|res| res.to_string())
}

// 用来获取WebView当前页面的Cookie
#[tauri::command(async)]
pub async fn get_cookie(
    app_state: tauri::State<'_, AppState>,
    user_name: String,
    password: String,
) -> Result<String, String> {
    let account = Account {
        user_name,
        password,
        check_code: None,
    };
    if let Some(tab) = (*app_state.tab.read().unwrap()).borrow() {
        let res = login_via_headless_browser(tab, account);
        match res {
            Ok(cookies) => {
                dbg!(&cookies[0]);
                *app_state.jsessionid.write().unwrap() =
                    cookies.first().map(|str| str.value.clone());
            }
            Err(err) => return Err(format!("Can't get cookies due to unknown error: {}", err)),
        }
        Ok(app_state
            .jsessionid
            .read()
            .unwrap()
            .clone()
            .unwrap_or_default())
    } else {
        Err("Headless Browser 没有页面🤔🤔🤔".to_string())
    }
}

#[tauri::command(async)]
pub async fn load_refresh_account(app_state: tauri::State<'_, AppState>) -> Result<String, String> {
    let session_id = match app_state.jsessionid.read().unwrap().clone() {
        Some(s) => s,
        None => return Err("SessionID为空，是否已经登录并单击获取Cookie按钮？".to_string()),
    };

    match get_refresh_account(&session_id).await {
        Ok(Some(str)) => {
            // 存一下用户名
            Regex::new(r#""welcome":\s*"([^"]+)\([^"]*\)""#)
                .unwrap()
                .captures(&str)
                .map(|caps| {
                    dbg!(&caps.get(1).map(|str| str.as_str().to_string()));
                    *app_state.account.write().unwrap() =
                        caps.get(1).map(|str| str.as_str().to_string())
                });
            Ok(str)
        }
        Ok(None) => Err("请确认是否已经登录".to_string()),
        Err(e) => Err(format!("Request Error，检查是否在校园网内: {}", e)),
    }
}

#[tauri::command(async)]
pub async fn load_user_flow_by_state(
    app_state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let account = app_state.account.read().unwrap().clone();
    match account {
        Some(account) => Ok(get_load_user_flow(&account)
            .await
            .map_err(|e| format!("Error while loading user flow: {}", e))
            .map(|res| res.to_string())?),
        None => Err("Account is none, try again".to_string()),
    }
}

#[tauri::command(async)]
pub async fn load_month_pay(
    app_state: tauri::State<'_, AppState>,
    year: u16,
) -> Result<String, String> {
    let session_id = match app_state.jsessionid.read().unwrap().clone() {
        Some(s) => s,
        None => return Err("SessionID为空，是否已经登录并单击获取Cookie按钮？".to_string()),
    };

    match get_month_pay(&session_id, year).await {
        Ok(Some(value)) => Ok(value.to_string()),
        Ok(None) => Err("请确认是否已经登录".to_string()),
        Err(e) => Err(format!("Request Error，检查是否在校园网内: {}", e)),
    }
}

#[tauri::command(async)]
pub async fn load_user_login_log(
    app_state: tauri::State<'_, AppState>,
    start_date: i64,
    end_date: i64,
) -> Result<String, String> {
    if start_date > end_date {
        return Err("起始日期比结束日期更大。。。".to_string());
    }
    let session_id = match app_state.jsessionid.read().unwrap().clone() {
        Some(s) => s,
        None => return Err("SessionID为空，是否已经登录并单击获取Cookie按钮？".to_string()),
    };
    let start_date = DateTime::from_timestamp(start_date, 0)
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    let end_date = DateTime::from_timestamp(end_date, 0)
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    match get_user_login_log(&session_id, &start_date, &end_date).await {
        Ok(Some(value)) => Ok(value.to_string()),
        Ok(None) => Err("请确认是否已经登录".to_string()),
        Err(e) => Err(format!("Request Error，检查是否在校园网内: {}", e)),
    }
}

#[tauri::command(async)]
pub async fn load_mac_address(app_state: tauri::State<'_, AppState>) -> Result<String, String> {
    let session_id = match app_state.jsessionid.read().unwrap().clone() {
        Some(s) => s,
        None => return Err("SessionID为空，是否已经登录并单击获取Cookie按钮？".to_string()),
    };

    match get_mac_address(&session_id).await {
        Ok(Some(value)) => Ok(value.to_string()),
        Ok(None) => Err("请确认是否已经登录".to_string()),
        Err(e) => Err(format!("Request Error，检查是否在校园网内: {}", e)),
    }
}

#[tauri::command]
pub fn get_current_device_mac() -> Result<String, String> {
    match std::env::consts::OS {
        "windows" => mac_address::mac_address_by_name("WLAN")
            .map_err(|e| format!("获取 MAC 地址错误: {}", e))
            .map(|mac_address| mac_address.unwrap_or_default().to_string()),
        "macos" => mac_address::mac_address_by_name("en0")
            .map_err(|e| format!("获取 MAC 地址错误: {}", e))
            .map(|mac_address| mac_address.unwrap_or_default().to_string()),
        _ => Ok("不支持当前系统获取 MAC 地址".to_string()),
    }
}

// 传进来的应该是不需要解绑的，提醒。
#[tauri::command(async)]
pub async fn do_unbind_macs(
    app_state: tauri::State<'_, AppState>,
    macs: Vec<String>,
) -> Result<(), String> {
    let session_id = match app_state.jsessionid.read().unwrap().clone() {
        Some(s) => s,
        None => return Err("SessionID为空，是否已经登录并单击获取Cookie按钮？".to_string()),
    };

    match unbind_macs(&session_id, &macs).await {
        Ok(Some(())) => Ok(()),
        Ok(None) => Err("请确认是否已经登录".to_string()),
        Err(e) => Err(format!("Request Error，检查是否在校园网内: {}", e)),
    }
}

#[tauri::command(async)]
pub fn open_speed_test(app_handle: tauri::AppHandle) -> Result<(), String> {
    // 判断该窗口是否已存在
    if app_handle.get_window("speed_test").is_some() {
        return Err("已经打开一个测速窗口了".to_string());
    }

    tauri::WindowBuilder::new(
        &app_handle,
        "speed_test",
        tauri::WindowUrl::App("http://speed.ustb.edu.cn/".into()),
    )
    .build()
    .map_err(|e| format!("Error when building the speed_test window: {}", e))
    .map(|_| ())
}
