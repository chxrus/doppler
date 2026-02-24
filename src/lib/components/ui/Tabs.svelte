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

<div class="border-b border-gray-200">
  <nav class="flex -mb-px" aria-label="Tabs">
    {#each tabs as tab}
      <button
        type="button"
        onclick={() => handleTabClick(tab.id)}
        class="px-4 py-2 text-sm font-medium border-b-2 transition-colors {activeTab === tab.id
          ? 'border-blue-500 text-blue-600'
          : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
        aria-current={activeTab === tab.id ? 'page' : undefined}
      >
        {tab.label}
      </button>
    {/each}
  </nav>
</div>
