// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod tts;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            tts::reload_models,
            tts::synthesize,
            tts::models
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
