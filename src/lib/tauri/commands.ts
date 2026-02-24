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

export async function setCaptureVisibility(hideFromCapture: boolean): Promise<void> {
  await invoke('set_capture_visibility', { hideFromCapture });
}

export async function setWindowAlwaysOnTop(alwaysOnTop: boolean): Promise<void> {
  await invoke('set_window_always_on_top', { alwaysOnTop });
}

export async function setWindowClickThrough(clickThrough: boolean): Promise<void> {
  await invoke('set_window_click_through', { clickThrough });
}

export interface AppSettings {
  gemini_model: string;
  gemini_temperature: number;
  opacity: number;
  always_on_top: boolean;
  click_through: boolean;
  screen_capture_protection: boolean;
  hotkey_toggle: string;
  hotkey_record: string;
}

export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('get_settings');
}

export async function updateSettings(settings: AppSettings): Promise<void> {
  await invoke('update_settings', { settings });
}
