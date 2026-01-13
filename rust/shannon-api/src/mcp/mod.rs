//! Model Context Protocol (MCP) integration.
//!
//! This module provides comprehensive MCP server management including:
//! - Server lifecycle management (start, stop, restart)
//! - Tool discovery and execution
//! - Per-conversation tool selection
//! - Built-in tools (E2B code execution)
//! - Health monitoring

pub mod built_in_tools;
pub mod client;
pub mod manager;

pub use built_in_tools::register_built_in_servers;
pub use client::MCPClient;
pub use manager::{MCPServerManager, MCPToolInfo};
