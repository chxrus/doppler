import { invoke } from '@tauri-apps/api/core';

export interface AppSettings {
  apiKey: string | null;
}

export async function loadSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('load_settings');
}

export async function saveSettings(settings: AppSettings): Promise<void> {
  await invoke('save_settings', { settings });
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
