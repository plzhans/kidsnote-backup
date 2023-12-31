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

    pub fn save(&self, config_path:String){
        println!("[config_update] Start.");
        let config_path = if config_path.starts_with("~/") {
            let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("./"));
            let next_dir = &config_path[2..];
            home_dir.join(next_dir)
        } else {
            PathBuf::from(config_path)
        };
        if let Some(parent_dir) = config_path.parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir).expect("Failed to create directory");
            }
        }

        println!("[config_update] Save. Path={:?}", config_path);
        let toml_string = toml::to_string(&self).unwrap();
        let mut file = File::create(config_path).expect("Failed to create file");
        file.write_all(toml_string.as_bytes()).expect("Failed to write to file");

        println!("[config_update] End.");
    }

}
