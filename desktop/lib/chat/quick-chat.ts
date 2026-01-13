import { invoke } from '@tauri-apps/api/core';
import type { ChatMessage, QuickChatConfig } from './types';
import { kbService } from '../knowledge/kb-service';
import type { SearchResult } from '../knowledge/types';

/**
 * Service for Quick Chat mode - fast, streaming responses with RAG integration
 */
export class QuickChatService {
  /**
   * Send a message in Quick Chat mode and stream the response
   *
   * CRITICAL: Integrates RAG by searching attached knowledge bases
   * and augmenting the prompt with relevant context before sending to LLM
   *
   * @param message - User message
   * @param history - Previous conversation history
   * @param config - Quick chat configuration
   * @param conversationId - Conversation ID for KB attachment lookup
   * @returns AsyncGenerator yielding response chunks
   */
  async sendMessage(
    message: string,
    history: ChatMessage[],
    config: QuickChatConfig,
    conversationId?: string
  ): Promise<AsyncGenerator<string>> {
    try {
      let finalMessage = message;

      // CRITICAL: RAG Integration - augment prompt with KB context
      if (conversationId) {
        try {
          const kbs = await kbService.getConversationKnowledgeBases(conversationId);

          if (kbs.length > 0) {
            const kbIds = kbs.map(kb => kb.id);

            // Search across all attached knowledge bases
            const searchResults = await kbService.searchMultiple(kbIds, message, 5);

            if (searchResults.length > 0) {
              // Format context from search results
              const context = searchResults
                .map((r: SearchResult, i: number) =>
                  `[Source ${i + 1}: ${r.documentTitle} (relevance: ${(r.score * 100).toFixed(1)}%)]\n${r.content}`
                )
                .join('\n\n');

              // Augment prompt with KB context
              finalMessage = `Context from knowledge base:\n\n${context}\n\n---\n\nUser query: ${message}`;

              console.log('[RAG] Augmented prompt with', searchResults.length, 'KB chunks');
            }
          }
        } catch (kbError) {
          console.warn('[RAG] Failed to search knowledge bases:', kbError);
          // Continue without KB augmentation on error
        }
      }

      const response = await invoke<string[]>('quick_chat', {
        message: finalMessage,
        history,
        config,
      });

      // Return async generator for streaming
      async function* streamChunks() {
        for (const chunk of response) {
          yield chunk;
        }
      }

      return streamChunks();
    } catch (error) {
      console.error('Quick chat error:', error);
      throw new Error(`Quick chat failed: ${error}`);
    }
  }

  /**
   * Send a message and get complete response (non-streaming)
   *
   * CRITICAL: Also integrates RAG via sendMessage
   *
   * @param message - User message
   * @param history - Previous conversation history
   * @param config - Quick chat configuration
   * @param conversationId - Conversation ID for KB attachment lookup
   * @returns Complete response text
   */
  async sendMessageSync(
    message: string,
    history: ChatMessage[],
    config: QuickChatConfig,
    conversationId?: string
  ): Promise<string> {
    const stream = await this.sendMessage(message, history, config, conversationId);
    let fullResponse = '';

    for await (const chunk of stream) {
      fullResponse += chunk;
    }

    return fullResponse;
  }
}
