//! # MCP Tool Registry for Actions
//!
//! Registers browser and filesystem actions as MCP tools for AI agents.

use crate::mcp::MCPServerManager;
use anyhow::Result;
use serde_json::json;
use tracing::info;

/// Register all action tools with the MCP server manager
///
/// # Arguments
///
/// * `manager` - The MCP server manager to register tools with
///
/// # Errors
///
/// Returns an error if tool registration fails
pub async fn register_action_tools(manager: &MCPServerManager) -> Result<()> {
    info!("Registering action tools with MCP");

    // Browser actions
    register_browser_tools(manager).await?;

    // Filesystem actions
    register_filesystem_tools(manager).await?;

    info!("Successfully registered all action tools");

    Ok(())
}

/// Register browser automation tools
async fn register_browser_tools(manager: &MCPServerManager) -> Result<()> {
    // Browser navigate
    manager
        .register_tool(
            "browser_navigate",
            json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The URL to navigate to",
                        "format": "uri"
                    }
                },
                "required": ["url"],
                "additionalProperties": false
            }),
            "Navigate to a web page and capture a screenshot with content".to_string(),
        )
        .await?;

    // Browser extract
    manager
        .register_tool(
            "browser_extract",
            json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The URL to navigate to",
                        "format": "uri"
                    },
                    "selector": {
                        "type": "string",
                        "description": "CSS selector to find the element"
                    }
                },
                "required": ["url", "selector"],
                "additionalProperties": false
            }),
            "Extract text data from a web page using a CSS selector".to_string(),
        )
        .await?;

    // Browser click
    manager
        .register_tool(
            "browser_click",
            json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The URL to navigate to",
                        "format": "uri"
                    },
                    "selector": {
                        "type": "string",
                        "description": "CSS selector for the element to click"
                    }
                },
                "required": ["url", "selector"],
                "additionalProperties": false
            }),
            "Click an element on a web page".to_string(),
        )
        .await?;

    // Browser fill form
    manager
        .register_tool(
            "browser_fill_form",
            json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The URL to navigate to",
                        "format": "uri"
                    },
                    "fields": {
                        "type": "array",
                        "description": "Form fields to fill",
                        "items": {
                            "type": "object",
                            "properties": {
                                "selector": {
                                    "type": "string",
                                    "description": "CSS selector for the field"
                                },
                                "value": {
                                    "type": "string",
                                    "description": "Value to enter"
                                }
                            },
                            "required": ["selector", "value"]
                        }
                    }
                },
                "required": ["url", "fields"],
                "additionalProperties": false
            }),
            "Fill out a form on a web page with multiple fields".to_string(),
        )
        .await?;

    Ok(())
}

/// Register filesystem tools
async fn register_filesystem_tools(manager: &MCPServerManager) -> Result<()> {
    // Filesystem read
    manager
        .register_tool(
            "fs_read",
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "File path relative to sandbox root"
                    }
                },
                "required": ["path"],
                "additionalProperties": false
            }),
            "Read the contents of a file from the sandboxed filesystem".to_string(),
        )
        .await?;

    // Filesystem write
    manager
        .register_tool(
            "fs_write",
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "File path relative to sandbox root"
                    },
                    "content": {
                        "type": "string",
                        "description": "Content to write to the file"
                    }
                },
                "required": ["path", "content"],
                "additionalProperties": false
            }),
            "Write content to a file in the sandboxed filesystem".to_string(),
        )
        .await?;

    // Filesystem list
    manager
        .register_tool(
            "fs_list",
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Directory path relative to sandbox root (use '.' for root)",
                        "default": "."
                    }
                },
                "additionalProperties": false
            }),
            "List files and directories in the sandboxed filesystem".to_string(),
        )
        .await?;

    // Filesystem delete
    manager
        .register_tool(
            "fs_delete",
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "File or directory path to delete"
                    }
                },
                "required": ["path"],
                "additionalProperties": false
            }),
            "Delete a file or directory from the sandboxed filesystem".to_string(),
        )
        .await?;

    // Filesystem create directory
    manager
        .register_tool(
            "fs_mkdir",
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Directory path to create"
                    }
                },
                "required": ["path"],
                "additionalProperties": false
            }),
            "Create a directory in the sandboxed filesystem".to_string(),
        )
        .await?;

    // Filesystem get info
    manager
        .register_tool(
            "fs_info",
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to get information about"
                    }
                },
                "required": ["path"],
                "additionalProperties": false
            }),
            "Get information about a file or directory".to_string(),
        )
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_schemas() {
        // Verify browser_navigate schema
        let schema = json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to navigate to",
                    "format": "uri"
                }
            },
            "required": ["url"],
            "additionalProperties": false
        });

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["url"]["type"].is_string());
    }
}
