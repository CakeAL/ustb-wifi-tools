use std::path::PathBuf;

use chrono::DateTime;
use rfd::FileDialog;
use tauri::{utils::config::WindowConfig, Manager};

use crate::{
    entities::{Account, AppState},
    requests::{
        get_address, get_load_user_flow, get_mac_address, get_month_pay, get_refresh_account,
        get_user_login_log, unbind_macs,
    },
    setting::Setting,
    utils::{
        get_browser_path, login_via_headless_browser, login_vpn_via_headless_browser,
        try_open_headless_browser,
    },
};

#[tauri::command(async)]
pub fn open_nav_login(app_handle: tauri::AppHandle) -> Result<(), String> {
    // 判断该窗口是否已存在
    if let Some(window) = app_handle.get_webview_window("nav_login") {
        window.close().map_err(|e| e.to_string())?;
    }

    tauri::WebviewWindowBuilder::new(
        &app_handle,
        "nav_login",
        tauri::WebviewUrl::App("http://202.204.60.7:8080/nav_login".into()),
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

// 没地方放它了
pub async fn load_user_flow(
    account: String,
    session_id: &str,
    via_vpn: bool,
) -> Result<String, String> {
    get_load_user_flow(&account, session_id, via_vpn)
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

    let browser_path = match app_state.setting.read().unwrap().browser_path.clone() {
        None => get_browser_path().unwrap(),
        Some(path) => {
            if path.is_empty() {
                get_browser_path().unwrap()
            } else {
                PathBuf::from(path)
            }
        }
    };
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
        Err(err) => return Err(format!("是否在校园网？或者其他问题：{}", err)),
    }
    Ok(app_state
        .jsessionid
        .read()
        .unwrap()
        .clone()
        .unwrap_or_default())
}

#[tauri::command(async)]
pub async fn get_cookie_vpn(
    app_state: tauri::State<'_, AppState>,
    user_name: String,
    password: String,
) -> Result<String, String> {
    let account = Account {
        user_name,
        password,
        check_code: None,
    };

    let browser_path = match app_state.setting.read().unwrap().browser_path.clone() {
        None => get_browser_path().unwrap(),
        Some(path) => {
            if path.is_empty() {
                get_browser_path().unwrap()
            } else {
                PathBuf::from(path)
            }
        }
    };
    let cookies =
        login_vpn_via_headless_browser(browser_path, &account).map_err(|err| err.to_string())?;

    dbg!(&cookies[3]); // wengine_vpn_ticketelib_ustb_edu_cn
    *app_state.jsessionid.write().unwrap() = cookies.get(3).map(|str| str.value.clone());
    *app_state.login_via_vpn.write().unwrap() = true;
    app_state
        .setting
        .write()
        .unwrap()
        .set_account(account.user_name, account.password);

    Ok(app_state
        .jsessionid
        .read()
        .unwrap()
        .clone()
        .unwrap_or_default())
}

#[tauri::command(async)]
pub fn logout(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
    window: tauri::Webview,
) -> Result<String, String> {
    if app_state.jsessionid.read().unwrap().is_none() {
        return Err("没登录之前不许登出😠".into());
    }
    *app_state.jsessionid.write().unwrap() = None;
    Setting::write_setting(&Setting {
        browser_path: app_state.setting.read().unwrap().browser_path.to_owned(),
        ..Default::default()
    }, &app)
    .map_err(|err| format!("写入配置错误: {}", err))?;
    window
        .eval("window.location.reload();")
        .map_err(|err| format!("刷新网页错误：{}", err))?;
    Ok("登出成功🤔".into())
}

#[tauri::command(async)]
pub async fn load_refresh_account(app_state: tauri::State<'_, AppState>) -> Result<String, String> {
    let session_id = match app_state.jsessionid.read().unwrap().clone() {
        Some(s) => s,
        None => return Err("SessionID为空，是否已经登录并单击获取Cookie按钮？".to_string()),
    };
    let via_vpn = *app_state.login_via_vpn.read().unwrap();
    match get_refresh_account(&session_id, via_vpn).await {
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
    let via_vpn = *app_state.login_via_vpn.read().unwrap();
    let session_id = match app_state.jsessionid.read().unwrap().clone() {
        Some(s) => s,
        None => return Err("SessionID为空，是否已经登录并单击获取Cookie按钮？".to_string()),
    };

    match account {
        Some(account) => Ok(get_load_user_flow(&account, &session_id, via_vpn)
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
    let via_vpn = *app_state.login_via_vpn.read().unwrap();

    match get_month_pay(&session_id, year, via_vpn).await {
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
    let via_vpn = *app_state.login_via_vpn.read().unwrap();

    match get_user_login_log(&session_id, &start_date, &end_date, via_vpn).await {
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
    let via_vpn = *app_state.login_via_vpn.read().unwrap();

    match get_mac_address(&session_id, via_vpn).await {
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
    let via_vpn = *app_state.login_via_vpn.read().unwrap();

    match unbind_macs(&session_id, &macs, via_vpn).await {
        Ok(Some(())) => Ok(()),
        Ok(None) => Err("请确认是否已经登录".to_string()),
        Err(e) => Err(format!("Request Error，检查是否在校园网内: {}", e)),
    }
}

#[tauri::command(async)]
pub fn open_speed_test(app_handle: tauri::AppHandle, site_num: i32) -> Result<(), String> {
    // 判断该窗口是否已存在
    if app_handle.get_webview_window("speed_test").is_some() {
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

    tauri::WebviewWindowBuilder::from_config(
        &app_handle,
        &WindowConfig {
            title: "测速".to_string(),
            label: "speed_test".to_string(),
            url: tauri::WebviewUrl::External(url.parse().unwrap()),
            // transparent: true,
            ..Default::default()
        },
    )
    .map_err(|e| format!("Error when building the speed_test window: {}", e))?
    .build()
    .map_err(|e| format!("Error when building the speed_test window: {}", e))
    .map(|_| ())
}

#[tauri::command(async)]
pub fn check_has_browser(app_state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let path = app_state.setting.read().unwrap().browser_path.clone();
    // 存在路径，并且路径不是空的
    if path.is_some() && !path.unwrap_or_default().is_empty() {
        return Ok(true);
    }
    match get_browser_path() {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

#[tauri::command(async)]
pub fn set_browser_path(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
    window: tauri::Webview,
) -> Result<(), String> {
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
            app_state
                .setting
                .write()
                .unwrap()
                .set_browser_path(browser_path.to_str().map(|str: &str| str.to_owned()));
            app_state
                .setting
                .write()
                .unwrap()
                .write_setting(&app)
                .map_err(|err| format!("写入配置错误: {}", err))?;
            window
                .eval("window.location.reload();")
                .map_err(|err| format!("刷新网页错误：{}", err))?;
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
pub fn set_setting(app: tauri::AppHandle, app_state: tauri::State<'_, AppState>) -> Result<(), String> {
    app_state
        .setting
        .read()
        .unwrap()
        .write_setting(&app)
        .map_err(|err| format!("{}", err))
}

#[tauri::command(async)]
pub fn load_setting(app: tauri::AppHandle, app_state: tauri::State<'_, AppState>) -> Result<String, String> {
    match Setting::load_setting(&app) {
        Ok(setting) => {
            *app_state.setting.write().unwrap() = setting.clone();
            Ok(serde_json::to_string(&setting).unwrap())
        }
        Err(err) => Err(format!("{err}")),
    }
}
