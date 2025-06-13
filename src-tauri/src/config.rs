use std::{
    fs,
    sync::{Arc, Mutex},
};

use anyhow::Error;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use tauri::{path::BaseDirectory, App, Manager};

use crate::database::Database;

pub const INIT_WEIDTH: f64 = 300.0;
pub const INIT_HEIGHT: f64 = 350.0;
pub const CONFIG_PATH: &str = "config.json";
pub const DB_FILE_PATH: &str = "translation_history.db";

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ThemeType {
    Dark,
    Light,
}

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
    pub theme: ThemeType,
    pub prompt: String,
    #[serde(rename = "systemPrompt")]
    pub system_prompt: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_key: "key".to_string(),
            api_url: "http://localhost:11434/api/chat".to_string(),
            platform: PlatformType::OLLama,
            model_name: "qwen3:1.7b".to_string(),
            theme: ThemeType::Dark,
            prompt: "Translate to {{to}} (output translation only):\n\n{{text}}".to_string(),
            system_prompt: "You are a professional {{to}} native translator who needs to fluently translate text into {{to}}.\n\n## Translation Rules\n1. Output only the translated content, without explanations or additional content (such as \"Here's the translation:\" or \"Translation as follows:\")\n2. The returned translation must maintain exactly the same number of paragraphs and format as the original text\n3. For content that should not be translated (such as proper nouns, code, etc.), keep the original text.\n".to_string(),
        }
    }
}

pub static CONFIG: OnceCell<Mutex<AppConfig>> = OnceCell::new();
pub static REQUEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);
// 全局数据库实例
pub static DATABASE: OnceCell<Arc<Mutex<Option<Database>>>> = OnceCell::new();

// 初始化数据库
pub async fn init_database(
    app_handle: &tauri::AppHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = app_handle
        .path()
        .resolve(DB_FILE_PATH, BaseDirectory::Resource)
        .expect("Failed to resolve resource path");

    let database = Database::new(db_path).await?;

    DATABASE
        .set(Arc::new(Mutex::new(Some(database))))
        .map_err(|_| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to initialize database",
            )) as Box<dyn std::error::Error>
        })?;

    Ok(())
}

// 获取数据库实例的辅助函数
pub fn get_database() -> Result<Arc<Mutex<Option<Database>>>, String> {
    DATABASE
        .get()
        .ok_or_else(|| "Database not initialized".to_string())
        .map(|db| db.clone())
}

// 初始化配置
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

// 获取配置
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
        theme: config_guard.theme.clone(),
        prompt: config_guard.prompt.clone(),
        system_prompt: config_guard.system_prompt.clone(),
    })
}
