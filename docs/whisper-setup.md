# Whisper setup (offline STT)

This guide is for Doppler users who want local speech-to-text with no network requests and no API key.

## 1. Download a Whisper model

Get a GGML model file from the official list:

- [whisper.cpp models](https://github.com/ggerganov/whisper.cpp#available-models-and-languages)

Common options:
- `ggml-tiny.bin` (fastest, lower accuracy)
- `ggml-base.bin` (balanced)
- `ggml-small.bin` (higher quality, slower)

Save the `.bin` file to a location you can access later.

## 2. Configure Doppler

Open `Settings -> AI`:

1. Set `Speech-to-Text Provider` to `Whisper (local)`.
2. Set `Model path` to the full path of your downloaded `.bin` file.
3. Optional: set `Language` (for example `en`, `ru`) or leave empty for auto-detect.
4. Optional: set `Threads` or leave empty for automatic value.

## 3. Check that offline transcription works

1. Record 3-10 seconds of speech.
2. Stop recording and wait for transcription.

Expected behavior:
- transcription is done locally
- no API key is required

## Common errors

- `Whisper model path is not set`
  - Set `Model path` in settings.
- `Whisper model file not found: <path>`
  - Check that file exists and path is correct.
- `Audio too short to transcribe`
  - Record a bit longer.
- `No speech detected`
  - Speak louder/clearer, reduce noise, or use a larger model.
