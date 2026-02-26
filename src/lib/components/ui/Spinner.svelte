<script lang="ts">
  interface Props {
    size?: 'sm' | 'md' | 'lg';
  }

  let { size = 'md' }: Props = $props();

  const containerClasses = {
    sm: 'h-4 gap-1',
    md: 'h-6 gap-1.5',
    lg: 'h-8 gap-2'
  };

  const dotClasses = {
    sm: 'h-1.5 w-1.5',
    md: 'h-2 w-2',
    lg: 'h-2.5 w-2.5'
  };
</script>

<div class="inline-flex items-center {containerClasses[size]}" role="status" aria-label="Loading">
  <span class="spinner-dot {dotClasses[size]}" style="animation-delay: 0ms;"></span>
  <span class="spinner-dot {dotClasses[size]}" style="animation-delay: 150ms;"></span>
  <span class="spinner-dot {dotClasses[size]}" style="animation-delay: 300ms;"></span>
</div>

<style>
  .spinner-dot {
    border-radius: 9999px;
    background: rgb(129 140 248 / 0.9);
    animation: dot-pulse 0.95s ease-in-out infinite;
  }

  @keyframes dot-pulse {
    0%,
    100% {
      opacity: 0.3;
      transform: scale(0.82);
    }

    50% {
      opacity: 1;
      transform: scale(1);
    }
  }
</style>
