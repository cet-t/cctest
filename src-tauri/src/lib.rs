mod mouse_listener;
mod gesture_detector;
mod settings;
mod app_detector;
mod theme;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GesturePoint {
    x: i32,
    y: i32,
    timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureConfig {
    enabled: bool,
    sensitivity: f32,
    min_gesture_length: i32,
}

#[tauri::command]
fn start_listening() -> Result<String, String> {
    Ok("Listening started".to_string())
}

#[tauri::command]
fn stop_listening() -> Result<String, String> {
    Ok("Listening stopped".to_string())
}

#[tauri::command]
fn get_global_settings() -> Result<GestureConfig, String> {
    settings::get_global_settings().map_err(|e| e.to_string())
}

#[tauri::command]
fn update_global_settings(config: GestureConfig) -> Result<String, String> {
    settings::update_global_settings(&config).map_err(|e| e.to_string())?;
    Ok("Settings updated".to_string())
}

#[tauri::command]
fn get_app_settings(app_name: String) -> Result<serde_json::Value, String> {
    settings::get_app_settings(&app_name).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_app_settings(app_name: String, config: serde_json::Value) -> Result<String, String> {
    settings::update_app_settings(&app_name, config).map_err(|e| e.to_string())?;
    Ok("App settings updated".to_string())
}

#[tauri::command]
fn get_active_window() -> Result<String, String> {
    app_detector::get_active_window().map_err(|e| e.to_string())
}

#[tauri::command]
fn detect_gesture(points: Vec<GesturePoint>) -> Result<serde_json::Value, String> {
    gesture_detector::detect_gesture(&points).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_listening,
            stop_listening,
            get_global_settings,
            update_global_settings,
            get_app_settings,
            update_app_settings,
            get_active_window,
            detect_gesture,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
