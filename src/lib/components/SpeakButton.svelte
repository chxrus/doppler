<script lang="ts">
  interface Props {
    speaking?: boolean;
    onclick?: () => void;
  }

  let { speaking = false, onclick }: Props = $props();
</script>

<button
  type="button"
  class="inline-flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-sm font-medium tracking-tight shadow-sm backdrop-blur transition disabled:opacity-50 disabled:cursor-not-allowed
    {speaking
      ? 'border-rose-300/65 bg-rose-500/25 text-rose-100 hover:bg-rose-500/35'
      : 'border-white/15 bg-slate-900/45 text-slate-100 hover:bg-slate-900/75'}"
  {onclick}
>
  {#if speaking}
    <span class="flex items-center gap-1.5">
      <span class="speaking-indicator">🔊</span>
      <span>Stop</span>
    </span>
  {:else}
    <span class="flex items-center gap-1.5">
      <span>🔊</span>
      <span>Speak</span>
    </span>
  {/if}
</button>

<style>
  @keyframes pulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }

  .speaking-indicator {
    animation: pulse 1.5s ease-in-out infinite;
  }
</style>
