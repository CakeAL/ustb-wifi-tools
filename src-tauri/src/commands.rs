use chrono::DateTime;
use tauri::{utils::config::WindowConfig, Manager};

use crate::{
    entities::{AppState, EveryLoginData},
    requests::*,
    setting::Setting, utils,
};

#[tauri::command(async)]
pub async fn load_user_flow(
    account: String,
    app_state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let via_vpn = *app_state.login_via_vpn.read().unwrap();
    let mut session_id = String::new();
    if via_vpn {
        session_id = match app_state.jsessionid.read().unwrap().clone() {
            Some(s) => s,
            None => return Err("SessionID为空，是否已经登录并单击获取Cookie按钮？".to_string()),
        };
    }
    get_load_user_flow(&account, &session_id, via_vpn)
        .await
        .map_err(|e| format!("Error while loading user flow: {}", e))
        .map(|res| res.to_string())
}

#[tauri::command(async)]
pub async fn get_cookie(
    app_state: tauri::State<'_, AppState>,
    user_name: String,
    password: String,
) -> Result<String, String> {
    let res = simulate_login(&user_name, &password)
        .await
        .map_err(|err| err.to_string())?;
    match res {
        Some(cookie) => {
            dbg!(&cookie);
            *app_state.jsessionid.write().unwrap() = Some(cookie.clone());
            app_state
                .setting
                .write()
                .unwrap()
                .set_account(user_name, password);
        }
        None => return Err("用户名或密码错误！".into()),
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
    let res = simulate_login_via_vpn(&user_name, &password)
        .await
        .map_err(|err| err.to_string())?;

    match res {
        Some(cookie) => {
            dbg!(&cookie);
            *app_state.jsessionid.write().unwrap() = Some(cookie.clone());
            *app_state.login_via_vpn.write().unwrap() = true;
            app_state
                .setting
                .write()
                .unwrap()
                .set_account(user_name, password);
        }
        None => return Err("用户名或密码错误！".into()),
    }

    Ok(app_state
        .jsessionid
        .read()
        .unwrap()
        .clone()
        .unwrap_or_default())
}

#[tauri::command(async)]
pub fn logout(
    app_state: tauri::State<'_, AppState>,
    window: tauri::Webview,
) -> Result<String, String> {
    if app_state.jsessionid.read().unwrap().is_none() {
        return Err("没登录之前不许登出😠".into());
    }
    *app_state.jsessionid.write().unwrap() = None;
    *app_state.login_via_vpn.write().unwrap() = false; // 这之前有个bug一直没人发现，说明没人用我的 app 😭
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
        Ok(Some(value)) => Ok(serde_json::json!(value).to_string()),
        Ok(None) => Err("请确认是否已经登录".to_string()),
        Err(e) => Err(format!("Request Error，检查是否在校园网内: {}", e)),
    }
}

#[tauri::command(async)]
pub async fn load_monthly_login_log(
    app_state: tauri::State<'_, AppState>,
    start_date: i64,
    days: i64,
) -> Result<String, String> {
    let session_id = match app_state.jsessionid.read().unwrap().clone() {
        Some(s) => s,
        None => return Err("SessionID为空，是否已经登录并单击获取Cookie按钮？".to_string()),
    };
    let end_date = start_date + 3600 * 24 * days;
    let start_date_string = DateTime::from_timestamp(start_date, 0)
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    let end_date_string = DateTime::from_timestamp(end_date, 0)
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    let via_vpn = *app_state.login_via_vpn.read().unwrap();

    match get_user_login_log(&session_id, &start_date_string, &end_date_string, via_vpn).await {
        Ok(Some(value)) => {
            let mut flow_every_day: Vec<EveryLoginData> = vec![];
            for i in 0..days {
                let mut sum = EveryLoginData::default();
                value
                    .every_login_data
                    .iter()
                    .filter(|a_data| {
                        // a_data.online_time > start_date + i * 24 * 3600
                        //     && a_data.online_time < start_date + (i + 1) * 24 * 3600
                        // 过滤，只要今天到今晚, 学校的网站上是按照下线时间算的
                        a_data.offline_time >= start_date + i * 24 * 3600
                            && a_data.offline_time < start_date + (i + 1) * 24 * 3600
                    })
                    .for_each(|data| {
                        sum.cost += data.cost;
                        sum.ipv4_down += data.ipv4_down;
                        sum.ipv4_up += data.ipv4_up;
                        sum.ipv6_down += data.ipv6_down;
                        sum.ipv6_up += data.ipv6_up;
                        sum.used_duration += data.used_duration;
                        sum.used_flow += data.used_flow;
                    });
                flow_every_day.push(sum);
            }
            Ok(serde_json::json!(flow_every_day).to_string())
        }
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
pub fn set_setting(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    app_state
        .setting
        .read()
        .unwrap()
        .write_setting(&app)
        .map_err(|err| format!("{}", err))
}

#[tauri::command(async)]
pub fn load_setting(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    match Setting::load_setting(&app) {
        Ok(setting) => {
            *app_state.setting.write().unwrap() = setting.clone();
            Ok(serde_json::to_string(&setting).unwrap())
        }
        Err(err) => Err(format!("{err}")),
    }
}

#[tauri::command(async)]
pub async fn manually_check_update(app: tauri::AppHandle) -> Result<(), String> {
    #[cfg(not(target_os = "android"))]
    crate::update(app, true)
        .await
        .map_err(|err| err.to_string())?;

    if cfg!(target_os = "android") {
        Err("安卓暂时不支持更新，请到 GitHub 查看是否有更新。".into())
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn load_ammeter(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
    ammeter_number: u32,
) -> Result<String, String> {
    let kwh = get_ammeter(ammeter_number)
        .await
        .map_err(|err| err.to_string())?;
    match kwh {
        Some(kwh) => {
            app_state
                .setting
                .write()
                .unwrap()
                .set_ammeter_number(ammeter_number);
            app_state
                .setting
                .read()
                .unwrap()
                .write_setting(&app)
                .map_err(|err| err.to_string())?;
            Ok(format!("{}", kwh))
        }
        None => Err("获取用电量失败，可能是电表号错误".to_string()),
    }
}

#[tauri::command]
pub async fn submit_login_ustb_wifi(user_name: String, password: String) -> Result<String, String> {
    match login_ustb_wifi(&user_name, &password).await {
        Ok(()) => Ok("登录成功".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn return_os_type() -> i32 {
    if cfg!(target_os = "windows") {
        if utils::get_windows_build_number() >= 22000 {
            1 // win11
        } else {
            2 // win10 及以下
        }
    } else if cfg!(target_os = "macos") {
        3 // macOS
    } else {
        4 // linux or android
    }
}