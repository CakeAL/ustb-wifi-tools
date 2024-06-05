use tauri::Manager;

use crate::{requests::get_load_user_flow, utils::get_webview2_cookie};

#[tauri::command(async)]
pub async fn load_user_flow(account: String) -> Result<String, String> {
    let res = get_load_user_flow(&account).await;
    match res {
        Ok(res) => Ok(res.to_string()),
        _ => Err("Error while loading user flow".to_string()),
    }
}

// 触发用来获取WebView当前页面的Cookie
#[tauri::command(async)]
pub async fn get_cookie(app_handle: tauri::AppHandle) -> Result<String, String> {
    if cfg!(target_os = "windows") {
        let windows = app_handle.windows();
        #[allow(unused_variables)]
        let url = "https://tauri.localhost/";
        #[cfg(debug_assertions)] // 如果是 debug 模式，把 url 替换为debug的
        let url = "http://localhost:1420/";
        let res = get_webview2_cookie(windows.get("main").unwrap(), url).await;
        match res {
            Ok(cookies) => {
                println!("{:?}", cookies[0])
            }
            Err(_) => return Err("can't get cookies".to_string()),
        }
    }
    Ok("Ok".to_string())
}
