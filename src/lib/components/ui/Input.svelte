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

  const baseClasses = 'w-full h-10 px-3.5 border rounded-xl shadow-sm backdrop-blur text-slate-100 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-teal-300/70 disabled:opacity-50 disabled:cursor-not-allowed';
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
  style="border-color: rgba(148, 163, 184, var(--doppler-border-alpha, 0.65)); background: rgb(var(--doppler-control-rgb, 15 23 42) / var(--doppler-control-alpha, 0.62));"
/>
