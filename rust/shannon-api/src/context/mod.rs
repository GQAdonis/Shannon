//! Context management module for conversation history optimization
//!
//! Provides multiple strategies for managing conversation context within token budgets:
//! - Sliding Window: Keep only recent messages
//! - Progressive Summarization: Summarize older messages
//! - Hierarchical Memory: Three-tier system (verbatim, summarized, key facts)
//! - Keep First & Last: Preserve initial instructions + recent context

pub mod manager;
pub mod summarizer;
pub mod tokenizer;

pub use manager::{ContextItem, ContextManager};
pub use summarizer::{LLMSummarizer, MockSummarizer, Summarizer};
pub use tokenizer::{SimpleTokenizer, TiktokenTokenizer, Tokenizer};
