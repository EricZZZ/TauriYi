use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};

pub fn setup_tray_icon(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_i, &quit_i])?;
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(true)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            "show" => {
                println!("show menu item was clicked");
                let main_window = app.get_webview_window("main");
                if let Some(window) = main_window {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                } else {
                    println!("main window not found");
                }
            }
            _ => {
                println!("menu item with id: {} was clicked", event.id.as_ref())
            }
        })
        .build(app)?;
    Ok(())
}
