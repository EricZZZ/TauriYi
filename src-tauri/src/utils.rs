use enigo::Mouse;
use enigo::{Enigo, Settings};
use tauri::Monitor;

use crate::config;

/**
 * 根据鼠标位置和缩放因子计算窗口位置
 */
pub fn calculate_window_position(primary_monitor: Monitor, scale_factor: f64) -> (u32, u32) {
    let enigo = Enigo::new(&Settings::default()).expect("Failed to create Enigo instance");

    let (mouse_x, mouse_y) = enigo.location().expect("Failed to get mouse position");

    println!(
        "Mouse position: ({}, {}), scale_factor: {}",
        mouse_x, mouse_y, scale_factor
    );

    // 简单的偏移量，例如在鼠标位置右下角 +10 像素
    let target_x = mouse_x as f64 * scale_factor + 10.0 * scale_factor;
    let target_y = mouse_y as f64 * scale_factor + 10.0 * scale_factor;

    // 考虑屏幕边缘，防止窗口超出屏幕 (可选但推荐)
    // 需要获取屏幕信息， Tauri 2.0 有 Monitor API

    let monitor_size = primary_monitor.size(); // Width and height in logical pixels
    let monitor_position = primary_monitor.position(); // Top-left corner position

    let mut final_x = target_x;
    let mut final_y = target_y;

    // 简单的边缘检查 (需要 monitor size 和 position)
    if target_x + config::INIT_WEIDTH * scale_factor
        > (monitor_position.x as u32 + monitor_size.width) as f64
    {
        final_x = target_x - config::INIT_WEIDTH * scale_factor - 20.0 * scale_factor;
        // 显示在左侧
    }
    if target_y + config::INIT_HEIGHT * scale_factor
        > (monitor_position.y as u32 + monitor_size.height) as f64
    {
        final_y = target_y - config::INIT_HEIGHT * scale_factor - 20.0 * scale_factor;
        // 显示在上方
    }
    println!("Final position: ({}, {})", final_x, final_y);
    (final_x as u32, final_y as u32)
}
