//! # Action Engine
//!
//! Provides browser automation and sandboxed filesystem operations for AI agents.
//! Implements Manus.ai "General Action Engine" parity with security controls.

pub mod browser;
pub mod filesystem;
pub mod mcp_registry;
pub mod permissions;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub use browser::{BrowserService, FormField, PageSnapshot};
pub use filesystem::{FileInfo, FilesystemService};
pub use permissions::{ActionPermission, PermissionManager};

/// Unified action state for all services
#[derive(Clone)]
pub struct ActionState {
    pub browser_service: Arc<BrowserService>,
    pub filesystem_service: Arc<FilesystemService>,
    pub permission_manager: Arc<PermissionManager>,
}

impl ActionState {
    /// Create new action state with default configuration
    pub fn new() -> Result<Self> {
        let sandbox_root = std::env::var("SHANNON_SANDBOX_ROOT")
            .unwrap_or_else(|_| "/tmp/shannon_sandbox".to_string());

        Ok(Self {
            browser_service: Arc::new(BrowserService::new()?),
            filesystem_service: Arc::new(FilesystemService::new(sandbox_root.into())?),
            permission_manager: Arc::new(PermissionManager::new()),
        })
    }

    /// Create with custom sandbox root
    pub fn with_sandbox_root(sandbox_root: std::path::PathBuf) -> Result<Self> {
        Ok(Self {
            browser_service: Arc::new(BrowserService::new()?),
            filesystem_service: Arc::new(FilesystemService::new(sandbox_root)?),
            permission_manager: Arc::new(PermissionManager::new()),
        })
    }
}

impl Default for ActionState {
    fn default() -> Self {
        Self::new().expect("Failed to create default ActionState")
    }
}

/// Action request envelope for permission checking
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ActionRequest {
    BrowserNavigate { url: String },
    BrowserExtract { url: String, selector: String },
    BrowserClick { url: String, selector: String },
    BrowserFillForm { url: String, fields: Vec<FormField> },
    FilesystemRead { path: String },
    FilesystemWrite { path: String, content: String },
    FilesystemList { path: String },
    FilesystemDelete { path: String },
}

impl ActionRequest {
    /// Get human-readable description of the action
    pub fn description(&self) -> String {
        match self {
            Self::BrowserNavigate { url } => format!("Navigate to {}", url),
            Self::BrowserExtract { url, selector } => {
                format!("Extract data from {} using selector '{}'", url, selector)
            }
            Self::BrowserClick { url, selector } => {
                format!("Click element '{}' on {}", selector, url)
            }
            Self::BrowserFillForm { url, fields } => {
                format!("Fill {} form fields on {}", fields.len(), url)
            }
            Self::FilesystemRead { path } => format!("Read file {}", path),
            Self::FilesystemWrite { path, .. } => format!("Write file {}", path),
            Self::FilesystemList { path } => format!("List directory {}", path),
            Self::FilesystemDelete { path } => format!("Delete {}", path),
        }
    }

    /// Get the permission required for this action
    pub fn required_permission(&self) -> ActionPermission {
        match self {
            Self::BrowserNavigate { .. }
            | Self::BrowserExtract { .. }
            | Self::BrowserClick { .. }
            | Self::BrowserFillForm { .. } => ActionPermission::BrowserAccess,

            Self::FilesystemRead { .. } => ActionPermission::FilesystemRead,

            Self::FilesystemWrite { .. } | Self::FilesystemDelete { .. } => {
                ActionPermission::FilesystemWrite
            }

            Self::FilesystemList { .. } => ActionPermission::FilesystemRead,
        }
    }
}

/// Action response envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ActionResponse {
    BrowserSnapshot(PageSnapshot),
    BrowserData(String),
    BrowserSuccess,
    FilesystemData(String),
    FilesystemList(Vec<FileInfo>),
    FilesystemSuccess,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_request_description() {
        let action = ActionRequest::BrowserNavigate {
            url: "https://example.com".to_string(),
        };
        assert_eq!(action.description(), "Navigate to https://example.com");

        let action = ActionRequest::FilesystemRead {
            path: "test.txt".to_string(),
        };
        assert_eq!(action.description(), "Read file test.txt");
    }

    #[test]
    fn test_required_permissions() {
        let action = ActionRequest::BrowserNavigate {
            url: "https://example.com".to_string(),
        };
        assert_eq!(
            action.required_permission(),
            ActionPermission::BrowserAccess
        );

        let action = ActionRequest::FilesystemWrite {
            path: "test.txt".to_string(),
            content: "data".to_string(),
        };
        assert_eq!(
            action.required_permission(),
            ActionPermission::FilesystemWrite
        );
    }
}
