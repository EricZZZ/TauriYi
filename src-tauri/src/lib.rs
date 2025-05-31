use std::fs;

use config::AppConfig;
use resp::R;
use tauri::path::BaseDirectory;
use tauri::window::{Color, Effect, EffectState, EffectsBuilder};
use tauri::{Emitter, LogicalSize, Manager, Size};
use tauri::{WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use utils::calculate_window_position;

mod ai;

mod config;
mod lang;
mod resp;
mod utils;

/// 翻译
#[tauri::command]
async fn translate(
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
async fn close_window(webview_window: tauri::WebviewWindow) {
    println!("WebviewWindow: {}", webview_window.label());
    // 关闭webview_window
    webview_window.hide().unwrap();
}

/// 加载配置
#[tauri::command]
fn load_config() -> Result<R<AppConfig>, R<String>> {
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
fn update_config(app_handle: tauri::AppHandle, new_config: AppConfig) -> Result<R<()>, R<String>> {
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

#[cfg_attr(
    mobile,
    tauri::mobile_entry_point,
    not(debug_assertions),
    windows_subsystem = "windows"
)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            // 读取配置文件
            config::init_config(app);

            // 应用启动后的设置，可以在这里注册快捷键
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

                let ctrl_j_shortcut = Shortcut::new(Some(Modifiers::META), Code::KeyJ);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app_handle, shortcut, _event| {
                            // 按下快捷键时触发的操作
                            if shortcut == &ctrl_j_shortcut
                                && _event.state == ShortcutState::Pressed
                            {
                                let main_window = app_handle.get_webview_window("main");

                                if let Some(window) = main_window {
                                    let primary_monitor =
                                        window.primary_monitor().unwrap().unwrap();
                                    // 缩放因子
                                    let scale_factor = primary_monitor.scale_factor();
                                    // 计算鼠标位置并计算窗口的最终位置
                                    let (final_x, final_y) =
                                        calculate_window_position(primary_monitor, scale_factor);
                                    window
                                        .set_position(tauri::PhysicalPosition::new(
                                            final_x, final_y,
                                        ))
                                        .unwrap();

                                    window.show().unwrap();
                                    window.set_focus().unwrap();

                                    match app_handle.clipboard().read_text() {
                                        Ok(content) => {
                                            // read_text 返回 Option<String>
                                            println!("time:{:?}", std::time::SystemTime::now());
                                            println!("获取到的剪贴板内容:");
                                            println!("{}", content);
                                            app_handle
                                                .emit("clipboard-content", content)
                                                .unwrap_or_else(|e| {
                                                    eprintln!("Failed to emit event: {}", e)
                                                });
                                        }
                                        Err(err) => {
                                            eprintln!("获取剪贴板内容时发生错误: {:?}", err);
                                        }
                                    }
                                } else {
                                    println!("main window not found");
                                }

                                println!("Ctrl-J Detected!");
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(ctrl_j_shortcut)?;
            }

            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .inner_size(config::INIT_WEIDTH, config::INIT_HEIGHT);

            let window = win_builder.build().unwrap();
            window.set_effects(
                EffectsBuilder::new()
                    .effect(Effect::Popover)
                    .state(EffectState::Active)
                    .radius(5.)
                    .color(Color(0, 0, 0, 255))
                    .build(),
            )?;
            window.set_decorations(false)?;
            window.set_maximizable(false)?;
            window.set_minimizable(false)?;
            window.set_max_size(Some(Size::Logical(LogicalSize::new(
                config::INIT_WEIDTH,
                config::INIT_HEIGHT,
            ))))?;
            window.set_min_size(Some(Size::Logical(LogicalSize::new(
                config::INIT_WEIDTH,
                config::INIT_HEIGHT,
            ))))?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            translate,
            close_window,
            load_config,
            update_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
