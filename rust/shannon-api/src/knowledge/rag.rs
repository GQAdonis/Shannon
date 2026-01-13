//! Retrieval-Augmented Generation (RAG) service.
//!
//! Orchestrates document processing, chunking, embedding, and retrieval to
//! augment LLM queries with relevant context from knowledge bases.

use crate::database::knowledge::{Chunk, ChunkWithScore, Document, KnowledgeBase};
use crate::knowledge::{
    chunking::{get_chunker, ChunkingStrategyTrait},
    embeddings::EmbeddingProvider,
    vector_store::VectorStore,
};
use anyhow::{Context, Result};
use std::sync::Arc;

/// RAG service for knowledge base operations.
pub struct RAGService {
    embedding_provider: Arc<dyn EmbeddingProvider>,
    vector_store: Arc<VectorStore>,
}

impl RAGService {
    /// Create a new RAG service.
    #[must_use]
    pub fn new(
        embedding_provider: Arc<dyn EmbeddingProvider>,
        vector_store: Arc<VectorStore>,
    ) -> Self {
        Self {
            embedding_provider,
            vector_store,
        }
    }

    /// Process a document into chunks and store them.
    ///
    /// # Errors
    ///
    /// Returns an error if chunking, embedding, or storage fails.
    pub async fn process_document(
        &self,
        document: &Document,
        knowledge_base: &KnowledgeBase,
    ) -> Result<Vec<Chunk>> {
        // 1. Chunk the document
        let chunker = get_chunker(knowledge_base.chunking_strategy)?;
        let mut chunks = chunker
            .chunk(document, &knowledge_base.chunking_config)
            .await
            .context("Failed to chunk document")?;

        // 2. Generate embeddings for all chunks
        let contents: Vec<&str> = chunks.iter().map(|c| c.content.as_str()).collect();
        let embeddings = self
            .embedding_provider
            .embed_batch(contents)
            .await
            .context("Failed to generate embeddings")?;

        // 3. Attach embeddings to chunks
        for (chunk, embedding) in chunks.iter_mut().zip(embeddings.iter()) {
            chunk.embedding = embedding.clone();
        }

        // 4. Store chunks in vector store
        self.vector_store
            .add_chunks_batch(&chunks)
            .context("Failed to store chunks")?;

        tracing::info!(
            document_id = %document.id,
            knowledge_base_id = %knowledge_base.id,
            num_chunks = chunks.len(),
            "Document processed and stored"
        );

        Ok(chunks)
    }

    /// Search for relevant chunks across knowledge bases.
    ///
    /// # Errors
    ///
    /// Returns an error if embedding or search fails.
    pub async fn search(
        &self,
        query: &str,
        knowledge_base_ids: &[String],
        limit: usize,
    ) -> Result<Vec<ChunkWithScore>> {
        // Generate query embedding
        let query_embedding = self
            .embedding_provider
            .embed(query)
            .await
            .context("Failed to generate query embedding")?;

        // Search each knowledge base
        let mut all_results = Vec::new();
        for kb_id in knowledge_base_ids {
            let results = self
                .vector_store
                .search(kb_id, &query_embedding, limit)
                .context("Failed to search knowledge base")?;
            all_results.extend(results);
        }

        // Sort by score (descending)
        all_results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Take top results
        all_results.truncate(limit);

        Ok(all_results)
    }

    /// Augment a query with relevant context from knowledge bases.
    ///
    /// Returns a formatted prompt with context injected.
    ///
    /// # Errors
    ///
    /// Returns an error if search fails.
    pub async fn augment_query(
        &self,
        query: &str,
        knowledge_base_ids: &[String],
        limit: usize,
    ) -> Result<String> {
        if knowledge_base_ids.is_empty() {
            return Ok(query.to_string());
        }

        // Search for relevant chunks
        let results = self.search(query, knowledge_base_ids, limit).await?;

        if results.is_empty() {
            tracing::debug!("No relevant context found, using original query");
            return Ok(query.to_string());
        }

        // Format context
        let context = results
            .iter()
            .enumerate()
            .map(|(i, r)| {
                format!(
                    "[Source {}] (Relevance: {:.2})\n{}",
                    i + 1,
                    r.score,
                    r.chunk.content
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");

        // Inject context into query
        let augmented = format!(
            "# Relevant Context from Knowledge Base\n\n{}\n\n---\n\n# User Query\n\n{}",
            context, query
        );

        tracing::info!(
            num_sources = results.len(),
            avg_relevance = results.iter().map(|r| r.score).sum::<f32>() / results.len() as f32,
            "Query augmented with context"
        );

        Ok(augmented)
    }

    /// Get all chunks for a knowledge base.
    ///
    /// # Errors
    ///
    /// Returns an error if retrieval fails.
    pub fn get_chunks(&self, knowledge_base_id: &str) -> Result<Vec<Chunk>> {
        self.vector_store
            .get_chunks_by_kb(knowledge_base_id)
            .context("Failed to retrieve chunks")
    }

    /// Delete all chunks for a knowledge base.
    ///
    /// # Errors
    ///
    /// Returns an error if deletion fails.
    pub fn delete_chunks(&self, knowledge_base_id: &str) -> Result<()> {
        self.vector_store
            .delete_chunks_by_kb(knowledge_base_id)
            .context("Failed to delete chunks")
    }

    /// Calculate knowledge base statistics.
    ///
    /// # Errors
    ///
    /// Returns an error if retrieval fails.
    pub fn get_stats(&self, knowledge_base_id: &str) -> Result<KnowledgeBaseStats> {
        let chunks = self.get_chunks(knowledge_base_id)?;

        let total_tokens: usize = chunks.iter().map(|c| c.tokens).sum();
        let avg_tokens = if !chunks.is_empty() {
            total_tokens / chunks.len()
        } else {
            0
        };

        let unique_documents: std::collections::HashSet<_> =
            chunks.iter().map(|c| &c.document_id).collect();

        Ok(KnowledgeBaseStats {
            total_chunks: chunks.len(),
            total_tokens,
            avg_tokens_per_chunk: avg_tokens,
            num_documents: unique_documents.len(),
        })
    }
}

/// Knowledge base statistics.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct KnowledgeBaseStats {
    /// Total number of chunks.
    pub total_chunks: usize,
    /// Total token count across all chunks.
    pub total_tokens: usize,
    /// Average tokens per chunk.
    pub avg_tokens_per_chunk: usize,
    /// Number of unique documents.
    pub num_documents: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::knowledge::{
        ChunkingConfig, ChunkingStrategy, EmbeddingProvider as EmbeddingProviderEnum, ProcessorType,
    };
    use crate::knowledge::embeddings::OpenAIEmbeddings;
    use chrono::Utc;
    use tempfile::NamedTempFile;

    fn create_test_knowledge_base() -> KnowledgeBase {
        KnowledgeBase {
            id: "kb1".to_string(),
            user_id: "test_user".to_string(),
            name: "Test KB".to_string(),
            description: Some("Test knowledge base".to_string()),
            chunking_strategy: ChunkingStrategy::Semantic,
            chunking_config: ChunkingConfig::default(),
            embedding_provider: EmbeddingProviderEnum::OpenAI,
            embedding_model: "text-embedding-3-small".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_test_document() -> Document {
        Document {
            id: "doc1".to_string(),
            user_id: "test_user".to_string(),
            knowledge_base_id: "kb1".to_string(),
            title: "Test Document".to_string(),
            content: "This is a test document. It has multiple sentences. Each sentence provides context. The content is useful for testing RAG.".to_string(),
            file_path: None,
            file_type: "text/plain".to_string(),
            file_size: 100,
            processor: ProcessorType::Native,
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    #[ignore] // Requires OpenAI API key
    async fn test_rag_service_integration() -> Result<()> {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY required");
        let embeddings = Arc::new(OpenAIEmbeddings::new_default(api_key)?);

        let temp_file = NamedTempFile::new()?;
        let vector_store = Arc::new(VectorStore::new(temp_file.path(), embeddings.dimension())?);

        let rag = RAGService::new(embeddings, vector_store);

        // Process document
        let kb = create_test_knowledge_base();
        let doc = create_test_document();

        let chunks = rag.process_document(&doc, &kb).await?;
        assert!(!chunks.is_empty());

        // Search
        let results = rag.search("test document", &[kb.id.clone()], 5).await?;
        assert!(!results.is_empty());

        // Augment query
        let augmented = rag
            .augment_query("What is this about?", &[kb.id.clone()], 3)
            .await?;
        assert!(augmented.contains("Relevant Context"));

        // Stats
        let stats = rag.get_stats(&kb.id)?;
        assert!(stats.total_chunks > 0);
        assert!(stats.total_tokens > 0);

        Ok(())
    }

    #[test]
    fn test_empty_query_augmentation() {
        let temp_file = NamedTempFile::new().unwrap();
        let embeddings = Arc::new(OpenAIEmbeddings::new_default("dummy-key".to_string()).unwrap());
        let vector_store = Arc::new(VectorStore::new(temp_file.path(), 1536).unwrap());
        let rag = RAGService::new(embeddings, vector_store);

        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result = runtime.block_on(rag.augment_query("test query", &[], 5));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test query");
    }
}
