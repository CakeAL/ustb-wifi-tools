use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use rand::Rng;
use reqwest::Client;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tauri::{Manager, WebviewWindow};
use tauri_plugin_dialog::DialogExt;

use crate::entities::AppState;

#[tauri::command(async)]
pub async fn open_microsoft_login(app_handle: tauri::AppHandle) -> Result<(), String> {
    #[allow(unused_variables)]
    let url = "https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id=6c2e411f-bea9-4598-8cb9-bebadac59bdc&scope=Files.ReadWrite%20offline_access&response_type=code&redirect_uri=https%3A%2F%2Ftauri.localhost%2F";
    #[cfg(debug_assertions)]
    let url = "https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id=6c2e411f-bea9-4598-8cb9-bebadac59bdc&scope=Files.ReadWrite%20offline_access&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A1420%2F";

    let code_verifier = generate_random_string(128);
    let code_challenge = sha256_base64url(&code_verifier);
    *app_handle
        .state::<AppState>()
        .onedrive_code_verifier
        .write()
        .unwrap() = Some(code_verifier);

    let url = format!(
        "{}&code_challenge={}&code_challenge_method=S256",
        url, code_challenge
    );

    WebviewWindow::builder(
        &app_handle,
        "Onedrive",
        tauri::WebviewUrl::External(url.parse().unwrap()),
    )
    .inner_size(480.0, 670.0)
    .title("Onedrive 登录")
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

#[tauri::command(async)]
pub async fn code_to_access_token(
    app_handle: tauri::AppHandle,
    code: String,
) -> Result<(), String> {
    let window = app_handle
        .get_webview_window("Onedrive")
        .ok_or("?".to_string())?;
    let _ = window.hide();
    let code_verifier = app_handle
        .state::<AppState>()
        .onedrive_code_verifier
        .read()
        .unwrap()
        .clone()
        .ok_or("?".to_string())?;

    let response = match Client::new()
        .post("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Origin", "http://localhost:1420")
        .form(&[
            ("client_id", "6c2e411f-bea9-4598-8cb9-bebadac59bdc"),
            ("redirect_uri", "http://localhost:1420/"),
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
            let _ = window.close();
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
        false => download_setting_to_onedrive().await,
    }

    let _ = window.close();
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
    let response = Client::new()
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

async fn download_setting_to_onedrive() {}

fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
        .collect()
}

fn sha256_base64url(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE.encode(hash).trim_end_matches('=').to_string()
}
