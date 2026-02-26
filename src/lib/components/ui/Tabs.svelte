<script lang="ts">
  interface Tab {
    id: string;
    label: string;
  }

  interface Props {
    tabs: Tab[];
    activeTab?: string;
    onTabChange?: (tabId: string) => void;
  }

  let { tabs, activeTab = $bindable(tabs[0]?.id), onTabChange }: Props = $props();

  function handleTabClick(tabId: string) {
    activeTab = tabId;
    onTabChange?.(tabId);
  }
</script>

<div class="rounded-xl border p-1.5 shadow-sm"
  style="border-color: rgba(148, 163, 184, var(--doppler-border-alpha, 0.65)); background: rgb(var(--doppler-control-rgb, 15 23 42) / var(--doppler-control-alpha, 0.62));">
  <nav class="grid grid-cols-4 gap-1" aria-label="Tabs">
    {#each tabs as tab}
      <button
        type="button"
        onclick={() => handleTabClick(tab.id)}
        class="px-2.5 py-1.5 text-xs font-semibold rounded-lg transition {activeTab === tab.id
          ? 'bg-teal-500/22 text-teal-100 shadow-sm border border-teal-300/45'
          : 'text-slate-300 hover:bg-slate-900/70'}"
        aria-current={activeTab === tab.id ? 'page' : undefined}
      >
        {tab.label}
      </button>
    {/each}
  </nav>
</div>
