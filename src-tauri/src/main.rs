#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

#[derive(serde::Serialize)]
struct CustomResponse {
    message: String,
    other_val: usize,
}
async fn some_other_function() -> Option<String> {
    println!("Other called");
    Some("response".into())
}

#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JS! Everything is OK");
}

#[tauri::command]
fn cmd_with_return() {
    println!("Someone called me");
}
#[tauri::command]
async fn full_command(number: usize) -> Result<CustomResponse, String> {
    println!("full command called from JS");
    let result: Option<String> = some_other_function().await;
    if let Some(message) = result {
        Ok(CustomResponse {
            message,
            other_val: 42 + number,
        })
    } else {
        Err("No result".into())
    }
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_custom_command,cmd_with_return,full_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
