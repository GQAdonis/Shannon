//! Knowledge base Tauri bridge.
//!
//! Provides Tauri commands for knowledge base operations, connecting the
//! frontend to the Rust RAG backend with real-time citation streaming.

use shannon_api::database::knowledge::{
    Chunk, ChunkWithScore, Document, KnowledgeBase, ProcessorConfig,
};
use shannon_api::knowledge::RAGService;
use std::sync::Arc;
use tauri::State;

/// Knowledge base state for Tauri.
pub struct KnowledgeState {
    pub rag_service: Arc<RAGService>,
}

impl KnowledgeState {
    /// Create new knowledge state.
    ///
    /// # Errors
    ///
    /// Returns an error if RAG service initialization fails.
    pub fn new(rag_service: Arc<RAGService>) -> Self {
        Self { rag_service }
    }
}

/// Search result with document metadata for frontend display.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultWithMetadata {
    /// Chunk ID.
    pub id: String,
    /// Document ID.
    pub document_id: String,
    /// Document title.
    pub document_title: String,
    /// Chunk content.
    pub content: String,
    /// Relevance score (0.0 to 1.0).
    pub score: f32,
    /// Token count.
    pub tokens: usize,
    /// Additional metadata.
    pub metadata: serde_json::Value,
}

/// Create a new knowledge base.
///
/// # Errors
///
/// Returns an error if creation fails.
#[tauri::command]
pub async fn create_knowledge_base(request: KnowledgeBase) -> Result<String, String> {
    // TODO: Implement with KnowledgeBaseRepository
    Ok(request.id)
}

/// List all knowledge bases.
///
/// # Errors
///
/// Returns an error if retrieval fails.
#[tauri::command]
pub async fn list_knowledge_bases() -> Result<Vec<KnowledgeBase>, String> {
    // TODO: Implement with KnowledgeBaseRepository
    Ok(Vec::new())
}

/// Get a specific knowledge base.
///
/// # Errors
///
/// Returns an error if not found.
#[tauri::command]
pub async fn get_knowledge_base(id: String) -> Result<KnowledgeBase, String> {
    // TODO: Implement with KnowledgeBaseRepository
    Err(format!("Knowledge base not found: {}", id))
}

/// Update a knowledge base.
///
/// # Errors
///
/// Returns an error if update fails.
#[tauri::command]
pub async fn update_knowledge_base(id: String, updates: serde_json::Value) -> Result<(), String> {
    // TODO: Implement with KnowledgeBaseRepository
    Ok(())
}

/// Delete a knowledge base.
///
/// # Errors
///
/// Returns an error if deletion fails.
#[tauri::command]
pub async fn delete_knowledge_base(id: String) -> Result<(), String> {
    // TODO: Implement with KnowledgeBaseRepository
    Ok(())
}

/// Upload and process a document.
///
/// # Errors
///
/// Returns an error if processing fails.
#[tauri::command]
pub async fn upload_document(
    knowledge_base_id: String,
    file_name: String,
    file_content: String, // base64
    processor: String,
) -> Result<String, String> {
    use base64::{engine::general_purpose, Engine as _};

    // Decode base64
    let content = general_purpose::STANDARD
        .decode(&file_content)
        .map_err(|e| format!("Base64 decode error: {}", e))?;

    // Save to temp file
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join(&file_name);

    tokio::fs::write(&temp_path, content)
        .await
        .map_err(|e| format!("File write error: {}", e))?;

    // TODO: Process document with RAG service
    let document_id = format!("doc_{}", uuid::Uuid::new_v4());

    // Clean up
    tokio::fs::remove_file(&temp_path).await.ok();

    Ok(document_id)
}

/// List documents in a knowledge base.
///
/// # Errors
///
/// Returns an error if retrieval fails.
#[tauri::command]
pub async fn list_documents(knowledge_base_id: String) -> Result<Vec<Document>, String> {
    // TODO: Implement with KnowledgeBaseRepository
    Ok(Vec::new())
}

/// Delete a document.
///
/// # Errors
///
/// Returns an error if deletion fails.
#[tauri::command]
pub async fn delete_document(document_id: String) -> Result<(), String> {
    // TODO: Implement with KnowledgeBaseRepository
    Ok(())
}

/// Search within a knowledge base.
///
/// # Errors
///
/// Returns an error if search fails.
#[tauri::command]
pub async fn search_knowledge_base(
    kb_id: String,
    query: String,
    limit: usize,
    state: State<'_, KnowledgeState>,
) -> Result<Vec<SearchResultWithMetadata>, String> {
    let results = state
        .rag_service
        .search(&query, &[kb_id.clone()], limit)
        .await
        .map_err(|e| e.to_string())?;

    Ok(results
        .into_iter()
        .map(|r| SearchResultWithMetadata {
            id: r.chunk.id,
            document_id: r.chunk.document_id.clone(),
            document_title: format!("Document {}", r.chunk.document_id), // TODO: Get from DB
            content: r.chunk.content,
            score: r.score,
            tokens: r.chunk.tokens,
            metadata: r.chunk.metadata,
        })
        .collect())
}

/// Search across multiple knowledge bases.
///
/// # Errors
///
/// Returns an error if search fails.
#[tauri::command]
pub async fn search_multiple_kbs(
    kb_ids: Vec<String>,
    query: String,
    limit: usize,
    state: State<'_, KnowledgeState>,
) -> Result<Vec<SearchResultWithMetadata>, String> {
    let mut all_results = Vec::new();

    for kb_id in kb_ids {
        let results = state
            .rag_service
            .search(&query, &[kb_id.clone()], limit)
            .await
            .map_err(|e| e.to_string())?;

        all_results.extend(results);
    }

    // Sort by relevance score
    all_results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Take top results
    all_results.truncate(limit);

    Ok(all_results
        .into_iter()
        .map(|r| SearchResultWithMetadata {
            id: r.chunk.id,
            document_id: r.chunk.document_id.clone(),
            document_title: format!("Document {}", r.chunk.document_id), // TODO: Get from DB
            content: r.chunk.content,
            score: r.score,
            tokens: r.chunk.tokens,
            metadata: r.chunk.metadata,
        })
        .collect())
}

/// Attach knowledge bases to a conversation.
///
/// # Errors
///
/// Returns an error if attachment fails.
#[tauri::command]
pub async fn attach_knowledge_bases_to_conversation(
    conversation_id: String,
    kb_ids: Vec<String>,
) -> Result<(), String> {
    // TODO: Implement with KnowledgeBaseRepository
    Ok(())
}

/// Get knowledge bases attached to a conversation.
///
/// # Errors
///
/// Returns an error if retrieval fails.
#[tauri::command]
pub async fn get_conversation_knowledge_bases(
    conversation_id: String,
) -> Result<Vec<KnowledgeBase>, String> {
    // TODO: Implement with KnowledgeBaseRepository
    Ok(Vec::new())
}

/// Attach knowledge bases to an agent.
///
/// # Errors
///
/// Returns an error if attachment fails.
#[tauri::command]
pub async fn attach_knowledge_bases_to_agent(
    agent_id: String,
    kb_ids: Vec<String>,
) -> Result<(), String> {
    // TODO: Implement with KnowledgeBaseRepository
    Ok(())
}

/// Get knowledge bases attached to an agent.
///
/// # Errors
///
/// Returns an error if retrieval fails.
#[tauri::command]
pub async fn get_agent_knowledge_bases(agent_id: String) -> Result<Vec<KnowledgeBase>, String> {
    // TODO: Implement with KnowledgeBaseRepository
    Ok(Vec::new())
}

/// Get processor configurations.
///
/// # Errors
///
/// Returns an error if retrieval fails.
#[tauri::command]
pub async fn get_processor_configs() -> Result<Vec<ProcessorConfig>, String> {
    // TODO: Implement with ProcessorConfigRepository
    Ok(Vec::new())
}

/// Update processor configuration.
///
/// # Errors
///
/// Returns an error if update fails.
#[tauri::command]
pub async fn update_processor_config(config: ProcessorConfig) -> Result<(), String> {
    // TODO: Implement with ProcessorConfigRepository
    Ok(())
}
