/**
 * Knowledge Base Types
 *
 * Type definitions for the RAG knowledge base system
 */

export interface KnowledgeBase {
  id: string;
  name: string;
  description: string;
  chunkingStrategy: ChunkingStrategy;
  chunkingConfig: ChunkingConfig;
  embeddingProvider: 'openai' | 'local';
  embeddingModel: string;
  documentCount: number;
  totalChunks: number;
  createdAt: string;
  updatedAt: string;
}

export type ChunkingStrategy =
  | 'fixed_size'
  | 'semantic'          // DEFAULT
  | 'structure_aware'
  | 'hierarchical';

export interface ChunkingConfig {
  strategy: ChunkingStrategy;
  // Fixed size config
  chunkSize?: number;
  chunkOverlap?: number;
  // Semantic config
  semanticThreshold?: number;
  // Structure-aware config
  respectHeaders?: boolean;
  respectCodeBlocks?: boolean;
  // Hierarchical config
  levels?: number;
}

export interface Document {
  id: string;
  knowledgeBaseId: string;
  title: string;
  fileType: string;
  fileSize: number;
  processor: ProcessorType;
  chunkCount: number;
  status: 'processing' | 'completed' | 'failed';
  createdAt: string;
  error?: string;
}

export type ProcessorType =
  | 'mistral'
  | 'unstructured_hosted'
  | 'unstructured_self_hosted'
  | 'native';

export interface SearchResult {
  id: string;
  documentId: string;
  documentTitle: string;
  content: string;
  score: number;
  metadata: Record<string, unknown>;
}

export interface ProcessorConfig {
  processorType: ProcessorType;
  enabled: boolean;
  apiKey?: string;
  apiUrl?: string;
  supportedFileTypes: string[];
}

export interface UploadDocumentRequest {
  knowledgeBaseId: string;
  fileName: string;
  fileContent: string; // base64
  processor: ProcessorType;
}

export interface CreateKnowledgeBaseRequest {
  name: string;
  description: string;
  chunkingStrategy: ChunkingStrategy;
  chunkingConfig: ChunkingConfig;
  embeddingProvider: 'openai' | 'local';
  embeddingModel: string;
}
