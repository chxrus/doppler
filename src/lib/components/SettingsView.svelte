<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import {
    getApiKey,
    listRecordingDevices,
    type RecordingDeviceInfo,
    saveApiKey as persistApiKey,
    setScreenCaptureProtection,
    setWindowAlwaysOnTop,
    setWindowClickThrough
  } from '$lib/tauri/commands';
  import { settingsStore } from '$lib/stores/settingsStore';
  import Button from '$lib/components/ui/Button.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Slider from '$lib/components/ui/Slider.svelte';
  import Checkbox from '$lib/components/ui/Checkbox.svelte';
  import Tabs from '$lib/components/ui/Tabs.svelte';

  interface Props {
    onClose?: () => void;
  }

  let { onClose }: Props = $props();

  let apiKey = $state('');
  let isSavingApiKey = $state(false);
  let apiKeyStatusMessage = $state<string | null>(null);
  let apiKeyStatusKind = $state<'success' | 'error' | null>(null);
  let hotkeyPrevious = $state('Alt+Left');
  let hotkeyNext = $state('Alt+Right');
  let hotkeySend = $state('Enter');
  let hotkeyClickThrough = $state('CommandOrControl+Shift+X');
  let hotkeyCaptureVisibility = $state('CommandOrControl+Shift+H');
  let recordingDevices = $state<RecordingDeviceInfo[]>([]);

  // Tab state
  let activeTab = $state('general');

  const tabs = [
    { id: 'general', label: 'General' },
    { id: 'gemini', label: 'Gemini' },
    { id: 'overlay', label: 'Overlay' },
    { id: 'hotkeys', label: 'Hotkeys' }
  ];

  const geminiModelOptions = ['gemini-2.5-flash', 'gemini-2.5-pro', 'gemini-2.0-flash'];
  const DEFAULT_INPUT_DEVICE = 'Default input';

  const filteredRecordingDevices = $derived.by(() => {
    if ($settingsStore.recording_source === 'system') {
      const systemDevices = recordingDevices.filter((device) => device.likely_system_audio);
      return [
        ...recordingDevices.filter((device) => device.name === DEFAULT_INPUT_DEVICE),
        ...systemDevices
      ];
    }

    return recordingDevices.filter(
      (device) => device.name === DEFAULT_INPUT_DEVICE || !device.likely_system_audio
    );
  });

  async function saveApiKey() {
    const trimmedApiKey = apiKey.trim();

    isSavingApiKey = true;
    apiKeyStatusMessage = null;
    apiKeyStatusKind = null;

    try {
      await persistApiKey(trimmedApiKey);
      apiKey = trimmedApiKey;
      apiKeyStatusMessage = trimmedApiKey === '' ? 'API key removed' : 'API key saved';
      apiKeyStatusKind = 'success';
    } catch (error) {
      console.error('Could not save API key:', error);
      apiKeyStatusMessage = 'Could not save API key. Try again.';
      apiKeyStatusKind = 'error';
    } finally {
      isSavingApiKey = false;
    }
  }

  async function initializeSettings() {
    try {
      apiKey = (await getApiKey()) ?? '';
      await settingsStore.loadSettings();
      recordingDevices = await listRecordingDevices();
      
      // Apply current opacity to UI
      applyUiOpacity($settingsStore.opacity);
    } catch (error) {
      console.warn('Could not load settings:', error);
    }
  }

  function handleRecordingSourceChange() {
    settingsStore.updateField('recording_source', $settingsStore.recording_source);
    const hasSelectedDevice = filteredRecordingDevices.some(
      (device) => device.name === $settingsStore.recording_input_device
    );
    if (!hasSelectedDevice) {
      settingsStore.updateField('recording_input_device', DEFAULT_INPUT_DEVICE);
    }
  }

  function formatHotkey(hotkey: string): string[] {
    return hotkey.split('+').map((token) => {
      if (token === 'CommandOrControl') return 'cmd/^';
      if (token === 'Command') return 'cmd';
      if (token === 'Control' || token === 'Ctrl') return '^';
      return token;
    });
  }

  async function applyCaptureVisibility(value: boolean) {
    try {
      await setScreenCaptureProtection(value);
      settingsStore.updateField('screen_capture_protection', value);
    } catch (error) {
      console.warn('Failed to set capture visibility:', error);
    }
  }

  async function toggleCaptureVisibility() {
    const newValue = !$settingsStore.screen_capture_protection;
    await applyCaptureVisibility(newValue);
  }

  async function applyAlwaysOnTop(value: boolean) {
    try {
      await setWindowAlwaysOnTop(value);
      settingsStore.updateField('always_on_top', value);
    } catch (error) {
      console.warn('Failed to set always on top:', error);
    }
  }

  async function applyClickThrough(value: boolean) {
    try {
      await setWindowClickThrough(value);
      settingsStore.updateField('click_through', value);
    } catch (error) {
      console.warn('Failed to set click-through:', error);
    }
  }

  function applyUiOpacity(value: number) {
    const clamped = Math.max(0.1, Math.min(1, value));
    const surface = Math.max(0.04, Math.min(0.62, clamped * 0.58));
    const surfaceStrong = Math.max(0.05, Math.min(0.8, clamped * 0.72));
    const border = Math.max(0.12, Math.min(0.75, clamped * 0.78));
    document.documentElement.style.setProperty('--doppler-window-alpha', clamped.toString());
    document.documentElement.style.setProperty('--doppler-surface-alpha', surface.toString());
    document.documentElement.style.setProperty('--doppler-surface-strong-alpha', surfaceStrong.toString());
    document.documentElement.style.setProperty('--doppler-border-alpha', border.toString());
    settingsStore.updateField('opacity', value, true);
  }

  onMount(() => {
    const handleHotkeys = (event: KeyboardEvent) => {
      const isPrimaryModifier = event.metaKey || event.ctrlKey;

      if (
        ((isPrimaryModifier && !event.shiftKey && !event.altKey && event.code === 'Comma') ||
          (!event.metaKey && !event.ctrlKey && !event.altKey && event.code === 'Escape'))
      ) {
        event.preventDefault();
        onClose?.();
        return;
      }

      if (
        isPrimaryModifier &&
        event.shiftKey &&
        !event.altKey &&
        event.code === 'KeyH'
      ) {
        event.preventDefault();
        void toggleCaptureVisibility();
      }
    };

    window.addEventListener('keydown', handleHotkeys, true);
    void initializeSettings();
    
    const unlistenPromise = listen<boolean>('click-through-changed', (event) => {
      settingsStore.updateField('click_through', event.payload);
    });

    return () => {
      window.removeEventListener('keydown', handleHotkeys, true);
      void unlistenPromise.then((unlisten) => unlisten());
    };
  });
</script>

<div class="h-full flex flex-col gap-3 text-slate-900 p-3 md:p-4">
  <div class="flex-1 min-h-0 rounded-2xl backdrop-blur-xl p-3 md:p-4 flex flex-col gap-3"
    style="border-color: rgba(255, 255, 255, var(--doppler-border-alpha, 0.7)); background: rgba(255, 255, 255, var(--doppler-surface-alpha, 0.5));">
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
      <section class="space-y-3 rounded-2xl border p-3"
        style="border-color: rgba(255, 255, 255, var(--doppler-border-alpha, 0.7)); background: rgba(255, 255, 255, var(--doppler-surface-strong-alpha, 0.75));">
        <h3 class="text-sm font-semibold text-slate-800">General Settings</h3>
        <div class="space-y-2">
          <label class="block text-xs font-medium text-slate-600" for="recording-source">
            Recording Source
          </label>
          <select
            id="recording-source"
            class="w-full rounded-xl border border-white/75 bg-white px-3 py-2 text-sm text-slate-900"
            bind:value={$settingsStore.recording_source}
            onchange={handleRecordingSourceChange}
          >
            <option value="microphone">Microphone</option>
            <option value="system">System audio (loopback)</option>
          </select>
        </div>

        <div class="space-y-2">
          <label class="block text-xs font-medium text-slate-600" for="recording-device">
            Recording Device
          </label>
          <select
            id="recording-device"
            class="w-full rounded-xl border border-white/75 bg-white px-3 py-2 text-sm text-slate-900"
            bind:value={$settingsStore.recording_input_device}
            onchange={() => settingsStore.updateField('recording_input_device', $settingsStore.recording_input_device)}
          >
            {#each filteredRecordingDevices as device}
              <option value={device.name}>
                {device.name}{device.likely_system_audio ? ' (loopback)' : ''}
              </option>
            {/each}
          </select>
          {#if filteredRecordingDevices.length === 0}
            <p class="text-xs text-slate-500">No input devices detected.</p>
          {/if}
        </div>

        <div class="space-y-2">
          <label class="block text-xs font-medium text-slate-600" for="tts-rate">
            Speech Speed: {$settingsStore.tts_rate.toFixed(2)}x
          </label>
          <Slider
            min={0.7}
            max={1.8}
            step={0.05}
            bind:value={$settingsStore.tts_rate}
            oninput={() => settingsStore.updateField('tts_rate', $settingsStore.tts_rate, true)}
          />
        </div>
      </section>

    {:else if activeTab === 'gemini'}
      <!-- Gemini Tab -->
      <section class="space-y-3 rounded-2xl border p-3"
        style="border-color: rgba(255, 255, 255, var(--doppler-border-alpha, 0.7)); background: rgba(255, 255, 255, var(--doppler-surface-strong-alpha, 0.75));">
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
          <Button
            variant="primary"
            size="sm"
            onclick={() => {
              void saveApiKey();
            }}
            disabled={isSavingApiKey}
          >
            {#if isSavingApiKey}Saving...{:else}Save API Key{/if}
          </Button>
          {#if apiKeyStatusMessage !== null}
            <p class="text-xs {apiKeyStatusKind === 'error' ? 'text-rose-700' : 'text-emerald-700'}">
              {apiKeyStatusMessage}
            </p>
          {/if}
        </div>
        <div class="space-y-2">
          <label class="block text-xs font-medium text-slate-600" for="gemini-model">
            Model
          </label>
          <select
            id="gemini-model"
            class="w-full rounded-xl border border-white/75 bg-white px-3 py-2 text-sm text-slate-900"
            bind:value={$settingsStore.gemini_model}
            onchange={() => settingsStore.updateField('gemini_model', $settingsStore.gemini_model)}
          >
            {#each geminiModelOptions as model}
              <option value={model}>{model}</option>
            {/each}
          </select>
        </div>
        <div class="space-y-2">
          <label class="block text-xs font-medium text-slate-600" for="gemini-temperature">
            Temperature: {$settingsStore.gemini_temperature.toFixed(2)}
          </label>
          <Slider
            min={0}
            max={1}
            step={0.05}
            bind:value={$settingsStore.gemini_temperature}
            oninput={() => settingsStore.updateField('gemini_temperature', $settingsStore.gemini_temperature, true)}
          />
        </div>
      </section>

    {:else if activeTab === 'overlay'}
      <!-- Overlay Tab -->
      <section class="space-y-3 rounded-2xl border p-3"
        style="border-color: rgba(255, 255, 255, var(--doppler-border-alpha, 0.7)); background: rgba(255, 255, 255, var(--doppler-surface-strong-alpha, 0.75));">
        <h3 class="text-sm font-semibold text-slate-800">Overlay Settings</h3>
        
        <!-- Opacity Slider -->
        <div class="space-y-2">
          <label for="opacity" class="block text-xs font-medium text-slate-600">
            Opacity: {Math.round($settingsStore.opacity * 100)}%
          </label>
          <Slider
            min={0.1}
            max={1}
            step={0.05}
            bind:value={$settingsStore.opacity}
            oninput={() => applyUiOpacity($settingsStore.opacity)}
          />
        </div>

        <!-- Checkboxes -->
        <div class="space-y-2">
          <Checkbox 
            bind:checked={$settingsStore.always_on_top} 
            label="Always on top" 
            onchange={() => applyAlwaysOnTop($settingsStore.always_on_top)} 
          />
          <Checkbox 
            bind:checked={$settingsStore.click_through} 
            label="Click-through" 
            onchange={() => applyClickThrough($settingsStore.click_through)} 
          />
          {#if $settingsStore.click_through}
            <p class="rounded-lg border border-amber-300/70 bg-amber-50/90 px-2.5 py-2 text-xs font-medium text-amber-900">
              Click-through is on. Turn off with <span class="font-semibold">cmd/^ + Shift + X</span>.
            </p>
          {/if}
        </div>
      </section>

    {:else if activeTab === 'hotkeys'}
      <!-- Hotkeys Tab -->
      <section class="space-y-3 rounded-2xl border p-3"
        style="border-color: rgba(255, 255, 255, var(--doppler-border-alpha, 0.7)); background: rgba(255, 255, 255, var(--doppler-surface-strong-alpha, 0.75));">
        <h3 class="text-sm font-semibold text-slate-800">Hotkeys</h3>
        
        <div class="space-y-2.5">
          <div class="space-y-1.5">
            <div class="text-xs font-medium text-slate-600">
              Toggle Settings
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/70 rounded-lg bg-white/70">
                <div class="flex gap-1 flex-wrap">
                  {#each formatHotkey($settingsStore.hotkey_toggle) as key}
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
                  {#each formatHotkey($settingsStore.hotkey_record) as key}
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
              Previous Exchange
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/70 rounded-lg bg-white/70">
                <div class="flex gap-1 flex-wrap">
                  {#each formatHotkey(hotkeyPrevious) as key}
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
              Next Exchange
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/70 rounded-lg bg-white/70">
                <div class="flex gap-1 flex-wrap">
                  {#each formatHotkey(hotkeyNext) as key}
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
              Send Question
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/70 rounded-lg bg-white/70">
                <div class="flex gap-1 flex-wrap">
                  {#each formatHotkey(hotkeySend) as key}
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
              Toggle Click-through
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/70 rounded-lg bg-white/70">
                <div class="flex gap-1 flex-wrap">
                  {#each formatHotkey(hotkeyClickThrough) as key}
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
              Toggle Capture Visibility
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/70 rounded-lg bg-white/70">
                <div class="flex gap-1 flex-wrap">
                  {#each formatHotkey(hotkeyCaptureVisibility) as key}
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

  <div class="rounded-2xl border backdrop-blur-xl p-2.5 md:p-3"
    style="border-color: rgba(255, 255, 255, var(--doppler-border-alpha, 0.7)); background: rgba(255, 255, 255, var(--doppler-surface-strong-alpha, 0.7));">
    <div class="flex items-center gap-2">
      <button
        type="button"
        class="h-11 w-11 shrink-0 rounded-xl border border-white/70 bg-white/55 text-slate-500/70 cursor-not-allowed"
        aria-label="Voice input"
        title="Voice input (Ctrl+R)"
        data-hotkey="Ctrl+R"
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
        data-hotkey="Ctrl+,"
      >
        <svg viewBox="0 0 24 24" class="mx-auto h-5 w-5" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M6 6l12 12M18 6L6 18" stroke-linecap="round" />
        </svg>
      </button>
      <button
        type="button"
        class="h-11 flex-1 rounded-xl border text-sm font-semibold transition {$settingsStore.screen_capture_protection
          ? 'border-emerald-300/80 bg-emerald-50/95 text-emerald-800 hover:bg-emerald-100/90'
          : 'border-amber-300/80 bg-amber-50/95 text-amber-800 hover:bg-amber-100/90'}"
        onclick={toggleCaptureVisibility}
        title={$settingsStore.screen_capture_protection
          ? 'Window is hidden from screen recording (click to make visible)'
          : 'Window is visible to screen recording (click to hide)'}
        data-hotkey="Ctrl+Shift+H"
      >
        {#if $settingsStore.screen_capture_protection}
          Hidden in capture
        {:else}
          Visible in capture
        {/if}
      </button>
    </div>
  </div>
</div>
