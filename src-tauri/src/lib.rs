mod ai;
mod app_setup;
mod commands;
mod config;
mod database;
mod lang;
mod resp;
mod tray_menu;
mod utils;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

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
            // 初始化配置文件
            config::init_config(app);

            // 初始化数据库
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = config::init_database(&app_handle).await {
                    eprintln!("数据库初始化失败: {}", e);
                }
            });

            // 应用启动后的设置，可以在这里注册快捷键
            #[cfg(desktop)]
            app_setup::setup_desktop(app.handle())?;
            // 构建主窗口
            app_setup::build_main_window(app.handle())?;
            // 构建托盘菜单
            tray_menu::setup_tray_icon(app.handle())?;
            // 监控main窗口状态
            app_setup::monitor_main_window_state(app.handle().clone());

            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::translate,
            commands::close_window,
            commands::load_config,
            commands::update_config,
            commands::reset_config,
            commands::get_translation_history,
            commands::search_translations,
            commands::delete_translation,
            commands::clear_translation_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
