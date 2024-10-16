use std::sync::OnceLock;

use crate::sys::mail_pathes::{app_path_initial, SysPath};
use crate::sys::mail_tray::build_tray;

mod sys;

pub static SYS_PATH: OnceLock<SysPath> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // .plugin(tauri_plugin_updater::Builder::new().build())

        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            SYS_PATH.get_or_init(|| app_path_initial().unwrap());
            let _ = build_tray(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
