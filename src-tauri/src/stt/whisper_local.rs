use crate::audio::RecordedAudio;
use crate::audio_processing;
use crate::models::Settings;
use std::path::Path;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

const MIN_AUDIO_DURATION_SECONDS: f32 = 0.20;

pub async fn transcribe(
    recorded_audio: RecordedAudio,
    settings: &Settings,
) -> Result<String, String> {
    if recorded_audio.sample_rate == 0 {
        return Err("Audio too short to transcribe".to_string());
    }

    let duration_seconds = recorded_audio.samples.len() as f32
        / recorded_audio.sample_rate as f32
        / recorded_audio.channel_count.max(1) as f32;
    if duration_seconds < MIN_AUDIO_DURATION_SECONDS {
        return Err("Audio too short to transcribe".to_string());
    }

    let model_path = settings
        .whisper_model_path
        .as_deref()
        .map(str::trim)
        .filter(|path| !path.is_empty())
        .ok_or_else(|| "Whisper model path is not set".to_string())?;
    let model_path = model_path.to_string();

    if !Path::new(&model_path).is_file() {
        return Err(format!("Whisper model file not found: {model_path}"));
    }

    let language = settings
        .whisper_language
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);

    let threads = settings
        .whisper_threads
        .and_then(|value| i32::try_from(value).ok())
        .filter(|value| *value > 0)
        .unwrap_or_else(default_thread_count);

    let mono_samples =
        audio_processing::downmix_to_mono(&recorded_audio.samples, recorded_audio.channel_count);
    let mono_16khz = audio_processing::resample_to_16k(&mono_samples, recorded_audio.sample_rate);

    let result_text = tokio::task::spawn_blocking(move || {
        transcribe_with_whisper(&model_path, language.as_deref(), threads, &mono_16khz)
    })
    .await
    .map_err(|error| format!("Whisper failed: {error}"))??;

    if result_text.trim().is_empty() {
        return Err("No speech detected".to_string());
    }

    Ok(result_text.trim().to_string())
}

fn transcribe_with_whisper(
    model_path: &str,
    language: Option<&str>,
    threads: i32,
    samples_16khz: &[f32],
) -> Result<String, String> {
    let context_parameters = WhisperContextParameters::default();
    let context = WhisperContext::new_with_params(model_path, context_parameters)
        .map_err(|error| format!("Whisper failed: {error}"))?;
    let mut state = context
        .create_state()
        .map_err(|error| format!("Whisper failed: {error}"))?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_print_special(false);
    params.set_translate(false);
    params.set_n_threads(threads);
    if let Some(language_code) = language {
        params.set_language(Some(language_code));
    }

    state
        .full(params, samples_16khz)
        .map_err(|error| format!("Whisper failed: {error}"))?;

    let segment_count = state.full_n_segments();

    let mut segments = Vec::new();
    for index in 0..segment_count {
        let segment = state
            .get_segment(index)
            .ok_or_else(|| format!("Whisper failed: segment {index} is out of bounds"))?;
        let segment_text = segment
            .to_str_lossy()
            .map_err(|error| format!("Whisper failed: {error}"))?;
        let trimmed = segment_text.trim();
        if !trimmed.is_empty() {
            segments.push(trimmed.to_string());
        }
    }

    Ok(segments.join(" "))
}

fn default_thread_count() -> i32 {
    std::thread::available_parallelism()
        .map(|parallelism| parallelism.get())
        .ok()
        .and_then(|value| i32::try_from(value).ok())
        .filter(|value| *value > 0)
        .unwrap_or(1)
}
