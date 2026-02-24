<script lang="ts">
  import Button from './ui/Button.svelte';
  import Input from './ui/Input.svelte';
  import Slider from './ui/Slider.svelte';
  import Checkbox from './ui/Checkbox.svelte';
  import Tabs from './ui/Tabs.svelte';

  // Mock settings state (no backend calls)
  let apiKey = $state('');
  let opacity = $state(0.95);
  let alwaysOnTop = $state(true);
  let clickThrough = $state(false);
  let screenCaptureProtection = $state(true);
  let hotkeyToggle = $state('CommandOrControl+Shift+Space');
  let hotkeyRecord = $state('CommandOrControl+Shift+R');

  // Tab state
  let activeTab = $state('general');

  const tabs = [
    { id: 'general', label: 'General' },
    { id: 'gemini', label: 'Gemini' },
    { id: 'overlay', label: 'Overlay' },
    { id: 'hotkeys', label: 'Hotkeys' }
  ];

  function saveApiKey() {
    // Mock save - no backend call
    console.log('API Key saved (mock):', apiKey);
  }
</script>

<div class="flex flex-col h-full bg-white">
  <!-- Tabs Navigation -->
  <Tabs {tabs} bind:activeTab />

  <!-- Tab Content -->
  <div class="flex-1 overflow-y-auto p-4">
    {#if activeTab === 'general'}
      <!-- General Tab (Placeholder) -->
      <section class="space-y-3">
        <h2 class="text-base font-semibold text-gray-900">General Settings</h2>
        <p class="text-sm text-gray-600">
          General application settings will be available here.
        </p>
      </section>

    {:else if activeTab === 'gemini'}
      <!-- Gemini Tab -->
      <section class="space-y-3">
        <h2 class="text-base font-semibold text-gray-900">Gemini API</h2>
        <div class="space-y-2">
          <label for="api-key" class="block text-xs font-medium text-gray-600">
            API Key
          </label>
          <Input
            type="password"
            bind:value={apiKey}
            placeholder="Enter your Gemini API key"
          />
          <Button variant="primary" size="sm" onclick={saveApiKey}>
            Save API Key
          </Button>
        </div>
      </section>

    {:else if activeTab === 'overlay'}
      <!-- Overlay Tab -->
      <section class="space-y-3">
        <h2 class="text-base font-semibold text-gray-900">Overlay Settings</h2>
        
        <!-- Opacity Slider -->
        <div class="space-y-2">
          <label for="opacity" class="block text-xs font-medium text-gray-600">
            Opacity: {Math.round(opacity * 100)}%
          </label>
          <Slider
            min={0.2}
            max={1}
            step={0.05}
            bind:value={opacity}
          />
        </div>

        <!-- Checkboxes -->
        <div class="space-y-2">
          <Checkbox bind:checked={alwaysOnTop} label="Always on top" />
          <Checkbox bind:checked={clickThrough} label="Click-through" />
          <Checkbox bind:checked={screenCaptureProtection} label="Screen capture protection" />
        </div>
      </section>

    {:else if activeTab === 'hotkeys'}
      <!-- Hotkeys Tab -->
      <section class="space-y-3">
        <h2 class="text-base font-semibold text-gray-900">Hotkeys</h2>
        
        <div class="space-y-3">
          <div class="space-y-1.5">
            <div class="text-xs font-medium text-gray-600">
              Toggle Window
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-3 py-1.5 border border-gray-200 rounded-md bg-gray-50">
                <div class="flex gap-1 flex-wrap">
                  {#each hotkeyToggle.split('+') as key}
                    <span class="px-1.5 py-0.5 bg-white text-gray-700 rounded text-xs font-medium border border-gray-200">
                      {key}
                    </span>
                  {/each}
                </div>
              </div>
              <Button variant="secondary" size="sm">
                Change
              </Button>
            </div>
          </div>

          <div class="space-y-1.5">
            <div class="text-xs font-medium text-gray-600">
              Start/Stop Recording
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-3 py-1.5 border border-gray-200 rounded-md bg-gray-50">
                <div class="flex gap-1 flex-wrap">
                  {#each hotkeyRecord.split('+') as key}
                    <span class="px-1.5 py-0.5 bg-white text-gray-700 rounded text-xs font-medium border border-gray-200">
                      {key}
                    </span>
                  {/each}
                </div>
              </div>
              <Button variant="secondary" size="sm">
                Change
              </Button>
            </div>
          </div>
        </div>
      </section>
    {/if}
  </div>
</div>
