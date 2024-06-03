use crate::requests::get_load_user_flow;

#[tauri::command(async)]
pub async fn load_user_flow(account: String) -> Result<String, String> {
    let res = get_load_user_flow(&account).await;
    match res {
        Ok(res) => Ok(res.to_string()),
        _ => Err("Error while loading user flow".to_string()),
    }
}
