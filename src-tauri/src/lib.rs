#[cfg_attr(mobile, tauri::mobile_entry_point)]

use tauri::Manager;
use tauri::{AppHandle, WebviewWindow};

#[tauri::command]
async fn set_app_title(app: AppHandle, title: String) -> Result<(), ()> {
    let window: WebviewWindow = app.get_webview_window("main").unwrap();
    let _ = window.set_title(&title);

    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window: WebviewWindow = app.get_webview_window("main").unwrap();
            let _ = window.set_title("TTT");
            let _ = window.center();
            let _ = window.minimize();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![set_app_title])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
