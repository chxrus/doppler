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
