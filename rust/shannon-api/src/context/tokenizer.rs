//! Token counting for context management
//!
//! Provides accurate token counting using tiktoken-rs for various model encodings.

use anyhow::{Context as AnyhowContext, Result};
use std::sync::Arc;

/// Trait for token counting implementations
///
/// Allows for different tokenization strategies based on model family.
pub trait Tokenizer: Send + Sync {
    /// Count tokens in text
    ///
    /// # Arguments
    ///
    /// * `text` - Text to tokenize and count
    ///
    /// # Returns
    ///
    /// Number of tokens in the text
    fn count(&self, text: &str) -> Result<usize>;

    /// Truncate text to fit within token budget
    ///
    /// # Arguments
    ///
    /// * `text` - Text to truncate
    /// * `max_tokens` - Maximum number of tokens allowed
    ///
    /// # Returns
    ///
    /// Truncated text that fits within the token budget
    fn truncate(&self, text: &str, max_tokens: usize) -> Result<String>;
}

/// Tiktoken-based tokenizer using cl100k_base encoding
///
/// This encoding is used by GPT-4, GPT-3.5-turbo, and text-embedding-ada-002.
/// It's a good general-purpose encoding for most modern LLMs.
#[derive(Debug, Clone)]
pub struct TiktokenTokenizer {
    encoding: Arc<tiktoken_rs::CoreBPE>,
}

impl TiktokenTokenizer {
    /// Create a new tiktoken tokenizer
    ///
    /// Uses the cl100k_base encoding which is compatible with most modern LLMs.
    ///
    /// # Errors
    ///
    /// Returns an error if the encoding cannot be initialized.
    pub fn new() -> Result<Self> {
        let encoding = tiktoken_rs::cl100k_base()
            .context("Failed to initialize tiktoken cl100k_base encoding")?;

        Ok(Self {
            encoding: Arc::new(encoding),
        })
    }

    /// Create a tokenizer with a specific encoding
    ///
    /// # Arguments
    ///
    /// * `encoding_name` - Name of the encoding (e.g., "cl100k_base", "p50k_base")
    ///
    /// # Errors
    ///
    /// Returns an error if the encoding cannot be initialized.
    pub fn with_encoding(encoding_name: &str) -> Result<Self> {
        let encoding = tiktoken_rs::get_bpe_from_model(encoding_name)
            .context("Failed to initialize tiktoken encoding")?;

        Ok(Self {
            encoding: Arc::new(encoding),
        })
    }
}

impl Default for TiktokenTokenizer {
    fn default() -> Self {
        Self::new().expect("Failed to initialize default tokenizer")
    }
}

impl Tokenizer for TiktokenTokenizer {
    fn count(&self, text: &str) -> Result<usize> {
        let tokens = self.encoding.encode_with_special_tokens(text);
        Ok(tokens.len())
    }

    fn truncate(&self, text: &str, max_tokens: usize) -> Result<String> {
        let tokens = self.encoding.encode_with_special_tokens(text);

        if tokens.len() <= max_tokens {
            return Ok(text.to_string());
        }

        // Truncate to max_tokens
        let truncated_tokens = &tokens[..max_tokens];

        // Decode back to text
        let truncated_text = self
            .encoding
            .decode(truncated_tokens.to_vec())
            .context("Failed to decode truncated tokens")?;

        Ok(truncated_text)
    }
}

/// Simple character-based tokenizer for testing
///
/// Estimates tokens as approximately 4 characters per token.
/// Not accurate but useful for testing without tiktoken dependency.
#[derive(Debug, Clone, Default)]
pub struct SimpleTokenizer;

impl Tokenizer for SimpleTokenizer {
    fn count(&self, text: &str) -> Result<usize> {
        // Rough estimate: 1 token ≈ 4 characters
        Ok((text.len() + 3) / 4)
    }

    fn truncate(&self, text: &str, max_tokens: usize) -> Result<String> {
        let max_chars = max_tokens * 4;
        if text.len() <= max_chars {
            Ok(text.to_string())
        } else {
            Ok(text.chars().take(max_chars).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiktoken_counting() {
        let tokenizer = TiktokenTokenizer::new().unwrap();

        let text = "Hello, world! This is a test.";
        let count = tokenizer.count(text).unwrap();

        // The exact count may vary, but should be reasonable
        assert!(count > 0);
        assert!(count < 20); // Should be around 7-8 tokens
    }

    #[test]
    fn test_tiktoken_truncation() {
        let tokenizer = TiktokenTokenizer::new().unwrap();

        let text = "This is a long text that needs to be truncated to fit within the token budget.";
        let truncated = tokenizer.truncate(text, 5).unwrap();

        let truncated_count = tokenizer.count(&truncated).unwrap();
        assert!(truncated_count <= 5);
        assert!(!truncated.is_empty());
    }

    #[test]
    fn test_simple_tokenizer() {
        let tokenizer = SimpleTokenizer;

        let text = "Hello, world!";
        let count = tokenizer.count(text).unwrap();

        // 13 characters / 4 ≈ 3 tokens
        assert_eq!(count, 4);

        let truncated = tokenizer.truncate(text, 2).unwrap();
        assert_eq!(truncated.len(), 8); // 2 tokens * 4 chars
    }
}
