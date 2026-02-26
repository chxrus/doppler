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
        class="tab-button px-2.5 py-1.5 text-xs font-semibold rounded-lg transition {activeTab === tab.id
          ? 'is-active shadow-sm border'
          : ''}"
        aria-current={activeTab === tab.id ? 'page' : undefined}
      >
        {tab.label}
      </button>
    {/each}
  </nav>
</div>

<style>
  .tab-button {
    color: rgb(var(--doppler-tab-inactive-text-rgb, 203 213 225));
  }

  .tab-button:hover {
    background: rgb(var(--doppler-tab-hover-rgb, 30 41 59) / var(--doppler-tab-hover-alpha, 0.62));
  }

  .tab-button.is-active {
    border-color: rgb(var(--doppler-accent-rgb, 20 184 166) / 0.45);
    background: rgb(var(--doppler-accent-rgb, 20 184 166) / var(--doppler-tab-active-bg-alpha, 0.22));
    color: rgb(var(--doppler-tab-active-text-rgb, 204 251 241));
  }
</style>
