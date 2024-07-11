use std::path::PathBuf;

use chrono::DateTime;
use rfd::FileDialog;
use tauri::Manager;

use crate::{
    entities::{Account, AppState},
    requests::{
        get_address, get_load_user_flow, get_mac_address, get_month_pay, get_refresh_account,
        get_user_login_log, unbind_macs,
    },
    setting::Setting,
    utils::{get_browser_path, login_via_headless_browser, try_open_headless_browser},
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

// 对 headless browser 进行操作，获取登陆后的 Cookie
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
    let browser_path = get_browser_path().unwrap();
    let res = login_via_headless_browser(browser_path, &account);
    match res {
        Ok(cookies) => {
            dbg!(&cookies[0]);
            *app_state.jsessionid.write().unwrap() = cookies.first().map(|str| str.value.clone());
            app_state
                .setting
                .write()
                .unwrap()
                .set_account(account.user_name, account.password);
        }
        Err(err) => return Err(format!("{}", err)),
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
        Ok(Some(str)) => Ok(str),
        Ok(None) => Err("请确认是否已经登录".to_string()),
        Err(e) => Err(format!("Request Error，检查是否在校园网内: {}", e)),
    }
}

#[tauri::command(async)]
pub async fn load_user_flow_by_state(
    app_state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let account = app_state.setting.read().unwrap().username.clone();
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
pub fn open_speed_test(app_handle: tauri::AppHandle, site_num: i32) -> Result<(), String> {
    // 判断该窗口是否已存在
    if app_handle.get_window("speed_test").is_some() {
        return Err("已经打开一个测速窗口了".to_string());
    }
    let url = match site_num {
        1 => "http://speed.ustb.edu.cn/",      // 北科 内网
        2 => "https://test6.ustc.edu.cn/",     // 中科大 ipv6
        3 => "https://speed.neu6.edu.cn/",     // 东北大学 ipv6
        4 => "https://test6.nju.edu.cn/",      // 南京大学 ipv6
        5 => "https://speedtest6.shu.edu.cn/", // 上海大学 ipv6
        6 => "http://speed6.ujs.edu.cn/",      // 江苏大学 ipv6
        _ => return Err("未知测速网站".to_string()),
    };
    tauri::WindowBuilder::new(&app_handle, "speed_test", tauri::WindowUrl::App(url.into()))
        .build()
        .map_err(|e| format!("Error when building the speed_test window: {}", e))
        .map(|_| ())
}

#[tauri::command(async)]
pub fn check_has_browser(app_state: tauri::State<'_, AppState>) -> Result<bool, String> {
    if app_state.setting.read().unwrap().browser_path.is_some() {
        return Ok(true);
    }
    match get_browser_path() {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

#[tauri::command(async)]
pub fn set_browser_path(app_state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut browser_path: PathBuf;
    match FileDialog::new().pick_file() {
        Some(path) => browser_path = path.to_owned(),
        None => return Err("没有选择任何文件".to_string()),
    }
    if std::env::consts::OS == "macos" {
        let app_name = browser_path.file_name().unwrap().to_str().unwrap();
        browser_path.push(format!(
            "Contents/MacOS/{}",
            &app_name[..app_name.len() - 4]
        ));
    }

    let res = try_open_headless_browser(browser_path.clone());
    match res {
        Ok(()) => {
            // *app_state.browser.write().unwrap() = Some(b);
            // *app_state.tab.write().unwrap() = Some(t);
            // *app_state.browser_state.write().unwrap() = true;
            // do nothing
            app_state
                .setting
                .write()
                .unwrap()
                .set_browser_path(browser_path.to_str().map(|str: &str| str.to_owned()));
            Ok(())
        }
        Err(e) => Err(format!("在该路径找不到浏览器可执行文件：{}", e)),
    }
}

#[tauri::command(async)]
pub async fn load_ip_address() -> Result<String, String> {
    match get_address().await {
        Ok(ips) => Ok(format!("[\"{}\", \"{}\"]", ips[0], ips[1])),
        Err(e) => Err(format!("获取 IP 地址失败：{}", e)),
    }
}

#[tauri::command(async)]
pub fn get_jsessionid(app_state: tauri::State<'_, AppState>) -> Result<String, String> {
    Ok(app_state
        .jsessionid
        .read()
        .unwrap()
        .clone()
        .unwrap_or_default())
}

#[tauri::command(async)]
pub fn set_setting(app_state: tauri::State<'_, AppState>) -> Result<(), String> {
    app_state
        .setting
        .read()
        .unwrap()
        .write_setting()
        .map_err(|err| format!("{}", err))
}

#[tauri::command(async)]
pub fn load_setting(app_state: tauri::State<'_, AppState>) -> Result<String, String> {
    match Setting::load_setting() {
        Ok(setting) => {
            *app_state.setting.write().unwrap() = setting.clone();
            Ok(serde_json::to_string(&setting).unwrap())
        }
        Err(err) => Err(format!("{err}")),
    }
}
