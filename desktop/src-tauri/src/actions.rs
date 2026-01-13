//! # Tauri Commands for Action Engine
//!
//! Provides IPC commands for browser automation and filesystem operations.

use shannon_api::actions::{ActionState, FileInfo, FormField, PageSnapshot};
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;

/// Shared action state for Tauri
pub struct TauriActionState {
    action_state: Arc<RwLock<Option<Arc<ActionState>>>>,
}

impl TauriActionState {
    /// Create a new Tauri action state
    pub fn new() -> Self {
        Self {
            action_state: Arc::new(RwLock::new(None)),
        }
    }

    /// Initialize the action state with a sandbox root
    pub async fn initialize(&self, sandbox_root: std::path::PathBuf) -> Result<(), String> {
        let mut state = self.action_state.write().await;
        let action_state = ActionState::with_sandbox_root(sandbox_root)
            .map_err(|e| format!("Failed to initialize action state: {}", e))?;
        *state = Some(Arc::new(action_state));
        Ok(())
    }

    /// Get the action state, initializing if needed
    async fn get_or_init(&self) -> Result<Arc<ActionState>, String> {
        let read_lock = self.action_state.read().await;
        if let Some(state) = read_lock.as_ref() {
            return Ok(Arc::clone(state));
        }
        drop(read_lock);

        // Initialize with default sandbox
        let default_sandbox = dirs::data_local_dir()
            .ok_or_else(|| "Could not determine data directory".to_string())?
            .join("Shannon")
            .join("sandbox");

        self.initialize(default_sandbox).await?;

        let read_lock = self.action_state.read().await;
        read_lock
            .as_ref()
            .cloned()
            .ok_or_else(|| "Failed to initialize action state".to_string())
    }
}

impl Default for TauriActionState {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Browser Commands
// ============================================================================

/// Navigate to a URL and capture a page snapshot
#[tauri::command]
pub async fn browser_navigate(
    state: State<'_, TauriActionState>,
    url: String,
) -> Result<PageSnapshot, String> {
    let action_state = state.get_or_init().await?;

    action_state
        .browser_service
        .navigate(&url)
        .await
        .map_err(|e| format!("Navigation failed: {}", e))
}

/// Extract text data from a web page using a CSS selector
#[tauri::command]
pub async fn browser_extract(
    state: State<'_, TauriActionState>,
    url: String,
    selector: String,
) -> Result<String, String> {
    let action_state = state.get_or_init().await?;

    action_state
        .browser_service
        .extract_data(&url, &selector)
        .await
        .map_err(|e| format!("Extraction failed: {}", e))
}

/// Click an element on a web page
#[tauri::command]
pub async fn browser_click(
    state: State<'_, TauriActionState>,
    url: String,
    selector: String,
) -> Result<(), String> {
    let action_state = state.get_or_init().await?;

    action_state
        .browser_service
        .click_element(&url, &selector)
        .await
        .map_err(|e| format!("Click failed: {}", e))
}

/// Fill form fields on a web page
#[tauri::command]
pub async fn browser_fill_form(
    state: State<'_, TauriActionState>,
    url: String,
    fields: Vec<FormField>,
) -> Result<(), String> {
    let action_state = state.get_or_init().await?;

    action_state
        .browser_service
        .fill_form(&url, &fields)
        .await
        .map_err(|e| format!("Form fill failed: {}", e))
}

// ============================================================================
// Filesystem Commands
// ============================================================================

/// Read a file from the sandboxed filesystem
#[tauri::command]
pub async fn fs_read(state: State<'_, TauriActionState>, path: String) -> Result<String, String> {
    let action_state = state.get_or_init().await?;

    action_state
        .filesystem_service
        .read_file(&path)
        .await
        .map_err(|e| format!("Read failed: {}", e))
}

/// Write a file to the sandboxed filesystem
#[tauri::command]
pub async fn fs_write(
    state: State<'_, TauriActionState>,
    path: String,
    content: String,
) -> Result<(), String> {
    let action_state = state.get_or_init().await?;

    action_state
        .filesystem_service
        .write_file(&path, &content)
        .await
        .map_err(|e| format!("Write failed: {}", e))
}

/// List files in a directory
#[tauri::command]
pub async fn fs_list(
    state: State<'_, TauriActionState>,
    path: String,
) -> Result<Vec<FileInfo>, String> {
    let action_state = state.get_or_init().await?;

    action_state
        .filesystem_service
        .list_directory(&path)
        .await
        .map_err(|e| format!("List failed: {}", e))
}

/// Delete a file or directory
#[tauri::command]
pub async fn fs_delete(state: State<'_, TauriActionState>, path: String) -> Result<(), String> {
    let action_state = state.get_or_init().await?;

    action_state
        .filesystem_service
        .delete(&path)
        .await
        .map_err(|e| format!("Delete failed: {}", e))
}

/// Create a directory
#[tauri::command]
pub async fn fs_mkdir(state: State<'_, TauriActionState>, path: String) -> Result<(), String> {
    let action_state = state.get_or_init().await?;

    action_state
        .filesystem_service
        .create_directory(&path)
        .await
        .map_err(|e| format!("Mkdir failed: {}", e))
}

/// Get information about a file or directory
#[tauri::command]
pub async fn fs_info(state: State<'_, TauriActionState>, path: String) -> Result<FileInfo, String> {
    let action_state = state.get_or_init().await?;

    action_state
        .filesystem_service
        .get_info(&path)
        .await
        .map_err(|e| format!("Info failed: {}", e))
}

// ============================================================================
// Permission Commands
// ============================================================================

/// Check if an action permission is granted
#[tauri::command]
pub async fn check_permission(
    state: State<'_, TauriActionState>,
    session_id: Option<String>,
    permission: String,
) -> Result<bool, String> {
    let action_state = state.get_or_init().await?;

    let perm = match permission.as_str() {
        "browser_access" => shannon_api::actions::ActionPermission::BrowserAccess,
        "filesystem_read" => shannon_api::actions::ActionPermission::FilesystemRead,
        "filesystem_write" => shannon_api::actions::ActionPermission::FilesystemWrite,
        _ => return Err(format!("Unknown permission: {}", permission)),
    };

    Ok(action_state
        .permission_manager
        .is_allowed(session_id.as_deref(), perm)
        .await)
}

/// Grant permission for a session
#[tauri::command]
pub async fn grant_permission(
    state: State<'_, TauriActionState>,
    session_id: String,
    permission: String,
) -> Result<(), String> {
    let action_state = state.get_or_init().await?;

    let perm = match permission.as_str() {
        "browser_access" => shannon_api::actions::ActionPermission::BrowserAccess,
        "filesystem_read" => shannon_api::actions::ActionPermission::FilesystemRead,
        "filesystem_write" => shannon_api::actions::ActionPermission::FilesystemWrite,
        _ => return Err(format!("Unknown permission: {}", permission)),
    };

    action_state
        .permission_manager
        .allow_for_session(session_id, perm)
        .await;

    Ok(())
}

/// Grant permission globally
#[tauri::command]
pub async fn grant_permission_always(
    state: State<'_, TauriActionState>,
    permission: String,
) -> Result<(), String> {
    let action_state = state.get_or_init().await?;

    let perm = match permission.as_str() {
        "browser_access" => shannon_api::actions::ActionPermission::BrowserAccess,
        "filesystem_read" => shannon_api::actions::ActionPermission::FilesystemRead,
        "filesystem_write" => shannon_api::actions::ActionPermission::FilesystemWrite,
        _ => return Err(format!("Unknown permission: {}", permission)),
    };

    action_state.permission_manager.allow_always(perm).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tauri_action_state_creation() {
        let state = TauriActionState::new();
        assert!(state.action_state.try_read().is_ok());
    }
}
