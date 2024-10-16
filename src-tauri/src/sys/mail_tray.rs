use std::error::Error;

use tauri::{App, Manager};
use tauri::image::Image;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

pub fn build_tray(app: &mut App) -> Result<(), Box<dyn Error>> {
    let toggle = MenuItemBuilder::with_id("refresh", "Refresh").build(app)?;
    let new_mail = MenuItemBuilder::with_id("new_mail", "New Mail").build(app)?;
    let menu = MenuBuilder::new(app).items(&[&toggle, &new_mail]).build()?;
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .icon(Image::from_path("./icons/icon.png")?)
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                let app = tray.app_handle();

                #[cfg(not(target_os = "macos"))] {
                    if let Some(webview_window) = app.get_webview_window("main") {
                        let _ = webview_window.show();
                        let _ = webview_window.set_focus();
                    }
                }

                #[cfg(target_os = "macos")] {
                    tauri::AppHandle::show(&app.app_handle()).unwrap();
                }
            }
            _ => {}
        })
        .on_menu_event(|_, event| {
            if event.id == "refresh" {
                println!("Refresh clicked");
            }
        })
        .build(app)?;
    Ok(())
}