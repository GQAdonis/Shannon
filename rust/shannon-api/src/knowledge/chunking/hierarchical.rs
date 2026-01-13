//! Hierarchical chunking with parent-child relationships.
//!
//! This strategy creates a hierarchy of chunks with parent chunks containing
//! context and child chunks providing details. Default: 2048-token parents,
//! 512-token children, max depth 3.

use super::{create_chunk, ChunkingStrategyTrait, Tokenizer};
use crate::database::knowledge::{Chunk, ChunkingConfig, Document};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;

/// Hierarchical chunking strategy.
pub struct HierarchicalChunker {
    tokenizer: Arc<dyn Tokenizer>,
}

impl HierarchicalChunker {
    /// Create a new hierarchical chunker.
    #[must_use]
    pub fn new(tokenizer: Arc<dyn Tokenizer>) -> Self {
        Self { tokenizer }
    }

    /// Create parent chunks.
    async fn create_parent_chunks(
        &self,
        document: &Document,
        parent_size: usize,
    ) -> Result<Vec<Chunk>> {
        let tokens = self.tokenizer.encode(&document.content)?;
        let mut chunks = Vec::new();
        let mut position = 0;

        while position < tokens.len() {
            let end = (position + parent_size).min(tokens.len());
            let chunk_tokens = &tokens[position..end];
            let content = self.tokenizer.decode(chunk_tokens)?;

            chunks.push(create_chunk(
                document,
                content,
                chunk_tokens.len(),
                chunks.len(),
                None,
                json!({
                    "strategy": "hierarchical",
                    "level": 0,
                    "is_parent": true,
                }),
            ));

            position = end;
        }

        Ok(chunks)
    }

    /// Create child chunks from a parent chunk.
    async fn create_child_chunks(
        &self,
        document: &Document,
        parent: &Chunk,
        child_size: usize,
        base_position: usize,
    ) -> Result<Vec<Chunk>> {
        let tokens = self.tokenizer.encode(&parent.content)?;
        let mut chunks = Vec::new();

        if tokens.len() <= child_size {
            // Parent is small enough, no need for children
            return Ok(chunks);
        }

        let mut position = 0;
        while position < tokens.len() {
            let end = (position + child_size).min(tokens.len());
            let chunk_tokens = &tokens[position..end];
            let content = self.tokenizer.decode(chunk_tokens)?;

            chunks.push(create_chunk(
                document,
                content,
                chunk_tokens.len(),
                base_position + chunks.len(),
                Some(parent.id.clone()),
                json!({
                    "strategy": "hierarchical",
                    "level": 1,
                    "is_parent": false,
                    "parent_id": parent.id,
                }),
            ));

            position = end;
        }

        Ok(chunks)
    }
}

#[async_trait]
impl ChunkingStrategyTrait for HierarchicalChunker {
    async fn chunk(&self, document: &Document, config: &ChunkingConfig) -> Result<Vec<Chunk>> {
        let parent_size = config.parent_chunk_size.unwrap_or(2048);
        let child_size = config.child_chunk_size.unwrap_or(512);
        let max_depth = config.max_depth.unwrap_or(3);

        // Create parent chunks
        let parent_chunks = self.create_parent_chunks(document, parent_size).await?;

        // Create all chunks (parents + children)
        let mut all_chunks = Vec::new();
        let mut next_position = 0;

        for parent in &parent_chunks {
            // Add parent
            let mut parent_with_position = parent.clone();
            parent_with_position.position = next_position;
            all_chunks.push(parent_with_position);
            next_position += 1;

            // Add children if max_depth allows
            if max_depth > 1 {
                let children = self
                    .create_child_chunks(document, parent, child_size, next_position)
                    .await?;
                next_position += children.len();
                all_chunks.extend(children);
            }
        }

        tracing::debug!(
            document_id = %document.id,
            num_parents = parent_chunks.len(),
            total_chunks = all_chunks.len(),
            "Hierarchical chunking complete"
        );

        Ok(all_chunks)
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
    async fn test_hierarchical_chunking() -> Result<()> {
        let tokenizer = Arc::new(TiktokenTokenizer::new()?);
        let chunker = HierarchicalChunker::new(tokenizer);

        let content = "This is a test document. ".repeat(500); // ~1000 tokens
        let document = create_test_document(content);

        let config = ChunkingConfig {
            parent_chunk_size: Some(300),
            child_chunk_size: Some(100),
            max_depth: Some(2),
            ..Default::default()
        };

        let chunks = chunker.chunk(&document, &config).await?;

        assert!(!chunks.is_empty());

        // Should have both parents and children
        let parents: Vec<_> = chunks
            .iter()
            .filter(|c| c.parent_chunk_id.is_none())
            .collect();
        let children: Vec<_> = chunks
            .iter()
            .filter(|c| c.parent_chunk_id.is_some())
            .collect();

        assert!(!parents.is_empty());
        assert!(!children.is_empty());

        // Verify child references
        for child in &children {
            assert!(child.parent_chunk_id.is_some());
            let parent_id = child.parent_chunk_id.as_ref().unwrap();
            assert!(chunks.iter().any(|c| &c.id == parent_id));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_single_parent() -> Result<()> {
        let tokenizer = Arc::new(TiktokenTokenizer::new()?);
        let chunker = HierarchicalChunker::new(tokenizer);

        let content = "Short document.";
        let document = create_test_document(content.to_string());

        let config = ChunkingConfig {
            parent_chunk_size: Some(1000),
            child_chunk_size: Some(100),
            max_depth: Some(2),
            ..Default::default()
        };

        let chunks = chunker.chunk(&document, &config).await?;

        // Should have just one parent, no children needed
        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].parent_chunk_id.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_depth_limit() -> Result<()> {
        let tokenizer = Arc::new(TiktokenTokenizer::new()?);
        let chunker = HierarchicalChunker::new(tokenizer);

        let content = "Test ".repeat(1000);
        let document = create_test_document(content);

        let config = ChunkingConfig {
            parent_chunk_size: Some(300),
            child_chunk_size: Some(100),
            max_depth: Some(1), // Only parents, no children
            ..Default::default()
        };

        let chunks = chunker.chunk(&document, &config).await?;

        // Should only have parents
        let children_count = chunks
            .iter()
            .filter(|c| c.parent_chunk_id.is_some())
            .count();
        assert_eq!(children_count, 0);

        Ok(())
    }
}
