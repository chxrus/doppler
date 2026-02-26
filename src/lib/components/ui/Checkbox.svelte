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

  const checkboxClasses = 'w-4 h-4 text-teal-400 border-slate-500 rounded-md bg-slate-900/70 focus:ring-2 focus:ring-teal-300/70 disabled:opacity-50 disabled:cursor-not-allowed';
  const labelClasses = 'ml-1 text-sm text-slate-200';
</script>

<label class="flex items-center gap-2 cursor-pointer rounded-lg px-2 py-1 hover:bg-slate-900/45">
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
