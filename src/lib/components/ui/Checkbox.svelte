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

  const checkboxClasses = 'w-4 h-4 text-sky-500 border-slate-300 rounded-md focus:ring-2 focus:ring-sky-300 disabled:opacity-50 disabled:cursor-not-allowed';
  const labelClasses = 'ml-1 text-sm text-slate-700';
</script>

<label class="flex items-center gap-2 cursor-pointer rounded-lg px-2 py-1 hover:bg-white/55">
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
