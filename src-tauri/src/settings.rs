use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

const SETTINGS_FILE_NAME: &str = "settings.json";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub api_key: Option<String>,
}

impl AppSettings {
    pub fn normalized(mut self) -> Self {
        self.api_key = self
            .api_key
            .take()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        self
    }
}

pub fn load_settings(app: &AppHandle) -> Result<AppSettings, String> {
    let settings_file_path = settings_file_path(app)?;

    if !settings_file_path.exists() {
        return Ok(AppSettings::default());
    }

    let raw_content = fs::read_to_string(&settings_file_path).map_err(|error| error.to_string())?;
    let settings = serde_json::from_str::<AppSettings>(&raw_content)
        .map_err(|error| format!("Failed to parse settings: {error}"))?;

    Ok(settings.normalized())
}

pub fn save_settings(app: &AppHandle, settings: AppSettings) -> Result<(), String> {
    let settings_file_path = settings_file_path(app)?;
    if let Some(parent_directory) = settings_file_path.parent() {
        fs::create_dir_all(parent_directory).map_err(|error| error.to_string())?;
    }

    let normalized_settings = settings.normalized();
    let content = serde_json::to_string_pretty(&normalized_settings)
        .map_err(|error| format!("Failed to serialize settings: {error}"))?;
    fs::write(settings_file_path, content).map_err(|error| error.to_string())
}

fn settings_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_config_directory = app
        .path()
        .app_config_dir()
        .map_err(|error| error.to_string())?;
    Ok(app_config_directory.join(SETTINGS_FILE_NAME))
}
