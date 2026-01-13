//! Knowledge base and RAG system.
//!
//! Provides comprehensive document processing, chunking, embedding, and
//! retrieval-augmented generation (RAG) capabilities.
//!
//! # Components
//!
//! - **Processor**: Document extraction with Mistral and Unstructured.io
//! - **Chunking**: Four strategies (fixed-size, semantic, structure-aware, hierarchical)
//! - **Embeddings**: OpenAI embeddings with batch support
//! - **Vector Store**: USearch + SQLite hybrid storage
//! - **RAG**: End-to-end RAG orchestration
//!
//! # Example
//!
//! ```rust,ignore
//! use shannon_api::knowledge::{
//!     processor::{DocumentProcessor, ProcessorType},
//!     embeddings::OpenAIEmbeddings,
//!     vector_store::VectorStore,
//!     rag::RAGService,
//! };
//!
//! // Initialize components
//! let embeddings = Arc::new(OpenAIEmbeddings::new_default(api_key)?);
//! let vector_store = Arc::new(VectorStore::new(db_path, 1536)?);
//! let rag = RAGService::new(embeddings, vector_store);
//!
//! // Process document
//! let chunks = rag.process_document(&document, &knowledge_base).await?;
//!
//! // Search and augment query
//! let augmented = rag.augment_query(
//!     "What is machine learning?",
//!     &["kb_id"],
//!     5
//! ).await?;
//! ```

pub mod chunking;
pub mod embeddings;
pub mod processor;
pub mod rag;
pub mod vector_store;

pub use chunking::{
    get_chunker, ChunkingStrategyTrait, FixedSizeChunker, HierarchicalChunker, SemanticChunker,
    StructureAwareChunker,
};
pub use embeddings::{create_provider, EmbeddingProvider, OpenAIEmbeddings};
pub use processor::{DocumentProcessor, ProcessedDocument};
pub use rag::{KnowledgeBaseStats, RAGService};
pub use vector_store::VectorStore;
