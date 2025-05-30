use std::sync::Mutex;

use anyhow::Error;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use tauri::{path::BaseDirectory, App, Manager};

pub const INIT_WEIDTH: f64 = 300.0;
pub const INIT_HEIGHT: f64 = 350.0;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum PlatformType {
    OLLama,
    DeepSeek,
    ChatGPT,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "apiUrl")]
    pub api_url: String,
    pub platform: PlatformType,
    #[serde(rename = "modelName")]
    pub model_name: String,
}

pub static CONFIG: OnceCell<Mutex<Config>> = OnceCell::new();
pub static REQUEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

pub fn load_config(app: &App) {
    let resource_path = app
        .path()
        .resolve("config.json", BaseDirectory::Resource)
        .expect("Failed to resolve resource path");

    let file = std::fs::File::open(&resource_path).unwrap();
    let config: Config = serde_json::from_reader(file).unwrap();
    CONFIG
        .set(Mutex::new(config))
        .expect("Failed to set config");
    println!(
        "读取配置文件成功，内容为：{:?}",
        CONFIG.get().unwrap().lock().unwrap()
    )
}

pub fn get_config() -> Result<Config, Error> {
    let config_guard = CONFIG
        .get()
        .expect("Config not initialized")
        .lock()
        .expect("Mutex poisoned");
    Ok(Config {
        api_key: config_guard.api_key.clone(),
        api_url: config_guard.api_url.clone(),
        platform: config_guard.platform,
        model_name: config_guard.model_name.clone(),
    })
}
