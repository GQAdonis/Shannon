//! # Action Permission System
//!
//! Manages permissions for browser and filesystem actions.
//! Supports per-action, per-session, and always-allow permissions.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Action permission manager
#[derive(Clone)]
pub struct PermissionManager {
    always_allowed: Arc<RwLock<HashSet<ActionPermission>>>,
    session_allowed: Arc<RwLock<HashMap<String, HashSet<ActionPermission>>>>,
}

impl PermissionManager {
    /// Create a new permission manager
    pub fn new() -> Self {
        Self {
            always_allowed: Arc::new(RwLock::new(HashSet::new())),
            session_allowed: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if an action permission is granted
    ///
    /// # Arguments
    ///
    /// * `session_id` - Optional session identifier
    /// * `permission` - The permission to check
    ///
    /// # Returns
    ///
    /// True if the permission is granted globally or for the session
    pub async fn is_allowed(&self, session_id: Option<&str>, permission: ActionPermission) -> bool {
        // Check always-allowed permissions
        let always = self.always_allowed.read().await;
        if always.contains(&permission) {
            return true;
        }
        drop(always);

        // Check session-specific permissions
        if let Some(sid) = session_id {
            let session = self.session_allowed.read().await;
            if let Some(perms) = session.get(sid) {
                if perms.contains(&permission) {
                    return true;
                }
            }
        }

        false
    }

    /// Grant permission for a specific session
    ///
    /// # Arguments
    ///
    /// * `session_id` - Session identifier
    /// * `permission` - The permission to grant
    pub async fn allow_for_session(&self, session_id: String, permission: ActionPermission) {
        let mut session = self.session_allowed.write().await;
        session
            .entry(session_id.clone())
            .or_insert_with(HashSet::new)
            .insert(permission);

        debug!(
            "Granted {:?} permission for session: {}",
            permission, session_id
        );
    }

    /// Grant permission globally (always allow)
    ///
    /// # Arguments
    ///
    /// * `permission` - The permission to grant globally
    pub async fn allow_always(&self, permission: ActionPermission) {
        let mut always = self.always_allowed.write().await;
        always.insert(permission);

        info!("Granted {:?} permission globally", permission);
    }

    /// Revoke a global permission
    ///
    /// # Arguments
    ///
    /// * `permission` - The permission to revoke
    pub async fn revoke_always(&self, permission: ActionPermission) {
        let mut always = self.always_allowed.write().await;
        always.remove(&permission);

        info!("Revoked {:?} permission globally", permission);
    }

    /// Clear all permissions for a session
    ///
    /// # Arguments
    ///
    /// * `session_id` - Session identifier
    pub async fn clear_session(&self, session_id: &str) {
        let mut session = self.session_allowed.write().await;
        session.remove(session_id);

        debug!("Cleared permissions for session: {}", session_id);
    }

    /// Get all globally allowed permissions
    pub async fn get_always_allowed(&self) -> Vec<ActionPermission> {
        let always = self.always_allowed.read().await;
        always.iter().copied().collect()
    }

    /// Get all session-specific permissions
    ///
    /// # Arguments
    ///
    /// * `session_id` - Session identifier
    pub async fn get_session_permissions(&self, session_id: &str) -> Vec<ActionPermission> {
        let session = self.session_allowed.read().await;
        session
            .get(session_id)
            .map(|perms| perms.iter().copied().collect())
            .unwrap_or_default()
    }
}

impl Default for PermissionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Types of actions that require permission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionPermission {
    /// Permission to access browser automation
    BrowserAccess,
    /// Permission to read files from sandbox
    FilesystemRead,
    /// Permission to write/delete files in sandbox
    FilesystemWrite,
    /// Permission to execute commands (future)
    CommandExecution,
    /// Permission to access network (future)
    NetworkAccess,
}

impl ActionPermission {
    /// Get a human-readable description of the permission
    pub fn description(&self) -> &'static str {
        match self {
            Self::BrowserAccess => "Access web pages through browser automation",
            Self::FilesystemRead => "Read files from the sandboxed filesystem",
            Self::FilesystemWrite => "Write or delete files in the sandboxed filesystem",
            Self::CommandExecution => "Execute system commands",
            Self::NetworkAccess => "Make network requests",
        }
    }

    /// Get the permission category
    pub fn category(&self) -> &'static str {
        match self {
            Self::BrowserAccess => "Browser",
            Self::FilesystemRead | Self::FilesystemWrite => "Filesystem",
            Self::CommandExecution => "System",
            Self::NetworkAccess => "Network",
        }
    }

    /// Check if this permission is considered high-risk
    pub fn is_high_risk(&self) -> bool {
        matches!(self, Self::CommandExecution | Self::FilesystemWrite)
    }
}

/// Permission approval request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRequest {
    /// The action requesting permission
    pub action_type: String,
    /// Detailed description of the action
    pub description: String,
    /// Required permission
    pub permission: ActionPermission,
    /// Session identifier
    pub session_id: String,
    /// Additional context
    pub context: Option<serde_json::Value>,
}

/// Permission approval response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionResponse {
    /// Whether permission was granted
    pub approved: bool,
    /// Whether to remember this decision
    pub remember: bool,
    /// Scope of remembering (session or always)
    pub scope: PermissionScope,
}

/// Scope for permission approval
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionScope {
    /// One-time approval
    Once,
    /// Remember for this session
    Session,
    /// Always allow
    Always,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_permission_manager() {
        let manager = PermissionManager::new();

        // Initially denied
        assert!(
            !manager
                .is_allowed(Some("session1"), ActionPermission::BrowserAccess)
                .await
        );

        // Allow for session
        manager
            .allow_for_session("session1".to_string(), ActionPermission::BrowserAccess)
            .await;
        assert!(
            manager
                .is_allowed(Some("session1"), ActionPermission::BrowserAccess)
                .await
        );
        assert!(
            !manager
                .is_allowed(Some("session2"), ActionPermission::BrowserAccess)
                .await
        );

        // Allow globally
        manager.allow_always(ActionPermission::FilesystemRead).await;
        assert!(
            manager
                .is_allowed(None, ActionPermission::FilesystemRead)
                .await
        );
        assert!(
            manager
                .is_allowed(Some("session1"), ActionPermission::FilesystemRead)
                .await
        );

        // Clear session
        manager.clear_session("session1").await;
        assert!(
            !manager
                .is_allowed(Some("session1"), ActionPermission::BrowserAccess)
                .await
        );
    }

    #[test]
    fn test_permission_descriptions() {
        assert_eq!(
            ActionPermission::BrowserAccess.description(),
            "Access web pages through browser automation"
        );
        assert_eq!(ActionPermission::BrowserAccess.category(), "Browser");
        assert!(!ActionPermission::BrowserAccess.is_high_risk());
        assert!(ActionPermission::CommandExecution.is_high_risk());
    }

    #[test]
    fn test_permission_scope() {
        let scope = PermissionScope::Once;
        let json = serde_json::to_string(&scope).unwrap();
        assert_eq!(json, r#""once""#);
    }
}
