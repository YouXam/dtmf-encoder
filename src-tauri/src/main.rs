// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dtmf;

#[tauri::command]
fn tone(char: char) -> Result<Vec<u8>, String> {
    match dtmf::tone(char) {
        Ok(data) => Ok(data),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn save(path: &str, value: &str) -> Result<(), String> {
    match dtmf::save(path, value) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![tone, save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
