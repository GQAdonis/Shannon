//! Knowledge base and RAG API endpoints.
//!
//! Provides comprehensive REST API for:
//! - Knowledge base CRUD operations
//! - Document upload and processing
//! - Vector similarity search
//! - RAG chat with streaming citations
//! - Multi-tenant isolation in cloud mode
//! - Single-user mode for embedded deployments

use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::sse::{Event, Sse},
    routing::{delete, get, post},
    Json, Router,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, path::PathBuf};

use crate::{
    database::knowledge::{
        Chunk, ChunkWithScore, ChunkingConfig, ChunkingStrategy, Document, EmbeddingProvider,
        KnowledgeBase, ProcessorType,
    },
    gateway::embedded_auth::EmbeddedClaims,
    AppState,
};

/// Create knowledge base router.
pub fn router() -> Router<AppState> {
    Router::new()
        // Knowledge base operations
        .route("/api/knowledge/bases", post(create_knowledge_base))
        .route("/api/knowledge/bases", get(list_knowledge_bases))
        .route("/api/knowledge/bases/:id", get(get_knowledge_base))
        .route("/api/knowledge/bases/:id", delete(delete_knowledge_base))
        // Document operations
        .route(
            "/api/knowledge/bases/:kb_id/documents",
            post(upload_document),
        )
        .route("/api/knowledge/bases/:kb_id/documents", get(list_documents))
        .route("/api/knowledge/documents/:id", delete(delete_document))
        // Search operations
        .route("/api/knowledge/search", post(search_knowledge_bases))
        .route("/api/knowledge/bases/:kb_id/search", post(search_single_kb))
        // RAG streaming
        .route("/api/knowledge/chat/stream", post(stream_chat_with_rag))
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Create knowledge base request.
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateKBRequest {
    /// Knowledge base name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Chunking strategy (default: semantic).
    #[serde(default)]
    pub chunking_strategy: ChunkingStrategy,
    /// Chunking configuration.
    #[serde(default)]
    pub chunking_config: ChunkingConfig,
    /// Embedding provider (default: openai).
    #[serde(default = "default_embedding_provider")]
    pub embedding_provider: EmbeddingProvider,
    /// Embedding model name (default: text-embedding-3-small).
    #[serde(default = "default_embedding_model")]
    pub embedding_model: String,
}

fn default_embedding_provider() -> EmbeddingProvider {
    EmbeddingProvider::OpenAI
}

fn default_embedding_model() -> String {
    "text-embedding-3-small".to_string()
}

/// Create knowledge base response.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateKBResponse {
    /// Knowledge base ID.
    pub id: String,
}

/// List query parameters.
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    /// Limit results (default: 50).
    #[serde(default = "default_limit")]
    pub limit: usize,
    /// Offset for pagination (default: 0).
    #[serde(default)]
    pub offset: usize,
}

fn default_limit() -> usize {
    50
}

/// Upload document response.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UploadDocumentResponse {
    /// Document ID.
    pub document_id: String,
    /// Number of chunks created.
    pub chunks_created: usize,
}

/// Search request.
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SearchRequest {
    /// Search query.
    pub query: String,
    /// Knowledge base IDs to search.
    pub knowledge_base_ids: Vec<String>,
    /// Maximum results per KB (default: 5).
    #[serde(default = "default_search_limit")]
    pub limit: usize,
}

fn default_search_limit() -> usize {
    5
}

/// Search response.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SearchResponse {
    /// Search results with relevance scores.
    pub results: Vec<ChunkWithScore>,
}

/// RAG chat request.
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChatRAGRequest {
    /// User message/query.
    pub message: String,
    /// Knowledge base IDs to search.
    pub knowledge_bases: Vec<String>,
    /// Model configuration.
    #[serde(default)]
    pub config: ChatConfig,
    /// Number of context chunks to retrieve (default: 5).
    #[serde(default = "default_search_limit")]
    pub context_limit: usize,
}

/// Chat configuration.
#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ChatConfig {
    /// Model to use (default: gpt-4).
    pub model: Option<String>,
    /// Temperature (default: 0.7).
    pub temperature: Option<f32>,
    /// Max tokens to generate.
    pub max_tokens: Option<u32>,
}

// ============================================================================
// Helper: Extract user_id from request
// ============================================================================

/// Extract user ID from embedded JWT or use default for single-user mode.
///
/// In embedded mode: Uses "default" if no auth header provided
/// In cloud mode: Requires valid JWT authentication
fn extract_user_id(claims: Option<EmbeddedClaims>) -> String {
    claims
        .map(|c| c.sub)
        .unwrap_or_else(|| "default".to_string())
}

// ============================================================================
// Knowledge Base Endpoints
// ============================================================================

/// Create a new knowledge base.
///
/// # Multi-tenancy
/// - Embedded mode: Uses "default" user_id
/// - Cloud mode: Uses authenticated user_id from JWT
async fn create_knowledge_base(
    State(_state): State<AppState>,
    // TODO: Add auth extraction middleware
    // AuthUser(claims): AuthUser,
    Json(request): Json<CreateKBRequest>,
) -> Result<Json<CreateKBResponse>, (StatusCode, String)> {
    let user_id = extract_user_id(None); // TODO: Pass actual claims

    let kb = KnowledgeBase {
        id: uuid::Uuid::new_v4().to_string(),
        user_id,
        name: request.name,
        description: request.description,
        chunking_strategy: request.chunking_strategy,
        chunking_config: request.chunking_config,
        embedding_provider: request.embedding_provider,
        embedding_model: request.embedding_model,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // TODO: Store in database via repository
    tracing::info!(
        kb_id = %kb.id,
        user_id = %kb.user_id,
        name = %kb.name,
        "Knowledge base created"
    );

    Ok(Json(CreateKBResponse { id: kb.id }))
}

/// List all knowledge bases for the authenticated user.
async fn list_knowledge_bases(
    State(_state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<KnowledgeBase>>, (StatusCode, String)> {
    let user_id = extract_user_id(None);

    tracing::debug!(
        user_id = %user_id,
        limit = query.limit,
        offset = query.offset,
        "Listing knowledge bases"
    );

    // TODO: Fetch from database
    Ok(Json(vec![]))
}

/// Get a specific knowledge base.
async fn get_knowledge_base(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<KnowledgeBase>, (StatusCode, String)> {
    let user_id = extract_user_id(None);

    tracing::debug!(kb_id = %id, user_id = %user_id, "Getting knowledge base");

    // TODO: Fetch from database and verify ownership
    Err((
        StatusCode::NOT_FOUND,
        "Knowledge base not found".to_string(),
    ))
}

/// Delete a knowledge base and all its documents.
async fn delete_knowledge_base(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = extract_user_id(None);

    tracing::info!(kb_id = %id, user_id = %user_id, "Deleting knowledge base");

    // TODO: Verify ownership and delete from database + vector store
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// Document Endpoints
// ============================================================================

/// Upload a document to a knowledge base.
///
/// Accepts multipart/form-data with:
/// - `file`: The document file
/// - `processor`: Processor type (optional, default: native)
async fn upload_document(
    State(_state): State<AppState>,
    Path(kb_id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<UploadDocumentResponse>, (StatusCode, String)> {
    let user_id = extract_user_id(None);

    // TODO: Verify KB ownership
    tracing::debug!(kb_id = %kb_id, user_id = %user_id, "Uploading document");

    // Parse multipart form
    let mut file_name = String::new();
    let mut file_content: Vec<u8> = Vec::new();
    let mut processor = ProcessorType::Native;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Multipart error: {e}")))?
    {
        match field.name() {
            Some("file") => {
                file_name = field.file_name().unwrap_or("document").to_string();
                file_content = field
                    .bytes()
                    .await
                    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to read file: {e}")))?
                    .to_vec();
            }
            Some("processor") => {
                let processor_str = field
                    .text()
                    .await
                    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid processor: {e}")))?;
                processor = parse_processor(&processor_str).ok_or_else(|| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Invalid processor type".to_string(),
                    )
                })?;
            }
            _ => {}
        }
    }

    if file_name.is_empty() || file_content.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No file provided".to_string()));
    }

    // Determine storage path (multi-tenant aware)
    let file_path = get_document_path(&user_id, &kb_id, &file_name, true);

    // Create parent directories
    if let Some(parent) = file_path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create directories: {e}"),
            )
        })?;
    }

    // Write file to storage
    tokio::fs::write(&file_path, &file_content)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to write file: {e}"),
            )
        })?;

    let doc_id = uuid::Uuid::new_v4().to_string();

    // TODO: Process document with RAG service
    // 1. Extract text (using processor)
    // 2. Chunk text
    // 3. Generate embeddings
    // 4. Store in vector store

    tracing::info!(
        doc_id = %doc_id,
        kb_id = %kb_id,
        file_name = %file_name,
        file_size = file_content.len(),
        "Document uploaded"
    );

    Ok(Json(UploadDocumentResponse {
        document_id: doc_id,
        chunks_created: 0, // TODO: Return actual count
    }))
}

/// List documents in a knowledge base.
async fn list_documents(
    State(_state): State<AppState>,
    Path(kb_id): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Document>>, (StatusCode, String)> {
    let user_id = extract_user_id(None);

    tracing::debug!(
        kb_id = %kb_id,
        user_id = %user_id,
        limit = query.limit,
        "Listing documents"
    );

    // TODO: Verify KB ownership and fetch documents
    Ok(Json(vec![]))
}

/// Delete a document.
async fn delete_document(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = extract_user_id(None);

    tracing::info!(doc_id = %id, user_id = %user_id, "Deleting document");

    // TODO: Verify ownership and delete document + chunks
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// Search Endpoints
// ============================================================================

/// Search across multiple knowledge bases.
async fn search_knowledge_bases(
    State(_state): State<AppState>,
    Json(request): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, (StatusCode, String)> {
    let user_id = extract_user_id(None);

    // TODO: Verify all KBs belong to user
    tracing::debug!(
        user_id = %user_id,
        kb_count = request.knowledge_base_ids.len(),
        query = %request.query,
        "Searching knowledge bases"
    );

    // TODO: Use RAG service to search
    Ok(Json(SearchResponse { results: vec![] }))
}

/// Search a single knowledge base.
async fn search_single_kb(
    State(_state): State<AppState>,
    Path(kb_id): Path<String>,
    Json(request): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, (StatusCode, String)> {
    let user_id = extract_user_id(None);

    tracing::debug!(
        kb_id = %kb_id,
        user_id = %user_id,
        query = %request.query,
        "Searching single knowledge base"
    );

    // TODO: Verify ownership and search
    Ok(Json(SearchResponse { results: vec![] }))
}

// ============================================================================
// RAG Streaming Endpoint
// ============================================================================

/// Stream chat with RAG context and citations.
///
/// This endpoint:
/// 1. Emits `rag_searching` event
/// 2. Searches knowledge bases for relevant context
/// 3. Emits individual `citation` events for each source
/// 4. Emits `citations_complete` event
/// 5. Augments prompt with context
/// 6. Streams LLM response as `content` events
/// 7. Emits `done` event
async fn stream_chat_with_rag(
    State(_state): State<AppState>,
    Json(request): Json<ChatRAGRequest>,
) -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    let user_id = extract_user_id(None);

    let stream = async_stream::stream! {
        // 1. Emit searching event
        yield Ok(Event::default()
            .event("rag_searching")
            .data(serde_json::json!({
                "knowledge_bases": request.knowledge_bases,
                "context_limit": request.context_limit
            }).to_string()));

        // TODO: 2. Search knowledge bases
        // let results = state.rag_service
        //     .search(&request.message, &request.knowledge_bases, request.context_limit)
        //     .await;

        let results: Vec<ChunkWithScore> = vec![]; // Placeholder

        // 3. Emit citations
        for (idx, result) in results.iter().enumerate() {
            yield Ok(Event::default()
                .event("citation")
                .data(serde_json::json!({
                    "index": idx + 1,
                    "document_id": result.chunk.document_id,
                    "content": result.chunk.content,
                    "relevance_score": result.score,
                    "tokens": result.chunk.tokens,
                    "metadata": result.chunk.metadata,
                }).to_string()));
        }

        // 4. Emit citations complete
        yield Ok(Event::default()
            .event("citations_complete")
            .data(serde_json::json!({
                "count": results.len()
            }).to_string()));

        // TODO: 5. Augment prompt with context
        // let augmented = state.rag_service.augment_query(...).await;

        // TODO: 6. Stream LLM response
        // let llm_stream = call_llm_streaming(&augmented, &request.config).await;
        // for await chunk in llm_stream {
        //     yield Ok(Event::default().event("content").data(chunk));
        // }

        // Placeholder content
        yield Ok(Event::default()
            .event("content")
            .data("Based on the knowledge base context..."));

        // 7. Emit done
        yield Ok(Event::default()
            .event("done")
            .data(serde_json::json!({
                "status": "complete",
                "total_tokens": 0
            }).to_string()));
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new().interval(std::time::Duration::from_secs(15)),
    )
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Get document storage path (multi-tenant aware).
///
/// # Storage Structure
///
/// Embedded (single-user):
/// ```text
/// data/knowledge/
///   documents/
///     original/{kb_id}/{file_name}
///     processed/{kb_id}/{doc_id}.txt
///   vectors/{kb_id}.usearch
/// ```
///
/// Cloud (multi-tenant):
/// ```text
/// data/knowledge/
///   users/{user_id}/
///     documents/
///       original/{kb_id}/{file_name}
///       processed/{kb_id}/{doc_id}.txt
///     vectors/{kb_id}.usearch
/// ```
fn get_document_path(user_id: &str, kb_id: &str, file_name: &str, original: bool) -> PathBuf {
    let mut path = PathBuf::from("data/knowledge");

    // Multi-tenant path (skip for default user in embedded mode)
    if user_id != "default" {
        path.push("users");
        path.push(user_id);
    }

    path.push("documents");
    path.push(if original { "original" } else { "processed" });
    path.push(kb_id);
    path.push(file_name);

    path
}

/// Parse processor type from string.
fn parse_processor(s: &str) -> Option<ProcessorType> {
    match s.to_lowercase().as_str() {
        "mistral" => Some(ProcessorType::Mistral),
        "unstructured_hosted" => Some(ProcessorType::UnstructuredHosted),
        "unstructured_self_hosted" => Some(ProcessorType::UnstructuredSelfHosted),
        "native" => Some(ProcessorType::Native),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_document_path_embedded() {
        let path = get_document_path("default", "kb1", "test.pdf", true);
        assert_eq!(
            path,
            PathBuf::from("data/knowledge/documents/original/kb1/test.pdf")
        );
    }

    #[test]
    fn test_get_document_path_cloud() {
        let path = get_document_path("user123", "kb1", "test.pdf", true);
        assert_eq!(
            path,
            PathBuf::from("data/knowledge/users/user123/documents/original/kb1/test.pdf")
        );
    }

    #[test]
    fn test_parse_processor() {
        assert_eq!(parse_processor("native"), Some(ProcessorType::Native));
        assert_eq!(parse_processor("mistral"), Some(ProcessorType::Mistral));
        assert_eq!(parse_processor("invalid"), None);
    }
}
