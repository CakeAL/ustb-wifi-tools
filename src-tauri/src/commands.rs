use tauri::Manager;

use crate::{
    entities::JsessionId,
    requests::{get_load_user_flow, get_refresh_account},
    utils::get_webview2_cookie,
};

#[tauri::command(async)]
pub fn open_nav_login(app_handle: tauri::AppHandle) -> Result<(), String> {
    let nav_login_window = tauri::WindowBuilder::new(
        &app_handle,
        "nav_login",
        tauri::WindowUrl::App("http://202.204.60.7:8080/nav_login".into()),
    )
    .build();
    // 什么Golang😡
    if nav_login_window.is_ok() != true {
        return Err("Error when building the nav_login window".into());
    };

    Ok(())
}

#[tauri::command(async)]
pub async fn load_user_flow(account: String) -> Result<String, String> {
    let res = get_load_user_flow(&account).await;
    match res {
        Ok(res) => Ok(res.to_string()),
        _ => Err("Error while loading user flow".to_string()),
    }
}

// 用来获取WebView当前页面的Cookie
#[tauri::command(async)]
pub async fn get_cookie(
    app_state: tauri::State<'_, JsessionId>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
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
                    *app_state.0.lock().unwrap() = cookies.get(0).map(|str| str.value.clone());
                }
                Err(_) => return Err("Can't get cookies due to unknown error".to_string()),
            }
        } else {
            return Err("Please open the login window.".into());
        }
    }
    Ok(())
}

#[tauri::command(async)]
pub async fn load_refresh_account(
    app_state: tauri::State<'_, JsessionId>,
) -> Result<String, String> {
    let session_id = match app_state.0.lock().unwrap().clone() {
        Some(s) => s,
        None => return Err("SessionID为空，是否已经登录并单击获取Cookie按钮？".to_string()),
    };
    let res = match get_refresh_account(&session_id).await {
        Ok(Some(str)) => str,
        Ok(None) => return Err("请确认是否已经登录".to_string()),
        Err(_) => return Err("Request Error，检查是否在校园网内".to_string()),
    };
    // dbg!(res);
    Ok(res)
}
