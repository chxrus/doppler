<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte';
  import Spinner from '$lib/components/ui/Spinner.svelte';
  import { chatStore, type VoiceProcessingState } from '$lib/stores/chatStore';
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
  let activeChatId = $state(1);
  let isLoading = $state(false);
  let errorMessage = $state<string | null>(null);
  let recordingErrorMessage = $state<string | null>(null);
  let isRecordingBusy = $state(false);
  let voiceProcessingState = $state<VoiceProcessingState>('idle');
  let speakingExchangeIndex = $state<number | null>(null);
  let ttsErrorMessage = $state<string | null>(null);
  let isClearingSession = $state(false);
  const voiceProcessingStatus = $derived.by(() => {
    if (voiceProcessingState === 'transcribing') return 'Transcribing...';
    if (voiceProcessingState === 'inserting') return 'Inserting...';
    return null;
  });

  $effect(() => {
    const unsubExchanges = chatStore.exchanges.subscribe(value => exchanges = value);
    const unsubChatId = chatStore.chatId.subscribe(value => activeChatId = value);
    const unsubLoading = chatStore.isLoading.subscribe(value => isLoading = value);
    const unsubError = chatStore.error.subscribe(value => errorMessage = value);
    const unsubInputDraft = chatStore.inputDraft.subscribe(value => input = value);
    const unsubVoiceState = chatStore.voiceProcessingState.subscribe(value => voiceProcessingState = value);
    
    return () => {
      unsubExchanges();
      unsubChatId();
      unsubLoading();
      unsubError();
      unsubInputDraft();
      unsubVoiceState();
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
        chatStore.setVoiceProcessingStateForChat(activeChatId, 'recording');
        await startRecording();
        isRecording = true;
        return;
      }

      await stopRecording();
      isRecording = false;
      const targetChatId = activeChatId;
      chatStore.setVoiceProcessingStateForChat(targetChatId, 'transcribing');
      void transcribeLastRecording()
        .then((transcription) => {
          chatStore.setVoiceProcessingStateForChat(targetChatId, 'inserting');
          const trimmedTranscription = transcription.trim();
          if (trimmedTranscription !== '') {
            input = input.trim() === '' ? trimmedTranscription : `${input.trim()} ${trimmedTranscription}`;
            chatStore.setInputDraft(input);

            if ($settingsStore.auto_send_transcription) {
              sendMessage();
            }
          }
        })
        .catch((error) => {
          recordingErrorMessage = error instanceof Error ? error.message : String(error);
        })
        .finally(() => {
          chatStore.setVoiceProcessingStateForChat(targetChatId, 'idle');
        });
    } catch (error) {
      isRecording = false;
      chatStore.setVoiceProcessingStateForChat(activeChatId, 'idle');
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
      isRecording = false;
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
      style="border-color: rgba(148, 163, 184, var(--doppler-border-alpha, 0.65)); background: rgb(var(--doppler-surface-rgb, 15 23 42) / var(--doppler-surface-alpha, 0.55));">
      <div class="flex items-center justify-between gap-3 select-none">
        <div
          class="h-9 shrink-0 rounded-lg border px-2.5 text-[11px] font-semibold uppercase tracking-wide flex items-center gap-1.5"
          style={$settingsStore.screen_capture_protection
            ? 'border-color: rgb(var(--doppler-capture-hidden-rgb) / 0.78); background: rgb(var(--doppler-capture-hidden-rgb) / var(--doppler-capture-hidden-bg-alpha, 0.28)); color: rgb(var(--doppler-capture-hidden-text-rgb, 167 243 208));'
            : 'border-color: rgb(var(--doppler-capture-visible-rgb) / 0.85); background: rgb(var(--doppler-capture-visible-rgb) / var(--doppler-capture-visible-bg-alpha, 0.24)); color: rgb(var(--doppler-capture-visible-text-rgb, 254 205 211));'}
          title={$settingsStore.screen_capture_protection
            ? 'Window is hidden from screen recording'
            : 'Window is visible in screen recording'}
        >
          {#if $settingsStore.screen_capture_protection}
            <svg viewBox="0 0 24 24" class="h-3.5 w-3.5" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M2 2l20 20" stroke-linecap="round" />
              <path d="M10.6 10.6A3 3 0 0012 15a3 3 0 002.4-4.8" stroke-linecap="round" />
              <path d="M9.4 5.1A10.7 10.7 0 0121 12a10.7 10.7 0 01-4 5.6" stroke-linecap="round" />
              <path d="M6.1 6.1A10.8 10.8 0 003 12a10.7 10.7 0 004.8 6.5" stroke-linecap="round" />
            </svg>
            Hidden in capture
          {:else}
            <svg viewBox="0 0 24 24" class="h-3.5 w-3.5" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7-10-7-10-7z" />
              <circle cx="12" cy="12" r="3" />
            </svg>
            Visible in capture
          {/if}
        </div>

	        <div class="flex items-center gap-2">
          <div
            class="h-9 w-32 rounded-lg border px-2 text-center text-xs font-medium flex items-center justify-center transition"
            style={voiceProcessingStatus !== null
              ? 'border-color: rgba(148, 163, 184, var(--doppler-border-alpha, 0.65)); background: rgb(var(--doppler-control-rgb, 15 23 42) / var(--doppler-control-alpha, 0.62)); color: rgb(203 213 225);'
              : 'border-color: transparent; background: transparent; color: transparent;'}
            aria-live="polite"
          >
            {voiceProcessingStatus ?? 'Idle'}
          </div>
	        <button
          type="button"
          class="h-9 px-2 rounded-lg border text-xs font-semibold text-slate-100 transition disabled:opacity-40 disabled:cursor-not-allowed"
          style="border-color: rgba(148, 163, 184, var(--doppler-border-alpha, 0.65)); background: rgb(var(--doppler-control-rgb, 15 23 42) / var(--doppler-control-alpha, 0.62));"
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
          class="h-9 w-9 rounded-lg border text-slate-100 transition disabled:opacity-40 disabled:cursor-not-allowed"
          style="border-color: rgba(148, 163, 184, var(--doppler-border-alpha, 0.65)); background: rgb(var(--doppler-control-rgb, 15 23 42) / var(--doppler-control-alpha, 0.62));"
          onclick={goToPreviousExchange}
          disabled={!canGoPrevious}
          aria-label="Previous exchange"
          title={`Previous exchange (${formatHotkeyLabel($settingsStore.hotkey_previous)})`}
          data-hotkey={formatHotkeyLabel($settingsStore.hotkey_previous)}
          data-hotkey-position="bottom"
        >
          ←
        </button>
        <div class="min-w-16 text-center text-xs font-semibold text-slate-300">
          {#if hasExchanges}
            {currentExchangeIndex + 1} / {exchanges.length}
          {:else}
            0 / 0
          {/if}
        </div>
        <button
          type="button"
          class="h-9 w-9 rounded-lg border text-slate-100 transition disabled:opacity-40 disabled:cursor-not-allowed"
          style="border-color: rgba(148, 163, 184, var(--doppler-border-alpha, 0.65)); background: rgb(var(--doppler-control-rgb, 15 23 42) / var(--doppler-control-alpha, 0.62));"
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
      </div>

      <div class="flex-1 min-h-0 overflow-auto space-y-3">
        {#if currentExchange != null}
          <div class="rounded-2xl border p-3 md:p-4 space-y-2"
            style="border-color: rgb(var(--doppler-accent-rgb) / 0.62); background: rgb(var(--doppler-accent-rgb) / 0.18); box-shadow: inset 3px 0 0 rgb(var(--doppler-accent-rgb) / 0.65);">
            <div class="flex items-center gap-2 text-[11px] uppercase tracking-wide font-semibold"
              style="color: rgb(var(--doppler-label-question-rgb, 204 251 241));">
              <span class="inline-flex h-5 min-w-5 items-center justify-center rounded-md border border-teal-200/40 bg-teal-400/20 px-1">You</span>
              <span>Question</span>
            </div>
            <p class="text-[1.02rem] leading-relaxed text-slate-100 break-words">{currentExchange.question}</p>
          </div>

          <div class="rounded-2xl border p-3 md:p-4 space-y-2"
            style="border-color: rgba(99, 102, 241, 0.45); background: rgb(var(--doppler-surface-rgb, 15 23 42) / var(--doppler-surface-strong-alpha, 0.74)); box-shadow: inset 3px 0 0 rgba(129, 140, 248, 0.58);">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2 text-[11px] uppercase tracking-wide font-semibold"
                style="color: rgb(var(--doppler-label-answer-rgb, 224 231 255));">
                <span class="inline-flex h-5 min-w-5 items-center justify-center rounded-md border border-indigo-200/40 bg-indigo-400/20 px-1">AI</span>
                <span>Answer</span>
              </div>
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
              <div class="flex items-center gap-2 text-slate-300">
                <Spinner size="sm" />
                <span class="text-sm">Generating response...</span>
              </div>
            {/if}
            <div
              class="markdown-content text-[1.02rem] leading-relaxed text-slate-100 break-words"
            >
              {@html renderMarkdown(currentExchange.answer)}
            </div>
          </div>
        {:else}
          <div class="h-full min-h-[140px] flex items-center justify-center">
            <p class="text-sm text-slate-300 text-center">No exchanges yet. Ask your first question below.</p>
          </div>
        {/if}
      </div>
    </div>

    <div
      class="rounded-2xl border backdrop-blur-xl p-2.5 md:p-3 transition-all"
      style="border-color: rgba(148, 163, 184, var(--doppler-border-alpha, 0.65)); background: rgb(var(--doppler-surface-rgb, 15 23 42) / var(--doppler-surface-strong-alpha, 0.7));"
      title={$settingsStore.screen_capture_protection
        ? 'Window is hidden from capture'
        : 'Warning: window is visible in capture'}
    >
      <div class="flex items-center gap-2 select-none">
        <button
          type="button"
          class="h-11 w-11 shrink-0 rounded-xl border text-lg transition {isRecording
            ? 'border-rose-300/70 bg-rose-500/28 text-rose-100 shadow-[0_0_0_1px_rgba(244,63,94,0.25)]'
            : 'border-white/15 bg-slate-900/45 text-slate-100 hover:bg-slate-900/75'}"
	          onclick={() => {
	            void toggleRecording();
	          }}
          disabled={isRecordingBusy || voiceProcessingState === 'transcribing' || voiceProcessingState === 'inserting'}
	          aria-label={isRecording ? 'Stop recording' : 'Start recording'}
          title={isRecording ? `Stop recording (${formatHotkeyLabel($settingsStore.hotkey_record)})` : voiceProcessingStatus ?? `Start recording (${formatHotkeyLabel($settingsStore.hotkey_record)})`}
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
          class="h-11 w-11 shrink-0 rounded-xl border border-white/15 bg-slate-900/45 text-slate-100 shadow-sm transition hover:bg-slate-900/75"
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
          class="select-text flex-1 min-w-0 h-11 rounded-xl border px-3.5 text-[1rem] text-slate-100 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-teal-300/70"
          style="border-color: rgba(148, 163, 184, var(--doppler-border-alpha, 0.65)); background: rgb(var(--doppler-control-rgb, 15 23 42) / var(--doppler-control-alpha, 0.62));"
          title={`Type question and press ${formatHotkeyLabel($settingsStore.hotkey_send)} to send`}
          data-hotkey={formatHotkeyLabel($settingsStore.hotkey_send)}
        />

        <button
          type="button"
          class="h-11 px-4 rounded-xl border border-teal-300/55 bg-teal-500/85 text-white text-sm font-semibold transition disabled:opacity-45 disabled:cursor-not-allowed hover:bg-teal-400"
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
    border-left: 3px solid rgba(148, 163, 184, 0.75);
    background: rgba(15, 23, 42, 0.58);
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
    background: rgba(148, 163, 184, 0.24);
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
    color: #5eead4;
    text-decoration: underline;
    text-underline-offset: 2px;
  }
</style>
