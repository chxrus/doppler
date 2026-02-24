<script lang="ts">
  import Button from './ui/Button.svelte';

  interface Props {
    speaking?: boolean;
    onclick?: () => void;
  }

  let { speaking = false, onclick }: Props = $props();
</script>

<Button
  variant={speaking ? 'danger' : 'secondary'}
  size="sm"
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
</Button>

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
