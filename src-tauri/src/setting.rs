use std::{
    collections::HashMap,
    fs::{self, create_dir, File, OpenOptions},
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
    pub mac_custom_name: HashMap<String, String>,
    pub background_image_path: Option<String>,
    pub background_transparence: Option<u32>,
    pub background_blur: Option<u32>,
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

    pub fn set_background_image_path(
        &mut self,
        app: &tauri::AppHandle,
        src_path: &PathBuf,
    ) -> Result<()> {
        self.reset_background_image();
        let mut dest_path = get_config_path(app)?;
        dest_path.pop();
        dest_path.push(
            src_path
                .file_name()
                .ok_or_else(|| anyhow!("Source image file name is invalid"))?,
        );
        fs::copy(src_path, &dest_path)?;
        self.background_image_path = Some(
            dest_path
                .to_str()
                .ok_or_else(|| anyhow!("Saved image file name is invalid"))?
                .to_string(),
        );
        Ok(())
    }

    pub fn reset_background_image(&mut self) {
        if let Some(path) = &self.background_image_path {
            let _err = fs::remove_file(path);
        }
        self.background_image_path = None;
    }

    pub fn set_background_transparence(&mut self, background_transparence: u32) {
        self.background_transparence = Some(background_transparence);
    }

    pub fn set_background_blur(&mut self, background_blur: u32) {
        self.background_blur = Some(background_blur);
    }

    pub fn set_mac_custom_name(&mut self, mac: &str, name: &str) {
        self.mac_custom_name
            .entry(mac.to_owned())
            .and_modify(|n| {
                *n = name.to_owned();
            })
            .or_insert(name.to_string());
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
