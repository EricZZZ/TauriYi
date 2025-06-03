use tauri::window::{Color, Effect, EffectState, EffectsBuilder};
use tauri::Emitter;
use tauri::{AppHandle, LogicalSize, Manager, Size, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use crate::config;
use crate::utils;

#[cfg(desktop)]
pub fn setup_desktop(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let ctrl_j_shortcut = Shortcut::new(Some(Modifiers::META), Code::KeyJ);
    app_handle.plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app_handle, shortcut, _event| {
                if shortcut == &ctrl_j_shortcut && _event.state == ShortcutState::Pressed {
                    handle_shortcut_press(app_handle);
                }
            })
            .build(),
    )?;

    app_handle.global_shortcut().register(ctrl_j_shortcut)?;
    Ok(())
}

#[cfg(desktop)]
fn handle_shortcut_press(app_handle: &AppHandle) {
    let main_window = app_handle.get_webview_window("main");

    if let Some(window) = main_window {
        let primary_monitor = window.primary_monitor().unwrap().unwrap();
        let scale_factor = primary_monitor.scale_factor();
        let (final_x, final_y) = utils::calculate_window_position(primary_monitor, scale_factor);

        window
            .set_position(tauri::PhysicalPosition::new(final_x, final_y))
            .unwrap();
        window.show().unwrap();
        window.set_focus().unwrap();

        match app_handle.clipboard().read_text() {
            Ok(content) => {
                println!("time:{:?}", std::time::SystemTime::now());
                println!("Clipboard content received:\n{}", content);
                app_handle
                    .emit("clipboard-content", content)
                    .unwrap_or_else(|e| eprintln!("Failed to emit event: {}", e));
            }
            Err(err) => {
                eprintln!("Error getting clipboard content: {:?}", err);
            }
        }
    } else {
        println!("Main window not found");
    }
    println!("Ctrl-J Detected!");
}

pub fn build_main_window(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
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
}
