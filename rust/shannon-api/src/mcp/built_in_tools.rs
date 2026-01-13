//! Built-in MCP tools and pre-configured servers.
//!
//! This module provides built-in tool implementations and seeds common
//! MCP servers like E2B code execution, web search, and filesystem tools.

use anyhow::Result;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

use crate::database::{MCPServerConfig, MCPServerRepository, MCPTool, ServerStatus};

/// Register all built-in MCP servers.
///
/// # Arguments
///
/// * `repository` - Repository to store server configurations
///
/// # Errors
///
/// Returns an error if any server registration fails.
pub async fn register_built_in_servers(repository: Arc<MCPServerRepository>) -> Result<()> {
    // Register E2B as built-in tool
    register_e2b_server(&repository).await?;

    // Register other common servers
    register_filesystem_server(&repository).await?;

    Ok(())
}

/// Register E2B code execution as a built-in MCP server.
///
/// E2B provides secure Python code execution in isolated sandboxes.
async fn register_e2b_server(repository: &MCPServerRepository) -> Result<()> {
    let e2b_server = MCPServerConfig {
        id: "built-in-e2b".to_string(),
        name: "E2B Code Interpreter".to_string(),
        description: "Execute Python code in secure sandbox with E2B".to_string(),
        command: "built-in".to_string(),
        args: vec![],
        env: HashMap::new(),
        auto_start: true,
        status: ServerStatus::Running,
        tools: vec![MCPTool {
            name: "execute_python".to_string(),
            description: "Execute Python code and return results, stdout, and stderr".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "code": {
                        "type": "string",
                        "description": "Python code to execute"
                    },
                    "timeout": {
                        "type": "number",
                        "description": "Timeout in seconds (default: 30)",
                        "default": 30
                    }
                },
                "required": ["code"]
            }),
        }],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // Check if already exists
    if repository.get(&e2b_server.id).await.is_ok() {
        tracing::info!("E2B server already registered, skipping");
        return Ok(());
    }

    repository.create(&e2b_server).await?;
    tracing::info!("Registered E2B as built-in MCP server");

    Ok(())
}

/// Register filesystem MCP server template.
///
/// Provides file read/write capabilities with sandboxing.
async fn register_filesystem_server(repository: &MCPServerRepository) -> Result<()> {
    let fs_server = MCPServerConfig {
        id: "built-in-filesystem".to_string(),
        name: "Filesystem Tools".to_string(),
        description: "Read and write files with security constraints".to_string(),
        command: "built-in".to_string(),
        args: vec![],
        env: HashMap::new(),
        auto_start: false,
        status: ServerStatus::Stopped,
        tools: vec![
            MCPTool {
                name: "read_file".to_string(),
                description: "Read contents of a file".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file to read"
                        }
                    },
                    "required": ["path"]
                }),
            },
            MCPTool {
                name: "write_file".to_string(),
                description: "Write contents to a file".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file to write"
                        },
                        "content": {
                            "type": "string",
                            "description": "Content to write to the file"
                        }
                    },
                    "required": ["path", "content"]
                }),
            },
            MCPTool {
                name: "list_directory".to_string(),
                description: "List contents of a directory".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the directory to list"
                        }
                    },
                    "required": ["path"]
                }),
            },
        ],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // Check if already exists
    if repository.get(&fs_server.id).await.is_ok() {
        tracing::info!("Filesystem server already registered, skipping");
        return Ok(());
    }

    repository.create(&fs_server).await?;
    tracing::info!("Registered filesystem as built-in MCP server");

    Ok(())
}

/// Pre-configured MCP server templates.
///
/// These templates can be instantiated by users with their own configurations.
pub fn get_server_templates() -> Vec<MCPServerTemplate> {
    vec![
        MCPServerTemplate {
            id: "template-github".to_string(),
            name: "GitHub".to_string(),
            description: "Interact with GitHub repositories, issues, and PRs".to_string(),
            command: "npx".to_string(),
            args: vec![
                "-y".to_string(),
                "@modelcontextprotocol/server-github".to_string(),
            ],
            required_env: vec!["GITHUB_TOKEN".to_string()],
        },
        MCPServerTemplate {
            id: "template-google-calendar".to_string(),
            name: "Google Calendar".to_string(),
            description: "Manage Google Calendar events".to_string(),
            command: "npx".to_string(),
            args: vec![
                "-y".to_string(),
                "@modelcontextprotocol/server-google-calendar".to_string(),
            ],
            required_env: vec![
                "GOOGLE_CLIENT_ID".to_string(),
                "GOOGLE_CLIENT_SECRET".to_string(),
            ],
        },
        MCPServerTemplate {
            id: "template-postgresql".to_string(),
            name: "PostgreSQL".to_string(),
            description: "Query and manage PostgreSQL databases".to_string(),
            command: "npx".to_string(),
            args: vec![
                "-y".to_string(),
                "@modelcontextprotocol/server-postgres".to_string(),
            ],
            required_env: vec!["POSTGRES_URL".to_string()],
        },
        MCPServerTemplate {
            id: "template-web-search".to_string(),
            name: "Web Search (Tavily)".to_string(),
            description: "Search the web using Tavily API".to_string(),
            command: "npx".to_string(),
            args: vec![
                "-y".to_string(),
                "@modelcontextprotocol/server-tavily".to_string(),
            ],
            required_env: vec!["TAVILY_API_KEY".to_string()],
        },
        MCPServerTemplate {
            id: "template-slack".to_string(),
            name: "Slack".to_string(),
            description: "Send messages and interact with Slack".to_string(),
            command: "npx".to_string(),
            args: vec![
                "-y".to_string(),
                "@modelcontextprotocol/server-slack".to_string(),
            ],
            required_env: vec!["SLACK_BOT_TOKEN".to_string()],
        },
    ]
}

/// Template for creating new MCP server instances.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MCPServerTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub command: String,
    pub args: Vec<String>,
    pub required_env: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_register_e2b() {
        let temp_file = NamedTempFile::new().unwrap();
        let repo = MCPServerRepository::new(temp_file.path().to_path_buf());
        repo.init_db().unwrap();

        register_e2b_server(&repo).await.unwrap();

        let server = repo.get("built-in-e2b").await.unwrap();
        assert_eq!(server.name, "E2B Code Interpreter");
        assert_eq!(server.tools.len(), 1);
        assert_eq!(server.tools[0].name, "execute_python");
    }

    #[tokio::test]
    async fn test_get_templates() {
        let templates = get_server_templates();
        assert!(!templates.is_empty());
        assert!(templates.iter().any(|t| t.name == "GitHub"));
        assert!(templates.iter().any(|t| t.name == "PostgreSQL"));
    }
}
