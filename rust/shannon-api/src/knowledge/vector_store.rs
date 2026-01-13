//! Vector store with USearch for similarity search.
//!
//! Uses a hybrid approach:
//! - **USearch**: Fast vector similarity search with HNSW index
//! - **SQLite**: Chunk metadata and content storage
//!
//! This provides both efficient nearest-neighbor search and full chunk data retrieval.

use crate::database::knowledge::{Chunk, ChunkWithScore};
use anyhow::{Context, Result};
use std::path::Path;
use std::sync::Arc;

#[cfg(feature = "usearch")]
use usearch::{Index, IndexOptions, MetricKind, ScalarKind};

/// Vector store for chunk embeddings and similarity search.
pub struct VectorStore {
    /// SQLite connection for chunk metadata.
    db: Arc<rusqlite::Connection>,
    /// USearch index for vector similarity search.
    #[cfg(feature = "usearch")]
    index: Option<Arc<Index>>,
    /// Embedding dimension.
    dimension: usize,
}

impl VectorStore {
    /// Create a new vector store.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails.
    pub fn new(db_path: impl AsRef<Path>, dimension: usize) -> Result<Self> {
        let db = rusqlite::Connection::open(db_path)?;

        // Create chunks table if not exists
        db.execute(
            r#"
            CREATE TABLE IF NOT EXISTS chunks (
                id TEXT PRIMARY KEY,
                document_id TEXT NOT NULL,
                knowledge_base_id TEXT NOT NULL,
                content TEXT NOT NULL,
                tokens INTEGER NOT NULL,
                position INTEGER NOT NULL,
                parent_chunk_id TEXT,
                metadata TEXT,
                created_at TEXT NOT NULL
            )
            "#,
            [],
        )?;

        db.execute(
            "CREATE INDEX IF NOT EXISTS idx_chunks_kb ON chunks(knowledge_base_id)",
            [],
        )?;
        db.execute(
            "CREATE INDEX IF NOT EXISTS idx_chunks_doc ON chunks(document_id)",
            [],
        )?;

        #[cfg(feature = "usearch")]
        let index = {
            let options = IndexOptions {
                dimensions: dimension,
                metric: MetricKind::Cos, // Cosine similarity
                quantization: ScalarKind::F32,
                connectivity: 16,
                expansion_add: 128,
                expansion_search: 64,
                multi: false, // Single-threaded indexing
            };

            Some(Arc::new(Index::new(&options)?))
        };

        #[cfg(not(feature = "usearch"))]
        let index = None;

        Ok(Self {
            db: Arc::new(db),
            #[cfg(feature = "usearch")]
            index,
            dimension,
        })
    }

    /// Add a chunk with its embedding to the store.
    ///
    /// # Errors
    ///
    /// Returns an error if storage fails.
    pub fn add_chunk(&self, chunk: &Chunk) -> Result<()> {
        // Store metadata in SQLite
        self.db.execute(
            r#"
            INSERT INTO chunks (id, document_id, knowledge_base_id, content, tokens, position, parent_chunk_id, metadata, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
            rusqlite::params![
                chunk.id,
                chunk.document_id,
                chunk.knowledge_base_id,
                chunk.content,
                chunk.tokens as i64,
                chunk.position as i64,
                chunk.parent_chunk_id,
                serde_json::to_string(&chunk.metadata)?,
                chunk.created_at.to_rfc3339(),
            ],
        )?;

        // Store vector in USearch index
        #[cfg(feature = "usearch")]
        if let Some(ref index) = self.index {
            if !chunk.embedding.is_empty() {
                // Use chunk position as the key (convert to u64)
                let key = chunk.position as u64;
                index.add(key, &chunk.embedding)?;
            }
        }

        Ok(())
    }

    /// Add multiple chunks in batch.
    ///
    /// # Errors
    ///
    /// Returns an error if storage fails.
    pub fn add_chunks_batch(&self, chunks: &[Chunk]) -> Result<()> {
        let tx = self.db.unchecked_transaction()?;

        for chunk in chunks {
            tx.execute(
                r#"
                INSERT INTO chunks (id, document_id, knowledge_base_id, content, tokens, position, parent_chunk_id, metadata, created_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                "#,
                rusqlite::params![
                    chunk.id,
                    chunk.document_id,
                    chunk.knowledge_base_id,
                    chunk.content,
                    chunk.tokens as i64,
                    chunk.position as i64,
                    chunk.parent_chunk_id,
                    serde_json::to_string(&chunk.metadata)?,
                    chunk.created_at.to_rfc3339(),
                ],
            )?;

            // Store vector in USearch
            #[cfg(feature = "usearch")]
            if let Some(ref index) = self.index {
                if !chunk.embedding.is_empty() {
                    let key = chunk.position as u64;
                    index.add(key, &chunk.embedding)?;
                }
            }
        }

        tx.commit()?;
        Ok(())
    }

    /// Search for similar chunks using vector similarity.
    ///
    /// # Errors
    ///
    /// Returns an error if search fails.
    pub fn search(
        &self,
        knowledge_base_id: &str,
        query_embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<ChunkWithScore>> {
        #[cfg(feature = "usearch")]
        if let Some(ref index) = self.index {
            // Search using USearch
            let results = index.search(query_embedding, limit)?;

            // Retrieve chunk metadata from SQLite
            let mut chunks = Vec::new();
            for result in results.keys.iter().zip(results.distances.iter()) {
                let (key, distance) = result;
                let position = *key as usize;

                // Get chunk by position and knowledge base
                let chunk_result: Result<Chunk, _> = self.db.query_row(
                    r#"
                    SELECT id, document_id, knowledge_base_id, content, tokens, position, parent_chunk_id, metadata, created_at
                    FROM chunks
                    WHERE knowledge_base_id = ?1 AND position = ?2
                    "#,
                    rusqlite::params![knowledge_base_id, position as i64],
                    |row| {
                        let metadata_str: String = row.get(7)?;
                        let metadata: serde_json::Value = serde_json::from_str(&metadata_str)
                            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                                7,
                                rusqlite::types::Type::Text,
                                Box::new(e)
                            ))?;

                        Ok(Chunk {
                            id: row.get(0)?,
                            document_id: row.get(1)?,
                            knowledge_base_id: row.get(2)?,
                            content: row.get(3)?,
                            embedding: vec![], // Don't return full embedding
                            tokens: row.get::<_, i64>(4)? as usize,
                            position: row.get::<_, i64>(5)? as usize,
                            parent_chunk_id: row.get(6)?,
                            metadata,
                            created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                                    8,
                                    rusqlite::types::Type::Text,
                                    Box::new(e)
                                ))?
                                .with_timezone(&chrono::Utc),
                        })
                    },
                );

                if let Ok(chunk) = chunk_result {
                    // Convert distance to similarity score (1 - distance for cosine)
                    let score = 1.0 - distance;
                    chunks.push(ChunkWithScore { chunk, score });
                }
            }

            Ok(chunks)
        } else {
            Err(anyhow::anyhow!("USearch feature not enabled"))
        }

        #[cfg(not(feature = "usearch"))]
        {
            // Fallback: return empty results
            tracing::warn!("USearch not available, returning empty results");
            Ok(Vec::new())
        }
    }

    /// Get all chunks for a knowledge base.
    ///
    /// # Errors
    ///
    /// Returns an error if query fails.
    pub fn get_chunks_by_kb(&self, knowledge_base_id: &str) -> Result<Vec<Chunk>> {
        let mut stmt = self.db.prepare(
            r#"
            SELECT id, document_id, knowledge_base_id, content, tokens, position, parent_chunk_id, metadata, created_at
            FROM chunks
            WHERE knowledge_base_id = ?1
            ORDER BY position
            "#,
        )?;

        let chunks = stmt
            .query_map([knowledge_base_id], |row| {
                let metadata_str: String = row.get(7)?;
                let metadata: serde_json::Value =
                    serde_json::from_str(&metadata_str).map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            7,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?;

                Ok(Chunk {
                    id: row.get(0)?,
                    document_id: row.get(1)?,
                    knowledge_base_id: row.get(2)?,
                    content: row.get(3)?,
                    embedding: vec![],
                    tokens: row.get::<_, i64>(4)? as usize,
                    position: row.get::<_, i64>(5)? as usize,
                    parent_chunk_id: row.get(6)?,
                    metadata,
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                        .map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                8,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?
                        .with_timezone(&chrono::Utc),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(chunks)
    }

    /// Delete all chunks for a knowledge base.
    ///
    /// # Errors
    ///
    /// Returns an error if deletion fails.
    pub fn delete_chunks_by_kb(&self, knowledge_base_id: &str) -> Result<()> {
        self.db.execute(
            "DELETE FROM chunks WHERE knowledge_base_id = ?1",
            [knowledge_base_id],
        )?;

        // Note: USearch doesn't support deletion, so we'd need to rebuild the index
        // For now, we'll just leave the vectors in the index
        // In production, you'd want to rebuild the index periodically

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use tempfile::NamedTempFile;

    fn create_test_chunk(kb_id: &str, position: usize, embedding: Vec<f32>) -> Chunk {
        Chunk {
            id: format!("chunk_{position}"),
            document_id: "doc1".to_string(),
            knowledge_base_id: kb_id.to_string(),
            content: format!("Test content {position}"),
            embedding,
            tokens: 10,
            position,
            parent_chunk_id: None,
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        }
    }

    #[test]
    #[cfg(feature = "usearch")]
    fn test_vector_store_creation() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let store = VectorStore::new(temp_file.path(), 384)?;
        assert_eq!(store.dimension, 384);
        Ok(())
    }

    #[test]
    #[cfg(feature = "usearch")]
    fn test_add_and_search() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let store = VectorStore::new(temp_file.path(), 3)?;

        // Add test chunks
        let chunk1 = create_test_chunk("kb1", 0, vec![1.0, 0.0, 0.0]);
        let chunk2 = create_test_chunk("kb1", 1, vec![0.0, 1.0, 0.0]);
        let chunk3 = create_test_chunk("kb1", 2, vec![0.9, 0.1, 0.0]);

        store.add_chunk(&chunk1)?;
        store.add_chunk(&chunk2)?;
        store.add_chunk(&chunk3)?;

        // Search with query similar to chunk1
        let query = vec![1.0, 0.0, 0.0];
        let results = store.search("kb1", &query, 2)?;

        assert!(!results.is_empty());
        // First result should be most similar
        assert!(results[0].score > 0.9);

        Ok(())
    }

    #[test]
    #[cfg(feature = "usearch")]
    fn test_batch_add() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let store = VectorStore::new(temp_file.path(), 3)?;

        let chunks = vec![
            create_test_chunk("kb1", 0, vec![1.0, 0.0, 0.0]),
            create_test_chunk("kb1", 1, vec![0.0, 1.0, 0.0]),
            create_test_chunk("kb1", 2, vec![0.0, 0.0, 1.0]),
        ];

        store.add_chunks_batch(&chunks)?;

        let retrieved = store.get_chunks_by_kb("kb1")?;
        assert_eq!(retrieved.len(), 3);

        Ok(())
    }
}
