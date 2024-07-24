#[cfg_attr(mobile, tauri::mobile_entry_point)]

use std::collections::HashMap;
use reqwest::Response;

#[tauri::command]
async fn ttt() -> Result<String, ()> {
    let response: Response = reqwest::get("https://httpbin.org/ip").await.unwrap();
    let data: HashMap<String, String> = response
        .json::<HashMap<String, String>>()
        .await.unwrap();

    Ok(serde_json::to_string(&data).unwrap())
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ttt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
