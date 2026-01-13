export interface Citation {
  url: string;
  title?: string;
  source?: string;
  source_type?: string;
  retrieved_at?: string;
  published_date?: string;
  credibility_score?: number;
}

/**
 * Knowledge base citation from RAG search results.
 * Corresponds to SearchResultWithMetadata from the Rust backend.
 */
export interface KnowledgeCitation {
  /** Citation index in the results list */
  index: number;
  /** Chunk ID */
  chunk_id: string;
  /** Document ID */
  document_id: string;
  /** Document title */
  document_title: string;
  /** Chunk content */
  content: string;
  /** Relevance score (0.0 to 1.0) */
  relevance_score: number;
  /** Token count */
  tokens: number;
  /** Additional metadata (e.g., page, section) */
  metadata: {
    page?: number | string;
    section?: string;
    [key: string]: unknown;
  };
}

