<script lang="ts">
  import ChatView from '$lib/components/ChatView.svelte';
  import SettingsView from '$lib/components/SettingsView.svelte';

  let isSettingsOpen = $state(false);

  function toggleSettings() {
    isSettingsOpen = !isSettingsOpen;
  }
</script>

<div class="min-h-screen p-3 bg-[radial-gradient(circle_at_top_left,_#eef4ff_0%,_#dfe8f6_45%,_#d5dfef_100%)]">
  <div class="h-[calc(100vh-1.5rem)] overflow-hidden flex flex-col">
    <main class="relative flex-1 overflow-hidden">
      {#if isSettingsOpen}
        <SettingsView onClose={toggleSettings} />
      {:else}
        <ChatView onToggleSettings={toggleSettings} {isSettingsOpen} />
      {/if}
    </main>
  </div>
</div>
