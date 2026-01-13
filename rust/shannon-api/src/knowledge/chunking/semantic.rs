//! Semantic chunking that respects sentence and paragraph boundaries.
//!
//! This is the default chunking strategy. It splits text at natural boundaries
//! (paragraphs, sentences) while staying within configured size limits.
//! Default: 256-1024 tokens, respecting sentence boundaries.

use super::{create_chunk, ChunkingStrategyTrait, Tokenizer};
use crate::database::knowledge::{Chunk, ChunkingConfig, Document};
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use serde_json::json;
use std::sync::Arc;

/// Semantic chunking strategy.
pub struct SemanticChunker {
    tokenizer: Arc<dyn Tokenizer>,
    sentence_regex: Regex,
}

impl SemanticChunker {
    /// Create a new semantic chunker.
    #[must_use]
    pub fn new(tokenizer: Arc<dyn Tokenizer>) -> Self {
        // Regex to split on sentence boundaries
        let sentence_regex = Regex::new(r"(?<=[.!?])\s+").unwrap();
        Self {
            tokenizer,
            sentence_regex,
        }
    }

    /// Split text into sentences.
    fn split_sentences(&self, text: &str) -> Vec<String> {
        self.sentence_regex
            .split(text)
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    /// Create a chunk from accumulated content.
    fn create_semantic_chunk(
        &self,
        document: &Document,
        content: &str,
        tokens: usize,
        position: usize,
    ) -> Result<Chunk> {
        Ok(create_chunk(
            document,
            content.to_string(),
            tokens,
            position,
            None,
            json!({
                "strategy": "semantic",
                "boundary_type": "sentence",
            }),
        ))
    }
}

#[async_trait]
impl ChunkingStrategyTrait for SemanticChunker {
    async fn chunk(&self, document: &Document, config: &ChunkingConfig) -> Result<Vec<Chunk>> {
        let min_size = config.min_chunk_size.unwrap_or(256);
        let max_size = config.max_chunk_size.unwrap_or(1024);

        // Split by paragraphs first
        let paragraphs: Vec<&str> = document
            .content
            .split("\n\n")
            .filter(|p| !p.trim().is_empty())
            .collect();

        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut current_tokens = 0;

        for paragraph in paragraphs {
            let para_tokens = self.tokenizer.count(paragraph)?;

            // If single paragraph exceeds max, split by sentences
            if para_tokens > max_size {
                // Flush current chunk if any
                if !current_chunk.is_empty() && current_tokens >= min_size {
                    chunks.push(self.create_semantic_chunk(
                        document,
                        &current_chunk,
                        current_tokens,
                        chunks.len(),
                    )?);
                    current_chunk.clear();
                    current_tokens = 0;
                }

                // Split paragraph by sentences
                let sentences = self.split_sentences(paragraph);
                for sentence in sentences {
                    let sent_tokens = self.tokenizer.count(&sentence)?;

                    if current_tokens + sent_tokens > max_size && current_tokens >= min_size {
                        // Flush chunk
                        chunks.push(self.create_semantic_chunk(
                            document,
                            &current_chunk,
                            current_tokens,
                            chunks.len(),
                        )?);
                        current_chunk = sentence;
                        current_tokens = sent_tokens;
                    } else {
                        // Add to current chunk
                        if !current_chunk.is_empty() {
                            current_chunk.push(' ');
                        }
                        current_chunk.push_str(&sentence);
                        current_tokens += sent_tokens;
                    }
                }
            } else {
                // Add paragraph to current chunk
                if current_tokens + para_tokens > max_size && current_tokens >= min_size {
                    // Flush chunk
                    chunks.push(self.create_semantic_chunk(
                        document,
                        &current_chunk,
                        current_tokens,
                        chunks.len(),
                    )?);
                    current_chunk = paragraph.to_string();
                    current_tokens = para_tokens;
                } else {
                    // Add to current chunk
                    if !current_chunk.is_empty() {
                        current_chunk.push_str("\n\n");
                    }
                    current_chunk.push_str(paragraph);
                    current_tokens += para_tokens;
                }
            }
        }

        // Add final chunk
        if !current_chunk.is_empty() {
            chunks.push(self.create_semantic_chunk(
                document,
                &current_chunk,
                current_tokens,
                chunks.len(),
            )?);
        }

        tracing::debug!(
            document_id = %document.id,
            num_chunks = chunks.len(),
            "Semantic chunking complete"
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
    async fn test_semantic_chunking() -> Result<()> {
        let tokenizer = Arc::new(TiktokenTokenizer::new()?);
        let chunker = SemanticChunker::new(tokenizer);

        let content = "This is the first paragraph. It has multiple sentences. They should stay together.\n\nThis is the second paragraph. It also has sentences. More text here.";
        let document = create_test_document(content.to_string());

        let config = ChunkingConfig {
            min_chunk_size: Some(10),
            max_chunk_size: Some(50),
            ..Default::default()
        };

        let chunks = chunker.chunk(&document, &config).await?;

        assert!(!chunks.is_empty());

        // Verify chunks respect boundaries
        for chunk in &chunks {
            assert!(chunk.tokens >= 10 || chunk.position == chunks.len() - 1);
            assert!(chunk.tokens <= 50);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_sentence_splitting() -> Result<()> {
        let tokenizer = Arc::new(TiktokenTokenizer::new()?);
        let chunker = SemanticChunker::new(tokenizer);

        let sentences = chunker.split_sentences("First sentence. Second sentence! Third sentence?");
        assert_eq!(sentences.len(), 3);

        Ok(())
    }
}
