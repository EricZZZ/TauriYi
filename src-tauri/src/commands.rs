use tauri::path::BaseDirectory;
use tauri::Manager;

use crate::config::AppConfig;
use crate::{ai, config, lang, resp::R};
use std::fs;

/// 翻译
#[tauri::command]
pub async fn translate(
    text: &str,
    target_lang: lang::Lang,
    source_lang: lang::Lang,
) -> Result<R<String>, R<String>> {
    println!("开始调用tauri::command translate: {:?}", text);
    match ai::translate(text.to_string(), target_lang, source_lang).await {
        Ok(data) => Ok(R::success(data)),
        Err(e) => Err(R::fail(1, &e.to_string())),
    }
}

/// 关闭窗口
#[tauri::command]
pub async fn close_window(webview_window: tauri::WebviewWindow) {
    println!("WebviewWindow: {}", webview_window.label());
    // 关闭webview_window
    webview_window.hide().unwrap();
}

/// 加载配置
#[tauri::command]
pub fn load_config() -> Result<R<AppConfig>, R<String>> {
    // 获取配置
    let confg = config::get_config();
    println!("配置文件: {:?}", confg);
    match confg {
        Ok(confg) => Ok(R::success(confg)),
        Err(_) => Err(R::fail(1, "获取配置失败")),
    }
}

/// 保存配置
#[tauri::command]
pub fn update_config(
    app_handle: tauri::AppHandle,
    new_config: AppConfig,
) -> Result<R<()>, R<String>> {
    let config_path = app_handle
        .path()
        .resolve(config::CONFIG_PATH, BaseDirectory::Resource)
        .expect("Failed to resolve resource path");
    let config_str = serde_json::to_string_pretty(&new_config).expect("Failed to serialize config");

    fs::write(config_path, config_str).expect("Failed to write config file");
    println!("配置文件已更新: {:?}", new_config);
    // 更新config：：CONFIG
    {
        let mut config_guard = config::CONFIG
            .get()
            .expect("Config not initialized")
            .lock()
            .expect("Mutex poisoned");
        *config_guard = new_config;
    }
    // 再次读取以验证修改
    {
        let config_guard = config::CONFIG.get().unwrap().lock().unwrap();
        println!("Verified config: {:?}", *config_guard);
    }

    Ok(R::success(()))
}

#[tauri::command]
pub fn reset_config(app_handle: tauri::AppHandle) -> Result<R<()>, R<String>> {
    let config_path = app_handle
        .path()
        .resolve(config::CONFIG_PATH, BaseDirectory::Resource)
        .expect("Failed to resolve resource path");
    
    // 重置配置
    let config = AppConfig::default();
    let config_str = serde_json::to_string_pretty(&config).expect("Failed to serialize config");
    fs::write(config_path, config_str).expect("Failed to write config file");
    println!("配置文件已重置: {:?}", config);

    // 更新config：：CONFIG
    {
        let mut config_guard = config::CONFIG
            .get()
            .expect("Config not initialized")
            .lock()
            .expect("Mutex poisoned");
        *config_guard = config;
    }
    // 再次读取以验证修改
    {
        let config_guard = config::CONFIG.get().unwrap().lock().unwrap();
        println!("Verified config: {:?}", *config_guard);
    }

    Ok(R::success(()))
}
