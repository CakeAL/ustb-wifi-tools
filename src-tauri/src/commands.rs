use std::{net::IpAddr, time::Duration};

use chrono::DateTime;
use reqwest::Client;
use serde::Serialize;
use tauri::{Manager, ipc::Channel, utils::config::WindowConfig};

use crate::{
    electric_bill::update_ammeter,
    entities::{AppState, DownloadEvent, UserType},
    requests::*,
    setting::Setting,
    utils::{get_cookie_str, get_store_path, webvpn},
};

#[cfg(not(any(target_os = "android", target_os = "linux")))]
use crate::utils::update;

// #[tauri::command(async)]
// pub async fn load_user_flow(
//     account: String,
//     app_state: tauri::State<'_, AppState>,
// ) -> Result<String, String> {
//     let user_type = *app_state.user_type.read().await;
//     match user_type {
//         UserType::Normal | UserType::ViaVpn => {
//             let cookie = get_cookie_str(&app_state).await?;
//             get_load_user_flow(&account, &cookie, user_type)
//                 .await
//                 .map_err(|e| format!("Error while loading user flow: {}", e))
//                 .map(|res| res.to_string())
//         }
//         UserType::LocalUser => Err("本地存储不适用此功能".to_string()),
//     }
// }

#[tauri::command(async)]
pub async fn get_cookie(
    app: tauri::AppHandle,
    user_name: String,
    password: String,
    via_vpn: bool,
) -> Result<Option<String>, String> {
    let app_state = app.state::<AppState>();

    let (cookie_str, user_dashboard) = if !via_vpn {
        simulate_login(&user_name, &password)
            .await
            .map_err(|err| err.to_string())?
    } else {
        simulate_login_via_vpn(&user_name, &password)
            .await
            .map_err(|err| err.to_string())?
    };
    match cookie_str {
        Some(cookie_str) => {
            dbg!(&cookie_str);
            *app_state.cookie_str.write().await = Some(cookie_str.clone());
            *app_state.user_type.write().await = if via_vpn {
                UserType::ViaVpn
            } else {
                UserType::Normal
            };
            app_state
                .setting
                .write()
                .await
                .set_account(user_name, password);
            app_state
                .setting
                .read()
                .await
                .write_setting(&app)
                .map_err(|e| e.to_string())?;
            cookie_str
        }
        None => return Err("用户名或密码错误！".into()),
    };
    Ok(user_dashboard)
}

#[tauri::command(async)]
pub async fn logout(
    app_state: tauri::State<'_, AppState>,
    window: tauri::Webview,
) -> Result<String, String> {
    if app_state.cookie_str.read().await.is_none() {
        return Err("没登录之前不许登出😠".into());
    }
    *app_state.cookie_str.write().await = None;
    *app_state.user_type.write().await = UserType::default(); // 这之前有个bug一直没人发现，说明没人用我的 app 😭
    window
        .eval("window.location.reload();")
        .map_err(|err| format!("刷新网页错误：{}", err))?;
    Ok("登出成功🤔".into())
}

#[tauri::command(async)]
pub async fn refresh_user_dashboard(
    app_state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let cookie_str = get_cookie_str(&app_state).await?;
    let user_type = *app_state.user_type.read().await;
    match get_user_dashboard(&cookie_str, user_type).await {
        Ok(Some(str)) => Ok(str),
        Ok(None) => Err("请确认是否已经登录".to_string()),
        Err(e) => Err(format!("Request Error，检查是否在校园网内: {}", e)),
    }
}

#[tauri::command(async)]
pub async fn load_online_list(app_state: tauri::State<'_, AppState>) -> Result<String, String> {
    let cookie_str = get_cookie_str(&app_state).await?;
    let user_type = *app_state.user_type.read().await;
    get_online_list(&cookie_str, user_type)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command(async)]
pub async fn load_login_history(app_state: tauri::State<'_, AppState>) -> Result<String, String> {
    let cookie_str = get_cookie_str(&app_state).await?;
    let user_type = *app_state.user_type.read().await;
    get_login_history(&cookie_str, user_type)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command(async)]
pub async fn do_to_offline(
    app_state: tauri::State<'_, AppState>,
    session_id: String,
) -> Result<(), String> {
    let cookie_str = get_cookie_str(&app_state).await?;
    let user_type = *app_state.user_type.read().await;
    to_offline(&cookie_str, user_type, &session_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command(async)]
pub async fn load_month_pay(app: tauri::AppHandle, year: u16) -> Result<String, String> {
    let app_state = app.state::<AppState>();
    let cookie_str = get_cookie_str(&app_state).await?;
    let user_type = *app_state.user_type.read().await;
    // if let UserType::LocalUser = user_type {
    //     let month_pay_info = app_state
    //         .cur_account
    //         .read()
    //         .await
    //         .get_local_month_pay(&app, year)
    //         .map_err(|e| e.to_string())?;
    //     return Ok(serde_json::json!(month_pay_info).to_string());
    // }

    get_month_pay(&cookie_str, year, user_type)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command(async)]
pub async fn load_user_online_log(
    app: tauri::AppHandle,
    start_date: i64,
    end_date: i64,
) -> Result<String, String> {
    let app_state = app.state::<AppState>();
    if start_date > end_date {
        return Err("起始日期比结束日期更大。。。".to_string());
    }
    let cookie_str = get_cookie_str(&app_state).await?;
    let user_type = *app_state.user_type.read().await;

    match user_type {
        UserType::Normal | UserType::ViaVpn => {
            let start_date = DateTime::from_timestamp(start_date, 0)
                .unwrap()
                .format("%Y-%m-%d")
                .to_string();
            let end_date = DateTime::from_timestamp(end_date, 0)
                .unwrap()
                .format("%Y-%m-%d")
                .to_string();
            get_user_online_log(&cookie_str, &start_date, &end_date, user_type).await
        }
    }
    .map_err(|e| {
        if e.to_string() == "NO DATA" {
            "目前暂时没有该数据".to_string()
        } else {
            format!("请检查网络或登录情况: {}", e)
        }
    })
}

#[tauri::command(async)]
pub async fn load_mac_address(app_state: tauri::State<'_, AppState>) -> Result<String, String> {
    let cookie_str = get_cookie_str(&app_state).await?;
    let user_type = *app_state.user_type.read().await;

    get_mac_address(&cookie_str, user_type)
        .await
        .map(|res| serde_json::to_string(&res).unwrap_or_default())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_mac_custom_name(
    app_state: tauri::State<'_, AppState>,
    mac_address: String,
    terminal_name: String,
    ajax_csrf_token: String,
) -> Result<String, String> {
    let cookie_str = get_cookie_str(&app_state).await?;
    let user_type = *app_state.user_type.read().await;
    update_terminal_name(
        &cookie_str,
        user_type,
        &mac_address,
        &terminal_name,
        &ajax_csrf_token,
    )
    .await
    .map_err(|e| e.to_string())
}

#[derive(Debug, Serialize)]
struct MacAddress {
    iface_name: String,
    mac_address: String,
}

#[tauri::command]
pub fn get_current_device_mac() -> Result<String, String> {
    use if_addrs::Interface;
    let ifaces = if_addrs::get_if_addrs()
        .map_err(|e| e.to_string())?
        .into_iter()
        .filter_map(|iface| match iface.addr {
            if_addrs::IfAddr::V4(_) if !iface.is_loopback() => Some(iface),
            _ => None,
        })
        .collect::<Vec<Interface>>();

    let macs: Vec<MacAddress> = ifaces
        .into_iter()
        .map(|iface| MacAddress {
            iface_name: iface.name.clone(),
            mac_address: mac_address::mac_address_by_name(&iface.name)
                .unwrap_or_default()
                .unwrap_or_default()
                .to_string()
                .replace(':', ""),
        })
        .collect();
    Ok(serde_json::json!(macs).to_string())
}

// 传进来的应该是不需要解绑的，提醒。
#[tauri::command(async)]
pub async fn do_unbind_mac(
    app_state: tauri::State<'_, AppState>,
    mac: String,
    ajax_csrf_token: String,
) -> Result<(), String> {
    let cookie_str = get_cookie_str(&app_state).await?;
    let user_type = *app_state.user_type.read().await;

    unbind_mac(&cookie_str, user_type, &mac, &ajax_csrf_token)
        .await
        .map_err(|e| e.to_string())
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
pub async fn get_stored_cookie_str(
    app_state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    get_cookie_str(&app_state).await
}

#[tauri::command(async)]
pub async fn load_setting(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    match Setting::load_setting(&app) {
        Ok(setting) => {
            *app_state.setting.write().await = setting.clone();
            Ok(serde_json::to_string(&setting).unwrap())
        }
        Err(err) => Err(format!("{err}")),
    }
}

#[tauri::command(async)]
pub async fn set_background_image(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    use tauri_plugin_dialog::DialogExt;
    let path = app
        .dialog()
        .file()
        .add_filter("", &["png", "jpg", "jpeg"])
        .blocking_pick_file();
    dbg!(&path);
    if let Some(path) = path {
        app_state
            .setting
            .write()
            .await
            .set_background_image_path(&app, &path.into_path().map_err(|err| err.to_string())?)
            .map_err(|err| err.to_string())?;
        app_state
            .setting
            .read()
            .await
            .write_setting(&app)
            .map_err(|err| err.to_string())?;
        Ok(())
    } else {
        Err("请选择一个图片".into())
    }
}

#[tauri::command(async)]
pub async fn reset_background_image(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    app_state.setting.write().await.reset_background_image();
    app_state
        .setting
        .read()
        .await
        .write_setting(&app)
        .map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command(async)]
pub async fn set_background_transparence(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
    transparence: u32,
) -> Result<(), String> {
    app_state
        .setting
        .write()
        .await
        .set_background_transparence(transparence);
    app_state
        .setting
        .read()
        .await
        .write_setting(&app)
        .map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command(async)]
pub async fn set_background_blur(
    app: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
    blur: u32,
) -> Result<(), String> {
    app_state.setting.write().await.set_background_blur(blur);
    app_state
        .setting
        .read()
        .await
        .write_setting(&app)
        .map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command(async)]
pub async fn manually_check_update(
    app: tauri::AppHandle,
    manually: bool,
    on_event: Channel<DownloadEvent>,
) -> Result<(), String> {
    static mut AUTO_CHECK: bool = true; // 只能自动检查更新一次

    #[cfg(not(any(target_os = "android", target_os = "linux")))]
    if unsafe { AUTO_CHECK } || manually {
        // 如果第一次自动或者手动更新
        update(app, manually, on_event)
            .await
            .map_err(|err| err.to_string())?;
    }
    if !manually {
        unsafe {
            AUTO_CHECK = false;
        }
    }
    if cfg!(target_os = "android") || cfg!(target_os = "linux") {
        Err("安卓/Linux 不支持更新，请到 GitHub 查看是否有更新。".into())
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn load_ammeter(app: tauri::AppHandle, ammeter_number: u32) -> Result<String, String> {
    let app_state = app.state::<AppState>();
    let kwh = get_ammeter(ammeter_number)
        .await
        .map_err(|err| err.to_string())?;
    match kwh {
        Some(kwh) => {
            app_state
                .setting
                .write()
                .await
                .set_ammeter_number(ammeter_number);
            app_state
                .setting
                .read()
                .await
                .write_setting(&app)
                .map_err(|err| err.to_string())?;
            Ok(kwh.to_string())
        }
        None => Err("获取用电量失败，可能是电表号错误".to_string()),
    }
}

#[tauri::command(async)]
pub async fn load_electric_bill(app: tauri::AppHandle) -> Result<String, String> {
    let app_state = app.state::<AppState>();
    let ammeter_number = app_state
        .setting
        .read()
        .await
        .ammeter_number
        .ok_or("无已存储电表号".to_string())?;
    let file_path = get_store_path(&app)
        .map_err(|e| e.to_string())?
        .join(format!("{}.json", ammeter_number));
    let res = update_ammeter(ammeter_number, file_path)
        .await
        .map_err(|e| e.to_string())?;
    Ok(serde_json::json!(res).to_string())
}

#[tauri::command(async)]
pub async fn submit_login_ustb_wifi(user_name: String, password: String) -> Result<String, String> {
    // 尝试 10 次登录
    let mut err = String::new();
    for _ in 0..10 {
        match login_ustb_wifi(&user_name, &password).await {
            Ok(()) => return Ok("登录成功".to_string()),
            Err(e) => err = e.to_string(),
        }
        // 不是，这登录为什么还不是每次都一定能登录上的啊😅
        // 大概是因为解绑 MAC 地址之后，需要给校园网后台留出处理时间
        dbg!(&err);
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    // 返回最后一次错误
    Err(err)
}

#[tauri::command]
pub async fn return_os_type() -> i32 {
    #[allow(unused_assignments)]
    let mut res = 0; // others

    #[cfg(target_os = "windows")]
    if crate::utils::get_windows_build_number() >= 22000 {
        res = 1; // win11
    } else {
        res = 2; // win10 及以下
    }

    #[cfg(target_os = "macos")]
    {
        res = 3; // macos
    }

    res
}

#[tauri::command(async)]
pub async fn collapse(
    app_state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
    value: bool,
) -> Result<(), String> {
    app_state.setting.write().await.set_collapsed(value);
    let _ = app_state.setting.write().await.write_setting(&app);
    Ok(())
}

#[tauri::command(async)]
pub async fn get_ip_location(ip: String) -> Result<String, String> {
    if let Err(e) = ip.parse::<IpAddr>() {
        return Err(format!("IP 格式错误：{e:?}"));
    }

    let response = Client::new()
        .get(format!("https://api.mir6.com/api/ip_json?ip={}", ip))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;

    Ok(text)
}

#[tauri::command]
pub fn translate_up(raw_url: String) -> String {
    match webvpn::translate_up(&raw_url) {
        Ok(res) => res,
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
pub fn translate_down(vpn_url: String) -> String {
    match webvpn::translate_down(&vpn_url) {
        Ok(res) => res,
        Err(e) => e.to_string(),
    }
}

// #[tauri::command(async)]
// pub async fn switch_login_ustb_wifi(
//     app_state: tauri::State<'_, AppState>,
//     user_name: String,
//     password: String,
// ) -> Result<String, String> {
//     // 获取本机 mac 地址
//     use if_addrs::Interface;
//     let ifaces = if_addrs::get_if_addrs()
//         .map_err(|e| e.to_string())?
//         .into_iter()
//         .filter_map(|iface| match iface.addr {
//             if_addrs::IfAddr::V4(_) if !iface.is_loopback() => Some(iface),
//             _ => None,
//         })
//         .collect::<Vec<Interface>>();

//     let cur_device_macs: HashSet<String> = ifaces
//         .into_iter()
//         .map(|iface| {
//             mac_address::mac_address_by_name(&iface.name)
//                 .unwrap_or_default()
//                 .unwrap_or_default()
//                 .to_string()
//                 .replace(':', "")
//         })
//         .collect();

//     // 获取该账号校园网记住的 mac 地址
//     let cookie_str = get_cookie_str(&app_state).await?;
//     let user_type = *app_state.user_type.read().await;
//     if let UserType::LocalUser | UserType::ViaVpn = user_type {
//         return Err("无法使用当前功能".to_string());
//     }

//     let macs = match get_mac_address(&cookie_str, user_type).await {
//         Ok(Some(value)) => Ok(value),
//         Ok(None) => Err("请确认是否已经登录".to_string()),
//         Err(e) => Err(format!("Request Error，检查是否在校园网内: {}", e)),
//     }?
//     .into_iter()
//     .map(|v| v.mac_address)
//     .collect::<HashSet<String>>();

//     // 取差集，减去当前设备的匹配的校园网后台已存在的 MAC 地址
//     let diff_macs = macs
//         .difference(&cur_device_macs)
//         .cloned()
//         .collect::<Vec<String>>();
//     // dbg!(&diff_macs);
//     if diff_macs.len() == macs.len() {
//         return Err("无法匹配 MAC 地址，请确认当前账号是否已经在这台设备登录了。".to_string());
//     }
//     match unbind_macs(&cookie_str, &diff_macs, user_type).await {
//         Ok(Some(())) => Ok(()),
//         Ok(None) => Err("请确认是否已经登录".to_string()),
//         Err(e) => Err(format!("Request Error，检查是否在校园网内: {}", e)),
//     }?;

//     // 登录新账号
//     tokio::time::sleep(Duration::from_millis(200)).await;
//     submit_login_ustb_wifi(user_name, password).await
// }

#[tauri::command(async)]
pub async fn get_current_user_name(
    app_state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    Ok(app_state.cur_account.read().await.clone())
}

#[tauri::command(async)]
pub async fn set_current_user_name(
    app_state: tauri::State<'_, AppState>,
    user_name: String,
) -> Result<(), String> {
    match *app_state.user_type.read().await {
        UserType::ViaVpn | UserType::Normal => {
            *app_state.cur_account.write().await = user_name;
        }
    }
    Ok(())
}
