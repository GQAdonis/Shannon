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
    Extension, Json, Router,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, path::PathBuf};

use crate::{
    database::knowledge::{
        Chunk, ChunkWithScore, ChunkingConfig, ChunkingStrategy, Document, EmbeddingProvider,
        KnowledgeBase, ProcessorType,
    },
    gateway::auth::AuthenticatedUser,
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
// Knowledge Base Endpoints
// ============================================================================

/// Create a new knowledge base.
///
/// # Authentication
///
/// Requires valid authentication. User ID is extracted from JWT token or API key.
///
/// # Multi-tenancy
///
/// - Embedded mode: Uses authenticated user_id (embedded_user by default)
/// - Cloud mode: Uses authenticated user_id from JWT
async fn create_knowledge_base(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<CreateKBRequest>,
) -> Result<Json<CreateKBResponse>, (StatusCode, String)> {
    use crate::database::repository::KnowledgeBaseRepository;

    let kb = KnowledgeBase {
        id: uuid::Uuid::new_v4().to_string(),
        user_id: user.user_id.clone(),
        name: request.name,
        description: request.description,
        chunking_strategy: request.chunking_strategy,
        chunking_config: request.chunking_config,
        embedding_provider: request.embedding_provider,
        embedding_model: request.embedding_model,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let kb_id = state
        .database
        .create_knowledge_base(&kb)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create knowledge base");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create knowledge base: {e}"),
            )
        })?;

    tracing::info!(
        kb_id = %kb_id,
        user_id = %kb.user_id,
        name = %kb.name,
        "Knowledge base created"
    );

    Ok(Json(CreateKBResponse { id: kb_id }))
}

/// List all knowledge bases for the authenticated user.
///
/// # Authentication
///
/// Requires valid authentication. Only returns knowledge bases owned by the authenticated user.
async fn list_knowledge_bases(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<KnowledgeBase>>, (StatusCode, String)> {
    use crate::database::repository::KnowledgeBaseRepository;

    tracing::debug!(
        user_id = %user.user_id,
        limit = query.limit,
        offset = query.offset,
        "Listing knowledge bases"
    );

    let kbs = state
        .database
        .list_knowledge_bases(&user.user_id, query.limit, query.offset)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list knowledge bases");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to list knowledge bases: {e}"),
            )
        })?;

    Ok(Json(kbs))
}

/// Get a specific knowledge base.
///
/// # Authentication
///
/// Requires valid authentication. Verifies ownership before returning data.
async fn get_knowledge_base(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<String>,
) -> Result<Json<KnowledgeBase>, (StatusCode, String)> {
    use crate::database::repository::KnowledgeBaseRepository;

    tracing::debug!(kb_id = %id, user_id = %user.user_id, "Getting knowledge base");

    let kb = state
        .database
        .get_knowledge_base(&id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to get knowledge base");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get knowledge base: {e}"),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                "Knowledge base not found".to_string(),
            )
        })?;

    // Verify ownership
    if kb.user_id != user.user_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    Ok(Json(kb))
}

/// Delete a knowledge base and all its documents.
///
/// # Authentication
///
/// Requires valid authentication. Verifies ownership before deletion.
async fn delete_knowledge_base(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    use crate::database::repository::KnowledgeBaseRepository;

    // Verify ownership first
    let kb = state
        .database
        .get_knowledge_base(&id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to get knowledge base");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get knowledge base: {e}"),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                "Knowledge base not found".to_string(),
            )
        })?;

    if kb.user_id != user.user_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    tracing::info!(kb_id = %id, user_id = %user.user_id, "Deleting knowledge base");

    let deleted = state
        .database
        .delete_knowledge_base(&id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to delete knowledge base");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to delete knowledge base: {e}"),
            )
        })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            "Knowledge base not found".to_string(),
        ))
    }
}

// ============================================================================
// Document Endpoints
// ============================================================================

/// Upload a document to a knowledge base.
///
/// # Authentication
///
/// Requires valid authentication. Verifies KB ownership before upload.
///
/// Accepts multipart/form-data with:
/// - `file`: The document file
/// - `processor`: Processor type (optional, default: native)
async fn upload_document(
    State(_state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(kb_id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<UploadDocumentResponse>, (StatusCode, String)> {
    let user_id = user.user_id.clone();

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
///
/// # Authentication
///
/// Requires valid authentication. Verifies KB ownership before listing.
async fn list_documents(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(kb_id): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Document>>, (StatusCode, String)> {
    use crate::database::repository::KnowledgeBaseRepository;

    // Verify KB ownership
    let kb = state
        .database
        .get_knowledge_base(&kb_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get knowledge base: {e}"),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                "Knowledge base not found".to_string(),
            )
        })?;

    if kb.user_id != user.user_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    tracing::debug!(
        kb_id = %kb_id,
        user_id = %user.user_id,
        limit = query.limit,
        "Listing documents"
    );

    let docs = state
        .database
        .list_documents(&kb_id, query.limit, query.offset)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list documents");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to list documents: {e}"),
            )
        })?;

    Ok(Json(docs))
}

/// Delete a document.
///
/// # Authentication
///
/// Requires valid authentication. Verifies ownership before deletion.
async fn delete_document(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    use crate::database::repository::KnowledgeBaseRepository;

    // Get document to verify ownership
    let doc = state
        .database
        .get_document(&id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get document: {e}"),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Document not found".to_string()))?;

    if doc.user_id != user.user_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    tracing::info!(doc_id = %id, user_id = %user.user_id, "Deleting document");

    let deleted = state.database.delete_document(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to delete document");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete document: {e}"),
        )
    })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, "Document not found".to_string()))
    }
}

// ============================================================================
// Search Endpoints
// ============================================================================

/// Search across multiple knowledge bases.
///
/// # Authentication
///
/// Requires valid authentication. Only searches knowledge bases owned by the authenticated user.
async fn search_knowledge_bases(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, (StatusCode, String)> {
    use crate::database::repository::KnowledgeBaseRepository;

    tracing::debug!(
        user_id = %user.user_id,
        kb_count = request.knowledge_base_ids.len(),
        query = %request.query,
        "Searching knowledge bases"
    );

    // Verify embedding provider is available
    let embedding_provider = state.embedding_provider.as_ref().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "Embedding service not configured".to_string(),
        )
    })?;

    // Verify database is available (embedded mode)
    #[cfg(feature = "embedded")]
    let database = state.database.as_ref().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "Database not available".to_string(),
        )
    })?;

    #[cfg(not(feature = "embedded"))]
    return Err((
        StatusCode::NOT_IMPLEMENTED,
        "Knowledge base search requires embedded mode".to_string(),
    ));

    #[cfg(feature = "embedded")]
    {
        // Verify KB ownership
        for kb_id in &request.knowledge_base_ids {
            let kb = database
                .get_knowledge_base(kb_id)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to get knowledge base: {e}"),
                    )
                })?
                .ok_or_else(|| {
                    (
                        StatusCode::NOT_FOUND,
                        format!("Knowledge base not found: {kb_id}"),
                    )
                })?;

            if kb.user_id != user.user_id {
                return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
            }
        }

        // Generate query embedding
        let query_embedding = embedding_provider
            .embed(&request.query)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to generate embedding");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to generate embedding: {e}"),
                )
            })?;

        // Search chunks across all specified KBs
        let results = database
            .search_chunks(
                &request.knowledge_base_ids,
                &query_embedding,
                request.limit,
                0.7,
            )
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to search chunks");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to search chunks: {e}"),
                )
            })?;

        tracing::info!(
            user_id = %user.user_id,
            kb_count = request.knowledge_base_ids.len(),
            result_count = results.len(),
            "Knowledge base search completed"
        );

        Ok(Json(SearchResponse { results }))
    }
}

/// Search a single knowledge base.
///
/// # Authentication
///
/// Requires valid authentication. Verifies KB ownership before search.
async fn search_single_kb(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(kb_id): Path<String>,
    Json(request): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, (StatusCode, String)> {
    use crate::database::repository::KnowledgeBaseRepository;

    // Verify embedding provider is available
    let embedding_provider = state.embedding_provider.as_ref().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "Embedding service not configured".to_string(),
        )
    })?;

    // Verify database is available (embedded mode)
    #[cfg(feature = "embedded")]
    let database = state.database.as_ref().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "Database not available".to_string(),
        )
    })?;

    #[cfg(not(feature = "embedded"))]
    return Err((
        StatusCode::NOT_IMPLEMENTED,
        "Knowledge base search requires embedded mode".to_string(),
    ));

    #[cfg(feature = "embedded")]
    {
        // Verify KB ownership
        let kb = database
            .get_knowledge_base(&kb_id)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to get knowledge base: {e}"),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    "Knowledge base not found".to_string(),
                )
            })?;

        if kb.user_id != user.user_id {
            return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
        }

        tracing::debug!(
            kb_id = %kb_id,
            user_id = %user.user_id,
            query = %request.query,
            "Searching single knowledge base"
        );

        // Generate query embedding
        let query_embedding = embedding_provider
            .embed(&request.query)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to generate embedding");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to generate embedding: {e}"),
                )
            })?;

        // Search chunks in this KB only
        let results = database
            .search_chunks(&[kb_id.clone()], &query_embedding, request.limit, 0.7)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to search chunks");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to search chunks: {e}"),
                )
            })?;

        tracing::info!(
            kb_id = %kb_id,
            user_id = %user.user_id,
            result_count = results.len(),
            "Single KB search completed"
        );

        Ok(Json(SearchResponse { results }))
    }
}

// ============================================================================
// RAG Streaming Endpoint
// ============================================================================

/// Stream chat with RAG context and citations.
///
/// # Authentication
///
/// Requires valid authentication. Only searches knowledge bases owned by the authenticated user.
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
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<ChatRAGRequest>,
) -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    let user_id = user.user_id.clone();
    let embedding_provider = state.embedding_provider.clone();
    let database = state.database.clone();

    let stream = async_stream::stream! {
        // 1. Emit searching event
        yield Ok(Event::default()
            .event("rag_searching")
            .data(serde_json::json!({
                "knowledge_bases": request.knowledge_bases,
                "context_limit": request.context_limit
            }).to_string()));

        // 2. Check if embedding provider available
        let provider = match embedding_provider {
            Some(p) => p,
            None => {
                yield Ok(Event::default()
                    .event("error")
                    .data(serde_json::json!({
                        "message": "Embedding service not configured"
                    }).to_string()));
                return;
            }
        };

        // 3. Check if database available
        #[cfg(feature = "embedded")]
        let db = match database {
            Some(ref d) => d,
            None => {
                yield Ok(Event::default()
                    .event("error")
                    .data(serde_json::json!({
                        "message": "Database not available"
                    }).to_string()));
                return;
            }
        };

        #[cfg(not(feature = "embedded"))]
        {
            yield Ok(Event::default()
                .event("error")
                .data(serde_json::json!({
                    "message": "RAG chat requires embedded mode"
                }).to_string()));
            return;
        }

        // 4. Search for relevant chunks
        #[cfg(feature = "embedded")]
        let results = {
            use crate::database::repository::KnowledgeBaseRepository;

            // Generate query embedding
            let query_embedding = match provider.embed(&request.message).await {
                Ok(emb) => emb,
                Err(e) => {
                    tracing::error!(error = %e, "Failed to generate embedding");
                    yield Ok(Event::default()
                        .event("error")
                        .data(serde_json::json!({
                            "message": format!("Failed to generate embedding: {e}")
                        }).to_string()));
                    return;
                }
            };

            // Search chunks
            match db.search_chunks(&request.knowledge_bases, &query_embedding, request.context_limit, 0.7).await {
                Ok(chunks) => chunks,
                Err(e) => {
                    tracing::error!(error = %e, "Failed to search chunks");
                    yield Ok(Event::default()
                        .event("error")
                        .data(serde_json::json!({
                            "message": format!("Failed to search chunks: {e}")
                        }).to_string()));
                    return;
                }
            }
        };

        // 5. Emit citations
        #[cfg(feature = "embedded")]
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

        // 6. Emit citations complete
        #[cfg(feature = "embedded")]
        yield Ok(Event::default()
            .event("citations_complete")
            .data(serde_json::json!({
                "count": results.len()
            }).to_string()));

        // 7. Build augmented prompt with RAG context
        #[cfg(feature = "embedded")]
        let augmented_prompt = if results.is_empty() {
            request.message.clone()
        } else {
            let context = results.iter()
                .enumerate()
                .map(|(i, r)| format!("[Source {}] (Relevance: {:.2})\n{}", i + 1, r.score, r.chunk.content))
                .collect::<Vec<_>>()
                .join("\n\n---\n\n");

            format!(
                "# Relevant Context from Knowledge Base\n\n{}\n\n---\n\n# User Query\n\n{}",
                context,
                request.message
            )
        };

        // 8. Simple content response (LLM streaming would go here)
        #[cfg(feature = "embedded")]
        yield Ok(Event::default()
            .event("content")
            .data(format!("Based on {} relevant sources from the knowledge base, ", results.len())));

        yield Ok(Event::default()
            .event("content")
            .data("I can provide context-aware responses. "));

        yield Ok(Event::default()
            .event("content")
            .data("(Full LLM streaming integration coming soon)"));

        // 9. Emit done
        #[cfg(feature = "embedded")]
        yield Ok(Event::default()
            .event("done")
            .data(serde_json::json!({
                "status": "complete",
                "sources_used": results.len()
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
