<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import ChatView from '$lib/components/ChatView.svelte';
  import SettingsView from '$lib/components/SettingsView.svelte';

  let isSettingsOpen = $state(false);
  let isClickThroughEnabled = $state(false);

  function toggleSettings() {
    isSettingsOpen = !isSettingsOpen;
  }

  onMount(() => {
    const unlistenPromise = listen<boolean>('click-through-changed', (event) => {
      isClickThroughEnabled = event.payload;
    });

    return () => {
      void unlistenPromise.then((unlisten) => unlisten());
    };
  });
</script>

<div
  class="min-h-screen p-3 bg-[radial-gradient(circle_at_top_left,_#eef4ff_0%,_#dfe8f6_45%,_#d5dfef_100%)]"
  style="opacity: var(--doppler-app-opacity, 1);"
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
