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

<div class="rounded-xl border border-slate-200 bg-white p-1.5 shadow-sm">
  <nav class="grid grid-cols-4 gap-1" aria-label="Tabs">
    {#each tabs as tab}
      <button
        type="button"
        onclick={() => handleTabClick(tab.id)}
        class="px-2.5 py-1.5 text-xs font-semibold rounded-lg transition {activeTab === tab.id
          ? 'bg-slate-100 text-slate-900 shadow-sm'
          : 'text-slate-600 hover:bg-slate-100/85'}"
        aria-current={activeTab === tab.id ? 'page' : undefined}
      >
        {tab.label}
      </button>
    {/each}
  </nav>
</div>
