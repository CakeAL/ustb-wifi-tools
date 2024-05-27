use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Flow {
    pub v4: f64,
    pub v6: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserFlow {
    pub result: i32,
    pub data: Flow,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserFlowFailed {
    pub result: i32,
    pub msg: String,
}