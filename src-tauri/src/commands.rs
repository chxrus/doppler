use crate::audio;
use crate::gemini;
use crate::storage;
use tauri::{Emitter, Manager};

const API_KEY_STORAGE_KEY: &str = "gemini_api_key";

#[tauri::command]
pub async fn save_api_key(api_key: String) -> Result<(), String> {
    let api_key = api_key.trim();

    if api_key.is_empty() {
        return storage::delete_from_keychain(API_KEY_STORAGE_KEY)
            .map_err(|e| format!("Failed to clear API key: {}", e));
    }

    // Validate the API key before saving
    gemini::validate_api_key(api_key)
        .await
        .map_err(|e| format!("Invalid API key: {}", e))?;

    // Save to secure storage
    storage::save_to_keychain(API_KEY_STORAGE_KEY, api_key)
        .map_err(|e| format!("Failed to save API key: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_api_key() -> Result<String, String> {
    storage::get_from_keychain(API_KEY_STORAGE_KEY).map_err(|error| match error {
        storage::StorageError::KeyNotFound => {
            "API key not found. Please set it in settings.".to_string()
        }
        _ => format!("Failed to retrieve API key: {error}"),
    })
}

#[tauri::command]
pub async fn send_message(message: String) -> Result<String, String> {
    let message = message.trim();

    if message.is_empty() {
        return Err("Message cannot be empty".to_string());
    }

    let api_key = storage::get_from_keychain(API_KEY_STORAGE_KEY).map_err(|error| match error {
        storage::StorageError::KeyNotFound => {
            "API key not found. Please set it in settings.".to_string()
        }
        _ => format!("Failed to retrieve API key: {error}"),
    })?;

    // Send message to Gemini
    gemini::send_message(&api_key, message)
        .await
        .map_err(|e| format!("Failed to send message: {}", e))
}

#[tauri::command]
pub async fn start_recording() -> Result<(), String> {
    audio::start_microphone().map_err(|error| format!("Failed to start recording: {error}"))
}

#[tauri::command]
pub async fn stop_recording_and_transcribe() -> Result<String, String> {
    let recorded_audio =
        audio::stop_microphone().map_err(|error| format!("Failed to stop recording: {error}"))?;

    let api_key = storage::get_from_keychain(API_KEY_STORAGE_KEY).map_err(|error| match error {
        storage::StorageError::KeyNotFound => {
            "API key not found. Please set it in settings.".to_string()
        }
        _ => format!("Failed to retrieve API key: {error}"),
    })?;

    audio::transcribe(
        &api_key,
        recorded_audio.samples,
        recorded_audio.sample_rate,
        recorded_audio.channel_count,
    )
    .await
    .map_err(|error| format!("Failed to transcribe audio: {error}"))
}

#[tauri::command]
pub async fn get_settings() -> Result<crate::models::Settings, String> {
    storage::load_settings().map_err(|error| format!("Failed to load settings: {}", error))
}

#[tauri::command]
pub async fn update_settings(settings: crate::models::Settings) -> Result<(), String> {
    storage::save_settings(&settings).map_err(|error| format!("Failed to save settings: {}", error))
}

#[tauri::command]
pub async fn set_window_always_on_top(
    app: tauri::AppHandle,
    always_on_top: bool,
) -> Result<(), String> {
    let main_window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    main_window
        .set_always_on_top(always_on_top)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn set_window_click_through(
    app: tauri::AppHandle,
    click_through: bool,
) -> Result<(), String> {
    let main_window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    main_window
        .set_ignore_cursor_events(click_through)
        .map_err(|error| error.to_string())?;

    let state = app.state::<crate::WindowInteractionState>();
    if let Ok(mut current_state) = state.click_through.lock() {
        *current_state = click_through;
    }
    let _ = app.emit("click-through-changed", click_through);

    Ok(())
}
