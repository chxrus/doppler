<script lang="ts">
  interface Props {
    min?: number;
    max?: number;
    step?: number;
    value?: number;
    oninput?: (event: Event) => void;
  }

  let {
    min = 0,
    max = 100,
    step = 1,
    value = $bindable(50),
    oninput
  }: Props = $props();

  const baseClasses = 'doppler-slider w-full h-2.5 appearance-none cursor-pointer bg-transparent';
</script>

<input
  type="range"
  {min}
  {max}
  {step}
  bind:value
  {oninput}
  class={baseClasses}
/>

<style>
  .doppler-slider::-webkit-slider-runnable-track {
    height: 0.625rem;
    border-radius: 9999px;
    background: rgb(var(--doppler-slider-track-rgb, 71 85 105) / var(--doppler-slider-track-alpha, 0.8));
    border: 1px solid rgb(var(--doppler-slider-track-border-rgb, 148 163 184) / var(--doppler-slider-track-border-alpha, 0.45));
    box-shadow: inset 0 1px 0 rgb(255 255 255 / 0.12);
  }

  .doppler-slider::-moz-range-track {
    height: 0.625rem;
    border-radius: 9999px;
    background: rgb(var(--doppler-slider-track-rgb, 71 85 105) / var(--doppler-slider-track-alpha, 0.8));
    border: 1px solid rgb(var(--doppler-slider-track-border-rgb, 148 163 184) / var(--doppler-slider-track-border-alpha, 0.45));
  }

  .doppler-slider::-webkit-slider-thumb {
    appearance: none;
    width: 1rem;
    height: 1rem;
    margin-top: -0.25rem;
    border-radius: 9999px;
    background: rgb(45 212 191 / 0.95);
    border: 1px solid rgb(153 246 228 / 0.8);
    box-shadow: 0 0 0 1px rgb(15 23 42 / 0.45);
    cursor: pointer;
  }

  .doppler-slider::-moz-range-thumb {
    width: 1rem;
    height: 1rem;
    border-radius: 9999px;
    background: rgb(45 212 191 / 0.95);
    border: 1px solid rgb(153 246 228 / 0.8);
    cursor: pointer;
  }
</style>
