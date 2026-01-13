//! Text chunking strategies for RAG.
//!
//! Provides multiple chunking strategies optimized for different content types:
//! - **Fixed-size**: Simple token-based chunking with overlap
//! - **Semantic**: Respects sentence and paragraph boundaries (default)
//! - **Structure-aware**: Preserves markdown, code blocks, and tables
//! - **Hierarchical**: Creates parent-child chunk relationships

mod fixed_size;
mod hierarchical;
mod semantic;
mod structure_aware;

pub use fixed_size::FixedSizeChunker;
pub use hierarchical::HierarchicalChunker;
pub use semantic::SemanticChunker;
pub use structure_aware::StructureAwareChunker;

use crate::database::knowledge::{Chunk, ChunkingConfig, ChunkingStrategy, Document};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use serde_json::json;
use std::sync::Arc;
use tiktoken_rs::CoreBPE;
use uuid::Uuid;

/// Trait for text chunking strategies.
#[async_trait]
pub trait ChunkingStrategyTrait: Send + Sync {
    /// Chunk a document according to the strategy.
    ///
    /// # Errors
    ///
    /// Returns an error if tokenization or chunking fails.
    async fn chunk(&self, document: &Document, config: &ChunkingConfig) -> Result<Vec<Chunk>>;
}

/// Tokenizer for counting tokens and encoding/decoding.
pub trait Tokenizer: Send + Sync {
    /// Count tokens in text.
    ///
    /// # Errors
    ///
    /// Returns an error if tokenization fails.
    fn count(&self, text: &str) -> Result<usize>;

    /// Encode text to token IDs.
    ///
    /// # Errors
    ///
    /// Returns an error if encoding fails.
    fn encode(&self, text: &str) -> Result<Vec<usize>>;

    /// Decode token IDs to text.
    ///
    /// # Errors
    ///
    /// Returns an error if decoding fails.
    fn decode(&self, tokens: &[usize]) -> Result<String>;
}

/// OpenAI tokenizer using tiktoken.
pub struct TiktokenTokenizer {
    bpe: CoreBPE,
}

impl TiktokenTokenizer {
    /// Create a new tiktoken tokenizer.
    ///
    /// # Errors
    ///
    /// Returns an error if the tokenizer cannot be initialized.
    pub fn new() -> Result<Self> {
        let bpe = tiktoken_rs::cl100k_base()?;
        Ok(Self { bpe })
    }
}

impl Tokenizer for TiktokenTokenizer {
    fn count(&self, text: &str) -> Result<usize> {
        Ok(self.bpe.encode_with_special_tokens(text).len())
    }

    fn encode(&self, text: &str) -> Result<Vec<usize>> {
        Ok(self
            .bpe
            .encode_with_special_tokens(text)
            .into_iter()
            .map(|t| t as usize)
            .collect())
    }

    fn decode(&self, tokens: &[usize]) -> Result<String> {
        let u32_tokens: Vec<u32> = tokens.iter().map(|&t| t as u32).collect();
        Ok(self.bpe.decode(u32_tokens)?)
    }
}

/// Get a chunking strategy implementation.
///
/// # Errors
///
/// Returns an error if the tokenizer cannot be initialized.
pub fn get_chunker(strategy: ChunkingStrategy) -> Result<Box<dyn ChunkingStrategyTrait>> {
    let tokenizer = Arc::new(TiktokenTokenizer::new()?);

    match strategy {
        ChunkingStrategy::FixedSize => Ok(Box::new(FixedSizeChunker::new(tokenizer))),
        ChunkingStrategy::Semantic => Ok(Box::new(SemanticChunker::new(tokenizer))),
        ChunkingStrategy::StructureAware => Ok(Box::new(StructureAwareChunker::new(tokenizer))),
        ChunkingStrategy::Hierarchical => Ok(Box::new(HierarchicalChunker::new(tokenizer))),
    }
}

/// Helper to create a chunk.
pub(crate) fn create_chunk(
    document: &Document,
    content: String,
    tokens: usize,
    position: usize,
    parent_chunk_id: Option<String>,
    metadata: serde_json::Value,
) -> Chunk {
    Chunk {
        id: Uuid::new_v4().to_string(),
        document_id: document.id.clone(),
        knowledge_base_id: document.knowledge_base_id.clone(),
        content,
        embedding: vec![],
        tokens,
        position,
        parent_chunk_id,
        metadata,
        created_at: Utc::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() -> Result<()> {
        let tokenizer = TiktokenTokenizer::new()?;
        let text = "Hello, world!";
        let count = tokenizer.count(text)?;
        assert!(count > 0);

        let tokens = tokenizer.encode(text)?;
        assert_eq!(tokens.len(), count);

        let decoded = tokenizer.decode(&tokens)?;
        assert_eq!(decoded, text);

        Ok(())
    }

    #[test]
    fn test_get_chunker() -> Result<()> {
        let _fixed = get_chunker(ChunkingStrategy::FixedSize)?;
        let _semantic = get_chunker(ChunkingStrategy::Semantic)?;
        let _structure = get_chunker(ChunkingStrategy::StructureAware)?;
        let _hierarchical = get_chunker(ChunkingStrategy::Hierarchical)?;
        Ok(())
    }
}
