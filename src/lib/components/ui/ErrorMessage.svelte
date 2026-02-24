<script lang="ts">
  import type { Snippet } from 'svelte';
  import Button from './Button.svelte';

  interface Props {
    message: string;
    onDismiss?: () => void;
    children?: Snippet;
  }

  let { message, onDismiss, children }: Props = $props();
</script>

<div class="flex items-start gap-2 p-3 bg-red-50 border border-red-200 rounded-lg text-sm">
  <div class="flex-shrink-0 mt-0.5">
    <svg
      class="w-4 h-4 text-red-500"
      fill="currentColor"
      viewBox="0 0 20 20"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        fill-rule="evenodd"
        d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
        clip-rule="evenodd"
      />
    </svg>
  </div>

  <div class="flex-1 text-red-800">
    <p>{message}</p>
    {#if children}
      {@render children()}
    {/if}
  </div>

  {#if onDismiss}
    <Button variant="secondary" size="sm" onclick={onDismiss}>
      ✕
    </Button>
  {/if}
</div>
