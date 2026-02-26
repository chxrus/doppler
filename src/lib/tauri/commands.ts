import { invoke } from '@tauri-apps/api/core';

export async function saveApiKey(apiKey: string): Promise<void> {
  await invoke('save_api_key', { apiKey });
}

export async function getApiKey(): Promise<string | null> {
  try {
    return await invoke<string>('get_api_key');
  } catch (error) {
    const message = String(error);
    if (message.toLowerCase().includes('not found')) {
      return null;
    }
    throw error;
  }
}

export async function sendMessage(message: string): Promise<string> {
  return invoke<string>('send_message', { message });
}

export async function sendMessageStream(message: string, requestId: number): Promise<string> {
  return invoke<string>('send_message_stream', { message, requestId });
}

export async function listOllamaModels(baseUrl: string): Promise<string[]> {
  return invoke<string[]>('list_ollama_models', { baseUrl });
}

export async function listLmStudioModels(baseUrl: string): Promise<string[]> {
  return invoke<string[]>('list_lmstudio_models', { baseUrl });
}

export async function startRecording(): Promise<void> {
  await invoke('start_recording');
}

export async function stopRecording(): Promise<void> {
  await invoke('stop_recording');
}

export async function stopRecordingAndTranscribe(): Promise<string> {
  return invoke<string>('stop_recording_and_transcribe');
}

export async function transcribeLastRecording(): Promise<string> {
  return invoke<string>('transcribe_last_recording');
}

export async function setCaptureVisibility(hideFromCapture: boolean): Promise<void> {
  await invoke('set_capture_visibility', { hideFromCapture });
}

export async function setScreenCaptureProtection(enabled: boolean): Promise<void> {
  await invoke('set_screen_capture_protection', { enabled });
}

export async function setWindowAlwaysOnTop(alwaysOnTop: boolean): Promise<void> {
  await invoke('set_window_always_on_top', { alwaysOnTop });
}

export async function setWindowClickThrough(clickThrough: boolean): Promise<void> {
  await invoke('set_window_click_through', { clickThrough });
}

export async function speakText(text: string): Promise<void> {
  await invoke('speak_text', { text });
}

export async function stopSpeaking(): Promise<void> {
  await invoke('stop_speaking');
}

export async function isSpeaking(): Promise<boolean> {
  return invoke<boolean>('is_speaking');
}

export interface RecordingDeviceInfo {
  name: string;
  is_default: boolean;
  likely_system_audio: boolean;
}

export interface WhisperDeviceInfo {
  id: string;
  label: string;
}

export async function listRecordingDevices(): Promise<RecordingDeviceInfo[]> {
  return invoke<RecordingDeviceInfo[]>('list_recording_devices');
}

export async function listWhisperDevices(): Promise<WhisperDeviceInfo[]> {
  return invoke<WhisperDeviceInfo[]>('list_whisper_devices');
}

export interface AppSettings {
  theme: 'dark' | 'light';
  text_provider: string;
  stt_provider: string;
  gemini_model: string;
  gemini_temperature: number;
  whisper_model_path: string;
  whisper_language: string;
  whisper_threads: number | null;
  whisper_device: 'auto' | 'cpu' | `gpu:${number}`;
  ollama_base_url: string;
  ollama_model: string;
  lmstudio_base_url: string;
  lmstudio_model: string;
  tts_rate: number;
  auto_send_transcription: boolean;
  recording_source: string;
  recording_input_device: string;
  opacity: number;
  always_on_top: boolean;
  click_through: boolean;
  screen_capture_protection: boolean;
  hotkey_toggle: string;
  hotkey_record: string;
  hotkey_previous: string;
  hotkey_next: string;
  hotkey_send: string;
  hotkey_click_through: string;
  hotkey_capture_visibility: string;
}

export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('get_settings');
}

export async function updateSettings(settings: AppSettings): Promise<void> {
  await invoke('update_settings', { settings });
}

export async function isWhisperSupported(): Promise<boolean> {
  return invoke<boolean>('is_whisper_supported');
}
