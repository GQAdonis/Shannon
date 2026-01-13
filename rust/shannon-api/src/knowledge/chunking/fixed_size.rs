//! Fixed-size chunking with configurable overlap.
//!
//! This strategy splits text into chunks of a fixed token size with overlap,
//! providing a baseline chunking approach. Default: 768 tokens with 15% overlap.

use super::{create_chunk, ChunkingStrategyTrait, Tokenizer};
use crate::database::knowledge::{Chunk, ChunkingConfig, Document};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;

/// Fixed-size chunking strategy.
pub struct FixedSizeChunker {
    tokenizer: Arc<dyn Tokenizer>,
}

impl FixedSizeChunker {
    /// Create a new fixed-size chunker.
    #[must_use]
    pub fn new(tokenizer: Arc<dyn Tokenizer>) -> Self {
        Self { tokenizer }
    }
}

#[async_trait]
impl ChunkingStrategyTrait for FixedSizeChunker {
    async fn chunk(&self, document: &Document, config: &ChunkingConfig) -> Result<Vec<Chunk>> {
        let chunk_size = config.chunk_size.unwrap_or(768);
        let overlap_percent = config.overlap_percent.unwrap_or(0.15);
        let overlap_tokens = (chunk_size as f32 * overlap_percent) as usize;

        // Encode the entire document
        let tokens = self.tokenizer.encode(&document.content)?;
        let mut chunks = Vec::new();
        let mut position = 0;

        while position < tokens.len() {
            let end = (position + chunk_size).min(tokens.len());
            let chunk_tokens = &tokens[position..end];
            let content = self.tokenizer.decode(chunk_tokens)?;

            let chunk = create_chunk(
                document,
                content,
                chunk_tokens.len(),
                chunks.len(),
                None,
                json!({
                    "strategy": "fixed_size",
                    "chunk_size": chunk_size,
                    "overlap_tokens": overlap_tokens,
                    "token_range": [position, end],
                }),
            );

            chunks.push(chunk);

            // Move forward with overlap
            if end == tokens.len() {
                break;
            }
            position += chunk_size.saturating_sub(overlap_tokens);
        }

        tracing::debug!(
            document_id = %document.id,
            total_tokens = tokens.len(),
            num_chunks = chunks.len(),
            "Fixed-size chunking complete"
        );

        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::knowledge::ProcessorType;
    use crate::knowledge::chunking::TiktokenTokenizer;
    use chrono::Utc;

    fn create_test_document(content: String) -> Document {
        Document {
            id: "doc1".to_string(),
            user_id: "test_user".to_string(),
            knowledge_base_id: "kb1".to_string(),
            title: "Test Document".to_string(),
            content,
            file_path: None,
            file_type: "text/plain".to_string(),
            file_size: 0,
            processor: ProcessorType::Native,
            metadata: json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_fixed_size_chunking() -> Result<()> {
        let tokenizer = Arc::new(TiktokenTokenizer::new()?);
        let chunker = FixedSizeChunker::new(tokenizer);

        let content = "This is a test document. ".repeat(200); // ~600 tokens
        let document = create_test_document(content);

        let config = ChunkingConfig {
            chunk_size: Some(100),
            overlap_percent: Some(0.2),
            ..Default::default()
        };

        let chunks = chunker.chunk(&document, &config).await?;

        assert!(!chunks.is_empty());
        assert!(chunks.len() > 1); // Should have multiple chunks

        // Verify chunk sizes
        for chunk in &chunks {
            assert!(chunk.tokens <= 100);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_no_overlap() -> Result<()> {
        let tokenizer = Arc::new(TiktokenTokenizer::new()?);
        let chunker = FixedSizeChunker::new(tokenizer);

        let content = "Word ".repeat(100);
        let document = create_test_document(content);

        let config = ChunkingConfig {
            chunk_size: Some(50),
            overlap_percent: Some(0.0), // No overlap
            ..Default::default()
        };

        let chunks = chunker.chunk(&document, &config).await?;
        assert!(!chunks.is_empty());

        Ok(())
    }
}
