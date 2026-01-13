import { invoke } from '@tauri-apps/api/core';
import type { TaskChatConfig, TaskHandle, TaskEvent } from './types';
import { kbService } from '../knowledge/kb-service';
import type { SearchResult } from '../knowledge/types';

/**
 * Service for Task Chat mode - orchestrated, multi-agent tasks with RAG integration
 */
export class TaskChatService {
  /**
   * Submit a task for execution
   *
   * CRITICAL: Integrates RAG by searching attached knowledge bases
   * and adding relevant context before task submission
   *
   * @param query - User task query
   * @param context - Additional context strings
   * @param config - Task chat configuration
   * @param conversationId - Conversation ID for KB attachment lookup
   * @returns Task handle with ID and initial status
   */
  async submitTask(
    query: string,
    context: string[],
    config: TaskChatConfig,
    conversationId?: string
  ): Promise<TaskHandle> {
    try {
      const augmentedContext = [...context];

      // CRITICAL: RAG Integration - search KBs and add to context
      if (conversationId) {
        try {
          const kbs = await kbService.getConversationKnowledgeBases(conversationId);

          if (kbs.length > 0) {
            const kbIds = kbs.map(kb => kb.id);

            // Search across all attached knowledge bases
            const searchResults = await kbService.searchMultiple(kbIds, query, 5);

            if (searchResults.length > 0) {
              // Format KB context
              const kbContext = searchResults
                .map((r: SearchResult) =>
                  `${r.documentTitle}: ${r.content}`
                )
                .join('\n\n');

              // Add KB context to task context
              augmentedContext.push(`Knowledge Base Context:\n${kbContext}`);

              console.log('[RAG] Added', searchResults.length, 'KB chunks to task context');
            }
          }
        } catch (kbError) {
          console.warn('[RAG] Failed to search knowledge bases:', kbError);
          // Continue without KB augmentation on error
        }
      }

      return await invoke<TaskHandle>('submit_task_chat', {
        query,
        context: augmentedContext,
        config,
      });
    } catch (error) {
      console.error('Task submission error:', error);
      throw new Error(`Failed to submit task: ${error}`);
    }
  }

  /**
   * Get current status of a task
   *
   * @param taskId - Task identifier
   * @returns Current task status
   */
  async getTaskStatus(taskId: string): Promise<TaskHandle> {
    try {
      return await invoke<TaskHandle>('get_task_status', { taskId });
    } catch (error) {
      console.error('Get task status error:', error);
      throw new Error(`Failed to get task status: ${error}`);
    }
  }

  /**
   * Cancel a running task
   *
   * @param taskId - Task identifier
   * @returns Success boolean
   */
  async cancelTask(taskId: string): Promise<boolean> {
    try {
      return await invoke<boolean>('cancel_task', { taskId });
    } catch (error) {
      console.error('Cancel task error:', error);
      throw new Error(`Failed to cancel task: ${error}`);
    }
  }

  /**
   * Stream task updates (polls for status changes)
   * In future phases, this will use SSE or WebSocket
   *
   * @param taskId - Task identifier
   * @param pollInterval - Polling interval in milliseconds (default 1000)
   * @returns AsyncGenerator yielding task events
   */
  async* streamUpdates(taskId: string, pollInterval = 1000): AsyncGenerator<TaskEvent> {
    let lastProgress = -1;
    let lastState: TaskHandle['state'] = 'pending';

    while (true) {
      try {
        const status = await this.getTaskStatus(taskId);

        // Emit state change event if state changed
        if (status.state !== lastState) {
          yield {
            type: 'state_changed',
            taskId,
            data: {
              state: status.state,
              message: status.message,
            },
          };
          lastState = status.state;
        }

        // Emit progress event if progress changed
        if (status.progress !== lastProgress) {
          yield {
            type: 'progress',
            taskId,
            data: {
              progress: status.progress,
              message: status.message,
            },
          };
          lastProgress = status.progress;
        }

        // Emit completion or failure events
        if (status.state === 'completed') {
          yield {
            type: 'completed',
            taskId,
            data: {
              message: status.message,
              progress: 100,
            },
          };
          break;
        }

        if (status.state === 'failed') {
          yield {
            type: 'failed',
            taskId,
            data: {
              error: status.message || 'Task failed',
            },
          };
          break;
        }

        // Wait before next poll
        await new Promise(resolve => setTimeout(resolve, pollInterval));
      } catch (error) {
        console.error('Stream update error:', error);
        yield {
          type: 'failed',
          taskId,
          data: {
            error: `Stream error: ${error}`,
          },
        };
        break;
      }
    }
  }

  /**
   * Wait for task completion
   *
   * @param taskId - Task identifier
   * @param timeout - Maximum wait time in milliseconds (default 300000 = 5 minutes)
   * @returns Final task handle
   */
  async waitForCompletion(taskId: string, timeout = 300000): Promise<TaskHandle> {
    const startTime = Date.now();

    while (Date.now() - startTime < timeout) {
      const status = await this.getTaskStatus(taskId);

      if (status.state === 'completed' || status.state === 'failed') {
        return status;
      }

      await new Promise(resolve => setTimeout(resolve, 1000));
    }

    throw new Error(`Task ${taskId} timed out after ${timeout}ms`);
  }
}
