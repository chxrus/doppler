use crate::gemini;
use crate::storage;

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
