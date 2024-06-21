use chrono::DateTime;
use regex::Regex;
use tauri::Manager;

use crate::{
    entities::AppState,
    requests::{
        get_load_user_flow, get_mac_address, get_month_pay, get_refresh_account,
        get_user_login_log, unbind_macs,
    },
    utils::get_webview2_cookie,
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
            e.to_string()
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
        .map_err(|e| format!("Error while loading user flow: {}", e.to_string()))
        .map(|res| res.to_string())
}

// 用来获取WebView当前页面的Cookie
#[tauri::command(async)]
pub async fn get_cookie(
    app_state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    if cfg!(target_os = "windows") {
        let windows = app_handle.windows();
        // #[allow(unused_variables)]
        // let url = "https://tauri.localhost";
        // #[cfg(debug_assertions)] // 如果是 debug 模式，把 url 替换为debug的
        // let url = "http://localhost:1420/";
        // let res = get_webview2_cookie(windows.get("main").unwrap(), url).await;
        let url = "http://202.204.60.7:8080/LoginAction.action";
        if let Some(window) = windows.get("nav_login") {
            let res = get_webview2_cookie(window, url).await;
            match res {
                Ok(cookies) => {
                    dbg!(&cookies[0]);
                    *app_state.jsessionid.write().unwrap() =
                        cookies.get(0).map(|str| str.value.clone());
                }
                Err(_) => return Err("Can't get cookies due to unknown error".to_string()),
            }
            window
                .hide()
                .map_err(|e| format!("Hide window error: {}", e.to_string()))?;
        } else {
            return Err("Please open the login window.".into());
        }
    }
    Ok(app_state
        .jsessionid
        .read()
        .unwrap()
        .clone()
        .unwrap_or_default())
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
        Ok(None) => return Err("请确认是否已经登录".to_string()),
        Err(e) => {
            return Err(format!(
                "Request Error，检查是否在校园网内: {}",
                e.to_string()
            ))
        }
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
            .map_err(|e| format!("Error while loading user flow: {}", e.to_string()))
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
        Err(e) => Err(format!(
            "Request Error，检查是否在校园网内: {}",
            e.to_string()
        )),
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
        Err(e) => Err(format!(
            "Request Error，检查是否在校园网内: {}",
            e.to_string()
        )),
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
        Err(e) => Err(format!(
            "Request Error，检查是否在校园网内: {}",
            e.to_string()
        )),
    }
}

#[tauri::command]
pub fn get_current_device_mac() -> Result<String, String> {
    mac_address::get_mac_address()
        .map_err(|e| format!("获取MAC地址错误: {}", e.to_string()))
        .map(|mac_address| mac_address.unwrap_or_default().to_string())
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
        Err(e) => Err(format!(
            "Request Error，检查是否在校园网内: {}",
            e.to_string()
        )),
    }
}
