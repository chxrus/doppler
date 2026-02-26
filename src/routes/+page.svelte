<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import ChatView from '$lib/components/ChatView.svelte';
  import SettingsView from '$lib/components/SettingsView.svelte';
  import { settingsStore } from '$lib/stores/settingsStore';
  import { applyTheme } from '$lib/utils/theme';
  import { setScreenCaptureProtection, setWindowAlwaysOnTop, setWindowClickThrough } from '$lib/tauri/commands';

  let isSettingsOpen = $state(false);
  let isClickThroughEnabled = $state(false);

  function toggleSettings() {
    isSettingsOpen = !isSettingsOpen;
  }

  function applyOpacityVariables(opacity: number) {
    const clamped = Math.max(0.1, Math.min(1, opacity));
    const surface = Math.max(0.04, Math.min(0.62, clamped * 0.58));
    const surfaceStrong = Math.max(0.05, Math.min(0.8, clamped * 0.72));
    const border = Math.max(0.12, Math.min(0.75, clamped * 0.78));
    const control = Math.max(0.16, Math.min(0.82, clamped * 0.66));
    const controlStrong = Math.max(0.24, Math.min(0.9, clamped * 0.8));
    document.documentElement.style.setProperty('--doppler-window-alpha', clamped.toString());
    document.documentElement.style.setProperty('--doppler-surface-alpha', surface.toString());
    document.documentElement.style.setProperty('--doppler-surface-strong-alpha', surfaceStrong.toString());
    document.documentElement.style.setProperty('--doppler-border-alpha', border.toString());
    document.documentElement.style.setProperty('--doppler-control-alpha', control.toString());
    document.documentElement.style.setProperty('--doppler-control-strong-alpha', controlStrong.toString());
  }

  async function restoreOverlaySettings() {
    await settingsStore.loadSettings();
    const settings = $settingsStore;

    applyTheme(settings.theme);
    applyOpacityVariables(settings.opacity);

    // Apply always-on-top
    try {
      await setWindowAlwaysOnTop(settings.always_on_top);
    } catch (error) {
      console.warn('Failed to restore always-on-top setting:', error);
    }

    // Apply click-through
    try {
      await setWindowClickThrough(settings.click_through);
      isClickThroughEnabled = settings.click_through;
    } catch (error) {
      console.warn('Failed to restore click-through setting:', error);
    }

    // Apply capture visibility
    try {
      await setScreenCaptureProtection(settings.screen_capture_protection);
    } catch (error) {
      console.warn('Failed to restore capture visibility setting:', error);
    }
  }

  onMount(() => {
    void restoreOverlaySettings();

    const unlistenPromise = listen<boolean>('click-through-changed', (event) => {
      isClickThroughEnabled = event.payload;
    });

    return () => {
      void unlistenPromise.then((unlisten) => unlisten());
    };
  });

  $effect(() => {
    applyTheme($settingsStore.theme);
  });
</script>

<div
  class="min-h-screen p-2"
  style="background: radial-gradient(circle at top left, rgb(var(--doppler-bg-start-rgb, 15 23 42) / var(--doppler-window-alpha, 0.95)) 0%, rgb(var(--doppler-bg-mid-rgb, 2 6 23) / var(--doppler-window-alpha, 0.95)) 48%, rgb(var(--doppler-bg-end-rgb, 3 10 27) / var(--doppler-window-alpha, 0.95)) 100%);"
>
  <div class="h-[calc(100vh-1rem)] overflow-hidden flex flex-col">
    <main class="relative flex-1 overflow-hidden">
      {#if isClickThroughEnabled}
        <div class="pointer-events-none absolute left-1/2 top-3 z-50 -translate-x-1/2 rounded-xl border px-3 py-1.5 text-xs font-semibold shadow-sm"
          style="border-color: rgba(245, 158, 11, 0.72); background: rgba(120, 53, 15, 0.72); color: rgba(254, 243, 199, 0.98);">
          Click-through ON. Turn off: cmd/^ + Shift + X
        </div>
      {/if}
      {#if isSettingsOpen}
        <SettingsView onClose={toggleSettings} />
      {:else}
        <ChatView onToggleSettings={toggleSettings} {isSettingsOpen} />
      {/if}
    </main>
  </div>
</div>
