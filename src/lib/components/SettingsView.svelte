<script lang="ts">
  import Button from '$lib/components/ui/Button.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Slider from '$lib/components/ui/Slider.svelte';
  import Checkbox from '$lib/components/ui/Checkbox.svelte';
  import Tabs from '$lib/components/ui/Tabs.svelte';

  interface Props {
    onClose?: () => void;
  }

  let { onClose }: Props = $props();

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

<div class="h-full flex flex-col gap-3 text-slate-900 p-3 md:p-4">
  <div class="flex-1 min-h-0 rounded-2xl border border-white/70 bg-white/50 backdrop-blur-xl p-3 md:p-4 flex flex-col gap-3">
    <!-- Tabs Navigation -->
    <div class="pb-1">
    <div>
      <h2 class="text-base font-semibold tracking-tight text-slate-900">Settings</h2>
      <p class="text-xs text-slate-600">Overlay and interaction preferences</p>
    </div>
    </div>
    <Tabs {tabs} bind:activeTab />

    <!-- Tab Content -->
    <div class="flex-1 overflow-y-auto space-y-3">
    {#if activeTab === 'general'}
      <!-- General Tab (Placeholder) -->
      <section class="space-y-3 rounded-2xl border border-white/70 bg-white/75 p-3">
        <h3 class="text-sm font-semibold text-slate-800">General Settings</h3>
        <p class="text-sm text-slate-600">
          General application settings will be available here.
        </p>
      </section>

    {:else if activeTab === 'gemini'}
      <!-- Gemini Tab -->
      <section class="space-y-3 rounded-2xl border border-white/70 bg-white/75 p-3">
        <h3 class="text-sm font-semibold text-slate-800">Gemini API</h3>
        <div class="space-y-2">
          <label for="api-key" class="block text-xs font-medium text-slate-600">
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
      <section class="space-y-3 rounded-2xl border border-white/70 bg-white/75 p-3">
        <h3 class="text-sm font-semibold text-slate-800">Overlay Settings</h3>
        
        <!-- Opacity Slider -->
        <div class="space-y-2">
          <label for="opacity" class="block text-xs font-medium text-slate-600">
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
      <section class="space-y-3 rounded-2xl border border-white/70 bg-white/75 p-3">
        <h3 class="text-sm font-semibold text-slate-800">Hotkeys</h3>
        
        <div class="space-y-3">
          <div class="space-y-1.5">
            <div class="text-xs font-medium text-slate-600">
              Toggle Window
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/70 rounded-lg bg-white/70">
                <div class="flex gap-1 flex-wrap">
                  {#each hotkeyToggle.split('+') as key}
                    <span class="px-1.5 py-0.5 bg-white text-slate-700 rounded-md text-xs font-medium border border-slate-200">
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
            <div class="text-xs font-medium text-slate-600">
              Start/Stop Recording
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/70 rounded-lg bg-white/70">
                <div class="flex gap-1 flex-wrap">
                  {#each hotkeyRecord.split('+') as key}
                    <span class="px-1.5 py-0.5 bg-white text-slate-700 rounded-md text-xs font-medium border border-slate-200">
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

  <div class="rounded-2xl border border-white/70 bg-white/70 backdrop-blur-xl p-2.5 md:p-3">
    <div class="flex items-center gap-2">
      <button
        type="button"
        class="h-11 w-11 shrink-0 rounded-xl border border-white/70 bg-white/55 text-slate-500/70 cursor-not-allowed"
        aria-label="Voice input"
        title="Voice input (Ctrl+R)"
        disabled
      >
        <svg viewBox="0 0 24 24" class="mx-auto h-5 w-5" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="9" y="3" width="6" height="12" rx="3" />
          <path d="M5 11a7 7 0 0014 0" stroke-linecap="round" />
          <path d="M12 18v3M8.5 21h7" stroke-linecap="round" />
        </svg>
      </button>

      <button
        type="button"
        class="h-11 w-11 shrink-0 rounded-xl border border-white bg-white text-slate-700 shadow-sm transition hover:bg-slate-50"
        onclick={onClose}
        aria-label="Close settings"
        title="Toggle settings (Ctrl+,)"
      >
        <svg viewBox="0 0 24 24" class="mx-auto h-5 w-5" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M6 6l12 12M18 6L6 18" stroke-linecap="round" />
        </svg>
      </button>
      <div class="h-11 flex-1 rounded-xl border border-slate-200/70 bg-slate-100/75 opacity-70"></div>
    </div>
  </div>
</div>
