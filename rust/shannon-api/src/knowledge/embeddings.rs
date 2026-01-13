//! Embedding generation service.
//!
//! Supports:
//! - **OpenAI**: text-embedding-3-small, text-embedding-3-large, text-embedding-ada-002
//! - **Local**: Future support for local embedding models

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Embedding provider trait.
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    /// Generate embedding for a single text.
    ///
    /// # Errors
    ///
    /// Returns an error if embedding generation fails.
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;

    /// Generate embeddings for multiple texts in batch.
    ///
    /// # Errors
    ///
    /// Returns an error if embedding generation fails.
    async fn embed_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>>;

    /// Get the embedding dimension for this provider.
    fn dimension(&self) -> usize;
}

/// OpenAI embeddings provider.
pub struct OpenAIEmbeddings {
    api_key: String,
    model: String,
    dimension: usize,
    http_client: reqwest::Client,
}

impl OpenAIEmbeddings {
    /// Create a new OpenAI embeddings provider.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be initialized.
    pub fn new(api_key: String, model: String) -> Result<Self> {
        let dimension = match model.as_str() {
            "text-embedding-3-small" => 1536,
            "text-embedding-3-large" => 3072,
            "text-embedding-ada-002" => 1536,
            _ => 1536, // Default
        };

        Ok(Self {
            api_key,
            model,
            dimension,
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()?,
        })
    }

    /// Create with default model (text-embedding-3-small).
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be initialized.
    pub fn new_default(api_key: String) -> Result<Self> {
        Self::new(api_key, "text-embedding-3-small".to_string())
    }
}

#[derive(Debug, Serialize)]
struct OpenAIEmbeddingRequest {
    input: serde_json::Value,
    model: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIEmbeddingResponse {
    data: Vec<OpenAIEmbeddingData>,
}

#[derive(Debug, Deserialize)]
struct OpenAIEmbeddingData {
    embedding: Vec<f32>,
}

#[async_trait]
impl EmbeddingProvider for OpenAIEmbeddings {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let request = OpenAIEmbeddingRequest {
            input: serde_json::json!(text),
            model: self.model.clone(),
        };

        let response = self
            .http_client
            .post("https://api.openai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("OpenAI API error: {error_text}"));
        }

        let result: OpenAIEmbeddingResponse = response
            .json()
            .await
            .context("Failed to parse OpenAI response")?;

        result
            .data
            .first()
            .map(|d| d.embedding.clone())
            .ok_or_else(|| anyhow::anyhow!("No embedding in response"))
    }

    async fn embed_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        let request = OpenAIEmbeddingRequest {
            input: serde_json::json!(texts),
            model: self.model.clone(),
        };

        let response = self
            .http_client
            .post("https://api.openai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .context("Failed to send batch request to OpenAI")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("OpenAI API error: {error_text}"));
        }

        let result: OpenAIEmbeddingResponse = response
            .json()
            .await
            .context("Failed to parse OpenAI response")?;

        Ok(result.data.into_iter().map(|d| d.embedding).collect())
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

/// Create an embedding provider from configuration.
///
/// # Errors
///
/// Returns an error if the provider cannot be initialized.
pub fn create_provider(
    provider_type: &str,
    api_key: Option<String>,
    model: Option<String>,
) -> Result<Arc<dyn EmbeddingProvider>> {
    match provider_type {
        "openai" => {
            let api_key = api_key.ok_or_else(|| anyhow::anyhow!("OpenAI API key required"))?;
            let embeddings = if let Some(model) = model {
                OpenAIEmbeddings::new(api_key, model)?
            } else {
                OpenAIEmbeddings::new_default(api_key)?
            };
            Ok(Arc::new(embeddings))
        }
        _ => Err(anyhow::anyhow!(
            "Unsupported embedding provider: {provider_type}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_dimensions() -> Result<()> {
        let embeddings =
            OpenAIEmbeddings::new("test-key".to_string(), "text-embedding-3-small".to_string())?;
        assert_eq!(embeddings.dimension(), 1536);

        let embeddings =
            OpenAIEmbeddings::new("test-key".to_string(), "text-embedding-3-large".to_string())?;
        assert_eq!(embeddings.dimension(), 3072);

        Ok(())
    }

    #[test]
    fn test_create_provider() -> Result<()> {
        let provider = create_provider(
            "openai",
            Some("test-key".to_string()),
            Some("text-embedding-3-small".to_string()),
        )?;
        assert_eq!(provider.dimension(), 1536);

        Ok(())
    }

    #[test]
    fn test_create_provider_no_key() {
        let result = create_provider("openai", None, None);
        assert!(result.is_err());
    }
}
