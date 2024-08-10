use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use crate::setting::Setting;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Flow {
//     pub v4: f64,
//     pub v6: f64,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserFlow {
//     pub result: i32,
//     pub data: Flow,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct GetUserFlowFailed {
//     pub result: i32,
//     pub msg: String,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub user_name: String,
    pub password: String,
    pub check_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthPayInfo {
    pub year_cost: f64,
    pub year_used_duration: u32,
    pub year_used_flow: f64,
    pub monthly_data: Vec<MonthlyData>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MonthlyData {
    pub month: u8,
    pub month_cost: f64,
    pub month_used_flow: f64,
    pub month_used_duration: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoginLog {
    pub ipv4_up: f64,
    pub ipv4_down: f64,
    pub ipv6_up: f64,
    pub ipv6_down: f64,
    pub used_flow: f64, // 实际上就是ipv4下行
    pub cost: f64,
    pub used_duration: u32,
    pub every_login_data: Vec<EveryLoginData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EveryLoginData {
    pub online_time: i64, // 时间戳，UTC
    pub offline_time: i64,
    pub used_duration: u32,
    pub used_flow: f64, // 实际上就是ipv4下行
    pub cost: f64,
    pub ipv4_up: f64,
    pub ipv4_down: f64,
    pub ipv6_up: f64,
    pub ipv6_down: f64,
    pub ipv4_addr: String,
    pub ipv6_addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacAddress {
    pub device_name: String,
    pub mac_address: String,
}

pub struct AppState {
    pub jsessionid: RwLock<Option<String>>,
    pub setting: RwLock<Setting>,
    pub login_via_vpn: RwLock<bool>,
}
