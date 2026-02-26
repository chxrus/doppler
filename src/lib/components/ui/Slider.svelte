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

  const baseClasses = 'w-full h-2.5 appearance-none cursor-pointer bg-transparent';
  const trackClasses = '[&::-webkit-slider-runnable-track]:h-2.5 [&::-webkit-slider-runnable-track]:rounded-full [&::-webkit-slider-runnable-track]:bg-slate-600/80 [&::-webkit-slider-runnable-track]:border [&::-webkit-slider-runnable-track]:border-slate-400/45 [&::-webkit-slider-runnable-track]:shadow-[inset_0_1px_0_rgba(255,255,255,0.12)] [&::-moz-range-track]:h-2.5 [&::-moz-range-track]:rounded-full [&::-moz-range-track]:bg-slate-600/80 [&::-moz-range-track]:border [&::-moz-range-track]:border-slate-400/45';
  const thumbClasses = '[&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:bg-teal-400 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:cursor-pointer [&::-webkit-slider-thumb]:border [&::-webkit-slider-thumb]:border-teal-200/75 [&::-webkit-slider-thumb]:shadow-[0_0_0_1px_rgba(15,23,42,0.45)] [&::-webkit-slider-thumb]:-mt-[4px] [&::-moz-range-thumb]:w-4 [&::-moz-range-thumb]:h-4 [&::-moz-range-thumb]:bg-teal-400 [&::-moz-range-thumb]:rounded-full [&::-moz-range-thumb]:cursor-pointer [&::-moz-range-thumb]:border [&::-moz-range-thumb]:border-teal-200/75';
</script>

<input
  type="range"
  {min}
  {max}
  {step}
  bind:value
  {oninput}
  class="{baseClasses} {trackClasses} {thumbClasses}"
/>
