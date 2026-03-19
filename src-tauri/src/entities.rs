use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::setting::Setting;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub user_name: String,
    pub password: String,
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacList {
    total: i32,
    rows: Vec<Vec<String>>,
}

#[derive(Default, Clone, Copy)]
pub enum UserType {
    #[default]
    Normal,
    ViaVpn,
}

#[derive(Default)]
pub struct AppState {
    pub cookie_str: RwLock<Option<String>>,
    pub cur_account: RwLock<String>,
    pub setting: RwLock<Setting>,
    pub user_type: RwLock<UserType>,
    pub onedrive_code_verifier: RwLock<Option<String>>, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmmeterData {
    #[serde(rename = "ServiceKey")]
    pub service_key: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemainingElectricity {
    pub date: i64,
    pub remain: i32,
    pub average: f64,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum DownloadEvent {
    #[serde(rename_all = "camelCase")]
    Started { new_version: bool },
    #[serde(rename_all = "camelCase")]
    Progress {
        downloaded: usize,
        content_length: u64,
    },
    #[serde(rename_all = "camelCase")]
    Finished { finished: bool },
}
