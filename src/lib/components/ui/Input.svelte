<script lang="ts">
  interface Props {
    type?: string;
    placeholder?: string;
    value?: string;
    disabled?: boolean;
    oninput?: (event: Event) => void;
    onkeypress?: (event: KeyboardEvent) => void;
  }

  let {
    type = 'text',
    placeholder = '',
    value = $bindable(''),
    disabled = false,
    oninput,
    onkeypress
  }: Props = $props();

  const baseClasses = 'w-full px-4 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:opacity-50 disabled:cursor-not-allowed';
</script>

<input
  {type}
  {placeholder}
  bind:value
  {disabled}
  {oninput}
  {onkeypress}
  class={baseClasses}
/>
