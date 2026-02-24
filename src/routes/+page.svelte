<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import ChatView from '$lib/components/ChatView.svelte';
  import SettingsView from '$lib/components/SettingsView.svelte';
  import { settingsStore } from '$lib/stores/settingsStore';
  import { setWindowAlwaysOnTop, setWindowClickThrough } from '$lib/tauri/commands';

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
    document.documentElement.style.setProperty('--doppler-window-alpha', clamped.toString());
    document.documentElement.style.setProperty('--doppler-surface-alpha', surface.toString());
    document.documentElement.style.setProperty('--doppler-surface-strong-alpha', surfaceStrong.toString());
    document.documentElement.style.setProperty('--doppler-border-alpha', border.toString());
  }

  async function restoreOverlaySettings() {
    await settingsStore.loadSettings();
    const settings = $settingsStore;

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
</script>

<div
  class="min-h-screen p-3"
  style="background: radial-gradient(circle at top left, rgba(238, 244, 255, var(--doppler-window-alpha, 0.95)) 0%, rgba(223, 232, 246, var(--doppler-window-alpha, 0.95)) 45%, rgba(213, 223, 239, var(--doppler-window-alpha, 0.95)) 100%);"
>
  <div class="h-[calc(100vh-1.5rem)] overflow-hidden flex flex-col">
    <main class="relative flex-1 overflow-hidden">
      {#if isClickThroughEnabled}
        <div class="pointer-events-none absolute left-1/2 top-3 z-50 -translate-x-1/2 rounded-xl border border-amber-300/80 bg-amber-50/95 px-3 py-1.5 text-xs font-semibold text-amber-900 shadow-sm">
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
