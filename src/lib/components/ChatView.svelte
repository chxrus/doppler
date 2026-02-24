<script lang="ts">
  interface Message {
    role: 'user' | 'assistant';
    content: string;
  }

  let messages = $state<Message[]>([
    { role: 'user', content: 'Hello! Can you help me with something?' },
    { role: 'assistant', content: 'Of course! I\'d be happy to help. What do you need assistance with?' },
    { role: 'user', content: 'I need to understand how this overlay chat works.' },
    { role: 'assistant', content: 'This is an overlay chat interface that allows you to interact with an LLM through text and voice. You can type messages or use the Record button to speak your questions.' }
  ]);

  let input = $state('');
  let isRecording = $state(false);

  function sendMessage() {
    if (input.trim() == null || input.trim() === '') return;

    const userMessage: Message = { role: 'user', content: input };
    messages = [...messages, userMessage];

    const messageText = input;
    input = '';

    setTimeout(() => {
      const assistantMessage: Message = {
        role: 'assistant',
        content: `You said: "${messageText}". This is a mock response.`
      };
      messages = [...messages, assistantMessage];
    }, 500);
  }

  function handleKeyPress(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      sendMessage();
    }
  }

  function toggleRecording() {
    isRecording = !isRecording;
  }
</script>

<div class="flex flex-col h-screen bg-white">
  <!-- Messages Area -->
  <div class="flex-1 overflow-y-auto p-4 space-y-4">
    {#each messages as message}
      <div class="flex {message.role === 'user' ? 'justify-end' : 'justify-start'}">
        <div
          class="max-w-[80%] px-4 py-3 rounded-lg {message.role === 'user'
            ? 'bg-blue-500 text-white rounded-br-none'
            : 'bg-gray-200 text-gray-900 rounded-bl-none'}"
        >
          <p class="whitespace-pre-wrap break-words">{message.content}</p>
        </div>
      </div>
    {/each}
  </div>

  <!-- Input Area -->
  <div class="border-t border-gray-200 p-4 bg-gray-50">
    <div class="flex gap-2">
      <button
        onclick={toggleRecording}
        class="px-4 py-2 rounded-lg font-medium transition-colors {isRecording
          ? 'bg-red-500 hover:bg-red-600 text-white'
          : 'bg-gray-500 hover:bg-gray-600 text-white'}"
        type="button"
      >
        {isRecording ? '⏹ Stop' : '🎤 Record'}
      </button>

      <input
        type="text"
        bind:value={input}
        onkeypress={handleKeyPress}
        placeholder="Type a message..."
        class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      />

      <button
        onclick={sendMessage}
        disabled={input.trim() === ''}
        class="px-6 py-2 bg-blue-500 text-white rounded-lg font-medium hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        type="button"
      >
        Send
      </button>
    </div>
  </div>
</div>
