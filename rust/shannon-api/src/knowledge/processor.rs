//! Document processing with multiple backends.
//!
//! Supports:
//! - **Mistral**: AI-powered document parsing with OCR
//! - **Unstructured.io**: Hosted or self-hosted document extraction
//! - **Native**: Built-in parsers for common formats

use crate::database::knowledge::{ProcessorConfig, ProcessorType};
use anyhow::{anyhow, Context, Result};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

/// Processed document result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedDocument {
    /// Document title.
    pub title: String,
    /// Extracted text content.
    pub content: String,
    /// Additional metadata.
    pub metadata: serde_json::Value,
}

/// Mistral API response.
#[derive(Debug, Deserialize)]
struct MistralResponse {
    text: String,
    #[serde(default)]
    pages: Option<usize>,
    #[serde(default)]
    language: Option<String>,
}

/// Unstructured.io element.
#[derive(Debug, Deserialize)]
struct UnstructuredElement {
    text: String,
    #[serde(rename = "type")]
    element_type: String,
}

/// Document processor service.
pub struct DocumentProcessor {
    configs: HashMap<ProcessorType, ProcessorConfig>,
    http_client: reqwest::Client,
}

impl DocumentProcessor {
    /// Create a new document processor.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be initialized.
    pub fn new() -> Result<Self> {
        Ok(Self {
            configs: HashMap::new(),
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(300))
                .build()?,
        })
    }

    /// Register a processor configuration.
    pub fn register_config(&mut self, config: ProcessorConfig) {
        self.configs.insert(config.processor_type, config);
    }

    /// Detect MIME type from file extension.
    fn detect_mime_type(&self, file_path: &Path) -> Result<String> {
        let extension = file_path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| anyhow!("No file extension"))?;

        let mime_type = match extension.to_lowercase().as_str() {
            "pdf" => "application/pdf",
            "txt" => "text/plain",
            "md" => "text/markdown",
            "html" => "text/html",
            "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            _ => "application/octet-stream",
        };

        Ok(mime_type.to_string())
    }

    /// Process a document using the specified processor.
    ///
    /// # Errors
    ///
    /// Returns an error if processing fails.
    pub async fn process(
        &self,
        file_path: &Path,
        processor_type: ProcessorType,
    ) -> Result<ProcessedDocument> {
        let mime_type = self.detect_mime_type(file_path)?;

        match processor_type {
            ProcessorType::Mistral => self.process_with_mistral(file_path, &mime_type).await,
            ProcessorType::UnstructuredHosted => {
                self.process_with_unstructured_hosted(file_path, &mime_type)
                    .await
            }
            ProcessorType::UnstructuredSelfHosted => {
                self.process_with_unstructured_self_hosted(file_path, &mime_type)
                    .await
            }
            ProcessorType::Native => self.process_native(file_path, &mime_type).await,
        }
    }

    /// Process with Mistral API.
    async fn process_with_mistral(
        &self,
        file_path: &Path,
        mime_type: &str,
    ) -> Result<ProcessedDocument> {
        let config = self
            .configs
            .get(&ProcessorType::Mistral)
            .ok_or_else(|| anyhow!("Mistral processor not configured"))?;

        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| anyhow!("Mistral API key required"))?;

        // Supported MIME types for Mistral:
        // - application/pdf
        // - application/vnd.openxmlformats-officedocument.* (DOCX, PPTX, XLSX)
        // - text/plain, text/markdown, text/html
        // - image/* (with OCR)

        let file_content = tokio::fs::read(file_path)
            .await
            .context("Failed to read file")?;
        let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

        let form = multipart::Form::new().part(
            "file",
            multipart::Part::bytes(file_content)
                .file_name(file_name.clone())
                .mime_str(mime_type)?,
        );

        let response = self
            .http_client
            .post("https://api.mistral.ai/v1/files/process")
            .header("Authorization", format!("Bearer {api_key}"))
            .multipart(form)
            .send()
            .await
            .context("Failed to send request to Mistral")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Mistral API error: {error_text}"));
        }

        let result: MistralResponse = response
            .json()
            .await
            .context("Failed to parse Mistral response")?;

        Ok(ProcessedDocument {
            title: file_name,
            content: result.text,
            metadata: serde_json::json!({
                "processor": "mistral",
                "pages": result.pages,
                "language": result.language,
                "mime_type": mime_type,
            }),
        })
    }

    /// Process with Unstructured.io hosted API.
    async fn process_with_unstructured_hosted(
        &self,
        file_path: &Path,
        mime_type: &str,
    ) -> Result<ProcessedDocument> {
        let config = self
            .configs
            .get(&ProcessorType::UnstructuredHosted)
            .ok_or_else(|| anyhow!("Unstructured hosted processor not configured"))?;

        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| anyhow!("Unstructured API key required"))?;

        self.process_with_unstructured(
            file_path,
            mime_type,
            "https://api.unstructured.io/general/v0/general",
            Some(api_key),
        )
        .await
    }

    /// Process with self-hosted Unstructured.io.
    async fn process_with_unstructured_self_hosted(
        &self,
        file_path: &Path,
        mime_type: &str,
    ) -> Result<ProcessedDocument> {
        let config = self
            .configs
            .get(&ProcessorType::UnstructuredSelfHosted)
            .ok_or_else(|| anyhow!("Unstructured self-hosted not configured"))?;

        let api_url = config
            .api_url
            .as_ref()
            .ok_or_else(|| anyhow!("Unstructured API URL required for self-hosted"))?;

        let endpoint = format!("{api_url}/general/v0/general");

        self.process_with_unstructured(file_path, mime_type, &endpoint, None)
            .await
    }

    /// Common Unstructured.io processing logic.
    async fn process_with_unstructured(
        &self,
        file_path: &Path,
        mime_type: &str,
        endpoint: &str,
        api_key: Option<&String>,
    ) -> Result<ProcessedDocument> {
        let file_content = tokio::fs::read(file_path)
            .await
            .context("Failed to read file")?;
        let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

        let form = multipart::Form::new().part(
            "files",
            multipart::Part::bytes(file_content)
                .file_name(file_name.clone())
                .mime_str(mime_type)?,
        );

        let mut request = self.http_client.post(endpoint).multipart(form);

        if let Some(key) = api_key {
            request = request.header("unstructured-api-key", key);
        }

        let response = request
            .send()
            .await
            .context("Failed to send request to Unstructured")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Unstructured API error: {error_text}"));
        }

        let elements: Vec<UnstructuredElement> = response
            .json()
            .await
            .context("Failed to parse Unstructured response")?;

        let content = elements
            .iter()
            .map(|e| e.text.clone())
            .collect::<Vec<_>>()
            .join("\n\n");

        Ok(ProcessedDocument {
            title: file_name,
            content,
            metadata: serde_json::json!({
                "processor": if api_key.is_some() { "unstructured_hosted" } else { "unstructured_self_hosted" },
                "elements": elements.len(),
                "mime_type": mime_type,
            }),
        })
    }

    /// Process with native parsers.
    async fn process_native(&self, file_path: &Path, mime_type: &str) -> Result<ProcessedDocument> {
        let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

        match mime_type {
            "text/plain" | "text/markdown" | "text/html" => {
                let content = tokio::fs::read_to_string(file_path)
                    .await
                    .context("Failed to read text file")?;

                Ok(ProcessedDocument {
                    title: file_name,
                    content,
                    metadata: serde_json::json!({
                        "processor": "native",
                        "mime_type": mime_type,
                    }),
                })
            }
            _ => Err(anyhow!(
                "Native processor does not support MIME type: {mime_type}"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_native_text_processing() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "This is a test document.")?;

        let processor = DocumentProcessor::new()?;
        let result = processor
            .process(temp_file.path(), ProcessorType::Native)
            .await?;

        assert!(result.content.contains("test document"));
        assert_eq!(result.metadata["processor"], "native");

        Ok(())
    }

    #[test]
    fn test_mime_type_detection() -> Result<()> {
        let processor = DocumentProcessor::new()?;

        assert_eq!(
            processor.detect_mime_type(Path::new("test.pdf"))?,
            "application/pdf"
        );
        assert_eq!(
            processor.detect_mime_type(Path::new("test.txt"))?,
            "text/plain"
        );
        assert_eq!(
            processor.detect_mime_type(Path::new("test.md"))?,
            "text/markdown"
        );

        Ok(())
    }
}
