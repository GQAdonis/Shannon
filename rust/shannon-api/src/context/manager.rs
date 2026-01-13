//! Context manager with multiple optimization strategies
//!
//! Manages conversation history using configurable strategies to optimize
//! token usage while preserving important context.

use crate::database::context_settings::{ContextSettings, ContextStrategy};
use anyhow::{Context as AnyhowContext, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use super::summarizer::Summarizer;
use super::tokenizer::Tokenizer;

/// A single message or context item in a conversation
///
/// Represents a message with metadata for context management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextItem {
    /// Unique identifier for this message
    pub id: String,

    /// Role of the message sender (system, user, assistant)
    pub role: String,

    /// Message content
    pub content: String,

    /// Token count for this message
    pub tokens: usize,

    /// Priority level (0-10, higher = more important)
    ///
    /// Pinned messages and system prompts typically have priority 9-10.
    pub priority: u8,

    /// Whether this message is pinned (always kept)
    pub pinned: bool,

    /// Timestamp when the message was created
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ContextItem {
    /// Create a new context item
    ///
    /// # Arguments
    ///
    /// * `role` - Message role (system, user, assistant)
    /// * `content` - Message content
    /// * `tokenizer` - Tokenizer to count tokens
    pub fn new(role: String, content: String, tokenizer: &dyn Tokenizer) -> Result<Self> {
        let tokens = tokenizer.count(&content)?;

        Ok(Self {
            id: uuid::Uuid::new_v4().to_string(),
            role,
            content,
            tokens,
            priority: 5,
            pinned: false,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        })
    }

    /// Create a system message
    pub fn system(content: String, tokenizer: &dyn Tokenizer) -> Result<Self> {
        let mut item = Self::new("system".to_string(), content, tokenizer)?;
        item.priority = 10; // System messages are high priority
        Ok(item)
    }

    /// Create a user message
    pub fn user(content: String, tokenizer: &dyn Tokenizer) -> Result<Self> {
        Self::new("user".to_string(), content, tokenizer)
    }

    /// Create an assistant message
    pub fn assistant(content: String, tokenizer: &dyn Tokenizer) -> Result<Self> {
        Self::new("assistant".to_string(), content, tokenizer)
    }
}

/// Context manager for optimizing conversation history
///
/// Applies different strategies to manage context within token budgets.
#[derive(Clone)]
pub struct ContextManager {
    settings: ContextSettings,
    tokenizer: Arc<dyn Tokenizer>,
    summarizer: Arc<dyn Summarizer>,
}

impl ContextManager {
    /// Create a new context manager
    ///
    /// # Arguments
    ///
    /// * `settings` - Context management settings
    /// * `tokenizer` - Tokenizer for counting tokens
    /// * `summarizer` - Summarizer for compressing context
    pub fn new(
        settings: ContextSettings,
        tokenizer: Arc<dyn Tokenizer>,
        summarizer: Arc<dyn Summarizer>,
    ) -> Self {
        Self {
            settings,
            tokenizer,
            summarizer,
        }
    }

    /// Process messages using the configured strategy
    ///
    /// # Arguments
    ///
    /// * `messages` - Input messages to process
    ///
    /// # Returns
    ///
    /// Optimized list of messages within token budget
    pub async fn process_messages(&self, messages: Vec<ContextItem>) -> Result<Vec<ContextItem>> {
        match self.settings.strategy {
            ContextStrategy::SlidingWindow => self.sliding_window(messages).await,
            ContextStrategy::ProgressiveSummarization => {
                self.progressive_summarization(messages).await
            }
            ContextStrategy::HierarchicalMemory => self.hierarchical_memory(messages).await,
            ContextStrategy::KeepFirstLast => self.keep_first_last(messages).await,
        }
    }

    /// Sliding Window strategy: Keep only recent messages within budget
    ///
    /// Keeps the most recent messages that fit within the token budget.
    /// Always preserves pinned messages.
    async fn sliding_window(&self, mut messages: Vec<ContextItem>) -> Result<Vec<ContextItem>> {
        // Sort by timestamp (newest first)
        messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        let mut result = Vec::new();
        let mut total_tokens = 0;
        let budget = self.settings.mid_term_budget as usize;

        // Always keep pinned messages
        let pinned: Vec<_> = messages.iter().filter(|m| m.pinned).cloned().collect();

        for msg in &pinned {
            total_tokens += msg.tokens;
        }
        result.extend(pinned);

        // Add recent messages until budget
        for msg in messages {
            if msg.pinned {
                continue; // Already added
            }

            if total_tokens + msg.tokens > budget {
                break;
            }

            result.push(msg.clone());
            total_tokens += msg.tokens;
        }

        // Sort back to chronological order
        result.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        Ok(result)
    }

    /// Progressive Summarization strategy: Summarize older messages
    ///
    /// Keeps recent messages verbatim and summarizes older messages.
    async fn progressive_summarization(
        &self,
        mut messages: Vec<ContextItem>,
    ) -> Result<Vec<ContextItem>> {
        // Sort chronologically
        messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        let short_term_count = self.settings.short_term_turns as usize * 2; // user + assistant
        let mid_budget = self.settings.mid_term_budget as usize;

        // Split into short-term (recent) and mid-term (older)
        let split_point = messages.len().saturating_sub(short_term_count);
        let (older, recent) = messages.split_at(split_point);

        let mut result = Vec::new();

        // Short-term: keep verbatim
        result.extend(recent.iter().cloned());
        let recent_tokens: usize = recent.iter().map(|m| m.tokens).sum();

        // Mid-term: summarize if needed
        if !older.is_empty() {
            let older_tokens: usize = older.iter().map(|m| m.tokens).sum();

            if recent_tokens + older_tokens > mid_budget {
                // Need to summarize older messages
                let summary = self.summarize_messages(older).await?;
                result.insert(0, summary);
            } else {
                // Fits in budget, keep verbatim
                result.splice(0..0, older.iter().cloned());
            }
        }

        Ok(result)
    }

    /// Hierarchical Memory strategy: Three-tier system
    ///
    /// - Tier 1 (Short-term): Recent messages kept verbatim
    /// - Tier 2 (Mid-term): Summarized conversation history
    /// - Tier 3 (Long-term): Key facts and important details
    async fn hierarchical_memory(
        &self,
        mut messages: Vec<ContextItem>,
    ) -> Result<Vec<ContextItem>> {
        messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        let short_term_count = self.settings.short_term_turns as usize * 2;
        let long_budget = self.settings.long_term_budget as usize;

        // Tier 1: Recent messages (verbatim)
        let split1 = messages.len().saturating_sub(short_term_count);
        let (older, tier1) = messages.split_at(split1);

        // Tier 2: Mid-term (summarized)
        let tier2 = if !older.is_empty() {
            let summary = self.summarize_messages(older).await?;
            vec![summary]
        } else {
            vec![]
        };

        // Tier 3: Long-term (key facts)
        let tier3 = if !older.is_empty() {
            self.extract_key_facts(older, long_budget).await?
        } else {
            vec![]
        };

        // Combine tiers
        let mut result = Vec::new();
        result.extend(tier3); // Long-term facts
        result.extend(tier2); // Mid-term summary
        result.extend(tier1.iter().cloned()); // Short-term verbatim

        Ok(result)
    }

    /// Keep First & Last strategy: Preserve system prompt + recent context
    ///
    /// Keeps the first message (usually system instructions) and the most
    /// recent messages, removing the middle section.
    async fn keep_first_last(&self, mut messages: Vec<ContextItem>) -> Result<Vec<ContextItem>> {
        if messages.len() <= 4 {
            // Too few messages, keep all
            return Ok(messages);
        }

        messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // Keep first message (system instructions)
        let first = messages.first().cloned();

        // Keep last N messages (recent context)
        let recent_count = self.settings.short_term_turns as usize * 2;
        let last_n: Vec<_> = messages
            .iter()
            .rev()
            .take(recent_count)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        // Always keep pinned
        let pinned: Vec<_> = messages.iter().filter(|m| m.pinned).cloned().collect();

        // Combine
        let mut result = Vec::new();
        if let Some(f) = first {
            result.push(f);
        }
        result.extend(pinned);
        result.extend(last_n);

        // Deduplicate and sort
        result.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        result.dedup_by_key(|m| m.id.clone());

        Ok(result)
    }

    /// Summarize a list of messages into a single summary message
    async fn summarize_messages(&self, messages: &[ContextItem]) -> Result<ContextItem> {
        let combined = messages
            .iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n\n");

        let summary_prompt = format!(
            "Summarize the following conversation, preserving key points and decisions:\n\n{}",
            combined
        );

        let summary = self
            .summarizer
            .summarize(&summary_prompt, &self.settings.summarization_model)
            .await?;

        let tokens = self.tokenizer.count(&summary)?;

        Ok(ContextItem {
            id: uuid::Uuid::new_v4().to_string(),
            role: "system".to_string(),
            content: format!("[Summary of {} messages] {}", messages.len(), summary),
            tokens,
            priority: 8,
            pinned: false,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::from([
                ("type".to_string(), "summary".to_string()),
                ("source_count".to_string(), messages.len().to_string()),
            ]),
        })
    }

    /// Extract key facts from messages
    async fn extract_key_facts(
        &self,
        messages: &[ContextItem],
        budget: usize,
    ) -> Result<Vec<ContextItem>> {
        let combined = messages
            .iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n\n");

        let facts_prompt = format!(
            "Extract key facts, decisions, and important details from this conversation as a bulleted list:\n\n{}",
            combined
        );

        let facts = self
            .summarizer
            .summarize(&facts_prompt, &self.settings.summarization_model)
            .await?;

        let mut tokens = self.tokenizer.count(&facts)?;
        let mut content = facts;

        // Truncate if exceeds budget
        if tokens > budget {
            content = self.tokenizer.truncate(&content, budget)?;
            tokens = budget;
        }

        Ok(vec![ContextItem {
            id: uuid::Uuid::new_v4().to_string(),
            role: "system".to_string(),
            content: format!("[Key Facts] {}", content),
            tokens,
            priority: 9,
            pinned: true,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::from([("type".to_string(), "facts".to_string())]),
        }])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{MockSummarizer, SimpleTokenizer};

    fn create_test_messages(count: usize) -> Vec<ContextItem> {
        let tokenizer = SimpleTokenizer;
        let mut messages = Vec::new();

        for i in 0..count {
            let role = if i % 2 == 0 { "user" } else { "assistant" };
            let content = format!("Message {} content", i);
            let mut item = ContextItem::new(role.to_string(), content, &tokenizer).unwrap();
            item.timestamp = chrono::Utc::now() - chrono::Duration::seconds((count - i) as i64);
            messages.push(item);
        }

        messages
    }

    #[tokio::test]
    async fn test_sliding_window() {
        let settings = ContextSettings {
            strategy: ContextStrategy::SlidingWindow,
            mid_term_budget: 50, // Small budget
            ..Default::default()
        };

        let manager = ContextManager::new(
            settings,
            Arc::new(SimpleTokenizer),
            Arc::new(MockSummarizer),
        );

        let messages = create_test_messages(10);
        let result = manager.process_messages(messages).await.unwrap();

        // Should keep only recent messages within budget
        assert!(result.len() < 10);
        assert!(result.len() > 0);
    }

    #[tokio::test]
    async fn test_hierarchical_memory() {
        let settings = ContextSettings {
            strategy: ContextStrategy::HierarchicalMemory,
            short_term_turns: 2, // Keep 4 messages (2 turns)
            ..Default::default()
        };

        let manager = ContextManager::new(
            settings,
            Arc::new(SimpleTokenizer),
            Arc::new(MockSummarizer),
        );

        let messages = create_test_messages(10);
        let result = manager.process_messages(messages).await.unwrap();

        // Should have summaries + recent messages
        assert!(result.len() > 0);

        // Check for summary markers
        let has_summary = result
            .iter()
            .any(|m| m.metadata.get("type") == Some(&"summary".to_string()));
        let has_facts = result
            .iter()
            .any(|m| m.metadata.get("type") == Some(&"facts".to_string()));

        assert!(has_summary || has_facts || result.len() <= 4);
    }

    #[tokio::test]
    async fn test_keep_first_last() {
        let settings = ContextSettings {
            strategy: ContextStrategy::KeepFirstLast,
            short_term_turns: 2,
            ..Default::default()
        };

        let manager = ContextManager::new(
            settings,
            Arc::new(SimpleTokenizer),
            Arc::new(MockSummarizer),
        );

        let messages = create_test_messages(10);
        let first_id = messages[0].id.clone();
        let result = manager.process_messages(messages).await.unwrap();

        // Should keep first message
        assert!(result.iter().any(|m| m.id == first_id));

        // Should be less than original
        assert!(result.len() < 10);
    }

    #[tokio::test]
    async fn test_pinned_messages_preserved() {
        let settings = ContextSettings {
            strategy: ContextStrategy::SlidingWindow,
            mid_term_budget: 20, // Very small budget
            ..Default::default()
        };

        let manager = ContextManager::new(
            settings,
            Arc::new(SimpleTokenizer),
            Arc::new(MockSummarizer),
        );

        let mut messages = create_test_messages(5);
        messages[1].pinned = true; // Pin a message

        let result = manager.process_messages(messages).await.unwrap();

        // Pinned message should be preserved
        assert!(result.iter().any(|m| m.pinned));
    }
}
