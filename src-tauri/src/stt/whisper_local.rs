use crate::audio::RecordedAudio;
use crate::audio_processing;
use crate::models::Settings;
use serde::Serialize;
use std::ffi::CStr;
use std::path::Path;
use whisper_rs::whisper_rs_sys;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

const MIN_AUDIO_DURATION_SECONDS: f32 = 0.20;

#[derive(Clone, Copy)]
enum WhisperDevicePreference {
    Auto,
    Cpu,
    Gpu { device_index: i32 },
}

#[derive(Debug, Clone, Serialize)]
pub struct WhisperComputeDeviceInfo {
    pub id: String,
    pub label: String,
}

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
    let device_preference = parse_device_preference(settings.whisper_device.as_deref());

    let mono_samples =
        audio_processing::downmix_to_mono(&recorded_audio.samples, recorded_audio.channel_count);
    let mono_16khz = audio_processing::resample_to_16k(&mono_samples, recorded_audio.sample_rate);

    let result_text = tokio::task::spawn_blocking(move || {
        transcribe_with_whisper(
            &model_path,
            language.as_deref(),
            threads,
            device_preference,
            &mono_16khz,
        )
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
    device_preference: WhisperDevicePreference,
    samples_16khz: &[f32],
) -> Result<String, String> {
    let mut context_parameters = WhisperContextParameters::default();
    apply_device_preference(&mut context_parameters, device_preference);
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

fn parse_device_preference(value: Option<&str>) -> WhisperDevicePreference {
    let normalized = value.unwrap_or("auto").trim().to_lowercase();
    if let Some(raw_device_index) = normalized.strip_prefix("gpu:") {
        if let Ok(device_index) = raw_device_index.parse::<i32>() {
            if device_index >= 0 {
                return WhisperDevicePreference::Gpu { device_index };
            }
        }
    }

    match normalized.as_str() {
        "cpu" => WhisperDevicePreference::Cpu,
        "gpu" => WhisperDevicePreference::Gpu { device_index: 0 },
        _ => WhisperDevicePreference::Auto,
    }
}

fn apply_device_preference(
    context_parameters: &mut WhisperContextParameters<'_>,
    device_preference: WhisperDevicePreference,
) {
    match device_preference {
        WhisperDevicePreference::Auto => {}
        WhisperDevicePreference::Cpu => {
            context_parameters.use_gpu(false);
        }
        WhisperDevicePreference::Gpu { device_index } => {
            context_parameters.use_gpu(true);
            context_parameters.gpu_device(device_index);
        }
    }
}

pub fn list_compute_devices() -> Vec<WhisperComputeDeviceInfo> {
    let mut devices = vec![WhisperComputeDeviceInfo {
        id: "cpu".to_string(),
        label: "CPU".to_string(),
    }];

    let mut gpu_index = 0_i32;

    unsafe {
        whisper_rs_sys::ggml_backend_load_all();
        let device_count = whisper_rs_sys::ggml_backend_dev_count();
        for index in 0..device_count {
            let device = whisper_rs_sys::ggml_backend_dev_get(index);
            if device.is_null() {
                continue;
            }

            let device_type = whisper_rs_sys::ggml_backend_dev_type(device);
            let is_gpu =
                device_type == whisper_rs_sys::ggml_backend_dev_type_GGML_BACKEND_DEVICE_TYPE_GPU;

            if !is_gpu {
                continue;
            }

            let device_name =
                c_string_or_default(whisper_rs_sys::ggml_backend_dev_name(device), "GPU");
            let description = c_string_or_default(
                whisper_rs_sys::ggml_backend_dev_description(device),
                &device_name,
            );
            let label = if description.trim().is_empty() || description == device_name {
                format!("GPU {gpu_index}: {device_name}")
            } else {
                format!("GPU {gpu_index}: {description}")
            };

            devices.push(WhisperComputeDeviceInfo {
                id: format!("gpu:{gpu_index}"),
                label,
            });
            gpu_index += 1;
        }
    }

    devices
}

fn c_string_or_default(value: *const std::os::raw::c_char, fallback: &str) -> String {
    if value.is_null() {
        return fallback.to_string();
    }

    unsafe { CStr::from_ptr(value) }
        .to_string_lossy()
        .trim()
        .to_string()
}
