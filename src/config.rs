use std::fs::{self, File};
use std::io::Write;
use toml;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub enum EElementType {
    File,
    Folder,
    Anything,
}

#[derive(Serialize, Deserialize)]
pub struct SConfigElement {
    pub path: String,
    pub content_type: EElementType,
}

#[derive(Serialize, Deserialize)]
pub struct SBackupConfig {
    #[serde(default)]
    pub elements: Vec<SConfigElement>,
    #[serde(skip)]
    pub path: String,
}

impl SBackupConfig {
    pub fn new(config_path: String) -> SBackupConfig {
        let mut config = SBackupConfig {
            elements: Vec::new(),
            path: String::new(),
        };

        if !config_path.is_empty() {
            config.load_config(config_path);
        }

        config
    }

    pub fn load_config(&mut self, config_path: String) {
        let contents = fs::read_to_string(config_path).expect("Failed to read config file");
        let config_des: SBackupConfig =
            toml::from_str(&contents).expect("Failde to deserialize config file");
        self.elements = config_des.elements;
    }

    pub fn auto_save(&mut self, mut config_path: String) {
        if self.path.is_empty() {
            config_path.push_str("/backup_config.toml");
            self.path = config_path;
        }

        self.save(self.path.clone())
    }
    pub fn save(&self, mut config_path: String) {
        config_path.push_str("/backup_config.toml");
        let toml_string = toml::to_string(&self).expect("Failed to serialize to TOML");

        let mut file = File::create(config_path).expect("Failed to create file");
        file.write_all(toml_string.as_bytes())
            .expect("Failed to write to file");
    }
}
