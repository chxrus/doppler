<script lang="ts">
  import Button from './ui/Button.svelte';
  import Input from './ui/Input.svelte';
  import ErrorMessage from './ui/ErrorMessage.svelte';
  import Spinner from './ui/Spinner.svelte';

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
  let isLoading = $state(false);
  let errorMessage = $state<string | null>('Failed to send message. Please check your API key and try again.');

  function sendMessage() {
    if (input.trim() == null || input.trim() === '') return;

    const userMessage: Message = { role: 'user', content: input };
    messages = [...messages, userMessage];

    const messageText = input;
    input = '';
    isLoading = true;

    setTimeout(() => {
      const assistantMessage: Message = {
        role: 'assistant',
        content: `You said: "${messageText}". This is a mock response.`
      };
      messages = [...messages, assistantMessage];
      isLoading = false;
    }, 1500);
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

  function dismissError() {
    errorMessage = null;
  }
</script>

<div class="flex flex-col h-full bg-white">
  <!-- Error Message -->
  {#if errorMessage != null}
    <div class="p-3 pb-0">
      <ErrorMessage message={errorMessage} onDismiss={dismissError} />
    </div>
  {/if}

  <!-- Messages Area -->
  <div class="flex-1 overflow-y-auto p-3 space-y-2">
    {#each messages as message}
      <div class="flex {message.role === 'user' ? 'justify-end' : 'justify-start'}">
        <div
          class="max-w-[80%] px-3 py-2 rounded-lg text-sm select-text {message.role === 'user'
            ? 'bg-blue-500 text-white rounded-br-sm'
            : 'bg-gray-100 text-gray-900 rounded-bl-sm border border-gray-200'}"
        >
          <p class="whitespace-pre-wrap break-words select-text">{message.content}</p>
        </div>
      </div>
    {/each}

    <!-- Loading Indicator in Message List -->
    {#if isLoading}
      <div class="flex justify-start">
        <div class="max-w-[80%] px-3 py-2 rounded-lg text-sm bg-gray-100 border border-gray-200 rounded-bl-sm">
          <Spinner size="sm" />
        </div>
      </div>
    {/if}
  </div>

  <!-- Input Area -->
  <div class="border-t border-gray-200 p-3 bg-white select-none">
    <div class="flex gap-2 items-center">
      <Button
        variant={isRecording ? 'danger' : 'secondary'}
        size="sm"
        onclick={toggleRecording}
      >
        {isRecording ? '⏹' : '🎤'}
      </Button>

      <div class="flex-1">
        <Input
          type="text"
          bind:value={input}
          onkeypress={handleKeyPress}
          placeholder="Type a message..."
        />
      </div>

      <Button
        variant="primary"
        size="sm"
        onclick={sendMessage}
        disabled={input.trim() === ''}
        loading={isLoading}
      >
        Send
      </Button>
    </div>
  </div>
</div>
