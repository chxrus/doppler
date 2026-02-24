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

  const baseClasses = 'w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer';
  const thumbClasses = '[&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:bg-blue-500 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:cursor-pointer [&::-moz-range-thumb]:w-4 [&::-moz-range-thumb]:h-4 [&::-moz-range-thumb]:bg-blue-500 [&::-moz-range-thumb]:rounded-full [&::-moz-range-thumb]:cursor-pointer [&::-moz-range-thumb]:border-0';
</script>

<input
  type="range"
  {min}
  {max}
  {step}
  bind:value
  {oninput}
  class="{baseClasses} {thumbClasses}"
/>
