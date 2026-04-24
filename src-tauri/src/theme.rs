use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub mode: ThemeMode,
    pub custom_theme: Option<CustomTheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Light,
    Dark,
    Sync,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTheme {
    pub name: String,
    pub colors: std::collections::HashMap<String, String>,
    pub fonts: std::collections::HashMap<String, String>,
}

fn config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let base = dirs::config_dir().ok_or("Could not find config directory")?;
    let config_path = base.join("tauri-gestures");
    fs::create_dir_all(&config_path)?;
    Ok(config_path)
}

pub fn get_theme_config() -> Result<ThemeConfig, Box<dyn std::error::Error>> {
    let config_path = config_dir()?.join("theme.json");

    if config_path.exists() {
        let data = fs::read_to_string(&config_path)?;
        let config: ThemeConfig = serde_json::from_str(&data)?;
        Ok(config)
    } else {
        Ok(ThemeConfig {
            mode: ThemeMode::Sync,
            custom_theme: None,
        })
    }
}

pub fn update_theme_config(config: &ThemeConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = config_dir()?.join("theme.json");
    let json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, json)?;
    Ok(())
}

pub fn validate_custom_theme(theme: &CustomTheme) -> Result<(), Box<dyn std::error::Error>> {
    if theme.name.is_empty() {
        return Err("Theme name cannot be empty".into());
    }

    if theme.colors.is_empty() {
        return Err("Theme must have at least one color defined".into());
    }

    Ok(())
}
