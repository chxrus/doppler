<script lang="ts">
  import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte';
  import Spinner from '$lib/components/ui/Spinner.svelte';

  interface Props {
    onToggleSettings?: () => void | Promise<void>;
    isSettingsOpen?: boolean;
  }

  let { onToggleSettings, isSettingsOpen = false }: Props = $props();

  interface Exchange {
    question: string;
    answer: string;
    isPending?: boolean;
  }

  let exchanges = $state<Exchange[]>([
    {
      question: 'How does this overlay workflow help in daily tasks?',
      answer: 'It keeps a compact ask/answer loop on top of your work without forcing a full chat thread view.'
    },
    {
      question: 'Can I navigate previous answers quickly?',
      answer: 'Yes. Use the left and right arrows to move between recent exchanges while keeping focus on one item at a time.'
    }
  ]);

  let currentExchangeIndex = $state(0);
  let isExchangeIndexInitialized = $state(false);
  let input = $state('');
  let isRecording = $state(false);
  let isLoading = $state(false);
  let errorMessage = $state<string | null>(null);

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

    const exchangeToAdd: Exchange = {
      question: trimmedInput,
      answer: '',
      isPending: true
    };

    exchanges = [...exchanges, exchangeToAdd];
    currentExchangeIndex = exchanges.length - 1;
    input = '';
    isLoading = true;
    errorMessage = null;

    const pendingExchangeIndex = currentExchangeIndex;
    const prompt = exchangeToAdd.question;

    setTimeout(() => {
      exchanges = exchanges.map((exchange, index) =>
        index === pendingExchangeIndex
          ? {
              ...exchange,
              isPending: false,
              answer: `You asked: "${prompt}". This is a mock answer in focused Q/A mode.`
            }
          : exchange
      );
      isLoading = false;
    }, 1200);
  }

  function handleKeyPress(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      sendMessage();
    }
  }

  function toggleRecording() {
    isRecording = !isRecording;
  }

  function dismissError() {
    errorMessage = null;
  }
</script>

<section class="h-full p-3 md:p-4">
  <div class="h-full flex flex-col gap-3">
    {#if errorMessage != null}
      <ErrorMessage message={errorMessage} onDismiss={dismissError} />
    {/if}

    <div class="flex-1 min-h-0 rounded-2xl border border-white/70 bg-white/50 backdrop-blur-xl p-3 md:p-4 flex flex-col gap-3">
      <div class="flex items-center justify-end gap-1.5">
        <button
          type="button"
          class="h-9 w-9 rounded-lg border border-white/70 bg-white/65 text-slate-700 transition disabled:opacity-40 disabled:cursor-not-allowed hover:bg-white/95"
          onclick={goToPreviousExchange}
          disabled={!canGoPrevious}
          aria-label="Previous exchange"
          title="Previous exchange (Alt+Left)"
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
          title="Next exchange (Alt+Right)"
        >
          →
        </button>
      </div>

      <div class="flex-1 min-h-0 overflow-auto space-y-3">
        {#if currentExchange != null}
          <div class="rounded-2xl border border-sky-200 bg-sky-50/85 p-3 md:p-4 space-y-2">
            <div class="text-[11px] uppercase tracking-wide font-semibold text-slate-500">Question</div>
            <p class="text-[1.02rem] leading-relaxed text-slate-900 break-words">{currentExchange.question}</p>
          </div>

          <div class="rounded-2xl border border-slate-200 bg-white p-3 md:p-4 space-y-2">
            <div class="text-[11px] uppercase tracking-wide font-semibold text-slate-500">Answer</div>
            {#if currentExchange.isPending}
              <div class="flex items-center gap-2 text-slate-600">
                <Spinner size="sm" />
                <span class="text-sm">Generating response...</span>
              </div>
            {:else}
              <p class="text-[1.02rem] leading-relaxed text-slate-900 break-words">{currentExchange.answer}</p>
            {/if}
          </div>
        {:else}
          <p class="text-sm text-slate-600">No exchanges yet. Ask your first question below.</p>
        {/if}
      </div>
    </div>

    <div class="rounded-2xl border border-white/70 bg-white/70 backdrop-blur-xl p-2.5 md:p-3">
      <div class="flex items-center gap-2">
        <button
          type="button"
          class="h-11 w-11 shrink-0 rounded-xl border text-lg transition {isRecording
            ? 'border-rose-400/80 bg-rose-500/18 text-rose-700 shadow-[0_0_0_1px_rgba(244,63,94,0.18)]'
            : 'border-white/85 bg-white text-slate-700 hover:bg-slate-50'}"
          onclick={toggleRecording}
          aria-label={isRecording ? 'Stop recording' : 'Start recording'}
          title={isRecording ? 'Stop recording (Ctrl+R)' : 'Start recording (Ctrl+R)'}
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
          title={isSettingsOpen ? 'Close settings (Esc)' : 'Open settings (Ctrl+,)'}
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
          onkeypress={handleKeyPress}
          placeholder="Ask your current question..."
          class="flex-1 min-w-0 h-11 rounded-xl border border-white/75 bg-white/78 px-3.5 text-[1rem] text-slate-900 placeholder:text-slate-500 focus:outline-none focus:ring-2 focus:ring-sky-300/80"
          title="Type question and press Enter to send"
        />

        <button
          type="button"
          class="h-11 px-4 rounded-xl border border-sky-400/45 bg-sky-500/85 text-white text-sm font-semibold transition disabled:opacity-45 disabled:cursor-not-allowed hover:bg-sky-500"
          onclick={sendMessage}
          disabled={input.trim() === '' || isLoading}
          title="Send question (Enter)"
        >
          {#if isLoading}...{:else}Send{/if}
        </button>
      </div>
    </div>
  </div>
</section>
