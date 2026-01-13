//! MCP server configuration and per-conversation tool management.
//!
//! This module provides database storage for Model Context Protocol (MCP) servers,
//! including server configurations, lifecycle status, available tools, and
//! per-conversation tool selection.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// MCP server configuration with available tools.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPServerConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub auto_start: bool,
    pub status: ServerStatus,
    pub tools: Vec<MCPTool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Server lifecycle status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "message")]
pub enum ServerStatus {
    Stopped,
    Starting,
    Running,
    Error(String),
}

impl ServerStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Stopped => "stopped".to_string(),
            Self::Starting => "starting".to_string(),
            Self::Running => "running".to_string(),
            Self::Error(msg) => format!("error:{}", msg),
        }
    }

    fn from_string(s: &str) -> Self {
        if s == "stopped" {
            Self::Stopped
        } else if s == "starting" {
            Self::Starting
        } else if s == "running" {
            Self::Running
        } else if let Some(msg) = s.strip_prefix("error:") {
            Self::Error(msg.to_string())
        } else {
            Self::Stopped
        }
    }
}

/// Tool provided by an MCP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// Per-conversation tool enablement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTool {
    pub server_id: String,
    pub tool_name: String,
    pub enabled: bool,
}

/// Repository for MCP server configurations and conversation tools.
pub struct MCPServerRepository {
    db_path: PathBuf,
}

impl MCPServerRepository {
    /// Create a new repository with the given database path.
    ///
    /// # Arguments
    ///
    /// * `db_path` - Path to the SQLite database file
    pub fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    /// Initialize database schema for MCP servers.
    ///
    /// Creates tables for server configurations and per-conversation tool associations.
    ///
    /// # Errors
    ///
    /// Returns an error if the database cannot be opened or tables cannot be created.
    pub fn init_db(&self) -> Result<()> {
        let conn = Connection::open(&self.db_path)
            .context("Failed to open database for MCP schema initialization")?;

        // MCP server configurations
        conn.execute(
            "CREATE TABLE IF NOT EXISTS mcp_servers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                command TEXT NOT NULL,
                args TEXT NOT NULL,
                env TEXT,
                auto_start INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL,
                tools TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )
        .context("Failed to create mcp_servers table")?;

        // Per-conversation tool associations
        conn.execute(
            "CREATE TABLE IF NOT EXISTS conversation_tools (
                conversation_id TEXT NOT NULL,
                server_id TEXT NOT NULL,
                tool_name TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                PRIMARY KEY (conversation_id, server_id, tool_name),
                FOREIGN KEY (server_id) REFERENCES mcp_servers(id) ON DELETE CASCADE
            )",
            [],
        )
        .context("Failed to create conversation_tools table")?;

        // Index for efficient conversation lookups
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_conversation_tools_conversation 
             ON conversation_tools(conversation_id)",
            [],
        )
        .context("Failed to create conversation_tools index")?;

        Ok(())
    }

    /// Create a new MCP server configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Server configuration to create
    ///
    /// # Returns
    ///
    /// The ID of the created server configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails.
    pub async fn create(&self, config: &MCPServerConfig) -> Result<String> {
        let conn = Connection::open(&self.db_path).context("Failed to open database")?;

        let args_json = serde_json::to_string(&config.args).context("Failed to serialize args")?;
        let env_json = serde_json::to_string(&config.env).context("Failed to serialize env")?;
        let tools_json =
            serde_json::to_string(&config.tools).context("Failed to serialize tools")?;

        conn.execute(
            "INSERT INTO mcp_servers (id, name, description, command, args, env, auto_start, status, tools, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                &config.id,
                &config.name,
                &config.description,
                &config.command,
                args_json,
                env_json,
                config.auto_start as i32,
                config.status.to_string(),
                tools_json,
                config.created_at.to_rfc3339(),
                config.updated_at.to_rfc3339(),
            ],
        )
        .context("Failed to insert MCP server")?;

        Ok(config.id.clone())
    }

    /// Get an MCP server configuration by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Server ID to retrieve
    ///
    /// # Errors
    ///
    /// Returns an error if the server is not found or database operation fails.
    pub async fn get(&self, id: &str) -> Result<MCPServerConfig> {
        let conn = Connection::open(&self.db_path).context("Failed to open database")?;

        let config = conn
            .query_row(
                "SELECT id, name, description, command, args, env, auto_start, status, tools, created_at, updated_at
                 FROM mcp_servers WHERE id = ?1",
                params![id],
                |row| {
                    let args_json: String = row.get(4)?;
                    let env_json: String = row.get(5)?;
                    let tools_json: String = row.get(8)?;
                    let status_str: String = row.get(7)?;
                    let created_at_str: String = row.get(9)?;
                    let updated_at_str: String = row.get(10)?;

                    Ok(MCPServerConfig {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2)?,
                        command: row.get(3)?,
                        args: serde_json::from_str(&args_json).unwrap_or_default(),
                        env: serde_json::from_str(&env_json).unwrap_or_default(),
                        auto_start: row.get::<_, i32>(6)? != 0,
                        status: ServerStatus::from_string(&status_str),
                        tools: serde_json::from_str(&tools_json).unwrap_or_default(),
                        created_at: DateTime::parse_from_rfc3339(&created_at_str)
                            .ok()
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(Utc::now),
                        updated_at: DateTime::parse_from_rfc3339(&updated_at_str)
                            .ok()
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(Utc::now),
                    })
                },
            )
            .optional()
            .context("Failed to query MCP server")?
            .ok_or_else(|| anyhow::anyhow!("MCP server not found: {}", id))?;

        Ok(config)
    }

    /// List all MCP server configurations.
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails.
    pub async fn list(&self) -> Result<Vec<MCPServerConfig>> {
        let conn = Connection::open(&self.db_path).context("Failed to open database")?;

        let mut stmt = conn
            .prepare(
                "SELECT id, name, description, command, args, env, auto_start, status, tools, created_at, updated_at
                 FROM mcp_servers ORDER BY name",
            )
            .context("Failed to prepare statement")?;

        let configs = stmt
            .query_map([], |row| {
                let args_json: String = row.get(4)?;
                let env_json: String = row.get(5)?;
                let tools_json: String = row.get(8)?;
                let status_str: String = row.get(7)?;
                let created_at_str: String = row.get(9)?;
                let updated_at_str: String = row.get(10)?;

                Ok(MCPServerConfig {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    command: row.get(3)?,
                    args: serde_json::from_str(&args_json).unwrap_or_default(),
                    env: serde_json::from_str(&env_json).unwrap_or_default(),
                    auto_start: row.get::<_, i32>(6)? != 0,
                    status: ServerStatus::from_string(&status_str),
                    tools: serde_json::from_str(&tools_json).unwrap_or_default(),
                    created_at: DateTime::parse_from_rfc3339(&created_at_str)
                        .ok()
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(Utc::now),
                    updated_at: DateTime::parse_from_rfc3339(&updated_at_str)
                        .ok()
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(Utc::now),
                })
            })
            .context("Failed to query MCP servers")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect MCP servers")?;

        Ok(configs)
    }

    /// Update an existing MCP server configuration.
    ///
    /// # Arguments
    ///
    /// * `id` - Server ID to update
    /// * `config` - New configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the server is not found or database operation fails.
    pub async fn update(&self, id: &str, config: &MCPServerConfig) -> Result<()> {
        let conn = Connection::open(&self.db_path).context("Failed to open database")?;

        let args_json = serde_json::to_string(&config.args).context("Failed to serialize args")?;
        let env_json = serde_json::to_string(&config.env).context("Failed to serialize env")?;
        let tools_json =
            serde_json::to_string(&config.tools).context("Failed to serialize tools")?;

        let updated = conn
            .execute(
                "UPDATE mcp_servers 
                 SET name = ?2, description = ?3, command = ?4, args = ?5, env = ?6, 
                     auto_start = ?7, status = ?8, tools = ?9, updated_at = ?10
                 WHERE id = ?1",
                params![
                    id,
                    &config.name,
                    &config.description,
                    &config.command,
                    args_json,
                    env_json,
                    config.auto_start as i32,
                    config.status.to_string(),
                    tools_json,
                    Utc::now().to_rfc3339(),
                ],
            )
            .context("Failed to update MCP server")?;

        if updated == 0 {
            anyhow::bail!("MCP server not found: {}", id);
        }

        Ok(())
    }

    /// Delete an MCP server configuration.
    ///
    /// # Arguments
    ///
    /// * `id` - Server ID to delete
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails.
    pub async fn delete(&self, id: &str) -> Result<()> {
        let conn = Connection::open(&self.db_path).context("Failed to open database")?;

        conn.execute("DELETE FROM mcp_servers WHERE id = ?1", params![id])
            .context("Failed to delete MCP server")?;

        Ok(())
    }

    /// Get tools enabled for a specific conversation.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - Conversation ID to query
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails.
    pub async fn get_conversation_tools(
        &self,
        conversation_id: &str,
    ) -> Result<Vec<ConversationTool>> {
        let conn = Connection::open(&self.db_path).context("Failed to open database")?;

        let mut stmt = conn
            .prepare(
                "SELECT server_id, tool_name, enabled 
                 FROM conversation_tools 
                 WHERE conversation_id = ?1",
            )
            .context("Failed to prepare statement")?;

        let tools = stmt
            .query_map(params![conversation_id], |row| {
                Ok(ConversationTool {
                    server_id: row.get(0)?,
                    tool_name: row.get(1)?,
                    enabled: row.get::<_, i32>(2)? != 0,
                })
            })
            .context("Failed to query conversation tools")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect conversation tools")?;

        Ok(tools)
    }

    /// Set tools for a specific conversation.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - Conversation ID
    /// * `tools` - Tools to enable/disable
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails.
    pub async fn set_conversation_tools(
        &self,
        conversation_id: &str,
        tools: Vec<ConversationTool>,
    ) -> Result<()> {
        let conn = Connection::open(&self.db_path).context("Failed to open database")?;

        // Delete existing associations
        conn.execute(
            "DELETE FROM conversation_tools WHERE conversation_id = ?1",
            params![conversation_id],
        )
        .context("Failed to delete existing conversation tools")?;

        // Insert new associations
        for tool in tools {
            conn.execute(
                "INSERT INTO conversation_tools (conversation_id, server_id, tool_name, enabled, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    conversation_id,
                    &tool.server_id,
                    &tool.tool_name,
                    tool.enabled as i32,
                    Utc::now().to_rfc3339(),
                ],
            )
            .context("Failed to insert conversation tool")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_create_and_get_server() {
        let temp_file = NamedTempFile::new().unwrap();
        let repo = MCPServerRepository::new(temp_file.path().to_path_buf());
        repo.init_db().unwrap();

        let config = MCPServerConfig {
            id: "test-server".to_string(),
            name: "Test Server".to_string(),
            description: "A test MCP server".to_string(),
            command: "node".to_string(),
            args: vec!["server.js".to_string()],
            env: HashMap::new(),
            auto_start: true,
            status: ServerStatus::Stopped,
            tools: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let id = repo.create(&config).await.unwrap();
        assert_eq!(id, "test-server");

        let retrieved = repo.get(&id).await.unwrap();
        assert_eq!(retrieved.name, "Test Server");
        assert_eq!(retrieved.command, "node");
    }

    #[tokio::test]
    async fn test_conversation_tools() {
        let temp_file = NamedTempFile::new().unwrap();
        let repo = MCPServerRepository::new(temp_file.path().to_path_buf());
        repo.init_db().unwrap();

        let tools = vec![
            ConversationTool {
                server_id: "server1".to_string(),
                tool_name: "tool1".to_string(),
                enabled: true,
            },
            ConversationTool {
                server_id: "server2".to_string(),
                tool_name: "tool2".to_string(),
                enabled: false,
            },
        ];

        repo.set_conversation_tools("conv1", tools.clone())
            .await
            .unwrap();

        let retrieved = repo.get_conversation_tools("conv1").await.unwrap();
        assert_eq!(retrieved.len(), 2);
        assert_eq!(retrieved[0].server_id, "server1");
        assert!(retrieved[0].enabled);
    }
}
