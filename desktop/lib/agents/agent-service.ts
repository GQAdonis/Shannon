/**
 * Agent management service for Shannon Desktop.
 *
 * Provides methods to create, read, update, and delete agent specifications
 * using the Tauri backend.
 */

import { invoke } from '@tauri-apps/api/core';
import type { AgentSpec } from './types';
import { kbService } from '../knowledge/kb-service';
import type { SearchResult } from '../knowledge/types';

/**
 * Service for managing AI agent specifications.
 */
export class AgentService {
  /**
   * Create a new agent.
   *
   * @param spec - Agent specification to create
   * @returns Promise resolving to the created agent ID
   */
  async create(spec: AgentSpec): Promise<string> {
    return await invoke<string>('create_agent', { spec });
  }

  /**
   * Get an agent by ID.
   *
   * @param id - Agent ID
   * @returns Promise resolving to the agent specification
   * @throws Error if agent is not found
   */
  async get(id: string): Promise<AgentSpec> {
    return await invoke<AgentSpec>('get_agent', { id });
  }

  /**
   * List all agents with optional filtering.
   *
   * @param options - Filter options
   * @returns Promise resolving to array of agent specifications
   */
  async list(options?: {
    category?: string;
    tags?: string[];
    search?: string;
  }): Promise<AgentSpec[]> {
    return await invoke<AgentSpec[]>('list_agents', {
      category: options?.category,
      tags: options?.tags,
      search: options?.search,
    });
  }

  /**
   * Update an existing agent.
   *
   * @param id - Agent ID to update
   * @param spec - Updated agent specification
   * @returns Promise resolving when update is complete
   */
  async update(id: string, spec: AgentSpec): Promise<void> {
    await invoke('update_agent', { id, spec });
  }

  /**
   * Delete an agent.
   *
   * @param id - Agent ID to delete
   * @returns Promise resolving to true if deleted, false if not found
   */
  async delete(id: string): Promise<boolean> {
    return await invoke<boolean>('delete_agent', { id });
  }

  /**
   * Export an agent to YAML format.
   *
   * @param id - Agent ID to export
   * @returns Promise resolving to YAML string
   */
  async export(id: string): Promise<string> {
    return await invoke<string>('export_agent', { id });
  }

  /**
   * Import an agent from YAML format.
   *
   * @param yaml - YAML string to import
   * @returns Promise resolving to the imported agent ID
   */
  async import(yaml: string): Promise<string> {
    return await invoke<string>('import_agent', { yaml });
  }

  /**
   * Execute an agent with a message, with RAG integration
   *
   * CRITICAL: If the agent has knowledge bases attached, this searches them
   * and augments the prompt with relevant context before execution
   *
   * @param agentId - Agent ID to execute
   * @param message - User message
   * @param conversationId - Conversation ID for tracking
   * @returns Promise resolving to response
   */
  async executeAgent(
    agentId: string,
    message: string,
    conversationId: string
  ): Promise<string> {
    // Load agent specification
    const agent = await this.get(agentId);

    let augmentedMessage = message;

    // CRITICAL: RAG Integration - search agent's knowledge bases
    if (agent.knowledgeBases && agent.knowledgeBases.length > 0) {
      try {
        // Search across agent's knowledge bases
        const searchResults = await kbService.searchMultiple(
          agent.knowledgeBases,
          message,
          5
        );

        if (searchResults.length > 0) {
          // Format KB context
          const context = searchResults
            .map((r: SearchResult) =>
              `${r.documentTitle}: ${r.content}`
            )
            .join('\n\n');

          // Augment with agent's system prompt and KB context
          augmentedMessage = `${agent.systemPrompt}\n\nKnowledge Base Context:\n${context}\n\nUser: ${message}`;

          console.log('[RAG] Agent', agentId, 'augmented with', searchResults.length, 'KB chunks');
        }
      } catch (kbError) {
        console.warn('[RAG] Failed to search agent knowledge bases:', kbError);
        // Continue without KB augmentation on error
        augmentedMessage = `${agent.systemPrompt}\n\nUser: ${message}`;
      }
    } else {
      // No KBs, just use system prompt
      augmentedMessage = `${agent.systemPrompt}\n\nUser: ${message}`;
    }

    // Execute agent with augmented message
    return await invoke<string>('execute_agent', {
      agentId,
      message: augmentedMessage,
      conversationId,
    });
  }

  /**
   * Create a default agent specification template.
   *
   * @param overrides - Optional overrides for default values
   * @returns New agent specification with defaults
   */
  createDefault(overrides?: Partial<AgentSpec>): AgentSpec {
    const now = new Date().toISOString();
    const id = crypto.randomUUID();

    return {
      id,
      name: 'New Agent',
      description: 'A new AI agent',
      version: '1.0.0',
      author: undefined,
      systemPrompt: 'You are a helpful AI assistant.',
      model: {
        provider: 'openai',
        name: 'gpt-4',
        temperature: 0.7,
        maxTokens: 2000,
      },
      tools: [],
      knowledgeBases: [],
      allowedActions: [],
      strategy: undefined,
      conversationStyle: undefined,
      tags: [],
      category: 'general',
      icon: '🤖',
      createdAt: now,
      updatedAt: now,
      ...overrides,
    };
  }
}

/**
 * Singleton instance of the agent service.
 */
export const agentService = new AgentService();
