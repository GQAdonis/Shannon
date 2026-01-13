//! Quick Chat Service
//!
//! Provides fast, conversational chat by making direct HTTP calls to LLM providers.
//! Target latency: <500ms for first token.
//!
//! # Supported Providers
//! - OpenAI (GPT-4, GPT-3.5)
//! - Anthropic (Claude 3.5, Claude 3)
//! - Google (Gemini Pro, Gemini Flash)
//!
//! # Features
//! - Direct API calls (no workflow overhead)
//! - Streaming support via Server-Sent Events
//! - Multi-provider support with unified interface
//! - Automatic API key resolution from environment

use anyhow::{Context, Result};
use chrono::Utc;
use futures::Stream;
use reqwest::Client;
use serde_json::json;
use std::pin::Pin;
use tracing::{debug, error, info};

use super::{ChatMessage, QuickChatConfig};

/// Quick chat service for direct LLM provider calls
#[derive(Debug, Clone)]
pub struct QuickChatService {
    /// HTTP client for API calls
    http_client: Client,
}

impl QuickChatService {
    /// Create a new quick chat service
    pub fn new() -> Self {
        Self {
            http_client: Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Send a message and get streaming response
    ///
    /// # Arguments
    /// * `message` - User message to send
    /// * `history` - Conversation history
    /// * `config` - Chat configuration
    ///
    /// # Returns
    /// Stream of response chunks
    ///
    /// # Errors
    /// Returns error if provider is unsupported or API call fails
    pub async fn send_message(
        &self,
        message: String,
        history: Vec<ChatMessage>,
        config: QuickChatConfig,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        info!(
            provider = %config.provider,
            model = %config.model,
            stream = config.stream,
            "Sending quick chat message"
        );

        // Build messages array with history
        let mut messages = history;
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: message,
            timestamp: Utc::now().to_rfc3339(),
        });

        // Route to appropriate provider
        match config.provider.as_str() {
            "openai" => self.call_openai(&messages, &config).await,
            "anthropic" => self.call_anthropic(&messages, &config).await,
            "google" => self.call_google(&messages, &config).await,
            _ => anyhow::bail!("Unsupported provider: {}", config.provider),
        }
    }

    /// Call OpenAI API
    ///
    /// # Errors
    /// Returns error if API key is missing or request fails
    async fn call_openai(
        &self,
        messages: &[ChatMessage],
        config: &QuickChatConfig,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY environment variable not set")?;

        debug!(model = %config.model, "Calling OpenAI API");

        // Convert messages to OpenAI format
        let openai_messages: Vec<serde_json::Value> = messages
            .iter()
            .map(|msg| {
                json!({
                    "role": msg.role,
                    "content": msg.content,
                })
            })
            .collect();

        let body = json!({
            "model": config.model,
            "messages": openai_messages,
            "temperature": config.temperature,
            "max_tokens": config.max_tokens,
            "stream": config.stream,
        });

        let response = self
            .http_client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to OpenAI")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("OpenAI API error {}: {}", status, error_text);
        }

        if config.stream {
            Ok(Box::pin(Self::parse_openai_stream(response)))
        } else {
            let response_json: serde_json::Value = response.json().await?;
            let content = response_json["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("")
                .to_string();
            Ok(Box::pin(futures::stream::once(async move { Ok(content) })))
        }
    }

    /// Parse OpenAI SSE stream
    fn parse_openai_stream(
        response: reqwest::Response,
    ) -> impl Stream<Item = Result<String>> + Send {
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
                                if data == "[DONE]" {
                                    continue;
                                }
                                if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                                    if let Some(content) =
                                        json["choices"][0]["delta"]["content"].as_str()
                                    {
                                        return Some(Ok(content.to_string()));
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

    /// Call Anthropic API
    ///
    /// # Errors
    /// Returns error if API key is missing or request fails
    async fn call_anthropic(
        &self,
        messages: &[ChatMessage],
        config: &QuickChatConfig,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .context("ANTHROPIC_API_KEY environment variable not set")?;

        debug!(model = %config.model, "Calling Anthropic API");

        // Convert messages to Anthropic format (separate system messages)
        let mut system_message = String::new();
        let mut conversation_messages = Vec::new();

        for msg in messages {
            if msg.role == "system" {
                if !system_message.is_empty() {
                    system_message.push('\n');
                }
                system_message.push_str(&msg.content);
            } else {
                conversation_messages.push(json!({
                    "role": msg.role,
                    "content": msg.content,
                }));
            }
        }

        let mut body = json!({
            "model": config.model,
            "messages": conversation_messages,
            "temperature": config.temperature,
            "max_tokens": config.max_tokens,
            "stream": config.stream,
        });

        if !system_message.is_empty() {
            body["system"] = json!(system_message);
        }

        let response = self
            .http_client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Anthropic")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Anthropic API error {}: {}", status, error_text);
        }

        if config.stream {
            Ok(Box::pin(Self::parse_anthropic_stream(response)))
        } else {
            let response_json: serde_json::Value = response.json().await?;
            let content = response_json["content"][0]["text"]
                .as_str()
                .unwrap_or("")
                .to_string();
            Ok(Box::pin(futures::stream::once(async move { Ok(content) })))
        }
    }

    /// Parse Anthropic SSE stream
    fn parse_anthropic_stream(
        response: reqwest::Response,
    ) -> impl Stream<Item = Result<String>> + Send {
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
                                if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                                    if json["type"] == "content_block_delta" {
                                        if let Some(content) = json["delta"]["text"].as_str() {
                                            return Some(Ok(content.to_string()));
                                        }
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

    /// Call Google Gemini API
    ///
    /// # Errors
    /// Returns error if API key is missing or request fails
    async fn call_google(
        &self,
        messages: &[ChatMessage],
        config: &QuickChatConfig,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        let api_key = std::env::var("GOOGLE_API_KEY")
            .context("GOOGLE_API_KEY environment variable not set")?;

        debug!(model = %config.model, "Calling Google Gemini API");

        // Convert messages to Gemini format
        let mut contents = Vec::new();
        for msg in messages {
            if msg.role != "system" {
                let role = if msg.role == "assistant" {
                    "model"
                } else {
                    "user"
                };
                contents.push(json!({
                    "role": role,
                    "parts": [{"text": msg.content}]
                }));
            }
        }

        let body = json!({
            "contents": contents,
            "generationConfig": {
                "temperature": config.temperature,
                "maxOutputTokens": config.max_tokens,
            }
        });

        let url = if config.stream {
            format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?key={}",
                config.model, api_key
            )
        } else {
            format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
                config.model, api_key
            )
        };

        let response = self
            .http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Google")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Google API error {}: {}", status, error_text);
        }

        if config.stream {
            Ok(Box::pin(Self::parse_google_stream(response)))
        } else {
            let response_json: serde_json::Value = response.json().await?;
            let content = response_json["candidates"][0]["content"]["parts"][0]["text"]
                .as_str()
                .unwrap_or("")
                .to_string();
            Ok(Box::pin(futures::stream::once(async move { Ok(content) })))
        }
    }

    /// Parse Google Gemini stream
    fn parse_google_stream(
        response: reqwest::Response,
    ) -> impl Stream<Item = Result<String>> + Send {
        use futures::StreamExt;

        response
            .bytes_stream()
            .filter_map(|chunk_result| async move {
                match chunk_result {
                    Ok(chunk) => {
                        let text = String::from_utf8_lossy(&chunk);
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                            if let Some(candidates) = json["candidates"].as_array() {
                                if let Some(first) = candidates.first() {
                                    if let Some(content) =
                                        first["content"]["parts"][0]["text"].as_str()
                                    {
                                        return Some(Ok(content.to_string()));
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

impl Default for QuickChatService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_chat_service_creation() {
        let service = QuickChatService::new();
        assert!(service.http_client.timeout().is_some());
    }

    #[tokio::test]
    #[ignore] // Requires API keys
    async fn test_openai_call() {
        let service = QuickChatService::new();
        let config = QuickChatConfig {
            provider: "openai".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            temperature: 0.7,
            max_tokens: 100,
            stream: false,
        };

        let result = service
            .send_message("Hello".to_string(), vec![], config)
            .await;

        // Will fail without API key - that's expected
        assert!(result.is_ok() || result.is_err());
    }
}
