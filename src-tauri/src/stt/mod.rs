mod whisper_local;

use crate::audio;
use crate::audio::RecordedAudio;
use crate::models::Settings;
use crate::storage;

const API_KEY_STORAGE_KEY: &str = "gemini_api_key";

pub async fn transcribe(
    recorded_audio: RecordedAudio,
    settings: &Settings,
) -> Result<String, String> {
    match settings.stt_provider.trim().to_lowercase().as_str() {
        "gemini" => {
            let api_key =
                storage::get_from_keychain(API_KEY_STORAGE_KEY).map_err(|error| match error {
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
        "whisper" => whisper_local::transcribe(recorded_audio, settings).await,
        other => Err(format!("Unsupported STT provider: {other}")),
    }
}
