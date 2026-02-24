<script lang="ts">
  import ChatView from '$lib/components/ChatView.svelte';
  import SettingsView from '$lib/components/SettingsView.svelte';
  import Button from '$lib/components/ui/Button.svelte';

  let currentView = $state<'chat' | 'settings'>('chat');

  function toggleView() {
    currentView = currentView === 'chat' ? 'settings' : 'chat';
  }
</script>

<div class="flex flex-col h-screen bg-gray-50">
  <!-- Top Bar -->
  <div class="flex items-center justify-between px-4 py-3 bg-white border-b border-gray-200 shadow-sm">
    <h1 class="text-lg font-semibold text-gray-900">Doppler</h1>
    <Button
      variant={currentView === 'settings' ? 'primary' : 'secondary'}
      size="sm"
      onclick={toggleView}
    >
      {currentView === 'settings' ? '← Back to Chat' : 'Settings'}
    </Button>
  </div>

  <!-- Content Area -->
  <div class="flex-1 overflow-hidden">
    {#if currentView === 'chat'}
      <ChatView />
    {:else}
      <SettingsView />
    {/if}
  </div>

  <!-- Bottom Bar -->
  <div class="flex items-center justify-between px-4 py-2 bg-white border-t border-gray-200 text-xs text-gray-500">
    <span>v0.1.0</span>
    <span>MVP Overlay Chat</span>
  </div>
</div>
