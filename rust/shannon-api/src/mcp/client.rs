//! MCP client for communicating with MCP servers via stdio.
//!
//! This module implements the Model Context Protocol (MCP) client that communicates
//! with MCP servers using JSON-RPC over standard input/output streams.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, ChildStdout};
use tokio::sync::Mutex;

use crate::database::MCPTool;

/// MCP client for JSON-RPC communication over stdio.
pub struct MCPClient {
    stdin: Arc<Mutex<ChildStdin>>,
    stdout: Arc<Mutex<BufReader<ChildStdout>>>,
    request_id: Arc<AtomicU64>,
}

impl MCPClient {
    /// Connect to an MCP server using stdio streams.
    ///
    /// # Arguments
    ///
    /// * `stdin` - Process standard input stream
    /// * `stdout` - Process standard output stream
    ///
    /// # Errors
    ///
    /// Returns an error if the streams cannot be initialized.
    pub async fn connect_stdio(stdin: ChildStdin, stdout: ChildStdout) -> Result<Self> {
        Ok(Self {
            stdin: Arc::new(Mutex::new(stdin)),
            stdout: Arc::new(Mutex::new(BufReader::new(stdout))),
            request_id: Arc::new(AtomicU64::new(1)),
        })
    }

    /// List available tools from the MCP server.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or response is invalid.
    pub async fn list_tools(&self) -> Result<Vec<MCPTool>> {
        let request = json!({
            "jsonrpc": "2.0",
            "id": self.next_id(),
            "method": "tools/list",
            "params": {}
        });

        let response = self.send_request(request).await?;

        // Parse tools from response
        let tools_value = response
            .get("result")
            .and_then(|r| r.get("tools"))
            .ok_or_else(|| anyhow::anyhow!("Invalid tools/list response"))?;

        let tools: Vec<MCPTool> =
            serde_json::from_value(tools_value.clone()).context("Failed to parse tools")?;

        Ok(tools)
    }

    /// Call a tool on the MCP server.
    ///
    /// # Arguments
    ///
    /// * `name` - Tool name to invoke
    /// * `arguments` - Tool arguments as JSON
    ///
    /// # Errors
    ///
    /// Returns an error if the tool call fails.
    pub async fn call_tool(
        &self,
        name: &str,
        arguments: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let request = json!({
            "jsonrpc": "2.0",
            "id": self.next_id(),
            "method": "tools/call",
            "params": {
                "name": name,
                "arguments": arguments
            }
        });

        let response = self.send_request(request).await?;

        let result = response
            .get("result")
            .ok_or_else(|| anyhow::anyhow!("No result in tool call response"))?
            .clone();

        Ok(result)
    }

    /// Send a JSON-RPC request and wait for response.
    ///
    /// # Arguments
    ///
    /// * `request` - JSON-RPC request object
    ///
    /// # Errors
    ///
    /// Returns an error if communication fails or the server returns an error.
    async fn send_request(&self, request: serde_json::Value) -> Result<serde_json::Value> {
        // Write request
        let req_str = serde_json::to_string(&request).context("Failed to serialize request")?;

        let mut stdin = self.stdin.lock().await;
        stdin
            .write_all(req_str.as_bytes())
            .await
            .context("Failed to write request")?;
        stdin
            .write_all(b"\n")
            .await
            .context("Failed to write newline")?;
        stdin.flush().await.context("Failed to flush stdin")?;
        drop(stdin); // Release lock

        // Read response
        let mut stdout = self.stdout.lock().await;
        let mut line = String::new();
        stdout
            .read_line(&mut line)
            .await
            .context("Failed to read response")?;

        if line.trim().is_empty() {
            anyhow::bail!("Empty response from MCP server");
        }

        let response: serde_json::Value =
            serde_json::from_str(&line).context("Failed to parse response")?;

        // Check for JSON-RPC error
        if let Some(error) = response.get("error") {
            anyhow::bail!("MCP error: {}", error);
        }

        Ok(response)
    }

    /// Get the next request ID.
    fn next_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }
}

/// JSON-RPC error response.
#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_id_increment() {
        let client = MCPClient {
            stdin: Arc::new(Mutex::new(unsafe {
                std::mem::zeroed() // Not actually used in test
            })),
            stdout: Arc::new(Mutex::new(BufReader::new(unsafe {
                std::mem::zeroed() // Not actually used in test
            }))),
            request_id: Arc::new(AtomicU64::new(1)),
        };

        assert_eq!(client.next_id(), 1);
        assert_eq!(client.next_id(), 2);
        assert_eq!(client.next_id(), 3);
    }
}
