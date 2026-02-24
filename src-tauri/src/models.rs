use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub gemini_model: String,
    pub gemini_temperature: f32,
    pub tts_rate: f32,
    pub recording_source: String,
    pub recording_input_device: String,
    pub opacity: f64,
    pub always_on_top: bool,
    pub click_through: bool,
    pub screen_capture_protection: bool,
    pub hotkey_toggle: String,
    pub hotkey_record: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            gemini_model: "gemini-2.5-flash".to_string(),
            gemini_temperature: 0.7,
            tts_rate: 1.0,
            recording_source: "microphone".to_string(),
            recording_input_device: "Default input".to_string(),
            opacity: 0.95,
            always_on_top: true,
            click_through: false,
            screen_capture_protection: true,
            hotkey_toggle: "CommandOrControl+Shift+Space".to_string(),
            hotkey_record: "CommandOrControl+Shift+R".to_string(),
        }
    }
}
