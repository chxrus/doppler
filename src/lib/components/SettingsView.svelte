<script lang="ts">
  // Mock settings state (no backend calls)
  let apiKey = $state('');
  let opacity = $state(0.95);
  let alwaysOnTop = $state(true);
  let clickThrough = $state(false);
  let screenCaptureProtection = $state(true);
  let hotkeyToggle = $state('CommandOrControl+Shift+Space');
  let hotkeyRecord = $state('CommandOrControl+Shift+R');

  function saveApiKey() {
    // Mock save - no backend call
    console.log('API Key saved (mock):', apiKey);
  }
</script>

<div class="flex flex-col h-full overflow-y-auto bg-white">
  <div class="p-4 space-y-6">
    <!-- Gemini API Section -->
    <section class="space-y-3">
      <h2 class="text-base font-semibold text-gray-900">Gemini API</h2>
      <div class="space-y-2">
        <label for="api-key" class="block text-xs font-medium text-gray-600">
          API Key
        </label>
        <input
          id="api-key"
          type="password"
          bind:value={apiKey}
          placeholder="Enter your Gemini API key"
          class="w-full px-3 py-1.5 text-sm border border-gray-200 rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
        />
        <button
          type="button"
          onclick={saveApiKey}
          class="px-3 py-1.5 bg-blue-500 text-white rounded-md text-sm font-medium hover:bg-blue-600 transition-colors"
        >
          Save API Key
        </button>
      </div>
    </section>

    <!-- Overlay Settings Section -->
    <section class="space-y-3">
      <h2 class="text-base font-semibold text-gray-900">Overlay</h2>
      
      <!-- Opacity Slider -->
      <div class="space-y-2">
        <label for="opacity" class="block text-xs font-medium text-gray-600">
          Opacity: {Math.round(opacity * 100)}%
        </label>
        <input
          id="opacity"
          type="range"
          min="0.2"
          max="1"
          step="0.05"
          bind:value={opacity}
          class="w-full h-1.5 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-blue-500"
        />
      </div>

      <!-- Checkboxes -->
      <div class="space-y-2">
        <label class="flex items-center gap-2 cursor-pointer group">
          <input
            type="checkbox"
            bind:checked={alwaysOnTop}
            class="w-3.5 h-3.5 text-blue-500 border-gray-300 rounded focus:ring-1 focus:ring-blue-500"
          />
          <span class="text-sm text-gray-700 group-hover:text-gray-900">Always on top</span>
        </label>

        <label class="flex items-center gap-2 cursor-pointer group">
          <input
            type="checkbox"
            bind:checked={clickThrough}
            class="w-3.5 h-3.5 text-blue-500 border-gray-300 rounded focus:ring-1 focus:ring-blue-500"
          />
          <span class="text-sm text-gray-700 group-hover:text-gray-900">Click-through</span>
        </label>

        <label class="flex items-center gap-2 cursor-pointer group">
          <input
            type="checkbox"
            bind:checked={screenCaptureProtection}
            class="w-3.5 h-3.5 text-blue-500 border-gray-300 rounded focus:ring-1 focus:ring-blue-500"
          />
          <span class="text-sm text-gray-700 group-hover:text-gray-900">Screen capture protection</span>
        </label>
      </div>
    </section>

    <!-- Hotkeys Section -->
    <section class="space-y-3">
      <h2 class="text-base font-semibold text-gray-900">Hotkeys</h2>
      
      <div class="space-y-3">
        <div class="space-y-1.5">
          <div class="text-xs font-medium text-gray-600">
            Toggle Window
          </div>
          <div class="flex items-center gap-2">
            <div class="flex-1 px-3 py-1.5 border border-gray-200 rounded-md bg-gray-50">
              <div class="flex gap-1 flex-wrap">
                {#each hotkeyToggle.split('+') as key}
                  <span class="px-1.5 py-0.5 bg-white text-gray-700 rounded text-xs font-medium border border-gray-200">
                    {key}
                  </span>
                {/each}
              </div>
            </div>
            <button
              type="button"
              class="px-2.5 py-1.5 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors text-xs border border-gray-200"
            >
              Change
            </button>
          </div>
        </div>

        <div class="space-y-1.5">
          <div class="text-xs font-medium text-gray-600">
            Start/Stop Recording
          </div>
          <div class="flex items-center gap-2">
            <div class="flex-1 px-3 py-1.5 border border-gray-200 rounded-md bg-gray-50">
              <div class="flex gap-1 flex-wrap">
                {#each hotkeyRecord.split('+') as key}
                  <span class="px-1.5 py-0.5 bg-white text-gray-700 rounded text-xs font-medium border border-gray-200">
                    {key}
                  </span>
                {/each}
              </div>
            </div>
            <button
              type="button"
              class="px-2.5 py-1.5 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors text-xs border border-gray-200"
            >
              Change
            </button>
          </div>
        </div>
      </div>
    </section>
  </div>
</div>
