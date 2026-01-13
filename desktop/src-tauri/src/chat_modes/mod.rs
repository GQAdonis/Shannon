//! Chat Modes Module
//!
//! This module provides dual-mode chat functionality:
//! - **Quick Chat**: Fast, conversational, direct LLM calls (<500ms target)
//! - **Task Chat**: Complex, workflow-based, durable Shannon execution
//!
//! # Architecture
//! ```text
//! ┌─────────────────────────────────────────────────┐
//! │            ModeDetector                         │
//! │  (analyzes query, selects Quick or Task)        │
//! └────────────┬────────────────────────────────────┘
//!              │
//!      ┌───────┴──────┐
//!      ▼              ▼
//! ┌─────────┐   ┌────────────┐
//! │  Quick  │   │    Task    │
//! │  Chat   │   │    Chat    │
//! │(Direct) │   │(Workflow)  │
//! └─────────┘   └────────────┘
//!      │              │
//!      ▼              ▼
//!   OpenAI      Shannon API
//! Anthropic     Orchestrator
//!   Google
//! ```

pub mod detector;
pub mod quick;
pub mod task;

use serde::{Deserialize, Serialize};

/// Chat mode selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatMode {
    /// Quick chat - direct LLM calls, <500ms latency
    Quick,
    /// Task chat - workflow-based, durable execution
    Task,
}

impl ChatMode {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Quick => "quick",
            Self::Task => "task",
        }
    }
}

impl std::fmt::Display for ChatMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Chat message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Role: "user", "assistant", or "system"
    pub role: String,
    /// Message content
    pub content: String,
    /// ISO 8601 timestamp
    pub timestamp: String,
}

/// Configuration for quick chat mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickChatConfig {
    /// LLM provider: "openai", "anthropic", "google"
    pub provider: String,
    /// Model name (e.g., "gpt-4", "claude-3-5-sonnet-20241022")
    pub model: String,
    /// Temperature (0.0-2.0)
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    /// Maximum tokens to generate
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    /// Enable streaming responses
    #[serde(default = "default_stream")]
    pub stream: bool,
}

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> u32 {
    2048
}

fn default_stream() -> bool {
    true
}

/// Configuration for task chat mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskChatConfig {
    /// Workflow strategy: "chain_of_thought", "scientific", "exploratory"
    #[serde(default = "default_strategy")]
    pub strategy: String,
    /// Require human approval before execution
    #[serde(default)]
    pub require_approval: bool,
    /// Maximum number of agents to use
    #[serde(default = "default_max_agents")]
    pub max_agents: u32,
    /// Token budget for the task
    #[serde(default = "default_token_budget")]
    pub token_budget: u32,
    /// Complexity classification
    #[serde(default)]
    pub complexity: TaskComplexity,
}

fn default_strategy() -> String {
    "chain_of_thought".to_string()
}

fn default_max_agents() -> u32 {
    5
}

fn default_token_budget() -> u32 {
    50000
}

/// Task complexity classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskComplexity {
    /// Simple task - Chain of Thought
    Simple,
    /// Complex task - Scientific method
    Complex,
    /// Exploratory task - Tree of Thoughts
    Exploratory,
}

impl Default for TaskComplexity {
    fn default() -> Self {
        Self::Simple
    }
}

/// Task handle for tracking workflow execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskHandle {
    /// Unique task identifier
    pub task_id: String,
    /// Task status
    pub status: String,
    /// Timestamp when task was created
    pub created_at: String,
}

/// Task event from workflow execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEvent {
    /// Event type: "progress", "tool_call", "completion", "error"
    pub event_type: String,
    /// Event payload
    pub payload: serde_json::Value,
    /// Timestamp
    pub timestamp: String,
}
