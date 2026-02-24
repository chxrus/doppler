use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub gemini_model: String,
    pub gemini_temperature: f32,
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
            gemini_model: "gemini-pro".to_string(),
            gemini_temperature: 0.7,
            opacity: 0.95,
            always_on_top: true,
            click_through: false,
            screen_capture_protection: true,
            hotkey_toggle: "CommandOrControl+Shift+Space".to_string(),
            hotkey_record: "CommandOrControl+Shift+R".to_string(),
        }
    }
}
