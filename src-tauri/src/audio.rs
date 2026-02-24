// Allow dead_code until audio commands are implemented in the next task
#![allow(dead_code)]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use once_cell::sync::OnceCell;
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
}

enum AudioCommand {
    Start(mpsc::Sender<Result<(), AudioError>>),
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
});

static AUDIO_WORKER: OnceCell<AudioWorker> = OnceCell::new();

/// Start microphone recording.
pub fn start_microphone() -> Result<(), AudioError> {
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
        .send(AudioCommand::Start(response_tx))
        .map_err(|_| AudioError::WorkerUnavailable)?;

    let start_result = response_rx
        .recv()
        .map_err(|_| AudioError::WorkerUnavailable)?;

    let mut state = AUDIO_STATE.lock().unwrap();
    state.starting = false;

    match start_result {
        Ok(()) => {
            state.recording = true;
            Ok(())
        }
        Err(error) => {
            state.recording = false;
            Err(error)
        }
    }
}

/// Stop microphone recording and return captured samples.
pub fn stop_microphone() -> Result<Vec<f32>, AudioError> {
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
    Ok(state.audio_buffer.clone())
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
            AudioCommand::Start(response_tx) => {
                if active_stream.is_some() {
                    let _ = response_tx.send(Err(AudioError::AlreadyRecording));
                    continue;
                }

                let start_result = create_and_start_stream().map(|stream| {
                    active_stream = Some(stream);
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

fn create_and_start_stream() -> Result<cpal::Stream, AudioError> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or(AudioError::NoInputDevice)?;
    let config = device
        .default_input_config()
        .map_err(|error| AudioError::ConfigError(error.to_string()))?;

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => build_input_stream::<f32>(&device, &config.into())?,
        cpal::SampleFormat::I16 => build_input_stream::<i16>(&device, &config.into())?,
        cpal::SampleFormat::U16 => build_input_stream::<u16>(&device, &config.into())?,
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

    Ok(stream)
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
