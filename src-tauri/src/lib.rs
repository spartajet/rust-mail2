use tauri::Emitter;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_stronghold::Builder::new(|_pass| todo!()).build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
            app.emit("single-instance", Payload { args: argv, cwd }).unwrap();
        }))
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .setup(|app| {
            // Get the autostart manager
            let autostart_manager = app.autolaunch();
            // 启用 autostart
            let _ = autostart_manager.enable();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}
