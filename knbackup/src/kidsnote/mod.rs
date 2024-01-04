use std::{path::PathBuf, fs::{self, File}, io::Write};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KnBackupConfig {
    pub default: Option<KidsnoteConfigProfile>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KidsnoteConfigProfile {
    pub user_id: Option<String>,
    pub refresh_token: Option<String>
}

impl KnBackupConfig {
    pub fn from_file(config_path:&str) -> KnBackupConfig {

        let config_path = if config_path.starts_with("~/") {
            let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("./"));
            let next_dir = &config_path[2..];
            home_dir.join(next_dir)
        } else {
            PathBuf::from(config_path)
        };

        if let Ok(toml_string) = fs::read_to_string(config_path) {
            if let Ok(config) = toml::from_str(&toml_string) {
                return config;
            }
        } 

        KnBackupConfig {
            default: None
        }
    }

    pub fn set_default(&mut self, user_id:String, refresh_token:String){
        self.default = Some(KidsnoteConfigProfile {
            user_id: Some(user_id),
            refresh_token: Some(refresh_token)
        });
    }

    pub fn save(&self, save_path:String){
        let config_path = if save_path.starts_with("~/") {
            let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("./"));
            let next_dir = &save_path[2..];
            home_dir.join(next_dir)
        } else {
            PathBuf::from(&save_path)
        };
        if let Some(parent_dir) = config_path.parent() {
            if !parent_dir.exists() {
                if let Err(err) = fs::create_dir_all(parent_dir){
                    log::info!(target:"config", "config file dir create fail. {}", err);
                }
            }
        }

        let toml_string = toml::to_string(&self).unwrap();
        match File::create(config_path) {
            Ok(mut file) => {
                if let Err(err) = file.write_all(toml_string.as_bytes()){
                    log::error!(target:"config", "config file save fail. {}", err);
                } else {
                    log::info!(target:"config", "config file save. path={}", save_path);
                }
            },
            Err(err) => {
                log::error!(target:"config", "config file create fail. {}", err);
            }
        }
    }

}
