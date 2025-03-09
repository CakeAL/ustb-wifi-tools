use crate::{entities::AppState, requests::CLIENT, setting::Setting};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tauri::{Manager, WebviewWindow};
use tauri_plugin_dialog::DialogExt;

#[tauri::command(async)]
pub async fn open_microsoft_login(app_handle: tauri::AppHandle) -> Result<(), String> {
    let url: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id=6c2e411f-bea9-4598-8cb9-bebadac59bdc&scope=Files.ReadWrite%20offline_access&response_type=code";

    let code_verifier = generate_random_string(128);
    let code_challenge = sha256_base64url(&code_verifier);
    *app_handle
        .state::<AppState>()
        .onedrive_code_verifier
        .write()
        .unwrap() = Some(code_verifier);

    let url = format!(
        "{}&code_challenge={}&code_challenge_method=S256&redirect_uri=https%3A%2F%2Flogin.microsoftonline.com%2Fcommon%2Foauth2%2Fnativeclient",
        url, code_challenge
    );
    let app_handle = app_handle.clone();
    let mut win = WebviewWindow::builder(
        &app_handle,
        "Onedrive",
        tauri::WebviewUrl::External(url.parse().unwrap()),
    );

    #[cfg(not(target_os = "android"))]
    {
        win = win.inner_size(480.0, 670.0).title("Onedrive 登录");
    }

    let app_handle = app_handle.clone();
    win.on_navigation(move |url| {
        dbg!(url.path());
        if url.path() == "/common/oauth2/nativeclient" {
            let pair = url.query_pairs().next().unwrap();
            let code = pair.1.to_string();
            // dbg!(code);
            let app_handle = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                let _ = code_to_access_token(app_handle, code).await;
            });
        }
        true
    })
    .build()
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
struct TokenResponse {
    token_type: Option<String>,
    expires_in: Option<u64>,
    scope: Option<String>,
    access_token: Option<String>,
    refresh_token: Option<String>,
}

async fn code_to_access_token(app_handle: tauri::AppHandle, code: String) -> Result<(), String> {
    dbg!(&code);
    let window = app_handle
        .get_webview_window("Onedrive")
        .ok_or("?".to_string())?;

    #[cfg(not(target_os = "android"))]
    {
        let _ = window.hide();
    }

    let code_verifier = app_handle
        .state::<AppState>()
        .onedrive_code_verifier
        .read()
        .unwrap()
        .clone()
        .ok_or("?".to_string())?;

    let response = match CLIENT
        .post("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        // .header("Origin", "https://login.microsoftonline.com/common/oauth2/nativeclient")
        .form(&[
            ("client_id", "6c2e411f-bea9-4598-8cb9-bebadac59bdc"),
            (
                "redirect_uri",
                "https://login.microsoftonline.com/common/oauth2/nativeclient",
            ),
            ("code", &code),
            ("grant_type", "authorization_code"),
            ("code_verifier", &code_verifier),
        ])
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            dbg!(&e);
            app_handle
                .dialog()
                .message("由于网络问题失败。")
                .blocking_show();
            #[cfg(not(target_os = "android"))]
            {
                let _ = window.close();
            }
            return Err(e.to_string());
        }
    };

    let text = response.text().await.unwrap();
    // dbg!(&text);
    let token_response: TokenResponse = serde_json::from_str(&text).unwrap();

    let ans = app_handle
        .dialog()
        .message("在 OneDrive 上传配置文件还是下载？")
        .title("同步选项")
        .buttons(tauri_plugin_dialog::MessageDialogButtons::OkCancelCustom(
            "上传".to_string(),
            "下载".to_string(),
        ))
        .blocking_show();

    match ans {
        true => upload_setting_to_onedrive(&app_handle, token_response).await,
        false => download_setting_to_onedrive(&app_handle, token_response).await,
    }

    #[cfg(not(target_os = "android"))]
    {
        let _ = window.close();
    }
    Ok(())
}

async fn upload_setting_to_onedrive(app_handle: &tauri::AppHandle, token_response: TokenResponse) {
    let state = app_handle
        .state::<AppState>()
        .setting
        .read()
        .unwrap()
        .clone();
    let s = serde_json::json!(&state).to_string();
    let s = URL_SAFE.encode(&s);
    let response = CLIENT
        .put("https://graph.microsoft.com/v1.0/drive/special/approot:/setting.txt:/content")
        .bearer_auth(token_response.access_token.unwrap())
        .header("Content-Type", "text/plain")
        .body(s)
        .send()
        .await;
    if let Err(e) = response {
        app_handle
            .dialog()
            .message("上传失败！可能由于网络问题")
            .blocking_show();
        println!("Error uploading setting: {}", e);
    } else {
        app_handle.dialog().message("上传成功！").blocking_show();
        // dbg!(response.unwrap().text().await);
    }
}

async fn download_setting_to_onedrive(
    app_handle: &tauri::AppHandle,
    token_response: TokenResponse,
) {
    let response = CLIENT
        .get("https://graph.microsoft.com/v1.0/drive/special/approot:/setting.txt:/content")
        .bearer_auth(token_response.access_token.unwrap())
        .send()
        .await;
    if let Err(e) = response {
        app_handle
            .dialog()
            .message("下载失败！可能由于网络问题")
            .blocking_show();
        println!("Error downloading setting: {}", e);
    } else {
        let text = response.unwrap().text().await.unwrap();
        let text = match URL_SAFE.decode(text.as_bytes()) {
            Ok(text) => text,
            Err(e) => {
                app_handle
                    .dialog()
                    .message(format!("解码base64错误：{e:?}"))
                    .blocking_show();
                return;
            }
        };
        let setting: Setting = match serde_json::from_slice(&text) {
            Ok(s) => s,
            Err(e) => {
                app_handle
                    .dialog()
                    .message(format!("配置文件格式不正确！{e:?}"))
                    .blocking_show();
                return;
            }
        };
        // dbg!(&setting);
        let state = app_handle.state::<AppState>();
        *state.setting.write().unwrap() = setting;
        let _ = state.setting.write().unwrap().write_setting(app_handle);
        app_handle.dialog().message("下载成功！").blocking_show();
        // dbg!(response.unwrap().text().await);
    }
}

fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
    let mut rng = rand::rng();
    (0..length)
        .map(|_| CHARSET[rng.random_range(0..CHARSET.len())] as char)
        .collect()
}

fn sha256_base64url(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE.encode(hash).trim_end_matches('=').to_string()
}
