<script lang="ts">
  interface Props {
    type?: string;
    placeholder?: string;
    list?: string;
    value?: string;
    disabled?: boolean;
    oninput?: (event: Event) => void;
    onkeypress?: (event: KeyboardEvent) => void;
  }

  let {
    type = 'text',
    placeholder = '',
    list = undefined,
    value = $bindable(''),
    disabled = false,
    oninput,
    onkeypress
  }: Props = $props();

  const baseClasses = 'w-full h-10 px-3.5 border border-white/80 bg-white/75 text-slate-900 rounded-xl shadow-sm backdrop-blur focus:outline-none focus:ring-2 focus:ring-sky-300/85 focus:border-white disabled:opacity-50 disabled:cursor-not-allowed';
</script>

<input
  {type}
  {placeholder}
  {list}
  bind:value
  {disabled}
  {oninput}
  {onkeypress}
  class={baseClasses}
/>
