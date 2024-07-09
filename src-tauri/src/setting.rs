use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Setting {
    pub username: Option<String>,
    pub password: Option<String>,
    pub browser_path: Option<String>,
}

impl Setting {
    pub fn new() -> Self {
        Setting::default()
    }

    pub fn load_setting() -> Result<Self> {
        match File::open("setting.json") {
            Ok(mut json_file) => {
                let mut buf = String::new();
                json_file.read_to_string(&mut buf)?;
                Ok(serde_json::from_str::<Setting>(&buf).unwrap_or_default())
            }
            // 没有该文件
            Err(_) => Ok(Setting::new()),
        }
    }

    pub fn write_setting(&self) -> Result<()> {
        let json_str = serde_json::to_string(self)?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("setting.json")?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_setting() {
        let mut setting = Setting::default();
        setting.username = Some("user_name".to_string());
        setting.password = Some("password".to_string());
        setting.browser_path = Some("/path/to/browser".to_string());
        setting.write_setting().unwrap();
    }

    #[test]
    fn test_load_setting() {
        let setting = Setting::load_setting().unwrap();
        println!("{:?}", setting);
    }
}