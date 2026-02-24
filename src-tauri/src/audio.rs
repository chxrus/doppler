// Allow dead_code until audio commands are implemented in the next task
#![allow(dead_code)]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use once_cell::sync::OnceCell;
use serde::Serialize;
use std::sync::{mpsc, Mutex};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("Failed to get default audio host")]
    NoHost,

    #[error("Failed to get default input device")]
    NoInputDevice,

    #[error("Failed to get device configuration: {0}")]
    ConfigError(String),

    #[error("Failed to build audio stream: {0}")]
    StreamBuildError(String),

    #[error("Failed to start audio stream: {0}")]
    StreamPlayError(String),

    #[error("Recording is not active")]
    NotRecording,

    #[error("Recording is already active")]
    AlreadyRecording,

    #[error("Audio worker unavailable")]
    WorkerUnavailable,
}

#[derive(Default)]
struct AudioState {
    recording: bool,
    starting: bool,
    audio_buffer: Vec<f32>,
    sample_rate: u32,
    channel_count: u16,
}

enum AudioCommand {
    Start(
        RecordingOptions,
        mpsc::Sender<Result<(u32, u16), AudioError>>,
    ),
    Stop(mpsc::Sender<Result<(), AudioError>>),
}

struct AudioWorker {
    tx: mpsc::Sender<AudioCommand>,
}

// Global state for recording flags and captured samples.
static AUDIO_STATE: Mutex<AudioState> = Mutex::new(AudioState {
    recording: false,
    starting: false,
    audio_buffer: Vec::new(),
    sample_rate: 0,
    channel_count: 0,
});

static AUDIO_WORKER: OnceCell<AudioWorker> = OnceCell::new();

#[derive(Debug, Clone)]
pub enum RecordingSource {
    Microphone,
    SystemAudio,
}

#[derive(Debug, Clone)]
pub struct RecordingOptions {
    pub source: RecordingSource,
    pub preferred_device_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AudioInputDeviceInfo {
    pub name: String,
    pub is_default: bool,
    pub likely_system_audio: bool,
}

/// Start microphone recording.
pub fn start_microphone(options: RecordingOptions) -> Result<(), AudioError> {
    {
        let mut state = AUDIO_STATE.lock().unwrap();
        if state.recording || state.starting {
            return Err(AudioError::AlreadyRecording);
        }
        state.audio_buffer.clear();
        state.starting = true;
    }

    let worker = audio_worker()?;
    let (response_tx, response_rx) = mpsc::channel();
    worker
        .tx
        .send(AudioCommand::Start(options, response_tx))
        .map_err(|_| AudioError::WorkerUnavailable)?;

    let start_result = response_rx
        .recv()
        .map_err(|_| AudioError::WorkerUnavailable)?;

    let mut state = AUDIO_STATE.lock().unwrap();
    state.starting = false;

    match start_result {
        Ok((sample_rate, channel_count)) => {
            state.recording = true;
            state.sample_rate = sample_rate;
            state.channel_count = channel_count;
            Ok(())
        }
        Err(error) => {
            state.recording = false;
            state.sample_rate = 0;
            state.channel_count = 0;
            Err(error)
        }
    }
}

/// Stop microphone recording and return captured samples.
pub fn stop_microphone() -> Result<RecordedAudio, AudioError> {
    {
        let state = AUDIO_STATE.lock().unwrap();
        if !state.recording {
            return Err(AudioError::NotRecording);
        }
    }

    // Stop accepting callback samples before stopping the stream.
    {
        let mut state = AUDIO_STATE.lock().unwrap();
        state.recording = false;
        state.starting = false;
    }

    let worker = audio_worker()?;
    let (response_tx, response_rx) = mpsc::channel();
    worker
        .tx
        .send(AudioCommand::Stop(response_tx))
        .map_err(|_| AudioError::WorkerUnavailable)?;

    response_rx
        .recv()
        .map_err(|_| AudioError::WorkerUnavailable)??;

    let state = AUDIO_STATE.lock().unwrap();
    Ok(RecordedAudio {
        samples: state.audio_buffer.clone(),
        sample_rate: state.sample_rate,
        channel_count: state.channel_count,
    })
}

fn audio_worker() -> Result<&'static AudioWorker, AudioError> {
    AUDIO_WORKER.get_or_try_init(|| {
        let (tx, rx) = mpsc::channel::<AudioCommand>();

        std::thread::Builder::new()
            .name("audio-worker".to_string())
            .spawn(move || run_audio_worker(rx))
            .map_err(|_| AudioError::WorkerUnavailable)?;

        Ok(AudioWorker { tx })
    })
}

fn run_audio_worker(rx: mpsc::Receiver<AudioCommand>) {
    let mut active_stream: Option<cpal::Stream> = None;

    while let Ok(command) = rx.recv() {
        match command {
            AudioCommand::Start(options, response_tx) => {
                if active_stream.is_some() {
                    let _ = response_tx.send(Err(AudioError::AlreadyRecording));
                    continue;
                }

                let start_result =
                    create_and_start_stream(options).map(|(stream, sample_rate, channel_count)| {
                        active_stream = Some(stream);
                        (sample_rate, channel_count)
                    });

                let _ = response_tx.send(start_result);
            }
            AudioCommand::Stop(response_tx) => {
                active_stream = None;
                let _ = response_tx.send(Ok(()));
            }
        }
    }
}

fn create_and_start_stream(
    options: RecordingOptions,
) -> Result<(cpal::Stream, u32, u16), AudioError> {
    let host = cpal::default_host();
    let device = choose_input_device(&host, &options)?;
    let config = device
        .default_input_config()
        .map_err(|error| AudioError::ConfigError(error.to_string()))?;

    let stream_config: cpal::StreamConfig = config.clone().into();
    let sample_rate = stream_config.sample_rate.0;
    let channel_count = stream_config.channels;

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => build_input_stream::<f32>(&device, &stream_config)?,
        cpal::SampleFormat::I16 => build_input_stream::<i16>(&device, &stream_config)?,
        cpal::SampleFormat::U16 => build_input_stream::<u16>(&device, &stream_config)?,
        sample_format => {
            return Err(AudioError::ConfigError(format!(
                "Unsupported sample format: {:?}",
                sample_format
            )));
        }
    };

    stream
        .play()
        .map_err(|error| AudioError::StreamPlayError(error.to_string()))?;

    Ok((stream, sample_rate, channel_count))
}

fn choose_input_device(
    host: &cpal::Host,
    options: &RecordingOptions,
) -> Result<cpal::Device, AudioError> {
    let mut input_devices = host
        .input_devices()
        .map_err(|error| AudioError::ConfigError(error.to_string()))?;
    let devices: Vec<cpal::Device> = input_devices.by_ref().collect();

    let preferred_device = options
        .preferred_device_name
        .as_ref()
        .map(|name| name.trim())
        .filter(|name| !name.is_empty() && *name != "Default input")
        .and_then(|preferred_device_name| {
            devices
                .iter()
                .find(|device| device.name().ok().as_deref() == Some(preferred_device_name))
                .cloned()
        });

    match options.source {
        RecordingSource::Microphone => {
            if let Some(device) = preferred_device {
                let device_name = device.name().unwrap_or_default();
                if !is_likely_system_audio_device(&device_name) {
                    return Ok(device);
                }
            }

            // Prefer a non-loopback input when microphone source is selected.
            if let Some(device) = host.default_input_device() {
                let device_name = device.name().unwrap_or_default();
                if !is_likely_system_audio_device(&device_name) {
                    return Ok(device);
                }
            }

            devices
                .into_iter()
                .find(|device| {
                    device
                        .name()
                        .ok()
                        .map(|name| !is_likely_system_audio_device(&name))
                        .unwrap_or(false)
                })
                .or_else(|| host.default_input_device())
                .ok_or(AudioError::NoInputDevice)
        }
        RecordingSource::SystemAudio => {
            if let Some(device) = preferred_device {
                return Ok(device);
            }

            devices
                .into_iter()
                .find(|device| {
                    device
                        .name()
                        .ok()
                        .map(|name| is_likely_system_audio_device(&name))
                        .unwrap_or(false)
                })
                .or_else(|| host.default_input_device())
                .ok_or(AudioError::NoInputDevice)
        }
    }
}

fn is_likely_system_audio_device(device_name: &str) -> bool {
    let normalized_name = device_name.to_lowercase();
    [
        "blackhole",
        "loopback",
        "soundflower",
        "vb-cable",
        "stereo mix",
        "aggregate",
        "monitor",
    ]
    .iter()
    .any(|keyword| normalized_name.contains(keyword))
}

pub fn list_input_devices() -> Result<Vec<AudioInputDeviceInfo>, AudioError> {
    let host = cpal::default_host();
    let default_input_name = host
        .default_input_device()
        .and_then(|device| device.name().ok());

    let mut devices = vec![AudioInputDeviceInfo {
        name: "Default input".to_string(),
        is_default: true,
        likely_system_audio: false,
    }];

    for device in host
        .input_devices()
        .map_err(|error| AudioError::ConfigError(error.to_string()))?
    {
        let name = device
            .name()
            .map_err(|error| AudioError::ConfigError(error.to_string()))?;
        devices.push(AudioInputDeviceInfo {
            is_default: default_input_name.as_deref() == Some(name.as_str()),
            likely_system_audio: is_likely_system_audio_device(&name),
            name,
        });
    }

    Ok(devices)
}

fn build_input_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
) -> Result<cpal::Stream, AudioError>
where
    T: cpal::SizedSample,
    f32: cpal::FromSample<T>,
{
    let err_fn = |error| eprintln!("Audio stream error: {}", error);

    device
        .build_input_stream(
            config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                let samples: Vec<f32> = data
                    .iter()
                    .map(|&sample| cpal::Sample::from_sample(sample))
                    .collect();

                if let Ok(mut state) = AUDIO_STATE.lock() {
                    if state.recording {
                        state.audio_buffer.extend_from_slice(&samples);
                    }
                }
            },
            err_fn,
            None,
        )
        .map_err(|error| AudioError::StreamBuildError(error.to_string()))
}
/// Convert audio samples to WAV format for API transmission.
fn samples_to_wav(
    samples: &[f32],
    sample_rate: u32,
    channel_count: u16,
) -> Result<Vec<u8>, AudioError> {
    use std::io::Write;

    let num_channels = channel_count;
    let bits_per_sample = 16u16;
    let byte_rate = sample_rate * u32::from(num_channels) * u32::from(bits_per_sample) / 8;
    let block_align = num_channels * bits_per_sample / 8;

    // Convert f32 samples to i16
    let i16_samples: Vec<i16> = samples
        .iter()
        .map(|&sample| {
            let clamped = sample.clamp(-1.0, 1.0);
            (clamped * 32767.0) as i16
        })
        .collect();

    let data_size = (i16_samples.len() * 2) as u32;
    let file_size = 36 + data_size;

    let mut wav_data = Vec::new();

    // RIFF header
    wav_data.write_all(b"RIFF")?;
    wav_data.write_all(&file_size.to_le_bytes())?;
    wav_data.write_all(b"WAVE")?;

    // fmt chunk
    wav_data.write_all(b"fmt ")?;
    wav_data.write_all(&16u32.to_le_bytes())?; // fmt chunk size
    wav_data.write_all(&1u16.to_le_bytes())?; // audio format (PCM)
    wav_data.write_all(&num_channels.to_le_bytes())?;
    wav_data.write_all(&sample_rate.to_le_bytes())?;
    wav_data.write_all(&byte_rate.to_le_bytes())?;
    wav_data.write_all(&block_align.to_le_bytes())?;
    wav_data.write_all(&bits_per_sample.to_le_bytes())?;

    // data chunk
    wav_data.write_all(b"data")?;
    wav_data.write_all(&data_size.to_le_bytes())?;

    for sample in i16_samples {
        wav_data.write_all(&sample.to_le_bytes())?;
    }

    Ok(wav_data)
}

impl From<std::io::Error> for AudioError {
    fn from(error: std::io::Error) -> Self {
        AudioError::StreamBuildError(format!("IO error: {}", error))
    }
}

#[derive(Debug, Error)]
pub enum TranscriptionError {
    #[error("Audio conversion failed: {0}")]
    ConversionError(String),

    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Invalid API response: {0}")]
    InvalidResponse(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("No transcription in response")]
    NoTranscription,

    #[error("Audio data is empty or too short")]
    EmptyAudio,
}

#[derive(Debug, Clone)]
pub struct RecordedAudio {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channel_count: u16,
}

/// Transcribe audio samples using Gemini API.
pub async fn transcribe(
    api_key: &str,
    samples: Vec<f32>,
    sample_rate: u32,
    channel_count: u16,
) -> Result<String, TranscriptionError> {
    const MIN_DURATION_MS: usize = 50;

    if sample_rate == 0 {
        return Err(TranscriptionError::ConversionError(
            "Audio sample rate is unavailable".to_string(),
        ));
    }
    if channel_count == 0 {
        return Err(TranscriptionError::ConversionError(
            "Audio channel count is unavailable".to_string(),
        ));
    }

    let min_samples =
        ((sample_rate as usize * MIN_DURATION_MS) / 1000) * usize::from(channel_count);
    if samples.len() < min_samples {
        return Err(TranscriptionError::EmptyAudio);
    }

    let api_key = api_key.trim();
    if api_key.is_empty() {
        return Err(TranscriptionError::InvalidResponse(
            "API key is empty".to_string(),
        ));
    }

    // Convert samples to WAV format
    let wav_data = samples_to_wav(&samples, sample_rate, channel_count)
        .map_err(|e| TranscriptionError::ConversionError(e.to_string()))?;

    // Encode WAV as base64
    let base64_audio = base64_encode(&wav_data);

    // Call Gemini API with audio input
    let client = reqwest::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
        crate::gemini::default_model()
    );

    let response = client
        .post(&url)
        .header("x-goog-api-key", api_key)
        .json(&serde_json::json!({
            "contents": [{
                "parts": [
                    {
                        "text": "Transcribe this audio exactly as spoken. Preserve the original language (including Russian) and do not translate. Return only the transcription text."
                    },
                    {
                        "inlineData": {
                            "mimeType": "audio/wav",
                            "data": base64_audio
                        }
                    }
                ]
            }]
        }))
        .send()
        .await?;

    let status = response.status();
    let raw_body = response.text().await?;

    if !status.is_success() {
        return Err(parse_transcription_error(status, &raw_body));
    }

    let body: serde_json::Value = serde_json::from_str(&raw_body).map_err(|error| {
        TranscriptionError::InvalidResponse(format!(
            "Failed to parse Gemini response (status {status}): {error}"
        ))
    })?;

    if let Some(message) = body["error"]["message"]
        .as_str()
        .map(str::trim)
        .filter(|message| !message.is_empty())
    {
        return Err(TranscriptionError::ApiError(message.to_string()));
    }

    if let Some(block_reason) = body["promptFeedback"]["blockReason"]
        .as_str()
        .map(str::trim)
        .filter(|reason| !reason.is_empty())
    {
        return Err(TranscriptionError::ApiError(format!(
            "Prompt blocked by Gemini: {block_reason}"
        )));
    }

    let transcription = body["candidates"].as_array().and_then(|candidates| {
        let mut fragments = Vec::new();
        for candidate in candidates {
            if let Some(parts) = candidate["content"]["parts"].as_array() {
                for part in parts {
                    if let Some(text) = part["text"].as_str() {
                        let trimmed = text.trim();
                        if !trimmed.is_empty() {
                            fragments.push(trimmed.to_string());
                        }
                    }
                }
            }
        }
        if fragments.is_empty() {
            None
        } else {
            Some(fragments.join("\n"))
        }
    });

    if let Some(text) = transcription {
        return Ok(text);
    }

    if let Some(finish_reason) = body["candidates"]
        .as_array()
        .and_then(|candidates| candidates.first())
        .and_then(|candidate| candidate["finishReason"].as_str())
    {
        return Err(TranscriptionError::ApiError(format!(
            "No transcription text in response (finishReason: {finish_reason})"
        )));
    }

    Err(TranscriptionError::NoTranscription)
}

fn base64_encode(data: &[u8]) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = Vec::new();

    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }

        let b1 = (buf[0] >> 2) as usize;
        let b2 = (((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize;
        let b3 = (((buf[1] & 0x0F) << 2) | (buf[2] >> 6)) as usize;
        let b4 = (buf[2] & 0x3F) as usize;

        result.push(CHARSET[b1]);
        result.push(CHARSET[b2]);

        if chunk.len() > 1 {
            result.push(CHARSET[b3]);
        } else {
            result.push(b'=');
        }

        if chunk.len() > 2 {
            result.push(CHARSET[b4]);
        } else {
            result.push(b'=');
        }
    }

    result.into_iter().map(char::from).collect()
}

fn parse_transcription_error(status: reqwest::StatusCode, raw_body: &str) -> TranscriptionError {
    let parsed = serde_json::from_str::<serde_json::Value>(raw_body)
        .ok()
        .and_then(|body| body["error"]["message"].as_str().map(|s| s.to_string()));

    match parsed {
        Some(message) if !message.trim().is_empty() => TranscriptionError::ApiError(message),
        _ => TranscriptionError::ApiError(format!("HTTP {status}")),
    }
}

#[derive(Debug, Error)]
pub enum TtsError {
    #[error("Failed to initialize TTS engine: {0}")]
    Initialization(String),

    #[error("Failed to speak text: {0}")]
    Speak(String),

    #[error("Failed to stop speaking: {0}")]
    Stop(String),
}

static TTS_INSTANCE: Mutex<Option<tts::Tts>> = Mutex::new(None);

/// Speak text using the default system voice.
pub fn speak(text: String) -> Result<(), TtsError> {
    let mut tts_lock = TTS_INSTANCE.lock().unwrap();

    // Initialize TTS if not already initialized
    if tts_lock.is_none() {
        let tts = tts::Tts::default().map_err(|e| TtsError::Initialization(e.to_string()))?;
        *tts_lock = Some(tts);
    }

    let tts = tts_lock.as_mut().unwrap();

    // Speak the text (non-blocking)
    tts.speak(text, false)
        .map_err(|e| TtsError::Speak(e.to_string()))?;

    Ok(())
}

/// Stop speaking.
pub fn stop_speaking() -> Result<(), TtsError> {
    let mut tts_lock = TTS_INSTANCE.lock().unwrap();

    if let Some(tts) = tts_lock.as_mut() {
        tts.stop().map_err(|e| TtsError::Stop(e.to_string()))?;
    }

    Ok(())
}

/// Check whether TTS is currently speaking.
pub fn is_speaking() -> Result<bool, TtsError> {
    let mut tts_lock = TTS_INSTANCE.lock().unwrap();

    if tts_lock.is_none() {
        return Ok(false);
    }

    let tts = tts_lock.as_mut().unwrap();
    tts.is_speaking()
        .map_err(|error| TtsError::Speak(error.to_string()))
}
