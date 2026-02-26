<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { buildHotkeyFromEvent, formatHotkeyLabel, isHotkeyPressed } from '$lib/utils/hotkeys';
  import { applyTheme, type AppTheme } from '$lib/utils/theme';
  import {
    getApiKey,
    isWhisperSupported,
    listLmStudioModels,
    listOllamaModels,
    listRecordingDevices,
    listWhisperDevices,
    type RecordingDeviceInfo,
    type WhisperDeviceInfo,
    saveApiKey as persistApiKey,
    setScreenCaptureProtection,
    setWindowAlwaysOnTop,
    setWindowClickThrough
  } from '$lib/tauri/commands';
  import { DEFAULT_SETTINGS, settingsStore } from '$lib/stores/settingsStore';
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
  let editingHotkey = $state<
    'toggle' | 'record' | 'previous' | 'next' | 'send' | 'clickThrough' | 'captureVisibility' | null
  >(null);
  let recordingDevices = $state<RecordingDeviceInfo[]>([]);
  let isRecordingDevicesLoading = $state(true);
  let ollamaModels = $state<string[]>([]);
  let isDetectingOllamaModels = $state(false);
  let ollamaModelsErrorMessage = $state<string | null>(null);
  let lmstudioModels = $state<string[]>([]);
  let isDetectingLmStudioModels = $state(false);
  let lmstudioModelsErrorMessage = $state<string | null>(null);
  let whisperSupported = $state(false);
  let whisperDevices = $state<WhisperDeviceInfo[]>([]);

  // Tab state
  let activeTab = $state('general');

  const tabs = [
    { id: 'general', label: 'General' },
    { id: 'ai', label: 'AI' },
    { id: 'overlay', label: 'Overlay' },
    { id: 'hotkeys', label: 'Hotkeys' }
  ];

  const textProviderOptions = [
    { id: 'gemini', label: 'Gemini' },
    { id: 'ollama', label: 'Ollama (local)' },
    { id: 'lmstudio', label: 'LM Studio (local)' }
  ];
  const themeOptions: Array<{ id: AppTheme; label: string }> = [
    { id: 'dark', label: 'Dark' },
    { id: 'light', label: 'Light' }
  ];
  const sttProviderOptions = $derived.by(() => [
    { id: 'gemini', label: 'Gemini', disabled: false },
    {
      id: 'whisper',
      label: whisperSupported ? 'Whisper (local)' : 'Whisper (local, unavailable in this build)',
      disabled: !whisperSupported
    }
  ]);
  const OLLAMA_SETUP_DOC_URL =
    'https://github.com/chxrus/doppler/blob/main/docs/ollama-setup.md';
  const LMSTUDIO_SETUP_DOC_URL =
    'https://github.com/chxrus/doppler/blob/main/docs/lmstudio-setup.md';
  const WHISPER_SETUP_DOC_URL =
    'https://github.com/chxrus/doppler/blob/main/docs/whisper-setup.md';
  const WHISPER_MODELS_URL =
    'https://huggingface.co/ggerganov/whisper.cpp/tree/main';
  const geminiModelOptions = ['gemini-2.5-flash', 'gemini-2.5-pro', 'gemini-2.0-flash'];
  const DEFAULT_INPUT_DEVICE = 'Default input';
  const isGeminiTextProvider = $derived($settingsStore.text_provider === 'gemini');
  const isGeminiSttProvider = $derived($settingsStore.stt_provider === 'gemini');
  const isWhisperSttProvider = $derived(whisperSupported && $settingsStore.stt_provider === 'whisper');
  const whisperDeviceOptions = $derived.by(() => [
    { id: 'auto', label: 'Auto (best available)' },
    ...whisperDevices
  ]);

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
  const hotkeyTargets = {
    toggle: 'hotkey_toggle',
    record: 'hotkey_record',
    previous: 'hotkey_previous',
    next: 'hotkey_next',
    send: 'hotkey_send',
    clickThrough: 'hotkey_click_through',
    captureVisibility: 'hotkey_capture_visibility'
  } as const;
  const defaultHotkeys: Record<keyof typeof hotkeyTargets, string> = {
    toggle: DEFAULT_SETTINGS.hotkey_toggle,
    record: DEFAULT_SETTINGS.hotkey_record,
    previous: DEFAULT_SETTINGS.hotkey_previous,
    next: DEFAULT_SETTINGS.hotkey_next,
    send: DEFAULT_SETTINGS.hotkey_send,
    clickThrough: DEFAULT_SETTINGS.hotkey_click_through,
    captureVisibility: DEFAULT_SETTINGS.hotkey_capture_visibility
  };

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
    isRecordingDevicesLoading = true;
    try {
      apiKey = (await getApiKey()) ?? '';
      whisperSupported = await isWhisperSupported();
      if (!whisperSupported && $settingsStore.stt_provider === 'whisper') {
        settingsStore.updateField('stt_provider', 'gemini');
      }
      if (whisperSupported) {
        whisperDevices = await listWhisperDevices();
        const savedDeviceId = $settingsStore.whisper_device;
        const hasSavedDevice = savedDeviceId === 'auto' || whisperDevices.some((device) => device.id === savedDeviceId);
        if (!hasSavedDevice) {
          settingsStore.updateField('whisper_device', 'auto');
        }
      }
      recordingDevices = await listRecordingDevices();
      if ($settingsStore.text_provider === 'ollama') {
        void detectOllamaModels();
      }
      if ($settingsStore.text_provider === 'lmstudio') {
        void detectLmStudioModels();
      }
      
      // Apply current opacity to UI
      applyUiOpacity($settingsStore.opacity);
    } catch (error) {
      console.warn('Could not load settings:', error);
    } finally {
      isRecordingDevicesLoading = false;
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

  function handleThemeChange() {
    const theme = applyTheme($settingsStore.theme);
    settingsStore.updateField('theme', theme);
  }

  async function detectOllamaModels() {
    const baseUrl = $settingsStore.ollama_base_url.trim();
    isDetectingOllamaModels = true;
    ollamaModelsErrorMessage = null;

    try {
      const models = await listOllamaModels(baseUrl);
      ollamaModels = models;

      if (models.length === 1) {
        settingsStore.updateField('ollama_model', models[0]);
      }
    } catch (error) {
      console.error('Could not detect Ollama models:', error);
      ollamaModels = [];
      ollamaModelsErrorMessage =
        error instanceof Error ? error.message : 'Could not load Ollama models.';
    } finally {
      isDetectingOllamaModels = false;
    }
  }

  async function detectLmStudioModels() {
    const baseUrl = $settingsStore.lmstudio_base_url.trim();
    isDetectingLmStudioModels = true;
    lmstudioModelsErrorMessage = null;

    try {
      const models = await listLmStudioModels(baseUrl);
      lmstudioModels = models;

      if (
        models.length > 0 &&
        ($settingsStore.lmstudio_model.trim() === '' ||
          !models.includes($settingsStore.lmstudio_model))
      ) {
        settingsStore.updateField('lmstudio_model', models[0]);
      }
    } catch (error) {
      console.error('Could not detect LM Studio models:', error);
      lmstudioModels = [];
      lmstudioModelsErrorMessage =
        error instanceof Error ? error.message : 'Could not load LM Studio models.';
    } finally {
      isDetectingLmStudioModels = false;
    }
  }

  async function openOllamaSetupDocs() {
    try {
      await openUrl(OLLAMA_SETUP_DOC_URL);
    } catch (error) {
      console.warn('Could not open Ollama setup docs:', error);
    }
  }

  async function openWhisperSetupDocs() {
    try {
      await openUrl(WHISPER_SETUP_DOC_URL);
    } catch (error) {
      console.warn('Could not open Whisper setup docs:', error);
    }
  }

  async function openLmStudioSetupDocs() {
    try {
      await openUrl(LMSTUDIO_SETUP_DOC_URL);
    } catch (error) {
      console.warn('Could not open LM Studio setup docs:', error);
    }
  }

  function handleTextProviderChange() {
    settingsStore.updateField('text_provider', $settingsStore.text_provider);

    if ($settingsStore.text_provider === 'ollama' && ollamaModels.length === 0) {
      void detectOllamaModels();
    }
    if ($settingsStore.text_provider === 'lmstudio' && lmstudioModels.length === 0) {
      void detectLmStudioModels();
    }
  }

  function handleSttProviderChange() {
    settingsStore.updateField('stt_provider', $settingsStore.stt_provider);
  }

  function updateWhisperModelPath(value: string) {
    settingsStore.updateField('whisper_model_path', value);
  }

  function updateWhisperLanguage(value: string) {
    settingsStore.updateField('whisper_language', value);
  }

  function updateWhisperThreads(value: string) {
    const trimmedValue = value.trim();
    if (trimmedValue === '') {
      settingsStore.updateField('whisper_threads', null);
      return;
    }

    const parsedValue = Number.parseInt(trimmedValue, 10);
    if (Number.isNaN(parsedValue) || parsedValue <= 0) {
      return;
    }

    settingsStore.updateField('whisper_threads', parsedValue);
  }

  function updateWhisperDevice(value: 'auto' | 'cpu' | `gpu:${number}`) {
    settingsStore.updateField('whisper_device', value);
  }

  function formatHotkey(hotkey: string): string[] {
    return hotkey.split('+').map((token) => {
      if (token === 'CommandOrControl') return 'cmd/^';
      if (token === 'Command') return 'cmd';
      if (token === 'Control' || token === 'Ctrl') return '^';
      if (token === ',') return ',';
      return token;
    });
  }

  function startHotkeyCapture(target: keyof typeof hotkeyTargets): void {
    editingHotkey = target;
  }

  function applyCapturedHotkey(target: keyof typeof hotkeyTargets, hotkey: string): void {
    const field = hotkeyTargets[target];
    settingsStore.updateField(field, hotkey);
  }

  function resetHotkey(target: keyof typeof hotkeyTargets): void {
    settingsStore.updateField(hotkeyTargets[target], defaultHotkeys[target]);
    if (editingHotkey === target) {
      editingHotkey = null;
    }
  }

  function resetAllHotkeys(): void {
    (Object.keys(hotkeyTargets) as Array<keyof typeof hotkeyTargets>).forEach((target) => {
      settingsStore.updateField(hotkeyTargets[target], defaultHotkeys[target]);
    });
    editingHotkey = null;
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
    const control = Math.max(0.16, Math.min(0.82, clamped * 0.66));
    const controlStrong = Math.max(0.24, Math.min(0.9, clamped * 0.8));
    document.documentElement.style.setProperty('--doppler-window-alpha', clamped.toString());
    document.documentElement.style.setProperty('--doppler-surface-alpha', surface.toString());
    document.documentElement.style.setProperty('--doppler-surface-strong-alpha', surfaceStrong.toString());
    document.documentElement.style.setProperty('--doppler-border-alpha', border.toString());
    document.documentElement.style.setProperty('--doppler-control-alpha', control.toString());
    document.documentElement.style.setProperty('--doppler-control-strong-alpha', controlStrong.toString());
    settingsStore.updateField('opacity', value, true);
  }

  onMount(() => {
    const handleHotkeys = (event: KeyboardEvent) => {
      if (editingHotkey != null) {
        event.preventDefault();
        event.stopPropagation();
        if (!event.repeat) {
          if (event.code === 'Escape') {
            editingHotkey = null;
            return;
          }
          const captured = buildHotkeyFromEvent(event);
          if (captured != null) {
            applyCapturedHotkey(editingHotkey, captured);
            editingHotkey = null;
          }
        }
        return;
      }

      if (
        isHotkeyPressed(event, $settingsStore.hotkey_toggle) ||
        (!event.metaKey && !event.ctrlKey && !event.altKey && event.code === 'Escape')
      ) {
        event.preventDefault();
        onClose?.();
        return;
      }

      if (isHotkeyPressed(event, $settingsStore.hotkey_click_through)) {
        event.preventDefault();
        void applyClickThrough(!$settingsStore.click_through);
        return;
      }

      if (isHotkeyPressed(event, $settingsStore.hotkey_capture_visibility)) {
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

<div class="h-full flex flex-col gap-2 text-slate-100">
  <div class="settings-select-fix flex-1 min-h-0 rounded-2xl backdrop-blur-xl p-2 flex flex-col gap-2 select-none"
    style="border-color: rgba(148, 163, 184, var(--doppler-border-alpha, 0.65)); background: rgb(var(--doppler-surface-rgb, 15 23 42) / var(--doppler-surface-alpha, 0.55));">
    <!-- Tabs Navigation -->
    <div class="pb-1">
    <div>
      <h2 class="text-base font-semibold tracking-tight text-slate-100">Settings</h2>
      <p class="text-xs text-slate-300">Overlay and interaction preferences</p>
    </div>
    </div>
    <Tabs {tabs} bind:activeTab />

    <!-- Tab Content -->
    <div class="flex-1 overflow-y-auto space-y-3">
    {#if activeTab === 'general'}
      <!-- General Tab (Placeholder) -->
      <section class="space-y-3 p-1">
        <h3 class="text-sm font-semibold text-slate-100">General Settings</h3>
        <div class="space-y-2">
          <label class="block text-xs font-medium text-slate-300" for="theme">
            Theme
          </label>
          <select
            id="theme"
            class="w-full rounded-xl border border-white/15 bg-slate-900/65 px-3 py-2 text-sm text-slate-100"
            bind:value={$settingsStore.theme}
            onchange={handleThemeChange}
          >
            {#each themeOptions as themeOption}
              <option value={themeOption.id}>{themeOption.label}</option>
            {/each}
          </select>
        </div>
        <div class="space-y-2">
          <label class="block text-xs font-medium text-slate-300" for="recording-source">
            Recording Source
          </label>
          <select
            id="recording-source"
            class="w-full rounded-xl border border-white/15 bg-slate-900/65 px-3 py-2 text-sm text-slate-100"
            bind:value={$settingsStore.recording_source}
            onchange={handleRecordingSourceChange}
          >
            <option value="microphone">Microphone</option>
            <option value="system">System audio (loopback)</option>
          </select>
        </div>

        <div class="space-y-2">
          <label class="block text-xs font-medium text-slate-300" for="recording-device">
            Recording Device
          </label>
          <select
            id="recording-device"
            class="w-full rounded-xl border border-white/15 bg-slate-900/65 px-3 py-2 text-sm text-slate-100"
            bind:value={$settingsStore.recording_input_device}
            onchange={() => settingsStore.updateField('recording_input_device', $settingsStore.recording_input_device)}
          >
            {#each filteredRecordingDevices as device}
              <option value={device.name}>
                {device.name}{device.likely_system_audio ? ' (loopback)' : ''}
              </option>
            {/each}
          </select>
          {#if !isRecordingDevicesLoading && filteredRecordingDevices.length === 0}
            <p class="text-xs text-slate-400">No input devices detected.</p>
          {/if}
        </div>

        <div class="space-y-2">
          <label class="block text-xs font-medium text-slate-300" for="tts-rate">
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

        <div class="space-y-2">
          <Checkbox
            bind:checked={$settingsStore.auto_send_transcription}
            label="Auto-send transcribed voice input"
            onchange={() =>
              settingsStore.updateField(
                'auto_send_transcription',
                $settingsStore.auto_send_transcription
              )}
          />
          <p class="text-xs text-slate-400">
            When enabled, transcribed voice text is sent immediately after it appears in the input.
          </p>
        </div>
      </section>

    {:else if activeTab === 'ai'}
      <!-- AI Tab -->
      <section class="space-y-3 p-1">
        <h3 class="text-sm font-semibold text-slate-100">AI Providers</h3>
        <div class="space-y-2">
          <label class="block text-xs font-medium text-slate-300" for="text-provider">
            Text API Provider
          </label>
          <select
            id="text-provider"
            class="w-full rounded-xl border border-white/15 bg-slate-900/65 px-3 py-2 text-sm text-slate-100"
            bind:value={$settingsStore.text_provider}
            onchange={handleTextProviderChange}
          >
            {#each textProviderOptions as provider}
              <option value={provider.id}>{provider.label}</option>
            {/each}
          </select>
        </div>
        <div class="space-y-2">
          <label class="block text-xs font-medium text-slate-300" for="stt-provider">
            Speech-to-Text Provider
          </label>
          <select
            id="stt-provider"
            class="w-full rounded-xl border border-white/15 bg-slate-900/65 px-3 py-2 text-sm text-slate-100"
            bind:value={$settingsStore.stt_provider}
            onchange={handleSttProviderChange}
          >
            {#each sttProviderOptions as provider}
              <option value={provider.id} disabled={provider.disabled}>{provider.label}</option>
            {/each}
          </select>
          {#if !whisperSupported}
            <p class="text-xs text-slate-300">
              Whisper is disabled in this app build. Rebuild backend with Cargo feature
              <span class="font-semibold">local-whisper</span>.
            </p>
          {/if}
        </div>

        {#if isGeminiTextProvider || isGeminiSttProvider}
          <div class="space-y-2 border-t border-slate-600/45 pt-2">
            <div class="text-xs font-semibold uppercase tracking-wide text-slate-200">Gemini</div>
            <label for="api-key" class="block text-xs font-medium text-slate-300">
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
        {/if}

        {#if isGeminiTextProvider}
          <div class="space-y-2 border-t border-slate-600/45 pt-2">
            <div class="text-xs font-semibold uppercase tracking-wide text-slate-200">Gemini Text</div>
            <label class="block text-xs font-medium text-slate-300" for="gemini-model">
              Model
            </label>
            <select
              id="gemini-model"
              class="w-full rounded-xl border border-white/15 bg-slate-900/65 px-3 py-2 text-sm text-slate-100"
              bind:value={$settingsStore.gemini_model}
              onchange={() => settingsStore.updateField('gemini_model', $settingsStore.gemini_model)}
            >
              {#each geminiModelOptions as model}
                <option value={model}>{model}</option>
              {/each}
            </select>
            <label class="block text-xs font-medium text-slate-300" for="gemini-temperature">
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
        {/if}

        {#if isWhisperSttProvider}
          <div class="space-y-2 border-t border-slate-600/45 pt-2">
            <div class="flex items-center justify-between gap-2">
              <div class="text-xs font-semibold uppercase tracking-wide text-slate-200">Whisper</div>
              <button
                type="button"
                class="h-6 w-6 shrink-0 rounded-md border border-white/15 bg-slate-900/55 text-slate-200 text-xs font-semibold transition hover:bg-slate-900/80"
                onclick={() => {
                  void openWhisperSetupDocs();
                }}
                aria-label="Open Whisper setup guide"
                title="Open Whisper setup guide"
              >
                ?
              </button>
            </div>
            <p class="text-xs text-slate-300">
              Download Whisper models:
              <a
                href={WHISPER_MODELS_URL}
                class="underline decoration-slate-400 underline-offset-2 hover:text-slate-100"
                target="_blank"
                rel="noreferrer"
              >
                here
              </a>
            </p>
            <label for="whisper-model-path" class="block text-xs font-medium text-slate-300">
              Model path
            </label>
            <Input
              type="text"
              bind:value={$settingsStore.whisper_model_path}
              placeholder="/path/to/ggml-model.bin"
              oninput={() => updateWhisperModelPath($settingsStore.whisper_model_path)}
            />
            {#if $settingsStore.whisper_model_path.trim() === ''}
              <p class="text-xs text-amber-700">
                Set model path for Whisper. Save is allowed; transcription will fail until path is set.
              </p>
            {/if}
            <label for="whisper-language" class="block text-xs font-medium text-slate-300">
              Language (optional)
            </label>
            <Input
              type="text"
              bind:value={$settingsStore.whisper_language}
              placeholder="auto (empty), en, ru..."
              oninput={() => updateWhisperLanguage($settingsStore.whisper_language)}
            />
            <label for="whisper-device" class="block text-xs font-medium text-slate-300">
              Device
            </label>
            <select
              id="whisper-device"
              class="w-full rounded-xl border border-white/15 bg-slate-900/65 px-3 py-2 text-sm text-slate-100"
              bind:value={$settingsStore.whisper_device}
              onchange={() => updateWhisperDevice($settingsStore.whisper_device)}
            >
              {#each whisperDeviceOptions as option}
                <option value={option.id}>{option.label}</option>
              {/each}
            </select>
            <label for="whisper-threads" class="block text-xs font-medium text-slate-300">
              Threads (optional)
            </label>
            <Input
              type="number"
              value={$settingsStore.whisper_threads?.toString() ?? ''}
              placeholder="auto"
              oninput={(event) => updateWhisperThreads((event.currentTarget as HTMLInputElement).value)}
            />
          </div>
        {/if}

        {#if $settingsStore.text_provider === 'ollama'}
          <div class="space-y-2 border-t border-slate-600/45 pt-2">
            <div class="flex items-center justify-between gap-2">
              <div class="text-xs font-semibold uppercase tracking-wide text-slate-200">Ollama</div>
              <button
                type="button"
                class="h-6 w-6 shrink-0 rounded-md border border-white/15 bg-slate-900/55 text-slate-200 text-xs font-semibold transition hover:bg-slate-900/80"
                onclick={() => {
                  void openOllamaSetupDocs();
                }}
                aria-label="Open Ollama setup guide"
                title="Open Ollama setup guide"
              >
                ?
              </button>
            </div>
            <p class="text-xs text-slate-300">
              Use a running Ollama server (usually local) and select an installed model tag.
            </p>
            <label class="block text-xs font-medium text-slate-300" for="ollama-base-url">
              Base URL
            </label>
                <Input
                  type="text"
                  bind:value={$settingsStore.ollama_base_url}
                  placeholder="http://localhost:11434"
                  oninput={() => settingsStore.updateField('ollama_base_url', $settingsStore.ollama_base_url)}
                />
            <label class="block text-xs font-medium text-slate-300" for="ollama-model">
              Model
            </label>
            <div class="flex items-center gap-2">
              <div class="flex-1">
                <Input
                  type="text"
                  bind:value={$settingsStore.ollama_model}
                  placeholder="llama3.2:3b"
                  list="ollama-model-suggestions"
                  oninput={() => settingsStore.updateField('ollama_model', $settingsStore.ollama_model)}
                />
              </div>
              <button
                type="button"
                class="h-10 w-10 shrink-0 rounded-xl border border-white/15 bg-slate-900/55 text-slate-200 transition hover:bg-slate-900/80 disabled:opacity-50 disabled:cursor-not-allowed"
                onclick={() => {
                  void detectOllamaModels();
                }}
                disabled={isDetectingOllamaModels}
                aria-label="Refresh Ollama models"
                title="Refresh Ollama models"
              >
                {#if isDetectingOllamaModels}
                  <svg class="mx-auto h-4 w-4 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M21 12a9 9 0 11-2.64-6.36" stroke-linecap="round" />
                    <path d="M21 3v6h-6" stroke-linecap="round" stroke-linejoin="round" />
                  </svg>
                {:else}
                  <svg class="mx-auto h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M21 12a9 9 0 11-2.64-6.36" stroke-linecap="round" />
                    <path d="M21 3v6h-6" stroke-linecap="round" stroke-linejoin="round" />
                  </svg>
                {/if}
              </button>
            </div>
            <datalist id="ollama-model-suggestions">
              {#each ollamaModels as model}
                <option value={model}></option>
              {/each}
            </datalist>
            {#if ollamaModels.length > 0}
              <p class="text-xs text-slate-300">Found models: {ollamaModels.length}</p>
            {/if}
            {#if ollamaModelsErrorMessage !== null}
              <p class="text-xs text-rose-700">{ollamaModelsErrorMessage}</p>
            {/if}
            <label class="block text-xs font-medium text-slate-300" for="ollama-temperature">
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
        {/if}

        {#if $settingsStore.text_provider === 'lmstudio'}
          <div class="space-y-2 border-t border-slate-600/45 pt-2">
            <div class="flex items-center justify-between gap-2">
              <div class="text-xs font-semibold uppercase tracking-wide text-slate-200">LM Studio</div>
              <button
                type="button"
                class="h-6 w-6 shrink-0 rounded-md border border-white/15 bg-slate-900/55 text-slate-200 text-xs font-semibold transition hover:bg-slate-900/80"
                onclick={() => {
                  void openLmStudioSetupDocs();
                }}
                aria-label="Open LM Studio setup guide"
                title="Open LM Studio setup guide"
              >
                ?
              </button>
            </div>
            <p class="text-xs text-slate-300">
              Use the LM Studio Local Server endpoint and choose a loaded model.
            </p>
            <label class="block text-xs font-medium text-slate-300" for="lmstudio-base-url">
              Base URL
            </label>
            <Input
              type="text"
              bind:value={$settingsStore.lmstudio_base_url}
              placeholder="http://localhost:1234/v1"
              oninput={() => settingsStore.updateField('lmstudio_base_url', $settingsStore.lmstudio_base_url)}
            />
            <label class="block text-xs font-medium text-slate-300" for="lmstudio-model">
              Model
            </label>
            <div class="flex items-center gap-2">
              <div class="flex-1">
                <select
                  id="lmstudio-model"
                  class="w-full rounded-xl border border-white/15 bg-slate-900/65 px-3 py-2 text-sm text-slate-100"
                  bind:value={$settingsStore.lmstudio_model}
                  onchange={() => settingsStore.updateField('lmstudio_model', $settingsStore.lmstudio_model)}
                >
                  {#if lmstudioModels.length === 0}
                    <option value="" disabled>Refresh models first</option>
                  {/if}
                  {#each lmstudioModels as model}
                    <option value={model}>{model}</option>
                  {/each}
                </select>
              </div>
              <button
                type="button"
                class="h-10 w-10 shrink-0 rounded-xl border border-white/15 bg-slate-900/55 text-slate-200 transition hover:bg-slate-900/80 disabled:opacity-50 disabled:cursor-not-allowed"
                onclick={() => {
                  void detectLmStudioModels();
                }}
                disabled={isDetectingLmStudioModels}
                aria-label="Refresh LM Studio models"
                title="Refresh LM Studio models"
              >
                {#if isDetectingLmStudioModels}
                  <svg class="mx-auto h-4 w-4 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M21 12a9 9 0 11-2.64-6.36" stroke-linecap="round" />
                    <path d="M21 3v6h-6" stroke-linecap="round" stroke-linejoin="round" />
                  </svg>
                {:else}
                  <svg class="mx-auto h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M21 12a9 9 0 11-2.64-6.36" stroke-linecap="round" />
                    <path d="M21 3v6h-6" stroke-linecap="round" stroke-linejoin="round" />
                  </svg>
                {/if}
              </button>
            </div>
            {#if lmstudioModels.length > 0}
              <p class="text-xs text-slate-300">Found models: {lmstudioModels.length}</p>
            {/if}
            {#if lmstudioModelsErrorMessage !== null}
              <p class="text-xs text-rose-700">{lmstudioModelsErrorMessage}</p>
            {/if}
            <label class="block text-xs font-medium text-slate-300" for="lmstudio-temperature">
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
        {/if}

      </section>

    {:else if activeTab === 'overlay'}
      <!-- Overlay Tab -->
      <section class="space-y-3 p-1">
        <h3 class="text-sm font-semibold text-slate-100">Overlay Settings</h3>
        
        <!-- Opacity Slider -->
        <div class="space-y-2">
          <label for="opacity" class="block text-xs font-medium text-slate-300">
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
            <p class="rounded-lg border border-amber-300/65 bg-amber-500/18 px-2.5 py-2 text-xs font-medium text-amber-100">
              Click-through is on. Turn off with <span class="font-semibold">{formatHotkeyLabel($settingsStore.hotkey_click_through)}</span>.
            </p>
          {/if}
        </div>
      </section>

    {:else if activeTab === 'hotkeys'}
      <!-- Hotkeys Tab -->
      <section class="space-y-3 p-1">
        <div class="flex items-center justify-between gap-2">
          <h3 class="text-sm font-semibold text-slate-100">Hotkeys</h3>
          <Button variant="secondary" size="sm" onclick={resetAllHotkeys}>
            Reset all
          </Button>
        </div>
        
        <div class="space-y-2.5">
          <div class="space-y-1.5">
            <div class="text-xs font-medium text-slate-300">
              Toggle Settings
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/15 rounded-lg bg-slate-900/55">
                <div class="flex gap-1 flex-wrap">
                  {#each formatHotkey($settingsStore.hotkey_toggle) as key}
                    <span class="px-1.5 py-0.5 bg-slate-900/65 text-slate-200 rounded-md text-xs font-medium border border-slate-600/45">
                      {key}
                    </span>
                  {/each}
                </div>
              </div>
              <Button variant="secondary" size="sm" onclick={() => startHotkeyCapture('toggle')}>
                {#if editingHotkey === 'toggle'}Press keys...{:else}Change{/if}
              </Button>
              <button
                type="button"
                class="h-9 w-9 shrink-0 rounded-lg border border-white/15 bg-slate-900/55 text-slate-200 transition hover:bg-slate-900/80"
                onclick={() => resetHotkey('toggle')}
                aria-label="Reset toggle settings hotkey"
                title="Reset"
              >
                <svg viewBox="0 0 24 24" class="mx-auto h-4 w-4" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M6 6l12 12M18 6L6 18" stroke-linecap="round" />
                </svg>
              </button>
            </div>
          </div>

          <div class="space-y-1.5">
            <div class="text-xs font-medium text-slate-300">
              Start/Stop Recording
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/15 rounded-lg bg-slate-900/55">
                <div class="flex gap-1 flex-wrap">
                  {#each formatHotkey($settingsStore.hotkey_record) as key}
                    <span class="px-1.5 py-0.5 bg-slate-900/65 text-slate-200 rounded-md text-xs font-medium border border-slate-600/45">
                      {key}
                    </span>
                  {/each}
                </div>
              </div>
              <Button variant="secondary" size="sm" onclick={() => startHotkeyCapture('record')}>
                {#if editingHotkey === 'record'}Press keys...{:else}Change{/if}
              </Button>
              <button
                type="button"
                class="h-9 w-9 shrink-0 rounded-lg border border-white/15 bg-slate-900/55 text-slate-200 transition hover:bg-slate-900/80"
                onclick={() => resetHotkey('record')}
                aria-label="Reset start stop recording hotkey"
                title="Reset"
              >
                <svg viewBox="0 0 24 24" class="mx-auto h-4 w-4" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M6 6l12 12M18 6L6 18" stroke-linecap="round" />
                </svg>
              </button>
            </div>
          </div>

          <div class="space-y-1.5">
            <div class="text-xs font-medium text-slate-300">
              Previous Exchange
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/15 rounded-lg bg-slate-900/55">
                <div class="flex gap-1 flex-wrap">
                  {#each formatHotkey($settingsStore.hotkey_previous) as key}
                    <span class="px-1.5 py-0.5 bg-slate-900/65 text-slate-200 rounded-md text-xs font-medium border border-slate-600/45">
                      {key}
                    </span>
                  {/each}
                </div>
              </div>
              <Button variant="secondary" size="sm" onclick={() => startHotkeyCapture('previous')}>
                {#if editingHotkey === 'previous'}Press keys...{:else}Change{/if}
              </Button>
              <button
                type="button"
                class="h-9 w-9 shrink-0 rounded-lg border border-white/15 bg-slate-900/55 text-slate-200 transition hover:bg-slate-900/80"
                onclick={() => resetHotkey('previous')}
                aria-label="Reset previous exchange hotkey"
                title="Reset"
              >
                <svg viewBox="0 0 24 24" class="mx-auto h-4 w-4" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M6 6l12 12M18 6L6 18" stroke-linecap="round" />
                </svg>
              </button>
            </div>
          </div>

          <div class="space-y-1.5">
            <div class="text-xs font-medium text-slate-300">
              Next Exchange
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/15 rounded-lg bg-slate-900/55">
                <div class="flex gap-1 flex-wrap">
                  {#each formatHotkey($settingsStore.hotkey_next) as key}
                    <span class="px-1.5 py-0.5 bg-slate-900/65 text-slate-200 rounded-md text-xs font-medium border border-slate-600/45">
                      {key}
                    </span>
                  {/each}
                </div>
              </div>
              <Button variant="secondary" size="sm" onclick={() => startHotkeyCapture('next')}>
                {#if editingHotkey === 'next'}Press keys...{:else}Change{/if}
              </Button>
              <button
                type="button"
                class="h-9 w-9 shrink-0 rounded-lg border border-white/15 bg-slate-900/55 text-slate-200 transition hover:bg-slate-900/80"
                onclick={() => resetHotkey('next')}
                aria-label="Reset next exchange hotkey"
                title="Reset"
              >
                <svg viewBox="0 0 24 24" class="mx-auto h-4 w-4" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M6 6l12 12M18 6L6 18" stroke-linecap="round" />
                </svg>
              </button>
            </div>
          </div>

          <div class="space-y-1.5">
            <div class="text-xs font-medium text-slate-300">
              Send Question
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/15 rounded-lg bg-slate-900/55">
                <div class="flex gap-1 flex-wrap">
                  {#each formatHotkey($settingsStore.hotkey_send) as key}
                    <span class="px-1.5 py-0.5 bg-slate-900/65 text-slate-200 rounded-md text-xs font-medium border border-slate-600/45">
                      {key}
                    </span>
                  {/each}
                </div>
              </div>
              <Button variant="secondary" size="sm" onclick={() => startHotkeyCapture('send')}>
                {#if editingHotkey === 'send'}Press keys...{:else}Change{/if}
              </Button>
              <button
                type="button"
                class="h-9 w-9 shrink-0 rounded-lg border border-white/15 bg-slate-900/55 text-slate-200 transition hover:bg-slate-900/80"
                onclick={() => resetHotkey('send')}
                aria-label="Reset send question hotkey"
                title="Reset"
              >
                <svg viewBox="0 0 24 24" class="mx-auto h-4 w-4" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M6 6l12 12M18 6L6 18" stroke-linecap="round" />
                </svg>
              </button>
            </div>
          </div>

          <div class="space-y-1.5">
            <div class="text-xs font-medium text-slate-300">
              Toggle Click-through
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/15 rounded-lg bg-slate-900/55">
                <div class="flex gap-1 flex-wrap">
                  {#each formatHotkey($settingsStore.hotkey_click_through) as key}
                    <span class="px-1.5 py-0.5 bg-slate-900/65 text-slate-200 rounded-md text-xs font-medium border border-slate-600/45">
                      {key}
                    </span>
                  {/each}
                </div>
              </div>
              <Button variant="secondary" size="sm" onclick={() => startHotkeyCapture('clickThrough')}>
                {#if editingHotkey === 'clickThrough'}Press keys...{:else}Change{/if}
              </Button>
              <button
                type="button"
                class="h-9 w-9 shrink-0 rounded-lg border border-white/15 bg-slate-900/55 text-slate-200 transition hover:bg-slate-900/80"
                onclick={() => resetHotkey('clickThrough')}
                aria-label="Reset click-through hotkey"
                title="Reset"
              >
                <svg viewBox="0 0 24 24" class="mx-auto h-4 w-4" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M6 6l12 12M18 6L6 18" stroke-linecap="round" />
                </svg>
              </button>
            </div>
          </div>

          <div class="space-y-1.5">
            <div class="text-xs font-medium text-slate-300">
              Toggle Capture Visibility
            </div>
            <div class="flex items-center gap-2">
              <div class="flex-1 px-2.5 py-1.5 border border-white/15 rounded-lg bg-slate-900/55">
                <div class="flex gap-1 flex-wrap">
                  {#each formatHotkey($settingsStore.hotkey_capture_visibility) as key}
                    <span class="px-1.5 py-0.5 bg-slate-900/65 text-slate-200 rounded-md text-xs font-medium border border-slate-600/45">
                      {key}
                    </span>
                  {/each}
                </div>
              </div>
              <Button variant="secondary" size="sm" onclick={() => startHotkeyCapture('captureVisibility')}>
                {#if editingHotkey === 'captureVisibility'}Press keys...{:else}Change{/if}
              </Button>
              <button
                type="button"
                class="h-9 w-9 shrink-0 rounded-lg border border-white/15 bg-slate-900/55 text-slate-200 transition hover:bg-slate-900/80"
                onclick={() => resetHotkey('captureVisibility')}
                aria-label="Reset capture visibility hotkey"
                title="Reset"
              >
                <svg viewBox="0 0 24 24" class="mx-auto h-4 w-4" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M6 6l12 12M18 6L6 18" stroke-linecap="round" />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </section>
    {/if}
    </div>
  </div>

  <div class="rounded-2xl border backdrop-blur-xl p-2.5 md:p-3"
    style="border-color: rgba(148, 163, 184, var(--doppler-border-alpha, 0.65)); background: rgb(var(--doppler-surface-rgb, 15 23 42) / var(--doppler-surface-strong-alpha, 0.7));">
    <div class="flex items-center gap-2">
      <button
        type="button"
        class="h-11 w-11 shrink-0 rounded-xl border border-white/15 bg-slate-900/45 text-slate-400/70 cursor-not-allowed"
        aria-label="Voice input"
        title={`Voice input (${formatHotkeyLabel($settingsStore.hotkey_record)})`}
        data-hotkey={formatHotkeyLabel($settingsStore.hotkey_record)}
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
        class="h-11 w-11 shrink-0 rounded-xl border border-white/15 bg-slate-900/45 text-slate-100 shadow-sm transition hover:bg-slate-900/80"
        onclick={onClose}
        aria-label="Close settings"
        title={`Toggle settings (${formatHotkeyLabel($settingsStore.hotkey_toggle)})`}
        data-hotkey={formatHotkeyLabel($settingsStore.hotkey_toggle)}
      >
        <svg viewBox="0 0 24 24" class="mx-auto h-5 w-5" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M6 6l12 12M18 6L6 18" stroke-linecap="round" />
        </svg>
      </button>
      <button
        type="button"
        class="h-11 flex-1 rounded-xl border text-sm font-semibold transition text-slate-100"
        style={$settingsStore.screen_capture_protection
          ? 'border-color: rgb(var(--doppler-capture-hidden-rgb) / 0.82); background: rgb(var(--doppler-capture-hidden-rgb) / var(--doppler-capture-hidden-bg-alpha, 0.34)); color: rgb(var(--doppler-capture-hidden-text-rgb, 167 243 208));'
          : 'border-color: rgb(var(--doppler-capture-visible-rgb) / 0.86); background: rgb(var(--doppler-capture-visible-rgb) / var(--doppler-capture-visible-bg-alpha, 0.26)); color: rgb(var(--doppler-capture-visible-text-rgb, 254 205 211));'}
        onclick={toggleCaptureVisibility}
        title={$settingsStore.screen_capture_protection
          ? 'Window is hidden from screen recording (click to make visible)'
          : 'Window is visible to screen recording (click to hide)'}
        data-hotkey={formatHotkeyLabel($settingsStore.hotkey_capture_visibility)}
      >
        <span class="inline-flex items-center gap-1.5">
          {#if $settingsStore.screen_capture_protection}
            <svg viewBox="0 0 24 24" class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M2 2l20 20" stroke-linecap="round" />
              <path d="M10.6 10.6A3 3 0 0012 15a3 3 0 002.4-4.8" stroke-linecap="round" />
              <path d="M9.4 5.1A10.7 10.7 0 0121 12a10.7 10.7 0 01-4 5.6" stroke-linecap="round" />
              <path d="M6.1 6.1A10.8 10.8 0 003 12a10.7 10.7 0 004.8 6.5" stroke-linecap="round" />
            </svg>
            Hidden in capture
          {:else}
            <svg viewBox="0 0 24 24" class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7-10-7-10-7z" />
              <circle cx="12" cy="12" r="3" />
            </svg>
            Visible in capture
          {/if}
        </span>
      </button>
    </div>
  </div>
</div>

<style>
  .settings-select-fix :global(input),
  .settings-select-fix :global(textarea),
  .settings-select-fix :global(select) {
    user-select: text;
    -webkit-user-select: text;
    border-color: rgba(148, 163, 184, var(--doppler-border-alpha, 0.65));
    background: rgb(var(--doppler-control-rgb, 15 23 42) / var(--doppler-control-alpha, 0.62));
    color: #e2e8f0;
  }

  .settings-select-fix :global(option) {
    background: #0f172a;
    color: #e2e8f0;
  }
</style>
