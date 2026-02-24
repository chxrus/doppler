<script lang="ts">
  import ChatView from '$lib/components/ChatView.svelte';
  import SettingsView from '$lib/components/SettingsView.svelte';

  let currentView = $state<'chat' | 'settings'>('chat');

  function showChat() {
    currentView = 'chat';
  }

  function showSettings() {
    currentView = 'settings';
  }
</script>

<div class="flex flex-col h-screen">
  <!-- Top Bar -->
  <div class="flex items-center justify-between px-3 py-2 bg-white border-b border-gray-200">
    <h1 class="text-sm font-semibold text-gray-900">Doppler</h1>
    <div class="flex gap-1">
      <button
        type="button"
        onclick={showChat}
        class="px-2.5 py-1 rounded-md text-xs font-medium {currentView === 'chat' ? 'bg-blue-500 text-white' : 'bg-gray-100 text-gray-700 hover:bg-gray-200 border border-gray-200'} transition-colors"
      >
        Chat
      </button>
      <button
        type="button"
        onclick={showSettings}
        class="px-2.5 py-1 rounded-md text-xs font-medium {currentView === 'settings' ? 'bg-blue-500 text-white' : 'bg-gray-100 text-gray-700 hover:bg-gray-200 border border-gray-200'} transition-colors"
      >
        Settings
      </button>
    </div>
  </div>

  <!-- Content Area -->
  <div class="flex-1 overflow-hidden">
    {#if currentView === 'chat'}
      <ChatView />
    {:else}
      <SettingsView />
    {/if}
  </div>
</div>
