#[cfg_attr(mobile, tauri::mobile_entry_point)]

use std::collections::HashMap;
use reqwest::{Client, Response, Error, header::HeaderMap, Request, Method, Url};
use serde::{Serialize, Deserialize};
use base64::{Engine, prelude::BASE64_STANDARD};
/*
#[tauri::command]
async fn check_auth() -> Result<String, ()> {
    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("lang".to_string(), "rust".to_string());
    data.insert("body".to_string(), "json".to_string());

    let client: Client = Client::new();
    let response: Response = client
        .post("http://127.0.0.1:3000")
        .header("xxx-my-header", "aabbb001-aaaaabbbbbcccccdddddeeeee")
        //.body(serde_json::to_string(&data).unwrap())
        .body("the exact body that is sent")
        .send()
        .await
        .unwrap();

    let data: String = response.text().await.unwrap();

    Ok(data)
}
*/

#[tauri::command]
async fn fetch_test_one() -> Result<String, String> {
    let client: Client = Client::new();
    let response: Result<Response, Error> = client
        .post("http://127.0.0.1:3000/test-one")
        .send()
        .await;

    Ok(response.unwrap().text().await.unwrap())
}

#[tauri::command]
async fn fetch_test_two() -> Result<String, String> {
    let client: Client = Client::new();
    let response: Result<Response, Error> = client
        .post("http://127.0.0.1:3000/test-two")
        .send()
        .await;

    Ok(response.unwrap().text().await.unwrap())
}

#[derive(Serialize, Deserialize, Debug)]
struct TTT {
    message: String
}

#[tauri::command]
async fn fetch_test_three() -> Result<String, String> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("xxx-test-header", "xxx-test-value".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());


    let ddd: TTT = TTT { message: "aaa".to_string() };

    let client: Client = Client::new();
    let response: Result<Response, Error> = client
        .post("http://127.0.0.1:3000/test-three")
        .headers(headers)
        .body(serde_json::to_string(&ddd).unwrap())
        .send()
        .await;

    Ok(response.unwrap().text().await.unwrap())
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_test_one, fetch_test_two, fetch_test_three])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
