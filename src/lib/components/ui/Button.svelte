<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    variant?: 'primary' | 'secondary' | 'danger';
    size?: 'sm' | 'md' | 'lg';
    disabled?: boolean;
    loading?: boolean;
    onclick?: () => void;
    children?: Snippet;
  }

  let {
    variant = 'primary',
    size = 'md',
    disabled = false,
    loading = false,
    onclick,
    children
  }: Props = $props();

  const variantClasses = {
    primary: 'bg-teal-500/85 text-white border-teal-300/55 hover:bg-teal-400',
    secondary: 'bg-slate-900/45 text-slate-100 border-white/15 hover:bg-slate-900/75',
    danger: 'bg-rose-500/78 text-white border-rose-300/55 hover:bg-rose-400'
  };

  const sizeClasses = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-3.5 py-2 text-sm',
    lg: 'px-4.5 py-2.5 text-base'
  };

  const spinnerSizeClasses = {
    sm: 'w-4 h-4 border-2',
    md: 'w-5 h-5 border-2',
    lg: 'w-6 h-6 border-3'
  };

  const baseClasses = 'rounded-xl border font-medium tracking-tight shadow-sm backdrop-blur transition disabled:opacity-50 disabled:cursor-not-allowed';

</script>

<button
  class="{baseClasses} {variantClasses[variant]} {sizeClasses[size]}"
  disabled={disabled || loading}
  {onclick}
>
  {#if loading}
    <span class="inline-flex items-center gap-2">
      <span class="inline-block {spinnerSizeClasses[size]} border-white border-t-transparent rounded-full animate-spin" role="status" aria-label="Loading"></span>
      <span>Loading...</span>
    </span>
  {:else}
    {@render children?.()}
  {/if}
</button>
