<script lang="ts">
  import Button from './ui/Button.svelte';

  interface Props {
    recording?: boolean;
    elapsedSeconds?: number;
    onRecord?: () => void;
    onStop?: () => void;
  }

  let {
    recording = false,
    elapsedSeconds = 0,
    onRecord,
    onStop
  }: Props = $props();

  function formatTime(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
</script>

<div class="flex items-center gap-3">
  {#if recording}
    <Button variant="danger" size="md" onclick={onStop}>
      <div class="flex items-center gap-2">
        <span class="recording-indicator"></span>
        <span>Stop</span>
      </div>
    </Button>
    <div class="elapsed-time">
      {formatTime(elapsedSeconds)}
    </div>
  {:else}
    <Button variant="secondary" size="md" onclick={onRecord}>
      <div class="flex items-center gap-2">
        <span class="record-icon">🎤</span>
        <span>Record</span>
      </div>
    </Button>
  {/if}
</div>

<style>
  .recording-indicator {
    display: inline-block;
    width: 8px;
    height: 8px;
    background-color: #ef4444;
    border-radius: 50%;
    animation: pulse 1.5s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 1;
      transform: scale(1);
    }
    50% {
      opacity: 0.5;
      transform: scale(1.2);
    }
  }

  .elapsed-time {
    font-family: 'Courier New', monospace;
    font-size: 1rem;
    font-weight: 600;
    color: #ef4444;
    min-width: 3.5rem;
    text-align: center;
  }

  .record-icon {
    font-size: 1rem;
    line-height: 1;
  }
</style>
