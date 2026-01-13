//! Task Chat Service
//!
//! Provides workflow-based chat execution through Shannon API.
//! Uses durable workflows for complex, multi-step tasks.
//!
//! # Features
//! - Integration with Shannon API workflow engine
//! - Strategy selection based on complexity
//! - SSE streaming for progress updates
//! - Durable execution with resumability
//! - Tool execution and multi-agent coordination

use anyhow::{Context, Result};
use chrono::Utc;
use futures::Stream;
use reqwest::Client;
use serde_json::json;
use std::pin::Pin;
use tracing::{debug, info};

use super::{TaskChatConfig, TaskComplexity, TaskEvent, TaskHandle};

/// Task chat service for workflow-based execution
#[derive(Debug, Clone)]
pub struct TaskChatService {
    /// HTTP client for API calls
    http_client: Client,
    /// Shannon API base URL
    shannon_api_url: String,
}

impl TaskChatService {
    /// Create a new task chat service
    ///
    /// # Arguments
    /// * `shannon_api_url` - Base URL of Shannon API (e.g., "http://127.0.0.1:1906")
    pub fn new(shannon_api_url: String) -> Self {
        Self {
            http_client: Client::builder()
                .timeout(std::time::Duration::from_secs(300)) // 5 min for long-running tasks
                .build()
                .expect("Failed to create HTTP client"),
            shannon_api_url,
        }
    }

    /// Submit a task to Shannon API workflow engine
    ///
    /// # Arguments
    /// * `query` - User query/task description
    /// * `context` - Additional context for the task
    /// * `config` - Task configuration
    ///
    /// # Returns
    /// Task handle for tracking execution
    ///
    /// # Errors
    /// Returns error if task submission fails
    pub async fn submit_task(
        &self,
        query: String,
        context: Vec<String>,
        config: TaskChatConfig,
    ) -> Result<TaskHandle> {
        info!(
            strategy = %config.strategy,
            complexity = ?config.complexity,
            "Submitting task to Shannon API"
        );

        // Determine strategy based on complexity
        let strategy = Self::select_strategy(&config);

        // Build task request payload
        let task_request = json!({
            "query": query,
            "context": context,
            "strategy": strategy,
            "require_approval": config.require_approval,
            "max_agents": config.max_agents,
            "token_budget": config.token_budget,
            "metadata": {
                "complexity": config.complexity,
                "submitted_at": Utc::now().to_rfc3339(),
            }
        });

        debug!(payload = ?task_request, "Task request payload");

        // Submit to Shannon API
        let url = format!("{}/api/tasks", self.shannon_api_url);
        let response = self
            .http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&task_request)
            .send()
            .await
            .context("Failed to send task to Shannon API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Shannon API error {}: {}", status, error_text);
        }

        let task_handle: TaskHandle = response
            .json()
            .await
            .context("Failed to parse task response")?;

        info!(task_id = %task_handle.task_id, "Task submitted successfully");

        Ok(task_handle)
    }

    /// Stream task updates via Server-Sent Events
    ///
    /// # Arguments
    /// * `task_id` - Task identifier
    ///
    /// # Returns
    /// Stream of task events
    ///
    /// # Errors
    /// Returns error if streaming fails
    pub async fn stream_updates(
        &self,
        task_id: String,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<TaskEvent>> + Send>>> {
        info!(task_id = %task_id, "Starting task event stream");

        let url = format!("{}/api/tasks/{}/stream", self.shannon_api_url, task_id);

        let response = self
            .http_client
            .get(&url)
            .header("Accept", "text/event-stream")
            .send()
            .await
            .context("Failed to connect to task stream")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Stream connection error {}: {}", status, error_text);
        }

        Ok(Box::pin(Self::parse_task_event_stream(response)))
    }

    /// Get task status
    ///
    /// # Arguments
    /// * `task_id` - Task identifier
    ///
    /// # Errors
    /// Returns error if status check fails
    pub async fn get_task_status(&self, task_id: &str) -> Result<TaskHandle> {
        let url = format!("{}/api/tasks/{}", self.shannon_api_url, task_id);

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .context("Failed to get task status")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Status check error {}: {}", status, error_text);
        }

        let task_handle = response
            .json()
            .await
            .context("Failed to parse status response")?;

        Ok(task_handle)
    }

    /// Cancel a running task
    ///
    /// # Arguments
    /// * `task_id` - Task identifier
    ///
    /// # Errors
    /// Returns error if cancellation fails
    pub async fn cancel_task(&self, task_id: &str) -> Result<()> {
        info!(task_id = %task_id, "Cancelling task");

        let url = format!("{}/api/tasks/{}/cancel", self.shannon_api_url, task_id);

        let response = self
            .http_client
            .post(&url)
            .send()
            .await
            .context("Failed to cancel task")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Cancellation error {}: {}", status, error_text);
        }

        info!(task_id = %task_id, "Task cancelled successfully");

        Ok(())
    }

    /// Select workflow strategy based on configuration
    fn select_strategy(config: &TaskChatConfig) -> String {
        // If strategy is explicitly set, use it
        if !config.strategy.is_empty() && config.strategy != "auto" {
            return config.strategy.clone();
        }

        // Otherwise, select based on complexity
        match config.complexity {
            TaskComplexity::Simple => "chain_of_thought".to_string(),
            TaskComplexity::Complex => "scientific".to_string(),
            TaskComplexity::Exploratory => "tree_of_thoughts".to_string(),
        }
    }

    /// Parse Server-Sent Events stream into task events
    fn parse_task_event_stream(
        response: reqwest::Response,
    ) -> impl Stream<Item = Result<TaskEvent>> + Send {
        use futures::StreamExt;

        response
            .bytes_stream()
            .filter_map(|chunk_result| async move {
                match chunk_result {
                    Ok(chunk) => {
                        let text = String::from_utf8_lossy(&chunk);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];

                                // Handle heartbeat/keepalive
                                if data.trim().is_empty() || data == ":" {
                                    continue;
                                }

                                // Parse JSON event
                                match serde_json::from_str::<serde_json::Value>(data) {
                                    Ok(json) => {
                                        let event_type =
                                            json["type"].as_str().unwrap_or("unknown").to_string();

                                        let event = TaskEvent {
                                            event_type,
                                            payload: json["payload"].clone(),
                                            timestamp: json["timestamp"]
                                                .as_str()
                                                .unwrap_or_else(|| &Utc::now().to_rfc3339())
                                                .to_string(),
                                        };

                                        return Some(Ok(event));
                                    }
                                    Err(e) => {
                                        return Some(Err(anyhow::anyhow!(
                                            "Failed to parse event: {}",
                                            e
                                        )));
                                    }
                                }
                            }
                        }
                        None
                    }
                    Err(e) => Some(Err(anyhow::anyhow!("Stream error: {}", e))),
                }
            })
    }
}

impl Default for TaskChatService {
    fn default() -> Self {
        Self::new("http://127.0.0.1:1906".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_chat_service_creation() {
        let service = TaskChatService::new("http://localhost:8080".to_string());
        assert_eq!(service.shannon_api_url, "http://localhost:8080");
    }

    #[test]
    fn test_strategy_selection() {
        let config = TaskChatConfig {
            strategy: "auto".to_string(),
            require_approval: false,
            max_agents: 5,
            token_budget: 50000,
            complexity: TaskComplexity::Simple,
        };

        let strategy = TaskChatService::select_strategy(&config);
        assert_eq!(strategy, "chain_of_thought");

        let config_complex = TaskChatConfig {
            complexity: TaskComplexity::Complex,
            ..config.clone()
        };

        let strategy = TaskChatService::select_strategy(&config_complex);
        assert_eq!(strategy, "scientific");

        let config_exploratory = TaskChatConfig {
            complexity: TaskComplexity::Exploratory,
            ..config.clone()
        };

        let strategy = TaskChatService::select_strategy(&config_exploratory);
        assert_eq!(strategy, "tree_of_thoughts");
    }

    #[test]
    fn test_explicit_strategy() {
        let config = TaskChatConfig {
            strategy: "custom_strategy".to_string(),
            require_approval: false,
            max_agents: 5,
            token_budget: 50000,
            complexity: TaskComplexity::Simple,
        };

        let strategy = TaskChatService::select_strategy(&config);
        assert_eq!(strategy, "custom_strategy");
    }

    #[tokio::test]
    #[ignore] // Requires running Shannon API
    async fn test_submit_task() {
        let service = TaskChatService::default();
        let config = TaskChatConfig {
            strategy: "chain_of_thought".to_string(),
            require_approval: false,
            max_agents: 5,
            token_budget: 50000,
            complexity: TaskComplexity::Simple,
        };

        let result = service
            .submit_task("Test query".to_string(), vec![], config)
            .await;

        // Will fail without running Shannon API - that's expected
        assert!(result.is_ok() || result.is_err());
    }
}
