import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { normalizeTheme } from '$lib/utils/theme';

export interface AppSettings {
  theme: 'dark' | 'light';
  text_provider: string;
  stt_provider: string;
  gemini_model: string;
  gemini_temperature: number;
  whisper_model_path: string;
  whisper_language: string;
  whisper_threads: number | null;
  ollama_base_url: string;
  ollama_model: string;
  tts_rate: number;
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

export const DEFAULT_SETTINGS: AppSettings = {
  theme: 'dark',
  text_provider: 'gemini',
  stt_provider: 'gemini',
  gemini_model: 'gemini-2.5-flash',
  gemini_temperature: 0.7,
  whisper_model_path: '',
  whisper_language: '',
  whisper_threads: null,
  ollama_base_url: 'http://localhost:11434',
  ollama_model: 'llama3.2:3b',
  tts_rate: 1.0,
  recording_source: 'microphone',
  recording_input_device: 'Default input',
  opacity: 0.95,
  always_on_top: true,
  click_through: false,
  screen_capture_protection: true,
  hotkey_toggle: 'CommandOrControl+,',
  hotkey_record: 'CommandOrControl+R',
  hotkey_previous: 'Alt+Left',
  hotkey_next: 'Alt+Right',
  hotkey_send: 'Enter',
  hotkey_click_through: 'CommandOrControl+Shift+X',
  hotkey_capture_visibility: 'CommandOrControl+Shift+H'
};

function createSettingsStore() {
  const { subscribe, set, update } = writable<AppSettings>(DEFAULT_SETTINGS);
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  return {
    subscribe,

    async loadSettings(): Promise<void> {
      try {
        const settings = await invoke<AppSettings>('get_settings');
        const merged = { ...DEFAULT_SETTINGS, ...settings };
        merged.theme = normalizeTheme(merged.theme);
        merged.whisper_model_path =
          typeof merged.whisper_model_path === 'string' ? merged.whisper_model_path : '';
        merged.whisper_language =
          typeof merged.whisper_language === 'string' ? merged.whisper_language : '';
        merged.whisper_threads =
          typeof merged.whisper_threads === 'number' && merged.whisper_threads > 0
            ? merged.whisper_threads
            : null;
        set(merged);
      } catch (error) {
        console.warn('Failed to load settings, using defaults:', error);
        set(DEFAULT_SETTINGS);
      }
    },

    async updateSettings(settings: AppSettings): Promise<void> {
      try {
        await invoke('update_settings', { settings });
        set(settings);
      } catch (error) {
        console.error('Failed to update settings:', error);
        throw error;
      }
    },

    updateField<K extends keyof AppSettings>(
      field: K,
      value: AppSettings[K],
      debounce = false
    ): void {
      update(current => {
        const updated = { ...current, [field]: value };

        if (debounce) {
          if (debounceTimer != null) {
            clearTimeout(debounceTimer);
          }
          debounceTimer = setTimeout(() => {
            void invoke('update_settings', { settings: updated });
          }, 500);
        } else {
          void invoke('update_settings', { settings: updated });
        }

        return updated;
      });
    }
  };
}

export const settingsStore = createSettingsStore();
