mod ai;
mod app_setup;
mod commands;
mod config;
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
            // 读取配置文件
            config::init_config(app);

            // 应用启动后的设置，可以在这里注册快捷键
            #[cfg(desktop)]
            app_setup::setup_desktop(app.handle())?;
            // 构建主窗口
            app_setup::build_main_window(app.handle())?;
            // 构建托盘菜单
            tray_menu::setup_tray_icon(app.handle())?;

            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::translate,
            commands::close_window,
            commands::load_config,
            commands::update_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
