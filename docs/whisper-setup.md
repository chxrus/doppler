# Whisper setup (offline STT)

This guide explains how to enable local speech-to-text in Doppler using `whisper.cpp` models with no network requests.

## 1. Build Doppler with Whisper support

Whisper is feature-gated to avoid extra build requirements when not needed.

Build/run with:

```bash
cargo run --features local-whisper
```

If you build from the project root via Tauri tooling, pass the same feature to Rust build (for example through your Tauri build configuration/CLI flags).

## 2. Install required build tools

`whisper-rs` depends on `whisper-rs-sys`, which requires `cmake` at build time.

- macOS: `brew install cmake`
- Ubuntu/Debian: `sudo apt-get install cmake`
- Windows: install [CMake](https://cmake.org/download/) and ensure it is in `PATH`

## 3. Download a Whisper model file

Download any GGML model from the official `whisper.cpp` model list:

- [whisper.cpp models](https://github.com/ggerganov/whisper.cpp#available-models-and-languages)

Example file names:
- `ggml-tiny.bin`
- `ggml-base.bin`
- `ggml-small.bin`

## 4. Configure Doppler settings

Open `Settings -> AI`:

1. Set `Speech-to-Text Provider` to `Whisper (local)`.
2. Set `Model path` to the local path of your `.bin` model file.
3. Optional: set `Language` (for example `en`, `ru`) or leave empty for auto-detect.
4. Optional: set `Threads` or leave empty to use CPU default.

## 5. Verify offline transcription

1. Turn off internet.
2. Record 3-10 seconds of speech.
3. Stop recording and wait for transcription.

Expected behavior:
- no network calls
- no API key required

Common errors:
- `Whisper model path is not set`
- `Whisper model file not found: <path>`
- `Audio too short to transcribe`
- `No speech detected`
