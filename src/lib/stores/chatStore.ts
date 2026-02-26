import { writable, derived } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import { sendMessageStream as sendMessageCommand } from '$lib/tauri/commands';

export interface Exchange {
  id: number;
  question: string;
  answer: string;
  isPending?: boolean;
}

interface AssistantStreamChunkPayload {
  request_id: number;
  chunk: string;
}

function createChatStore() {
  const exchanges = writable<Exchange[]>([]);
  const isLoading = writable(false);
  const error = writable<string | null>(null);
  const inputDraft = writable('');
  let nextExchangeId = 1;
  let pendingRequestCount = 0;

  return {
    exchanges,
    isLoading,
    error,
    inputDraft,

    async sendMessage(message: string): Promise<void> {
      const trimmedMessage = message.trim();
      if (trimmedMessage === '') {
        return;
      }

      const newExchange: Exchange = {
        id: nextExchangeId++,
        question: trimmedMessage,
        answer: '',
        isPending: true
      };

      exchanges.update(items => [...items, newExchange]);
      pendingRequestCount += 1;
      isLoading.set(pendingRequestCount > 0);
      error.set(null);

      let unlisten: (() => void) | null = null;

      try {
        unlisten = await listen<AssistantStreamChunkPayload>(
          'assistant-stream-chunk',
          (event) => {
            if (event.payload.request_id !== newExchange.id) {
              return;
            }

            exchanges.update((items) =>
              items.map((item) =>
                item.id === newExchange.id
                  ? {
                      ...item,
                      answer: `${item.answer}${event.payload.chunk}`
                    }
                  : item
              )
            );
          }
        );

        const response = await sendMessageCommand(trimmedMessage, newExchange.id);

        exchanges.update(items =>
          items.map(item =>
            item.id === newExchange.id
              ? {
                  ...item,
                  answer: response,
                  isPending: false
                }
              : item
          )
        );
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : String(err);
        error.set(errorMessage);

        exchanges.update(items =>
          items.map(item =>
            item.id === newExchange.id
              ? {
                  ...item,
                  answer: `Error: ${errorMessage}`,
                  isPending: false
                }
              : item
          )
        );
      } finally {
        unlisten?.();
        pendingRequestCount = Math.max(0, pendingRequestCount - 1);
        isLoading.set(pendingRequestCount > 0);
      }
    },

    clearError(): void {
      error.set(null);
    },

    clearExchanges(): void {
      exchanges.set([]);
    },

    setInputDraft(value: string): void {
      inputDraft.set(value);
    },

    clearSession(): void {
      pendingRequestCount = 0;
      exchanges.set([]);
      inputDraft.set('');
      error.set(null);
      isLoading.set(false);
    }
  };
}

export const chatStore = createChatStore();
