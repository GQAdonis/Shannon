//! Knowledge base database schema and repository.
//!
//! Provides data models and database operations for:
//! - Knowledge bases with configurable chunking strategies
//! - Document storage and processing
//! - Text chunks with embeddings for RAG
//! - Associations between agents/conversations and knowledge bases

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Knowledge base with configurable chunking and embedding.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct KnowledgeBase {
    /// Unique identifier.
    pub id: String,
    /// Owner user ID (for multi-tenant deployments).
    pub user_id: String,
    /// Human-readable name.
    pub name: String,
    /// Optional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Chunking strategy to use.
    pub chunking_strategy: ChunkingStrategy,
    /// Configuration for the chunking strategy.
    pub chunking_config: ChunkingConfig,
    /// Embedding provider.
    pub embedding_provider: EmbeddingProvider,
    /// Embedding model name.
    pub embedding_model: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Last update timestamp.
    pub updated_at: DateTime<Utc>,
}

/// Chunking strategy for document processing.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ChunkingStrategy {
    /// Fixed-size chunks with configurable overlap (baseline: 512-1024 tokens).
    FixedSize,
    /// Semantic chunking respecting sentence boundaries (default).
    Semantic,
    /// Structure-aware chunking preserving markdown/code/tables.
    StructureAware,
    /// Hierarchical chunking with parent-child relationships.
    Hierarchical,
}

impl Default for ChunkingStrategy {
    fn default() -> Self {
        Self::Semantic
    }
}

/// Configuration for chunking strategies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChunkingConfig {
    // Fixed-size parameters
    /// Chunk size in tokens (default: 768).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_size: Option<usize>,
    /// Overlap percentage (default: 0.15 for 15%).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlap_percent: Option<f32>,

    // Semantic parameters
    /// Minimum chunk size for semantic chunking (default: 256).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_chunk_size: Option<usize>,
    /// Maximum chunk size for semantic chunking (default: 1024).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_chunk_size: Option<usize>,
    /// Respect sentence boundaries (default: true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respect_sentences: Option<bool>,

    // Structure-aware parameters
    /// Preserve code blocks (default: true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_code_blocks: Option<bool>,
    /// Preserve tables (default: true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_tables: Option<bool>,
    /// Preserve lists (default: true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_lists: Option<bool>,

    // Hierarchical parameters
    /// Parent chunk size (default: 2048).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_chunk_size: Option<usize>,
    /// Child chunk size (default: 512).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_chunk_size: Option<usize>,
    /// Maximum depth (default: 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<usize>,
}

impl Default for ChunkingConfig {
    fn default() -> Self {
        Self {
            chunk_size: Some(768),
            overlap_percent: Some(0.15),
            min_chunk_size: Some(256),
            max_chunk_size: Some(1024),
            respect_sentences: Some(true),
            preserve_code_blocks: Some(true),
            preserve_tables: Some(true),
            preserve_lists: Some(true),
            parent_chunk_size: Some(2048),
            child_chunk_size: Some(512),
            max_depth: Some(3),
        }
    }
}

/// Document in a knowledge base.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Document {
    /// Unique identifier.
    pub id: String,
    /// Owner user ID (for multi-tenant deployments).
    pub user_id: String,
    /// Parent knowledge base ID.
    pub knowledge_base_id: String,
    /// Document title.
    pub title: String,
    /// Processed text content.
    pub content: String,
    /// Original file storage path (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    /// File MIME type.
    pub file_type: String,
    /// File size in bytes.
    pub file_size: u64,
    /// Processor used for extraction.
    pub processor: ProcessorType,
    /// Additional metadata.
    #[serde(default)]
    pub metadata: serde_json::Value,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Last update timestamp.
    pub updated_at: DateTime<Utc>,
}

/// Document processor type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ProcessorType {
    /// Mistral document processing API.
    Mistral,
    /// Unstructured.io hosted API.
    UnstructuredHosted,
    /// Self-hosted Unstructured.io.
    UnstructuredSelfHosted,
    /// Built-in native parsers.
    Native,
}

/// Processor configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ProcessorConfig {
    /// Processor type.
    pub processor_type: ProcessorType,
    /// API key (for Mistral/Unstructured hosted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// API URL (for self-hosted Unstructured).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
    /// Supported MIME types.
    pub supported_mime_types: Vec<String>,
}

/// Text chunk with embedding.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Chunk {
    /// Unique identifier.
    pub id: String,
    /// Parent document ID.
    pub document_id: String,
    /// Parent knowledge base ID.
    pub knowledge_base_id: String,
    /// Text content.
    pub content: String,
    /// Embedding vector.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embedding: Vec<f32>,
    /// Token count.
    pub tokens: usize,
    /// Position in document.
    pub position: usize,
    /// Parent chunk ID (for hierarchical).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_chunk_id: Option<String>,
    /// Additional metadata.
    #[serde(default)]
    pub metadata: serde_json::Value,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Chunk with similarity score.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChunkWithScore {
    /// The chunk.
    #[serde(flatten)]
    pub chunk: Chunk,
    /// Similarity score (0.0 to 1.0).
    pub score: f32,
}

/// Agent-to-knowledge-base association.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AgentKnowledgeBase {
    /// Agent ID.
    pub agent_id: String,
    /// Knowledge base ID.
    pub knowledge_base_id: String,
    /// Priority (higher = searched first).
    pub priority: u8,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Conversation-to-knowledge-base association.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ConversationKnowledgeBase {
    /// Conversation ID.
    pub conversation_id: String,
    /// Knowledge base ID.
    pub knowledge_base_id: String,
    /// Priority (higher = searched first).
    pub priority: u8,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
}

/// Embedding provider.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum EmbeddingProvider {
    /// OpenAI embeddings.
    OpenAI,
    /// Local embeddings (future).
    Local,
}

/// SQLite schema for knowledge bases.
pub const KNOWLEDGE_SQLITE_SCHEMA: &str = r#"
-- Knowledge bases table
CREATE TABLE IF NOT EXISTS knowledge_bases (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL DEFAULT 'default',
    name TEXT NOT NULL,
    description TEXT,
    chunking_strategy TEXT NOT NULL,
    chunking_config TEXT NOT NULL,
    embedding_provider TEXT NOT NULL,
    embedding_model TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_kb_user ON knowledge_bases(user_id);

-- Documents table
CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL DEFAULT 'default',
    knowledge_base_id TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    file_path TEXT,
    file_type TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    processor TEXT NOT NULL,
    metadata TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (knowledge_base_id) REFERENCES knowledge_bases(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_documents_kb ON documents(knowledge_base_id);
CREATE INDEX IF NOT EXISTS idx_documents_user ON documents(user_id);

-- Chunks table
CREATE TABLE IF NOT EXISTS chunks (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    knowledge_base_id TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding BLOB NOT NULL,
    tokens INTEGER NOT NULL,
    position INTEGER NOT NULL,
    parent_chunk_id TEXT,
    metadata TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (knowledge_base_id) REFERENCES knowledge_bases(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_chunks_kb ON chunks(knowledge_base_id);
CREATE INDEX IF NOT EXISTS idx_chunks_doc ON chunks(document_id);
CREATE INDEX IF NOT EXISTS idx_chunks_parent ON chunks(parent_chunk_id);

-- Agent-knowledge base associations
CREATE TABLE IF NOT EXISTS agent_knowledge_bases (
    agent_id TEXT NOT NULL,
    knowledge_base_id TEXT NOT NULL,
    priority INTEGER NOT NULL DEFAULT 5,
    created_at TEXT NOT NULL,
    PRIMARY KEY (agent_id, knowledge_base_id)
);

-- Conversation-knowledge base associations
CREATE TABLE IF NOT EXISTS conversation_knowledge_bases (
    conversation_id TEXT NOT NULL,
    knowledge_base_id TEXT NOT NULL,
    priority INTEGER NOT NULL DEFAULT 5,
    created_at TEXT NOT NULL,
    PRIMARY KEY (conversation_id, knowledge_base_id)
);

-- Processor configurations
CREATE TABLE IF NOT EXISTS processor_configs (
    id TEXT PRIMARY KEY,
    processor_type TEXT NOT NULL,
    api_key TEXT,
    api_url TEXT,
    supported_mime_types TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_chunking_strategy() {
        assert_eq!(ChunkingStrategy::default(), ChunkingStrategy::Semantic);
    }

    #[test]
    fn test_default_chunking_config() {
        let config = ChunkingConfig::default();
        assert_eq!(config.chunk_size, Some(768));
        assert_eq!(config.overlap_percent, Some(0.15));
        assert_eq!(config.min_chunk_size, Some(256));
        assert_eq!(config.max_chunk_size, Some(1024));
        assert_eq!(config.respect_sentences, Some(true));
    }

    #[test]
    fn test_serialization() {
        let kb = KnowledgeBase {
            id: "kb1".to_string(),
            user_id: "default".to_string(),
            name: "Test KB".to_string(),
            description: Some("Test description".to_string()),
            chunking_strategy: ChunkingStrategy::Semantic,
            chunking_config: ChunkingConfig::default(),
            embedding_provider: EmbeddingProvider::OpenAI,
            embedding_model: "text-embedding-3-small".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&kb).unwrap();
        let deserialized: KnowledgeBase = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, kb.id);
        assert_eq!(deserialized.name, kb.name);
        assert_eq!(deserialized.user_id, kb.user_id);
    }
}
