//! # Action API Endpoints
//!
//! REST API routes for browser automation and filesystem operations.

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::actions::{
    ActionRequest, ActionResponse, ActionState, FileInfo, FormField, PageSnapshot,
};
use crate::error::AppError;
use crate::runtime::AppState;

/// Create action routes
pub fn action_routes() -> Router<AppState> {
    Router::new()
        // Browser routes
        .route("/api/actions/browser/navigate", post(browser_navigate))
        .route("/api/actions/browser/extract", post(browser_extract))
        .route("/api/actions/browser/click", post(browser_click))
        .route("/api/actions/browser/fill", post(browser_fill_form))
        // Filesystem routes
        .route("/api/actions/filesystem/read", post(fs_read))
        .route("/api/actions/filesystem/write", post(fs_write))
        .route("/api/actions/filesystem/list", post(fs_list))
        .route("/api/actions/filesystem/delete", post(fs_delete))
        .route("/api/actions/filesystem/mkdir", post(fs_mkdir))
        .route("/api/actions/filesystem/info", post(fs_info))
        // Health check
        .route("/api/actions/health", get(actions_health))
}

// ============================================================================
// Browser Endpoints
// ============================================================================

#[derive(Debug, Deserialize)]
struct BrowserNavigateRequest {
    url: String,
}

#[derive(Debug, Serialize)]
struct BrowserNavigateResponse {
    snapshot: PageSnapshot,
}

async fn browser_navigate(
    State(state): State<AppState>,
    Json(request): Json<BrowserNavigateRequest>,
) -> Result<Json<BrowserNavigateResponse>, AppError> {
    debug!("Browser navigate request: {}", request.url);

    // Get action state from app state (would need to be added)
    // For now, create temporary instance
    let action_state = ActionState::new()
        .map_err(|e| AppError::Internal(format!("Failed to create action state: {}", e)))?;

    let snapshot = action_state
        .browser_service
        .navigate(&request.url)
        .await
        .map_err(|e| {
            error!("Browser navigate failed: {}", e);
            AppError::Internal(format!("Navigation failed: {}", e))
        })?;

    Ok(Json(BrowserNavigateResponse { snapshot }))
}

#[derive(Debug, Deserialize)]
struct BrowserExtractRequest {
    url: String,
    selector: String,
}

#[derive(Debug, Serialize)]
struct BrowserExtractResponse {
    data: String,
}

async fn browser_extract(
    State(state): State<AppState>,
    Json(request): Json<BrowserExtractRequest>,
) -> Result<Json<BrowserExtractResponse>, AppError> {
    debug!(
        "Browser extract request: {} -> {}",
        request.url, request.selector
    );

    let action_state = ActionState::new()
        .map_err(|e| AppError::Internal(format!("Failed to create action state: {}", e)))?;

    let data = action_state
        .browser_service
        .extract_data(&request.url, &request.selector)
        .await
        .map_err(|e| {
            error!("Browser extract failed: {}", e);
            AppError::Internal(format!("Extraction failed: {}", e))
        })?;

    Ok(Json(BrowserExtractResponse { data }))
}

#[derive(Debug, Deserialize)]
struct BrowserClickRequest {
    url: String,
    selector: String,
}

async fn browser_click(
    State(state): State<AppState>,
    Json(request): Json<BrowserClickRequest>,
) -> Result<StatusCode, AppError> {
    debug!(
        "Browser click request: {} -> {}",
        request.url, request.selector
    );

    let action_state = ActionState::new()
        .map_err(|e| AppError::Internal(format!("Failed to create action state: {}", e)))?;

    action_state
        .browser_service
        .click_element(&request.url, &request.selector)
        .await
        .map_err(|e| {
            error!("Browser click failed: {}", e);
            AppError::Internal(format!("Click failed: {}", e))
        })?;

    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
struct BrowserFillFormRequest {
    url: String,
    fields: Vec<FormField>,
}

async fn browser_fill_form(
    State(state): State<AppState>,
    Json(request): Json<BrowserFillFormRequest>,
) -> Result<StatusCode, AppError> {
    debug!(
        "Browser fill form request: {} ({} fields)",
        request.url,
        request.fields.len()
    );

    let action_state = ActionState::new()
        .map_err(|e| AppError::Internal(format!("Failed to create action state: {}", e)))?;

    action_state
        .browser_service
        .fill_form(&request.url, &request.fields)
        .await
        .map_err(|e| {
            error!("Browser fill form failed: {}", e);
            AppError::Internal(format!("Form fill failed: {}", e))
        })?;

    Ok(StatusCode::OK)
}

// ============================================================================
// Filesystem Endpoints
// ============================================================================

#[derive(Debug, Deserialize)]
struct FsReadRequest {
    path: String,
}

#[derive(Debug, Serialize)]
struct FsReadResponse {
    content: String,
}

async fn fs_read(
    State(state): State<AppState>,
    Json(request): Json<FsReadRequest>,
) -> Result<Json<FsReadResponse>, AppError> {
    debug!("Filesystem read request: {}", request.path);

    let action_state = ActionState::new()
        .map_err(|e| AppError::Internal(format!("Failed to create action state: {}", e)))?;

    let content = action_state
        .filesystem_service
        .read_file(&request.path)
        .await
        .map_err(|e| {
            error!("Filesystem read failed: {}", e);
            AppError::Internal(format!("Read failed: {}", e))
        })?;

    Ok(Json(FsReadResponse { content }))
}

#[derive(Debug, Deserialize)]
struct FsWriteRequest {
    path: String,
    content: String,
}

async fn fs_write(
    State(state): State<AppState>,
    Json(request): Json<FsWriteRequest>,
) -> Result<StatusCode, AppError> {
    debug!(
        "Filesystem write request: {} ({} bytes)",
        request.path,
        request.content.len()
    );

    let action_state = ActionState::new()
        .map_err(|e| AppError::Internal(format!("Failed to create action state: {}", e)))?;

    action_state
        .filesystem_service
        .write_file(&request.path, &request.content)
        .await
        .map_err(|e| {
            error!("Filesystem write failed: {}", e);
            AppError::Internal(format!("Write failed: {}", e))
        })?;

    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
struct FsListRequest {
    #[serde(default = "default_path")]
    path: String,
}

fn default_path() -> String {
    ".".to_string()
}

#[derive(Debug, Serialize)]
struct FsListResponse {
    files: Vec<FileInfo>,
}

async fn fs_list(
    State(state): State<AppState>,
    Json(request): Json<FsListRequest>,
) -> Result<Json<FsListResponse>, AppError> {
    debug!("Filesystem list request: {}", request.path);

    let action_state = ActionState::new()
        .map_err(|e| AppError::Internal(format!("Failed to create action state: {}", e)))?;

    let files = action_state
        .filesystem_service
        .list_directory(&request.path)
        .await
        .map_err(|e| {
            error!("Filesystem list failed: {}", e);
            AppError::Internal(format!("List failed: {}", e))
        })?;

    Ok(Json(FsListResponse { files }))
}

#[derive(Debug, Deserialize)]
struct FsDeleteRequest {
    path: String,
}

async fn fs_delete(
    State(state): State<AppState>,
    Json(request): Json<FsDeleteRequest>,
) -> Result<StatusCode, AppError> {
    debug!("Filesystem delete request: {}", request.path);

    let action_state = ActionState::new()
        .map_err(|e| AppError::Internal(format!("Failed to create action state: {}", e)))?;

    action_state
        .filesystem_service
        .delete(&request.path)
        .await
        .map_err(|e| {
            error!("Filesystem delete failed: {}", e);
            AppError::Internal(format!("Delete failed: {}", e))
        })?;

    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
struct FsMkdirRequest {
    path: String,
}

async fn fs_mkdir(
    State(state): State<AppState>,
    Json(request): Json<FsMkdirRequest>,
) -> Result<StatusCode, AppError> {
    debug!("Filesystem mkdir request: {}", request.path);

    let action_state = ActionState::new()
        .map_err(|e| AppError::Internal(format!("Failed to create action state: {}", e)))?;

    action_state
        .filesystem_service
        .create_directory(&request.path)
        .await
        .map_err(|e| {
            error!("Filesystem mkdir failed: {}", e);
            AppError::Internal(format!("Mkdir failed: {}", e))
        })?;

    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
struct FsInfoRequest {
    path: String,
}

#[derive(Debug, Serialize)]
struct FsInfoResponse {
    info: FileInfo,
}

async fn fs_info(
    State(state): State<AppState>,
    Json(request): Json<FsInfoRequest>,
) -> Result<Json<FsInfoResponse>, AppError> {
    debug!("Filesystem info request: {}", request.path);

    let action_state = ActionState::new()
        .map_err(|e| AppError::Internal(format!("Failed to create action state: {}", e)))?;

    let info = action_state
        .filesystem_service
        .get_info(&request.path)
        .await
        .map_err(|e| {
            error!("Filesystem info failed: {}", e);
            AppError::Internal(format!("Info failed: {}", e))
        })?;

    Ok(Json(FsInfoResponse { info }))
}

// ============================================================================
// Health Check
// ============================================================================

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    actions_available: bool,
}

async fn actions_health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        actions_available: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_path() {
        assert_eq!(default_path(), ".");
    }
}
