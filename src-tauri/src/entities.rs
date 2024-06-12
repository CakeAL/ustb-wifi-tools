use std::sync::Mutex;

use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
pub struct JsessionId(pub Mutex<Option<String>>);
