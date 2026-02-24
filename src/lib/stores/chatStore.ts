import { writable, derived } from 'svelte/store';
import { sendMessage as sendMessageCommand } from '$lib/tauri/commands';

export interface Exchange {
  id: number;
  question: string;
  answer: string;
  isPending?: boolean;
}

function createChatStore() {
  const exchanges = writable<Exchange[]>([]);
  const isLoading = writable(false);
  const error = writable<string | null>(null);
  let nextExchangeId = 1;
  let pendingRequestCount = 0;

  return {
    exchanges,
    isLoading,
    error,

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

      try {
        const response = await sendMessageCommand(trimmedMessage);

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
        pendingRequestCount = Math.max(0, pendingRequestCount - 1);
        isLoading.set(pendingRequestCount > 0);
      }
    },

    clearError(): void {
      error.set(null);
    },

    clearExchanges(): void {
      exchanges.set([]);
    }
  };
}

export const chatStore = createChatStore();
