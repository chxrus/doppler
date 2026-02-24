<script lang="ts">
  interface Props {
    size?: 'sm' | 'md' | 'lg';
  }

  let { size = 'md' }: Props = $props();

  const sizeClasses = {
    sm: 'w-4 h-4 border-2',
    md: 'w-6 h-6 border-4',
    lg: 'w-8 h-8 border-4'
  };

  const spinnerClasses = $derived(`inline-block ${sizeClasses[size]} border-gray-200 border-t-blue-500 rounded-full animate-spin`);
</script>

<div class={spinnerClasses} role="status" aria-label="Loading">
  <span class="sr-only">Loading...</span>
</div>
