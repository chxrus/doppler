<script lang="ts">
  interface Props {
    checked?: boolean;
    label?: string;
    disabled?: boolean;
    onchange?: (event: Event) => void;
  }

  let {
    checked = $bindable(false),
    label = '',
    disabled = false,
    onchange
  }: Props = $props();

  const checkboxClasses = 'w-4 h-4 text-blue-500 border-gray-300 rounded focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed';
  const labelClasses = 'ml-2 text-sm text-gray-700';
</script>

<label class="flex items-center gap-2 cursor-pointer">
  <input
    type="checkbox"
    bind:checked
    {disabled}
    {onchange}
    class={checkboxClasses}
  />
  {#if label}
    <span class={labelClasses}>{label}</span>
  {/if}
</label>
