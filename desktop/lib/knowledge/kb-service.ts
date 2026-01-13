/**
 * Knowledge Base Service
 *
 * Frontend service for knowledge base operations, communicating with Tauri backend
 */

import { invoke } from '@tauri-apps/api/core';
import type {
  KnowledgeBase,
  Document,
  SearchResult,
  ProcessorConfig,
  CreateKnowledgeBaseRequest,
  ProcessorType,
} from './types';

export class KnowledgeBaseService {
  /**
   * Create a new knowledge base
   */
  async create(request: CreateKnowledgeBaseRequest): Promise<string> {
    return await invoke('create_knowledge_base', { request });
  }

  /**
   * Get all knowledge bases
   */
  async list(): Promise<KnowledgeBase[]> {
    return await invoke('list_knowledge_bases');
  }

  /**
   * Get a specific knowledge base by ID
   */
  async get(id: string): Promise<KnowledgeBase> {
    return await invoke('get_knowledge_base', { id });
  }

  /**
   * Update a knowledge base
   */
  async update(id: string, updates: Partial<KnowledgeBase>): Promise<void> {
    await invoke('update_knowledge_base', { id, updates });
  }

  /**
   * Delete a knowledge base
   */
  async delete(id: string): Promise<void> {
    await invoke('delete_knowledge_base', { id });
  }

  /**
   * Upload and process a document
   */
  async uploadDocument(
    kbId: string,
    file: File,
    processor: ProcessorType
  ): Promise<string> {
    // Read file as base64
    const content = await this.fileToBase64(file);

    return await invoke('upload_document', {
      knowledgeBaseId: kbId,
      fileName: file.name,
      fileContent: content,
      processor,
    });
  }

  /**
   * Get all documents in a knowledge base
   */
  async listDocuments(kbId: string): Promise<Document[]> {
    return await invoke('list_documents', { knowledgeBaseId: kbId });
  }

  /**
   * Delete a document
   */
  async deleteDocument(documentId: string): Promise<void> {
    await invoke('delete_document', { documentId });
  }

  /**
   * Search within a single knowledge base
   */
  async search(
    kbId: string,
    query: string,
    limit: number = 5
  ): Promise<SearchResult[]> {
    return await invoke('search_knowledge_base', { kbId, query, limit });
  }

  /**
   * Search across multiple knowledge bases
   */
  async searchMultiple(
    kbIds: string[],
    query: string,
    limit: number = 5
  ): Promise<SearchResult[]> {
    return await invoke('search_multiple_kbs', { kbIds, query, limit });
  }

  /**
   * Attach knowledge bases to an agent
   */
  async attachToAgent(agentId: string, kbIds: string[]): Promise<void> {
    await invoke('attach_knowledge_bases_to_agent', { agentId, kbIds });
  }

  /**
   * Attach knowledge bases to a conversation
   */
  async attachToConversation(
    conversationId: string,
    kbIds: string[]
  ): Promise<void> {
    await invoke('attach_knowledge_bases_to_conversation', {
      conversationId,
      kbIds,
    });
  }

  /**
   * Get knowledge bases attached to an agent
   */
  async getAgentKnowledgeBases(agentId: string): Promise<KnowledgeBase[]> {
    return await invoke('get_agent_knowledge_bases', { agentId });
  }

  /**
   * Get knowledge bases attached to a conversation
   */
  async getConversationKnowledgeBases(
    conversationId: string
  ): Promise<KnowledgeBase[]> {
    return await invoke('get_conversation_knowledge_bases', {
      conversationId,
    });
  }

  /**
   * Get processor configurations
   */
  async getProcessorConfigs(): Promise<ProcessorConfig[]> {
    return await invoke('get_processor_configs');
  }

  /**
   * Update processor configuration
   */
  async updateProcessorConfig(config: ProcessorConfig): Promise<void> {
    await invoke('update_processor_config', { config });
  }

  /**
   * Helper: Convert File to base64
   */
  private fileToBase64(file: File): Promise<string> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => {
        const result = reader.result as string;
        // Remove data:mime/type;base64, prefix
        const base64 = result.split(',')[1];
        resolve(base64);
      };
      reader.onerror = reject;
      reader.readAsDataURL(file);
    });
  }
}

// Singleton instance
export const kbService = new KnowledgeBaseService();
