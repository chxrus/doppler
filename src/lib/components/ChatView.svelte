<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte';
  import Spinner from '$lib/components/ui/Spinner.svelte';
  import { chatStore } from '$lib/stores/chatStore';
  import { settingsStore } from '$lib/stores/settingsStore';
  import { formatHotkeyLabel, isHotkeyPressed } from '$lib/utils/hotkeys';
  import { renderMarkdown } from '$lib/utils/markdown';
  import {
    isSpeaking,
    setScreenCaptureProtection,
    setWindowClickThrough,
    speakText,
    startRecording,
    stopRecording,
    stopSpeaking,
    transcribeLastRecording
  } from '$lib/tauri/commands';
  import SpeakButton from '$lib/components/SpeakButton.svelte';

  interface Props {
    onToggleSettings?: () => void | Promise<void>;
    isSettingsOpen?: boolean;
  }

  let { onToggleSettings, isSettingsOpen = false }: Props = $props();

  let currentExchangeIndex = $state(0);
  let isExchangeIndexInitialized = $state(false);
  let input = $state('');
  let isRecording = $state(false);

  let exchanges = $state<any[]>([]);
  let isLoading = $state(false);
  let errorMessage = $state<string | null>(null);
  let recordingErrorMessage = $state<string | null>(null);
  let isRecordingBusy = $state(false);
  let isTranscribingRecording = $state(false);
  let speakingExchangeIndex = $state<number | null>(null);
  let ttsErrorMessage = $state<string | null>(null);
  let isClearingSession = $state(false);

  $effect(() => {
    const unsubExchanges = chatStore.exchanges.subscribe(value => exchanges = value);
    const unsubLoading = chatStore.isLoading.subscribe(value => isLoading = value);
    const unsubError = chatStore.error.subscribe(value => errorMessage = value);
    const unsubInputDraft = chatStore.inputDraft.subscribe(value => input = value);
    
    return () => {
      unsubExchanges();
      unsubLoading();
      unsubError();
      unsubInputDraft();
    };
  });

  const hasExchanges = $derived(exchanges.length > 0);
  const currentExchange = $derived(hasExchanges ? exchanges[currentExchangeIndex] : null);
  const canGoPrevious = $derived(currentExchangeIndex > 0);
  const canGoNext = $derived(currentExchangeIndex < exchanges.length - 1);

  $effect(() => {
    if (!isExchangeIndexInitialized && exchanges.length > 0) {
      currentExchangeIndex = exchanges.length - 1;
      isExchangeIndexInitialized = true;
    }
  });

  $effect(() => {
    if (exchanges.length === 0) {
      currentExchangeIndex = 0;
      return;
    }

    const maxIndex = exchanges.length - 1;
    if (currentExchangeIndex > maxIndex) {
      currentExchangeIndex = maxIndex;
    }
  });

  function goToPreviousExchange() {
    if (!canGoPrevious) return;
    currentExchangeIndex -= 1;
  }

  function goToNextExchange() {
    if (!canGoNext) return;
    currentExchangeIndex += 1;
  }

  function sendMessage() {
    const trimmedInput = input.trim();
    if (trimmedInput === '') return;

    const messageToSend = trimmedInput;
    const nextExchangeIndex = exchanges.length;
    input = '';
    chatStore.setInputDraft('');

    void chatStore.sendMessage(messageToSend);
    currentExchangeIndex = nextExchangeIndex;
  }

  function handleInput() {
    chatStore.setInputDraft(input);
  }

  function handleInputKeyDown(event: KeyboardEvent) {
    if (event.isComposing) return;
    if (isHotkeyPressed(event, $settingsStore.hotkey_send)) {
      event.preventDefault();
      sendMessage();
    }
  }

  async function applyCaptureVisibility(value: boolean) {
    try {
      await setScreenCaptureProtection(value);
      settingsStore.updateField('screen_capture_protection', value);
    } catch (error) {
      console.warn('Failed to set capture visibility:', error);
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

  async function toggleRecording() {
    if (isRecordingBusy) return;

    isRecordingBusy = true;
    recordingErrorMessage = null;

    try {
      if (!isRecording) {
        await startRecording();
        isRecording = true;
        return;
      }

      await stopRecording();
      isRecording = false;
      isTranscribingRecording = true;
      void transcribeLastRecording()
        .then((transcription) => {
          const trimmedTranscription = transcription.trim();
          if (trimmedTranscription !== '') {
            input = input.trim() === '' ? trimmedTranscription : `${input.trim()} ${trimmedTranscription}`;
            chatStore.setInputDraft(input);
          }
        })
        .catch((error) => {
          recordingErrorMessage = error instanceof Error ? error.message : String(error);
        })
        .finally(() => {
          isTranscribingRecording = false;
        });
    } catch (error) {
      isRecording = false;
      recordingErrorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      isRecordingBusy = false;
    }
  }

  async function toggleSpeaking(exchangeIndex: number, text: string) {
    if (speakingExchangeIndex === exchangeIndex) {
      try {
        await stopSpeaking();
        speakingExchangeIndex = null;
      } catch (error) {
        ttsErrorMessage = error instanceof Error ? error.message : String(error);
      }
      return;
    }

    try {
      if (speakingExchangeIndex != null) {
        await stopSpeaking();
      }
      
      speakingExchangeIndex = exchangeIndex;
      ttsErrorMessage = null;
      
      await speakText(text);
    } catch (error) {
      speakingExchangeIndex = null;
      ttsErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  function dismissError() {
    chatStore.clearError();
    recordingErrorMessage = null;
    ttsErrorMessage = null;
  }

  async function clearSession() {
    if (isClearingSession) return;
    isClearingSession = true;
    try {
      if (speakingExchangeIndex != null) {
        await stopSpeaking();
      }
    } catch {
      // Ignore stop-speaking failures while clearing session state.
    } finally {
      chatStore.clearSession();
      input = '';
      currentExchangeIndex = 0;
      isExchangeIndexInitialized = false;
      speakingExchangeIndex = null;
      recordingErrorMessage = null;
      ttsErrorMessage = null;
      isClearingSession = false;
    }
  }

  function isEditableTarget(target: EventTarget | null): boolean {
    if (!(target instanceof HTMLElement)) return false;
    if (target.isContentEditable) return true;
    const tagName = target.tagName.toLowerCase();
    return tagName === 'input' || tagName === 'textarea' || tagName === 'select';
  }

  onMount(() => {
    const handleHotkeys = (event: KeyboardEvent) => {
      if (isHotkeyPressed(event, $settingsStore.hotkey_toggle)) {
        event.preventDefault();
        void onToggleSettings?.();
        return;
      }

      if (isHotkeyPressed(event, $settingsStore.hotkey_record)) {
        event.preventDefault();
        void toggleRecording();
        return;
      }

      if (isHotkeyPressed(event, $settingsStore.hotkey_click_through)) {
        event.preventDefault();
        void applyClickThrough(!$settingsStore.click_through);
        return;
      }

      if (isHotkeyPressed(event, $settingsStore.hotkey_capture_visibility)) {
        event.preventDefault();
        void applyCaptureVisibility(!$settingsStore.screen_capture_protection);
        return;
      }

      if (isEditableTarget(event.target)) {
        return;
      }

      if (isHotkeyPressed(event, $settingsStore.hotkey_previous)) {
        event.preventDefault();
        goToPreviousExchange();
        return;
      }

      if (isHotkeyPressed(event, $settingsStore.hotkey_next)) {
        event.preventDefault();
        goToNextExchange();
      }
    };

    const pollSpeakingState = () => {
      if (speakingExchangeIndex == null) return;
      void isSpeaking()
        .then((currentlySpeaking) => {
          if (!currentlySpeaking) {
            speakingExchangeIndex = null;
          }
        })
        .catch(() => {
          // Ignore transient TTS polling errors.
        });
    };

    const speakingPollInterval = window.setInterval(pollSpeakingState, 400);
    window.addEventListener('keydown', handleHotkeys, true);

    // Listen for global hotkey events from backend
    const unlistenRecordingPromise = listen('hotkey-toggle-recording', () => {
      void toggleRecording();
    });

    return () => {
      window.clearInterval(speakingPollInterval);
      window.removeEventListener('keydown', handleHotkeys, true);
      void unlistenRecordingPromise.then((unlisten) => unlisten());
    };
  });
</script>

<section class="h-full">
  <div class="h-full flex flex-col gap-2">
    {#if errorMessage != null}
      <ErrorMessage message={errorMessage} onDismiss={dismissError} />
    {/if}
    {#if recordingErrorMessage != null}
      <ErrorMessage message={recordingErrorMessage} onDismiss={dismissError} />
    {/if}
    {#if ttsErrorMessage != null}
      <ErrorMessage message={ttsErrorMessage} onDismiss={dismissError} />
    {/if}

    <div class="flex-1 min-h-0 rounded-2xl border backdrop-blur-xl p-2 flex flex-col gap-2"
      style="border-color: rgba(255, 255, 255, var(--doppler-border-alpha, 0.7)); background: rgba(255, 255, 255, var(--doppler-surface-alpha, 0.5));">
      <div class="flex items-center justify-end gap-1.5 select-none">
        <button
          type="button"
          class="h-9 px-2 rounded-lg border border-white/70 bg-white/65 text-xs font-semibold text-slate-700 transition disabled:opacity-40 disabled:cursor-not-allowed hover:bg-white/95"
          onclick={() => {
            void clearSession();
          }}
          disabled={isClearingSession || (exchanges.length === 0 && input.trim() === '')}
          title="Clear session"
        >
          Clear
        </button>
        <button
          type="button"
          class="h-9 w-9 rounded-lg border border-white/70 bg-white/65 text-slate-700 transition disabled:opacity-40 disabled:cursor-not-allowed hover:bg-white/95"
          onclick={goToPreviousExchange}
          disabled={!canGoPrevious}
          aria-label="Previous exchange"
          title={`Previous exchange (${formatHotkeyLabel($settingsStore.hotkey_previous)})`}
          data-hotkey={formatHotkeyLabel($settingsStore.hotkey_previous)}
          data-hotkey-position="bottom"
        >
          ←
        </button>
        <div class="min-w-16 text-center text-xs font-semibold text-slate-600">
          {#if hasExchanges}
            {currentExchangeIndex + 1} / {exchanges.length}
          {:else}
            0 / 0
          {/if}
        </div>
        <button
          type="button"
          class="h-9 w-9 rounded-lg border border-white/70 bg-white/65 text-slate-700 transition disabled:opacity-40 disabled:cursor-not-allowed hover:bg-white/95"
          onclick={goToNextExchange}
          disabled={!canGoNext}
          aria-label="Next exchange"
          title={`Next exchange (${formatHotkeyLabel($settingsStore.hotkey_next)})`}
          data-hotkey={formatHotkeyLabel($settingsStore.hotkey_next)}
          data-hotkey-position="bottom"
        >
          →
        </button>
      </div>

      <div class="flex-1 min-h-0 overflow-auto space-y-3">
        {#if currentExchange != null}
          <div class="rounded-2xl border p-3 md:p-4 space-y-2"
            style="border-color: rgba(186, 230, 253, var(--doppler-border-alpha, 0.7)); background: rgba(224, 242, 254, calc(var(--doppler-surface-strong-alpha, 0.75) + 0.05));">
            <div class="text-[11px] uppercase tracking-wide font-semibold text-slate-500">Question</div>
            <p class="text-[1.02rem] leading-relaxed text-slate-900 break-words">{currentExchange.question}</p>
          </div>

          <div class="rounded-2xl border p-3 md:p-4 space-y-2"
            style="border-color: rgba(203, 213, 225, var(--doppler-border-alpha, 0.7)); background: rgba(255, 255, 255, var(--doppler-surface-strong-alpha, 0.78));">
            <div class="flex items-center justify-between">
              <div class="text-[11px] uppercase tracking-wide font-semibold text-slate-500">Answer</div>
              {#if !currentExchange.isPending}
                <SpeakButton
                  speaking={speakingExchangeIndex === currentExchangeIndex}
                  onclick={() => {
                    void toggleSpeaking(currentExchangeIndex, currentExchange.answer);
                  }}
                />
              {/if}
            </div>
            {#if currentExchange.isPending}
              <div class="flex items-center gap-2 text-slate-600">
                <Spinner size="sm" />
                <span class="text-sm">Generating response...</span>
              </div>
            {:else}
              <div
                class="markdown-content text-[1.02rem] leading-relaxed text-slate-900 break-words"
              >
                {@html renderMarkdown(currentExchange.answer)}
              </div>
            {/if}
          </div>
        {:else}
          <div class="h-full min-h-[140px] flex items-center justify-center">
            <p class="text-sm text-slate-600 text-center">No exchanges yet. Ask your first question below.</p>
          </div>
        {/if}
      </div>
    </div>

    <div
      class="rounded-2xl border backdrop-blur-xl p-2.5 md:p-3 transition-all {$settingsStore.screen_capture_protection
        ? ''
        : 'shadow-[inset_0_0_0_1px_rgba(245,158,11,0.4)]'}"
      style={$settingsStore.screen_capture_protection
        ? 'border-color: rgba(255, 255, 255, var(--doppler-border-alpha, 0.7)); background: rgba(255, 255, 255, var(--doppler-surface-strong-alpha, 0.7));'
        : 'border-color: rgba(245, 158, 11, 0.8); background: rgba(255, 255, 255, var(--doppler-surface-strong-alpha, 0.7));'}
      title={$settingsStore.screen_capture_protection
        ? 'Window is hidden from capture'
        : 'Warning: window is visible in capture'}
    >
      <div class="flex items-center gap-2 select-none">
        <button
          type="button"
          class="h-11 w-11 shrink-0 rounded-xl border text-lg transition {isRecording
            ? 'border-rose-400/80 bg-rose-500/18 text-rose-700 shadow-[0_0_0_1px_rgba(244,63,94,0.18)]'
            : 'border-white/85 bg-white text-slate-700 hover:bg-slate-50'}"
          onclick={() => {
            void toggleRecording();
          }}
          disabled={isRecordingBusy || isTranscribingRecording}
          aria-label={isRecording ? 'Stop recording' : 'Start recording'}
          title={isRecording ? `Stop recording (${formatHotkeyLabel($settingsStore.hotkey_record)})` : isTranscribingRecording ? 'Transcribing...' : `Start recording (${formatHotkeyLabel($settingsStore.hotkey_record)})`}
          data-hotkey={formatHotkeyLabel($settingsStore.hotkey_record)}
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
          onclick={() => onToggleSettings?.()}
          aria-label={isSettingsOpen ? 'Close settings' : 'Open settings'}
          title={isSettingsOpen ? 'Close settings (Esc)' : `Open settings (${formatHotkeyLabel($settingsStore.hotkey_toggle)})`}
          data-hotkey={isSettingsOpen ? 'Esc' : formatHotkeyLabel($settingsStore.hotkey_toggle)}
        >
          {#if isSettingsOpen}
            <svg viewBox="0 0 24 24" class="mx-auto h-5 w-5" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M6 6l12 12M18 6L6 18" stroke-linecap="round" />
            </svg>
          {:else}
            <svg viewBox="0 0 24 24" class="mx-auto h-5 w-5" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="4" y1="6" x2="20" y2="6" stroke-linecap="round" />
              <line x1="4" y1="12" x2="20" y2="12" stroke-linecap="round" />
              <line x1="4" y1="18" x2="20" y2="18" stroke-linecap="round" />
              <circle cx="9" cy="6" r="2" fill="currentColor" stroke="none" />
              <circle cx="15" cy="12" r="2" fill="currentColor" stroke="none" />
              <circle cx="11" cy="18" r="2" fill="currentColor" stroke="none" />
            </svg>
          {/if}
        </button>

        <input
          type="text"
          bind:value={input}
          oninput={handleInput}
          onkeydown={handleInputKeyDown}
          placeholder="Ask a question..."
          class="select-text flex-1 min-w-0 h-11 rounded-xl border px-3.5 text-[1rem] text-slate-900 placeholder:text-slate-500 focus:outline-none focus:ring-2 focus:ring-sky-300/80"
          style="border-color: rgba(255, 255, 255, var(--doppler-border-alpha, 0.75)); background: rgba(255, 255, 255, var(--doppler-surface-strong-alpha, 0.78));"
          title={`Type question and press ${formatHotkeyLabel($settingsStore.hotkey_send)} to send`}
          data-hotkey={formatHotkeyLabel($settingsStore.hotkey_send)}
        />

        <button
          type="button"
          class="h-11 px-4 rounded-xl border border-sky-400/45 bg-sky-500/85 text-white text-sm font-semibold transition disabled:opacity-45 disabled:cursor-not-allowed hover:bg-sky-500"
          onclick={sendMessage}
          disabled={input.trim() === '' || isLoading}
          title={`Send question (${formatHotkeyLabel($settingsStore.hotkey_send)})`}
          data-hotkey={formatHotkeyLabel($settingsStore.hotkey_send)}
        >
          {#if isLoading}...{:else}Send{/if}
        </button>
      </div>
    </div>
  </div>
</section>

<style>
  .markdown-content :global(h1),
  .markdown-content :global(h2),
  .markdown-content :global(h3),
  .markdown-content :global(h4),
  .markdown-content :global(h5),
  .markdown-content :global(h6) {
    margin: 0.5rem 0;
    font-weight: 700;
    line-height: 1.3;
  }

  .markdown-content :global(p) {
    margin: 0.5rem 0;
  }

  .markdown-content :global(ul),
  .markdown-content :global(ol) {
    margin: 0.5rem 0;
    padding-left: 1.2rem;
  }

  .markdown-content :global(li) {
    margin: 0.2rem 0;
  }

  .markdown-content :global(blockquote) {
    margin: 0.5rem 0;
    padding: 0.4rem 0.7rem;
    border-left: 3px solid rgba(148, 163, 184, 0.8);
    background: rgba(241, 245, 249, 0.65);
    border-radius: 0.5rem;
  }

  .markdown-content :global(pre) {
    margin: 0.6rem 0;
    padding: 0.7rem;
    overflow-x: auto;
    border: 1px solid rgba(148, 163, 184, 0.4);
    border-radius: 0.6rem;
    background: rgba(15, 23, 42, 0.95);
    color: #f8fafc;
    font-size: 0.92rem;
  }

  .markdown-content :global(code) {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono',
      'Courier New', monospace;
    background: rgba(148, 163, 184, 0.18);
    padding: 0.12rem 0.3rem;
    border-radius: 0.3rem;
    font-size: 0.9em;
  }

  .markdown-content :global(pre code) {
    background: transparent;
    padding: 0;
    border-radius: 0;
  }

  .markdown-content :global(a) {
    color: #0c4a6e;
    text-decoration: underline;
    text-underline-offset: 2px;
  }
</style>
