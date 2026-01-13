//! MCP server lifecycle management.
//!
//! This module manages MCP server processes, including starting, stopping,
//! health monitoring, and tool execution.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::RwLock;

use super::client::MCPClient;
use crate::database::{MCPServerConfig, MCPServerRepository, MCPTool, ServerStatus};

/// Information about an MCP tool with server context.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MCPToolInfo {
    pub server_id: String,
    pub server_name: String,
    pub tool_name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// Running MCP server instance.
struct RunningServer {
    config: MCPServerConfig,
    process: Child,
    client: MCPClient,
    last_health_check: chrono::DateTime<chrono::Utc>,
}

/// Manager for MCP server lifecycle and tool execution.
pub struct MCPServerManager {
    servers: Arc<RwLock<HashMap<String, RunningServer>>>,
    repository: Arc<MCPServerRepository>,
}

impl MCPServerManager {
    /// Create a new MCP server manager.
    ///
    /// # Arguments
    ///
    /// * `repository` - Repository for server configurations
    pub fn new(repository: Arc<MCPServerRepository>) -> Self {
        Self {
            servers: Arc::new(RwLock::new(HashMap::new())),
            repository,
        }
    }

    /// Start an MCP server by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Server ID to start
    ///
    /// # Errors
    ///
    /// Returns an error if the server cannot be started or connected.
    pub async fn start_server(&self, id: &str) -> Result<()> {
        let config = self.repository.get(id).await?;

        // Update status to starting
        let mut starting_config = config.clone();
        starting_config.status = ServerStatus::Starting;
        self.repository.update(id, &starting_config).await?;

        // Start subprocess
        let mut command = Command::new(&config.command);
        command
            .args(&config.args)
            .envs(&config.env)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true);

        let mut child = command
            .spawn()
            .context(format!("Failed to spawn MCP server: {}", config.name))?;

        // Wait for server to be ready
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // Connect MCP client
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("Failed to get stdin"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("Failed to get stdout"))?;

        let client = MCPClient::connect_stdio(stdin, stdout)
            .await
            .context("Failed to connect MCP client")?;

        // Fetch available tools
        let tools = match client.list_tools().await {
            Ok(tools) => tools,
            Err(e) => {
                // Kill the process if we can't get tools
                let _ = child.kill().await;

                // Update status to error
                let mut error_config = config.clone();
                error_config.status = ServerStatus::Error(format!("Failed to list tools: {}", e));
                self.repository.update(id, &error_config).await?;

                anyhow::bail!("Failed to list tools from MCP server: {}", e);
            }
        };

        // Update config with tools and running status
        let mut updated_config = config.clone();
        updated_config.tools = tools;
        updated_config.status = ServerStatus::Running;
        self.repository.update(id, &updated_config).await?;

        // Store running server
        self.servers.write().await.insert(
            id.to_string(),
            RunningServer {
                config: updated_config,
                process: child,
                client,
                last_health_check: chrono::Utc::now(),
            },
        );

        Ok(())
    }

    /// Stop an MCP server by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Server ID to stop
    ///
    /// # Errors
    ///
    /// Returns an error if the server cannot be stopped.
    pub async fn stop_server(&self, id: &str) -> Result<()> {
        let mut servers = self.servers.write().await;

        if let Some(mut server) = servers.remove(id) {
            // Kill the process
            server
                .process
                .kill()
                .await
                .context("Failed to kill MCP server process")?;

            // Wait for process to exit
            let _ = server.process.wait().await;

            // Update status in database
            let mut config = server.config;
            config.status = ServerStatus::Stopped;
            self.repository.update(id, &config).await?;
        }

        Ok(())
    }

    /// Execute a tool on a running MCP server.
    ///
    /// # Arguments
    ///
    /// * `server_id` - Server ID
    /// * `tool_name` - Tool name to invoke
    /// * `args` - Tool arguments as JSON
    ///
    /// # Errors
    ///
    /// Returns an error if the server is not running or tool execution fails.
    pub async fn execute_tool(
        &self,
        server_id: &str,
        tool_name: &str,
        args: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let servers = self.servers.read().await;
        let server = servers
            .get(server_id)
            .ok_or_else(|| anyhow::anyhow!("Server not running: {}", server_id))?;

        server
            .client
            .call_tool(tool_name, args)
            .await
            .context(format!("Failed to execute tool: {}", tool_name))
    }

    /// List all available tools from all running servers.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn list_tools(&self) -> Result<Vec<MCPToolInfo>> {
        let servers = self.servers.read().await;

        let mut all_tools = Vec::new();
        for (server_id, server) in servers.iter() {
            for tool in &server.config.tools {
                all_tools.push(MCPToolInfo {
                    server_id: server_id.clone(),
                    server_name: server.config.name.clone(),
                    tool_name: tool.name.clone(),
                    description: tool.description.clone(),
                    input_schema: tool.input_schema.clone(),
                });
            }
        }

        Ok(all_tools)
    }

    /// Get the current list of all servers from the repository.
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails.
    pub async fn list_servers(&self) -> Result<Vec<MCPServerConfig>> {
        self.repository.list().await
    }

    /// Add a new MCP server configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Server configuration to add
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails.
    pub async fn add_server(&self, config: MCPServerConfig) -> Result<String> {
        self.repository.create(&config).await
    }

    /// Remove an MCP server configuration and stop it if running.
    ///
    /// # Arguments
    ///
    /// * `id` - Server ID to remove
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn remove_server(&self, id: &str) -> Result<()> {
        // Stop if running
        let _ = self.stop_server(id).await;

        // Remove from database
        self.repository.delete(id).await
    }

    /// Start servers marked for auto-start.
    ///
    /// # Errors
    ///
    /// Returns an error if any auto-start server fails to start.
    pub async fn start_auto_servers(&self) -> Result<()> {
        let all_servers = self.repository.list().await?;

        for server in all_servers {
            if server.auto_start {
                if let Err(e) = self.start_server(&server.id).await {
                    tracing::warn!("Failed to auto-start MCP server {}: {}", server.name, e);
                }
            }
        }

        Ok(())
    }

    /// Perform health check on running servers.
    ///
    /// This should be called periodically to ensure servers are still responsive.
    pub async fn health_check(&self) -> Result<()> {
        let mut servers = self.servers.write().await;
        let mut failed_servers = Vec::new();

        for (id, server) in servers.iter_mut() {
            // Try to list tools as a health check
            match server.client.list_tools().await {
                Ok(_) => {
                    server.last_health_check = chrono::Utc::now();
                }
                Err(e) => {
                    tracing::error!("Health check failed for server {}: {}", id, e);
                    failed_servers.push(id.clone());
                }
            }
        }

        // Remove and update failed servers
        for id in failed_servers {
            if let Some(mut server) = servers.remove(&id) {
                let _ = server.process.kill().await;

                let mut config = server.config;
                config.status = ServerStatus::Error("Health check failed".to_string());
                let _ = self.repository.update(&id, &config).await;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_manager_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let repo = MCPServerRepository::new(temp_file.path().to_path_buf());
        repo.init_db().unwrap();

        let manager = MCPServerManager::new(Arc::new(repo));
        let servers = manager.list_servers().await.unwrap();
        assert_eq!(servers.len(), 0);
    }

    #[tokio::test]
    async fn test_add_server() {
        let temp_file = NamedTempFile::new().unwrap();
        let repo = MCPServerRepository::new(temp_file.path().to_path_buf());
        repo.init_db().unwrap();

        let manager = MCPServerManager::new(Arc::new(repo));

        let config = MCPServerConfig {
            id: "test-server".to_string(),
            name: "Test Server".to_string(),
            description: "A test server".to_string(),
            command: "echo".to_string(),
            args: vec![],
            env: HashMap::new(),
            auto_start: false,
            status: ServerStatus::Stopped,
            tools: vec![],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let id = manager.add_server(config).await.unwrap();
        assert_eq!(id, "test-server");

        let servers = manager.list_servers().await.unwrap();
        assert_eq!(servers.len(), 1);
    }
}
