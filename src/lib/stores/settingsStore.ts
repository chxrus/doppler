import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

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

const defaultSettings: AppSettings = {
  gemini_model: 'gemini-pro',
  gemini_temperature: 0.7,
  opacity: 0.95,
  always_on_top: true,
  click_through: false,
  screen_capture_protection: true,
  hotkey_toggle: 'CommandOrControl+Shift+Space',
  hotkey_record: 'CommandOrControl+Shift+R'
};

function createSettingsStore() {
  const { subscribe, set, update } = writable<AppSettings>(defaultSettings);
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  return {
    subscribe,

    async loadSettings(): Promise<void> {
      try {
        const settings = await invoke<AppSettings>('get_settings');
        set(settings);
      } catch (error) {
        console.warn('Failed to load settings, using defaults:', error);
        set(defaultSettings);
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
