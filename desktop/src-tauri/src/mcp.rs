//! MCP (Model Context Protocol) server management commands.
//!
//! This module provides Tauri commands for managing MCP servers, including:
//! - Server lifecycle (start, stop, restart)
//! - Tool discovery and execution
//! - Per-conversation tool selection

use serde::{Deserialize, Serialize};
use shannon_api::database::{ConversationTool, MCPServerConfig, MCPServerRepository};
use shannon_api::mcp::{MCPServerManager, MCPToolInfo};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;

/// Shared MCP state for Tauri application.
pub struct MCPState {
    pub manager: Arc<MCPServerManager>,
    pub repository: Arc<MCPServerRepository>,
}

impl MCPState {
    /// Create new MCP state with the given database path.
    ///
    /// # Errors
    ///
    /// Returns an error if the database cannot be initialized.
    pub fn new(db_path: PathBuf) -> anyhow::Result<Self> {
        let repository = Arc::new(MCPServerRepository::new(db_path));
        repository.init_db()?;

        let manager = Arc::new(MCPServerManager::new(repository.clone()));

        Ok(Self {
            manager,
            repository,
        })
    }

    /// Initialize built-in servers.
    pub async fn init_built_in_servers(&self) -> anyhow::Result<()> {
        shannon_api::mcp::register_built_in_servers(self.repository.clone()).await
    }

    /// Start auto-start servers.
    pub async fn start_auto_servers(&self) -> anyhow::Result<()> {
        self.manager.start_auto_servers().await
    }
}

/// List all MCP server configurations.
#[tauri::command]
pub async fn list_mcp_servers(
    state: State<'_, Arc<RwLock<MCPState>>>,
) -> Result<Vec<MCPServerConfig>, String> {
    let mcp_state = state.read().await;
    mcp_state
        .manager
        .list_servers()
        .await
        .map_err(|e| e.to_string())
}

/// Add a new MCP server configuration.
#[tauri::command]
pub async fn add_mcp_server(
    config: MCPServerConfig,
    state: State<'_, Arc<RwLock<MCPState>>>,
) -> Result<String, String> {
    let mcp_state = state.read().await;
    mcp_state
        .manager
        .add_server(config)
        .await
        .map_err(|e| e.to_string())
}

/// Start an MCP server by ID.
#[tauri::command]
pub async fn start_mcp_server(
    id: String,
    state: State<'_, Arc<RwLock<MCPState>>>,
) -> Result<(), String> {
    let mcp_state = state.read().await;
    mcp_state
        .manager
        .start_server(&id)
        .await
        .map_err(|e| e.to_string())
}

/// Stop an MCP server by ID.
#[tauri::command]
pub async fn stop_mcp_server(
    id: String,
    state: State<'_, Arc<RwLock<MCPState>>>,
) -> Result<(), String> {
    let mcp_state = state.read().await;
    mcp_state
        .manager
        .stop_server(&id)
        .await
        .map_err(|e| e.to_string())
}

/// Remove an MCP server configuration.
#[tauri::command]
pub async fn remove_mcp_server(
    id: String,
    state: State<'_, Arc<RwLock<MCPState>>>,
) -> Result<(), String> {
    let mcp_state = state.read().await;
    mcp_state
        .manager
        .remove_server(&id)
        .await
        .map_err(|e| e.to_string())
}

/// List all available tools from running MCP servers.
#[tauri::command]
pub async fn list_mcp_tools(
    state: State<'_, Arc<RwLock<MCPState>>>,
) -> Result<Vec<MCPToolInfo>, String> {
    let mcp_state = state.read().await;
    mcp_state
        .manager
        .list_tools()
        .await
        .map_err(|e| e.to_string())
}

/// Execute a tool on an MCP server.
#[tauri::command]
pub async fn execute_mcp_tool(
    server_id: String,
    tool_name: String,
    args: serde_json::Value,
    state: State<'_, Arc<RwLock<MCPState>>>,
) -> Result<serde_json::Value, String> {
    let mcp_state = state.read().await;
    mcp_state
        .manager
        .execute_tool(&server_id, &tool_name, args)
        .await
        .map_err(|e| e.to_string())
}

/// Get tools enabled for a specific conversation.
#[tauri::command]
pub async fn get_conversation_tools(
    conversation_id: String,
    state: State<'_, Arc<RwLock<MCPState>>>,
) -> Result<Vec<ConversationTool>, String> {
    let mcp_state = state.read().await;
    mcp_state
        .repository
        .get_conversation_tools(&conversation_id)
        .await
        .map_err(|e| e.to_string())
}

/// Set tools for a specific conversation.
#[tauri::command]
pub async fn set_conversation_tools(
    conversation_id: String,
    tools: Vec<ConversationTool>,
    state: State<'_, Arc<RwLock<MCPState>>>,
) -> Result<(), String> {
    let mcp_state = state.read().await;
    mcp_state
        .repository
        .set_conversation_tools(&conversation_id, tools)
        .await
        .map_err(|e| e.to_string())
}

/// Get available MCP server templates.
#[tauri::command]
pub async fn get_mcp_templates() -> Result<Vec<MCPServerTemplate>, String> {
    Ok(shannon_api::mcp::built_in_tools::get_server_templates())
}

/// MCP server template for frontend display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPServerTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub command: String,
    pub args: Vec<String>,
    pub required_env: Vec<String>,
}
