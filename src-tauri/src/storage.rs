use crate::models::Settings;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to save to keychain: {0}")]
    SaveFailed(String),

    #[error("Failed to retrieve from keychain: {0}")]
    RetrieveFailed(String),

    #[error("Key not found in keychain")]
    KeyNotFound,

    #[allow(dead_code)]
    #[error("Failed to save settings: {0}")]
    SettingsSaveFailed(String),

    #[error("Failed to load settings: {0}")]
    SettingsLoadFailed(String),
}

const SERVICE_NAME: &str = "com.chxrus.doppler";
#[cfg(any(
    all(target_os = "macos", debug_assertions),
    not(any(target_os = "macos", target_os = "windows"))
))]
const SETTINGS_FILE_NAME: &str = "settings.json";

/// Save a key-value pair to the OS secure storage (Keychain on macOS, Credential Manager on Windows)
pub fn save_to_keychain(key: &str, value: &str) -> Result<(), StorageError> {
    #[cfg(all(target_os = "macos", not(debug_assertions)))]
    {
        save_to_keychain_macos(key, value)
    }

    #[cfg(target_os = "windows")]
    {
        save_to_keychain_windows(key, value)
    }

    #[cfg(any(
        all(target_os = "macos", debug_assertions),
        not(any(target_os = "macos", target_os = "windows"))
    ))]
    {
        save_to_file_fallback(key, value)
    }
}

/// Retrieve a value from the OS secure storage
pub fn get_from_keychain(key: &str) -> Result<String, StorageError> {
    #[cfg(all(target_os = "macos", not(debug_assertions)))]
    {
        get_from_keychain_macos(key)
    }

    #[cfg(target_os = "windows")]
    {
        get_from_keychain_windows(key)
    }

    #[cfg(any(
        all(target_os = "macos", debug_assertions),
        not(any(target_os = "macos", target_os = "windows"))
    ))]
    {
        get_from_file_fallback(key)
    }
}

/// Delete a value from OS secure storage (or file fallback on Linux/other)
pub fn delete_from_keychain(key: &str) -> Result<(), StorageError> {
    #[cfg(all(target_os = "macos", not(debug_assertions)))]
    {
        delete_from_keychain_macos(key)
    }

    #[cfg(target_os = "windows")]
    {
        delete_from_keychain_windows(key)
    }

    #[cfg(any(
        all(target_os = "macos", debug_assertions),
        not(any(target_os = "macos", target_os = "windows"))
    ))]
    {
        delete_from_file_fallback(key)
    }
}

#[cfg(all(target_os = "macos", not(debug_assertions)))]
fn save_to_keychain_macos(key: &str, value: &str) -> Result<(), StorageError> {
    use security_framework::passwords::{delete_generic_password, set_generic_password};

    // Delete existing entry if present (to update)
    let _ = delete_generic_password(SERVICE_NAME, key);

    // Save new value
    set_generic_password(SERVICE_NAME, key, value.as_bytes())
        .map_err(|e| StorageError::SaveFailed(e.to_string()))
}

#[cfg(all(target_os = "macos", not(debug_assertions)))]
fn get_from_keychain_macos(key: &str) -> Result<String, StorageError> {
    use security_framework::passwords::get_generic_password;

    let password_bytes = get_generic_password(SERVICE_NAME, key).map_err(|error| {
        if error.code() == -25300 {
            StorageError::KeyNotFound
        } else {
            StorageError::RetrieveFailed(error.to_string())
        }
    })?;

    String::from_utf8(password_bytes)
        .map_err(|e| StorageError::RetrieveFailed(format!("Invalid UTF-8: {}", e)))
}

#[cfg(all(target_os = "macos", not(debug_assertions)))]
fn delete_from_keychain_macos(key: &str) -> Result<(), StorageError> {
    use security_framework::passwords::delete_generic_password;

    match delete_generic_password(SERVICE_NAME, key) {
        Ok(()) => Ok(()),
        Err(error) if error.code() == -25300 => Ok(()),
        Err(error) => Err(StorageError::RetrieveFailed(error.to_string())),
    }
}

#[cfg(target_os = "windows")]
fn save_to_keychain_windows(key: &str, value: &str) -> Result<(), StorageError> {
    use windows::Security::Credentials::{PasswordCredential, PasswordVault};

    let vault = PasswordVault::new().map_err(|e| {
        StorageError::SaveFailed(format!("Failed to access PasswordVault: {:?}", e))
    })?;

    // Try to remove existing credential first
    if let Ok(existing) = vault.Retrieve(&SERVICE_NAME.into(), &key.into()) {
        let _ = vault.Remove(&existing);
    }

    // Create and add new credential
    let credential = PasswordCredential::CreatePasswordCredential(
        &SERVICE_NAME.into(),
        &key.into(),
        &value.into(),
    )
    .map_err(|e| StorageError::SaveFailed(format!("Failed to create credential: {:?}", e)))?;

    vault
        .Add(&credential)
        .map_err(|e| StorageError::SaveFailed(format!("Failed to add credential: {:?}", e)))?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn get_from_keychain_windows(key: &str) -> Result<String, StorageError> {
    use windows::Security::Credentials::PasswordVault;

    let vault = PasswordVault::new().map_err(|e| {
        StorageError::RetrieveFailed(format!("Failed to access PasswordVault: {:?}", e))
    })?;

    let credential = vault
        .Retrieve(&SERVICE_NAME.into(), &key.into())
        .map_err(|e| {
            let error_msg = format!("{:?}", e);
            if error_msg.contains("not found") || error_msg.contains("0x80070490") {
                StorageError::KeyNotFound
            } else {
                StorageError::RetrieveFailed(error_msg)
            }
        })?;

    credential.RetrievePassword().map_err(|e| {
        StorageError::RetrieveFailed(format!("Failed to retrieve password: {:?}", e))
    })?;

    let password = credential
        .Password()
        .map_err(|e| StorageError::RetrieveFailed(format!("Failed to get password: {:?}", e)))?;

    Ok(password.to_string())
}

#[cfg(target_os = "windows")]
fn delete_from_keychain_windows(key: &str) -> Result<(), StorageError> {
    use windows::Security::Credentials::PasswordVault;

    let vault = PasswordVault::new().map_err(|e| {
        StorageError::RetrieveFailed(format!("Failed to access PasswordVault: {:?}", e))
    })?;

    match vault.Retrieve(&SERVICE_NAME.into(), &key.into()) {
        Ok(credential) => vault.Remove(&credential).map_err(|e| {
            StorageError::RetrieveFailed(format!("Failed to remove credential: {:?}", e))
        }),
        Err(error) => {
            let error_message = format!("{:?}", error);
            if error_message.contains("not found") || error_message.contains("0x80070490") {
                Ok(())
            } else {
                Err(StorageError::RetrieveFailed(error_message))
            }
        }
    }
}

#[cfg(any(
    all(target_os = "macos", debug_assertions),
    not(any(target_os = "macos", target_os = "windows"))
))]
fn save_to_file_fallback(key: &str, value: &str) -> Result<(), StorageError> {
    let mut settings = read_file_fallback_settings()?;
    settings.insert(key.to_string(), value.to_string());
    write_file_fallback_settings(&settings)
}

#[cfg(any(
    all(target_os = "macos", debug_assertions),
    not(any(target_os = "macos", target_os = "windows"))
))]
fn get_from_file_fallback(key: &str) -> Result<String, StorageError> {
    let settings = read_file_fallback_settings()?;
    settings.get(key).cloned().ok_or(StorageError::KeyNotFound)
}

#[cfg(any(
    all(target_os = "macos", debug_assertions),
    not(any(target_os = "macos", target_os = "windows"))
))]
fn delete_from_file_fallback(key: &str) -> Result<(), StorageError> {
    let mut settings = read_file_fallback_settings()?;
    settings.remove(key);
    write_file_fallback_settings(&settings)
}

#[cfg(any(
    all(target_os = "macos", debug_assertions),
    not(any(target_os = "macos", target_os = "windows"))
))]
fn read_file_fallback_settings() -> Result<std::collections::HashMap<String, String>, StorageError>
{
    let settings_file_path = file_fallback_settings_path()?;
    if !settings_file_path.exists() {
        return Ok(std::collections::HashMap::new());
    }

    let content = std::fs::read_to_string(&settings_file_path)
        .map_err(|error| StorageError::RetrieveFailed(error.to_string()))?;

    serde_json::from_str::<std::collections::HashMap<String, String>>(&content)
        .map_err(|error| StorageError::RetrieveFailed(format!("Failed to parse settings: {error}")))
}

#[cfg(any(
    all(target_os = "macos", debug_assertions),
    not(any(target_os = "macos", target_os = "windows"))
))]
fn write_file_fallback_settings(
    settings: &std::collections::HashMap<String, String>,
) -> Result<(), StorageError> {
    let settings_file_path = file_fallback_settings_path()?;
    if let Some(parent_directory) = settings_file_path.parent() {
        std::fs::create_dir_all(parent_directory)
            .map_err(|error| StorageError::SaveFailed(error.to_string()))?;
        set_secure_directory_permissions(parent_directory)
            .map_err(|error| StorageError::SaveFailed(error.to_string()))?;
    }

    let content = serde_json::to_string_pretty(settings).map_err(|error| {
        StorageError::SaveFailed(format!("Failed to serialize settings: {error}"))
    })?;
    std::fs::write(&settings_file_path, content)
        .map_err(|error| StorageError::SaveFailed(error.to_string()))?;
    set_secure_file_permissions(&settings_file_path)
        .map_err(|error| StorageError::SaveFailed(error.to_string()))
}

#[cfg(any(
    all(target_os = "macos", debug_assertions),
    not(any(target_os = "macos", target_os = "windows"))
))]
fn file_fallback_settings_path() -> Result<std::path::PathBuf, StorageError> {
    let config_base = std::env::var_os("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|home| std::path::PathBuf::from(home).join(".config"))
        })
        .ok_or_else(|| {
            StorageError::RetrieveFailed("Could not resolve config directory".to_string())
        })?;

    Ok(config_base.join(SERVICE_NAME).join(SETTINGS_FILE_NAME))
}

#[cfg(all(
    unix,
    any(
        all(target_os = "macos", debug_assertions),
        not(any(target_os = "macos", target_os = "windows"))
    )
))]
fn set_secure_directory_permissions(path: &std::path::Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let permissions = std::fs::Permissions::from_mode(0o700);
    std::fs::set_permissions(path, permissions)
}

#[cfg(all(
    unix,
    any(
        all(target_os = "macos", debug_assertions),
        not(any(target_os = "macos", target_os = "windows"))
    )
))]
fn set_secure_file_permissions(path: &std::path::Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let permissions = std::fs::Permissions::from_mode(0o600);
    std::fs::set_permissions(path, permissions)
}

#[cfg(all(
    not(unix),
    any(
        all(target_os = "macos", debug_assertions),
        not(any(target_os = "macos", target_os = "windows"))
    )
))]
fn set_secure_directory_permissions(_path: &std::path::Path) -> std::io::Result<()> {
    Ok(())
}

#[cfg(all(
    not(unix),
    any(
        all(target_os = "macos", debug_assertions),
        not(any(target_os = "macos", target_os = "windows"))
    )
))]
fn set_secure_file_permissions(_path: &std::path::Path) -> std::io::Result<()> {
    Ok(())
}

/// Get the path to the settings JSON file
fn get_settings_path() -> Result<PathBuf, StorageError> {
    let config_dir = dirs::config_dir().ok_or_else(|| {
        StorageError::SettingsLoadFailed("Could not resolve config directory".to_string())
    })?;

    let app_config_dir = config_dir.join(SERVICE_NAME);
    Ok(app_config_dir.join("settings.json"))
}

/// Load settings from JSON file, returning defaults if file doesn't exist
pub fn load_settings() -> Result<Settings, StorageError> {
    let settings_path = get_settings_path()?;

    if !settings_path.exists() {
        return Ok(Settings::default());
    }

    let content = std::fs::read_to_string(&settings_path).map_err(|error| {
        StorageError::SettingsLoadFailed(format!("Failed to read settings file: {}", error))
    })?;

    serde_json::from_str(&content).map_err(|error| {
        StorageError::SettingsLoadFailed(format!("Failed to parse settings JSON: {}", error))
    })
}

/// Save settings to JSON file
pub fn save_settings(settings: &Settings) -> Result<(), StorageError> {
    let settings_path = get_settings_path()?;

    if let Some(parent_dir) = settings_path.parent() {
        std::fs::create_dir_all(parent_dir).map_err(|error| {
            StorageError::SettingsSaveFailed(format!(
                "Failed to create config directory: {}",
                error
            ))
        })?;
    }

    let content = serde_json::to_string_pretty(settings).map_err(|error| {
        StorageError::SettingsSaveFailed(format!("Failed to serialize settings: {}", error))
    })?;

    std::fs::write(&settings_path, content).map_err(|error| {
        StorageError::SettingsSaveFailed(format!("Failed to write settings file: {}", error))
    })?;

    Ok(())
}

#[cfg(all(
    test,
    any(all(target_os = "macos", not(debug_assertions)), target_os = "windows")
))]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Requires access to OS keychain/credential vault"]
    #[cfg(any(all(target_os = "macos", not(debug_assertions)), target_os = "windows"))]
    fn test_save_and_retrieve() {
        let test_key = "test_api_key";
        let test_value = "test_secret_value_12345";

        // Save
        let save_result = save_to_keychain(test_key, test_value);
        assert!(save_result.is_ok(), "Failed to save: {:?}", save_result);

        // Retrieve
        let retrieve_result = get_from_keychain(test_key);
        assert!(
            retrieve_result.is_ok(),
            "Failed to retrieve: {:?}",
            retrieve_result
        );
        assert_eq!(retrieve_result.unwrap(), test_value);

        // Clean up - delete the test key
        #[cfg(target_os = "macos")]
        {
            use security_framework::passwords::delete_generic_password;
            let _ = delete_generic_password(SERVICE_NAME, test_key);
        }

        #[cfg(target_os = "windows")]
        {
            use windows::Security::Credentials::PasswordVault;
            if let Ok(vault) = PasswordVault::new() {
                if let Ok(credential) = vault.Retrieve(&SERVICE_NAME.into(), &test_key.into()) {
                    let _ = vault.Remove(&credential);
                }
            }
        }
    }

    #[test]
    #[ignore = "Requires access to OS keychain/credential vault"]
    #[cfg(any(all(target_os = "macos", not(debug_assertions)), target_os = "windows"))]
    fn test_retrieve_nonexistent_key() {
        let result = get_from_keychain("nonexistent_key_that_should_not_exist");
        assert!(matches!(result, Err(StorageError::KeyNotFound)));
    }
}
