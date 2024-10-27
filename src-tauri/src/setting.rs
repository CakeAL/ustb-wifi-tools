use std::{
    fs::{create_dir, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Setting {
    pub username: Option<String>,
    pub password: Option<String>,
    pub ammeter_number: Option<u32>,
}

impl Setting {
    pub fn new() -> Self {
        Setting::default()
    }

    pub fn load_setting(app: &tauri::AppHandle) -> Result<Self> {
        match File::open(get_config_path(app)?) {
            Ok(mut json_file) => {
                let mut buf = String::new();
                json_file.read_to_string(&mut buf)?;
                Ok(serde_json::from_str::<Setting>(&buf).unwrap_or_default())
            }
            // 没有该文件
            Err(_) => Ok(Setting::new()),
        }
    }

    pub fn write_setting(&self, app: &tauri::AppHandle) -> Result<()> {
        let json_str = serde_json::to_string(self)?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(get_config_path(app)?)?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }

    pub fn set_account(&mut self, username: String, password: String) {
        self.username = Some(username);
        self.password = Some(password);
    }

    pub fn set_ammeter_number(&mut self, ammeter_number: u32) {
        self.ammeter_number = Some(ammeter_number);
    }
}

fn get_config_path(app: &tauri::AppHandle) -> Result<PathBuf> {
    let path = match app.path().data_dir().ok() {
        Some(mut p) => {
            p.push("ustb-wifi-tools");
            if !p.exists() {
                // 如果不存在这个文件夹先创建
                create_dir(&p)?
            }
            p.push("config.json");
            p
        }
        None => return Err(anyhow!("There is no such app data dir!")),
    };
    dbg!(&path);
    Ok(path)
}
