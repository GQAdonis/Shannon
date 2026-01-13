//! Summarization service for context management
//!
//! Provides LLM-based summarization for compressing conversation history.

use anyhow::{Context as AnyhowContext, Result};
use async_trait::async_trait;
use serde_json::json;

/// Trait for text summarization implementations
///
/// Allows for different summarization backends (LLM providers, local models, etc.).
#[async_trait]
pub trait Summarizer: Send + Sync {
    /// Summarize text using the specified model
    ///
    /// # Arguments
    ///
    /// * `text` - Text to summarize
    /// * `model` - Model identifier (e.g., "claude-haiku-4-5@20251001")
    ///
    /// # Returns
    ///
    /// Summarized text
    async fn summarize(&self, text: &str, model: &str) -> Result<String>;
}

/// LLM-based summarizer using HTTP API calls
///
/// Supports multiple providers (Anthropic, OpenAI, Google) based on model string.
#[derive(Debug, Clone)]
pub struct LLMSummarizer {
    http_client: reqwest::Client,
    anthropic_api_key: Option<String>,
    openai_api_key: Option<String>,
    google_api_key: Option<String>,
}

impl LLMSummarizer {
    /// Create a new LLM summarizer
    ///
    /// # Arguments
    ///
    /// * `anthropic_api_key` - Optional Anthropic API key
    /// * `openai_api_key` - Optional OpenAI API key
    /// * `google_api_key` - Optional Google API key
    pub fn new(
        anthropic_api_key: Option<String>,
        openai_api_key: Option<String>,
        google_api_key: Option<String>,
    ) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            anthropic_api_key,
            openai_api_key,
            google_api_key,
        }
    }

    /// Parse model string to determine provider and model name
    ///
    /// # Arguments
    ///
    /// * `model` - Model string (e.g., "claude-haiku-4-5@20251001 | Vertex AI")
    ///
    /// # Returns
    ///
    /// (provider, model_name) tuple
    fn parse_model(model: &str) -> Result<(String, String)> {
        let model = model.split('|').next().unwrap_or(model).trim();

        if model.contains("claude") || model.contains("anthropic") {
            Ok(("anthropic".to_string(), model.to_string()))
        } else if model.contains("gpt") || model.contains("openai") {
            Ok(("openai".to_string(), model.to_string()))
        } else if model.contains("gemini") || model.contains("google") {
            Ok(("google".to_string(), model.to_string()))
        } else {
            // Default to Anthropic Claude Haiku
            Ok(("anthropic".to_string(), "claude-haiku-4-5".to_string()))
        }
    }

    /// Call Anthropic API for summarization
    async fn call_anthropic(&self, model: &str, messages: &[serde_json::Value]) -> Result<String> {
        let api_key = self
            .anthropic_api_key
            .as_ref()
            .context("Anthropic API key not configured")?;

        let response = self
            .http_client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&json!({
                "model": model,
                "max_tokens": 1024,
                "messages": messages,
            }))
            .send()
            .await
            .context("Failed to call Anthropic API")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Anthropic API error {}: {}", status, body);
        }

        let body: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse Anthropic response")?;

        let content = body["content"][0]["text"]
            .as_str()
            .context("Invalid Anthropic response format")?;

        Ok(content.to_string())
    }

    /// Call OpenAI API for summarization
    async fn call_openai(&self, model: &str, messages: &[serde_json::Value]) -> Result<String> {
        let api_key = self
            .openai_api_key
            .as_ref()
            .context("OpenAI API key not configured")?;

        let response = self
            .http_client
            .post("https://api.openai.com/v1/chat/completions")
            .header("authorization", format!("Bearer {}", api_key))
            .header("content-type", "application/json")
            .json(&json!({
                "model": model,
                "messages": messages,
                "max_tokens": 1024,
            }))
            .send()
            .await
            .context("Failed to call OpenAI API")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("OpenAI API error {}: {}", status, body);
        }

        let body: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse OpenAI response")?;

        let content = body["choices"][0]["message"]["content"]
            .as_str()
            .context("Invalid OpenAI response format")?;

        Ok(content.to_string())
    }

    /// Call Google AI API for summarization
    async fn call_google(&self, model: &str, messages: &[serde_json::Value]) -> Result<String> {
        let api_key = self
            .google_api_key
            .as_ref()
            .context("Google API key not configured")?;

        // Convert messages to Google format
        let parts: Vec<_> = messages
            .iter()
            .map(|msg| {
                json!({
                    "text": msg["content"].as_str().unwrap_or_default()
                })
            })
            .collect();

        let response = self
            .http_client
            .post(format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
                model
            ))
            .header("x-goog-api-key", api_key)
            .header("content-type", "application/json")
            .json(&json!({
                "contents": [{
                    "parts": parts
                }]
            }))
            .send()
            .await
            .context("Failed to call Google AI API")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Google AI API error {}: {}", status, body);
        }

        let body: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse Google AI response")?;

        let content = body["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .context("Invalid Google AI response format")?;

        Ok(content.to_string())
    }
}

#[async_trait]
impl Summarizer for LLMSummarizer {
    async fn summarize(&self, text: &str, model: &str) -> Result<String> {
        let (provider, model_name) = Self::parse_model(model)?;

        let messages = vec![json!({
            "role": "user",
            "content": text,
        })];

        match provider.as_str() {
            "anthropic" => self.call_anthropic(&model_name, &messages).await,
            "openai" => self.call_openai(&model_name, &messages).await,
            "google" => self.call_google(&model_name, &messages).await,
            _ => anyhow::bail!("Unsupported provider: {}", provider),
        }
    }
}

/// Mock summarizer for testing
///
/// Returns a simple truncated version of the input text.
#[derive(Debug, Clone, Default)]
pub struct MockSummarizer;

#[async_trait]
impl Summarizer for MockSummarizer {
    async fn summarize(&self, text: &str, _model: &str) -> Result<String> {
        // Simple mock: take first 100 characters and add "..."
        if text.len() > 100 {
            Ok(format!("{}...", &text[..100]))
        } else {
            Ok(text.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_model() {
        let (provider, model) = LLMSummarizer::parse_model("claude-haiku-4-5@20251001").unwrap();
        assert_eq!(provider, "anthropic");
        assert_eq!(model, "claude-haiku-4-5@20251001");

        let (provider, model) = LLMSummarizer::parse_model("gpt-4o-mini").unwrap();
        assert_eq!(provider, "openai");
        assert_eq!(model, "gpt-4o-mini");

        let (provider, model) = LLMSummarizer::parse_model("gemini-1.5-flash").unwrap();
        assert_eq!(provider, "google");
        assert_eq!(model, "gemini-1.5-flash");
    }

    #[tokio::test]
    async fn test_mock_summarizer() {
        let summarizer = MockSummarizer;
        let text = "This is a short text.";
        let summary = summarizer.summarize(text, "any-model").await.unwrap();
        assert_eq!(summary, text);

        let long_text = "A".repeat(150);
        let summary = summarizer.summarize(&long_text, "any-model").await.unwrap();
        assert_eq!(summary.len(), 103); // 100 + "..."
    }
}
