// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod tts;
use std::env;

use hf_hub::api::sync::Api;

#[tauri::command]
fn open() -> Result<(), String> {
    let dir = env::current_dir().map_err(|e| e.to_string())?;
    open::that(dir.join("models")).ok();
    Ok(())
}

#[tauri::command]
fn path() -> Result<String, String> {
    env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

fn main() {
    ort::init_from(
        Api::new()
            .unwrap()
            .model("googlefan/sbv2_onnx_models".to_string())
            .get(if cfg!(windows) {
                "onnxruntime.dll"
            } else if cfg!(target_os = "macos") {
                "libonnxruntime.dylib"
            } else {
                "libonnxruntime.so"
            })
            .unwrap()
            .to_string_lossy()
            .to_string(),
    )
    .commit()
    .unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            tts::reload_models,
            tts::synthesize,
            tts::models,
            open,
            path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
