use std::{fs, sync::Mutex};

use anyhow::Error;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use tauri::{path::BaseDirectory, App, Manager};

pub const INIT_WEIDTH: f64 = 300.0;
pub const INIT_HEIGHT: f64 = 350.0;
pub const CONFIG_PATH: &str = "config.json";

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum PlatformType {
    OLLama,
    DeepSeek,
    ChatGPT,
    MTranServer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "apiUrl")]
    pub api_url: String,
    pub platform: PlatformType,
    #[serde(rename = "modelName")]
    pub model_name: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_key: "key".to_string(),
            api_url: "http://localhost:11434/api/chat".to_string(),
            platform: PlatformType::OLLama,
            model_name: "qwen3:1.7b".to_string(),
        }
    }
}

pub static CONFIG: OnceCell<Mutex<AppConfig>> = OnceCell::new();
pub static REQUEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

pub fn init_config(app: &App) {
    let config_path = app
        .path()
        .resolve(CONFIG_PATH, BaseDirectory::Resource)
        .expect("Failed to resolve resource path");
    // 检查文件是否存在
    if !config_path.exists() {
        println!("配置文件不存在，使用默认配置");
        let default_config = AppConfig::default();
        let config_str = serde_json::to_string_pretty(&default_config)
            .expect("Failed to serialize default config");

        fs::write(config_path, config_str).expect("Failed to write config file");

        CONFIG
            .set(Mutex::new(default_config))
            .expect("Failed to set config");
    } else {
        let file = std::fs::File::open(&config_path).unwrap();
        let config: AppConfig = serde_json::from_reader(file).unwrap();

        CONFIG
            .set(Mutex::new(config))
            .expect("Failed to set config");
        println!(
            "读取配置文件成功，内容为：{:?}",
            CONFIG.get().unwrap().lock().unwrap()
        )
    }
}

pub fn get_config() -> Result<AppConfig, Error> {
    let config_guard = CONFIG
        .get()
        .expect("Config not initialized")
        .lock()
        .expect("Mutex poisoned");
    Ok(AppConfig {
        api_key: config_guard.api_key.clone(),
        api_url: config_guard.api_url.clone(),
        platform: config_guard.platform,
        model_name: config_guard.model_name.clone(),
    })
}
