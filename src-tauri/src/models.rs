use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub theme: String,
    pub text_provider: String,
    pub stt_provider: String,
    pub gemini_model: String,
    pub gemini_temperature: f32,
    pub whisper_model_path: Option<String>,
    pub whisper_language: Option<String>,
    pub whisper_threads: Option<u32>,
    pub ollama_base_url: String,
    pub ollama_model: String,
    pub tts_rate: f32,
    pub recording_source: String,
    pub recording_input_device: String,
    pub opacity: f64,
    pub always_on_top: bool,
    pub click_through: bool,
    pub screen_capture_protection: bool,
    pub hotkey_toggle: String,
    pub hotkey_record: String,
    pub hotkey_previous: String,
    pub hotkey_next: String,
    pub hotkey_send: String,
    pub hotkey_click_through: String,
    pub hotkey_capture_visibility: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            text_provider: "gemini".to_string(),
            stt_provider: "gemini".to_string(),
            gemini_model: "gemini-2.5-flash".to_string(),
            gemini_temperature: 0.7,
            whisper_model_path: None,
            whisper_language: None,
            whisper_threads: None,
            ollama_base_url: "http://localhost:11434".to_string(),
            ollama_model: "llama3.2:3b".to_string(),
            tts_rate: 1.0,
            recording_source: "microphone".to_string(),
            recording_input_device: "Default input".to_string(),
            opacity: 0.95,
            always_on_top: true,
            click_through: false,
            screen_capture_protection: true,
            hotkey_toggle: "CommandOrControl+,".to_string(),
            hotkey_record: "CommandOrControl+R".to_string(),
            hotkey_previous: "Alt+Left".to_string(),
            hotkey_next: "Alt+Right".to_string(),
            hotkey_send: "Enter".to_string(),
            hotkey_click_through: "CommandOrControl+Shift+X".to_string(),
            hotkey_capture_visibility: "CommandOrControl+Shift+H".to_string(),
        }
    }
}
