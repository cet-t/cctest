use serde_json::json;
use std::fs;
use std::path::PathBuf;

fn config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let base = dirs::config_dir().ok_or("Could not find config directory")?;
    let config_path = base.join("tauri-gestures");
    fs::create_dir_all(&config_path)?;
    Ok(config_path)
}

pub fn get_global_settings() -> Result<super::GestureConfig, Box<dyn std::error::Error>> {
    let config_path = config_dir()?.join("config.json");

    if config_path.exists() {
        let data = fs::read_to_string(&config_path)?;
        let config: super::GestureConfig = serde_json::from_str(&data)?;
        Ok(config)
    } else {
        Ok(super::GestureConfig {
            enabled: true,
            sensitivity: 0.8,
            min_gesture_length: 50,
        })
    }
}

pub fn update_global_settings(config: &super::GestureConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = config_dir()?.join("config.json");
    let json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, json)?;
    Ok(())
}

pub fn get_app_settings(app_name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let config_path = config_dir()?
        .join("apps")
        .join(format!("{}.json", app_name));

    if config_path.exists() {
        let data = fs::read_to_string(&config_path)?;
        let config: serde_json::Value = serde_json::from_str(&data)?;
        Ok(config)
    } else {
        Ok(json!({
            "appName": app_name,
            "overrides": {
                "enabled": true,
                "disabledGestures": [],
                "customGestures": {}
            }
        }))
    }
}

pub fn update_app_settings(app_name: &str, config: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    let apps_dir = config_dir()?.join("apps");
    fs::create_dir_all(&apps_dir)?;

    let config_path = apps_dir.join(format!("{}.json", app_name));
    let json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, json)?;
    Ok(())
}
