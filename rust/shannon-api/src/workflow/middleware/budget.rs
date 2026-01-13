//! Budget enforcement middleware.
//!
//! Provides token budget tracking and enforcement to prevent cost overruns.
//! Tracks token usage across multi-stage workflows and enforces configurable limits.
//!
//! # Architecture
//!
//! - Per-task token counters track prompt and completion tokens
//! - Pre-execution checks prevent exceeding budgets
//! - Post-execution tracking updates counters
//! - Warnings emitted at configurable thresholds
//!
//! # Usage
//!
//! ```rust,ignore
//! use shannon_api::workflow::middleware::budget::{BudgetMiddleware, BudgetLimits};
//!
//! let limits = BudgetLimits {
//!     max_tokens: 100_000,
//!     max_cost_usd: 1.0,
//!     warn_threshold: 0.8,
//! };
//!
//! let middleware = BudgetMiddleware::new(limits);
//!
//! // Before LLM call
//! middleware.check_before("task-123", 5000).await?;
//!
//! // After LLM call
//! let usage = TokenUsage {
//!     prompt_tokens: 1200,
//!     completion_tokens: 800,
//!     cost_usd: 0.05,
//! };
//! middleware.track_after("task-123", usage).await?;
//! ```

use anyhow::Result;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Budget limits configuration.
///
/// Defines maximum resource consumption and warning thresholds.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BudgetLimits {
    /// Maximum total tokens (prompt + completion) allowed per task.
    pub max_tokens: u32,
    /// Maximum cost in USD per task.
    pub max_cost_usd: f64,
    /// Warning threshold as fraction of limit (0.0-1.0).
    /// Default 0.8 means warn at 80% of budget.
    pub warn_threshold: f64,
}

impl Default for BudgetLimits {
    fn default() -> Self {
        Self {
            max_tokens: 100_000, // 100K tokens default
            max_cost_usd: 1.0,   // $1 default
            warn_threshold: 0.8, // Warn at 80%
        }
    }
}

impl BudgetLimits {
    /// Create budget limits with custom values.
    ///
    /// # Arguments
    ///
    /// * `max_tokens` - Maximum tokens allowed
    /// * `max_cost_usd` - Maximum cost in USD
    /// * `warn_threshold` - Warning threshold (0.0-1.0)
    #[must_use]
    pub fn new(max_tokens: u32, max_cost_usd: f64, warn_threshold: f64) -> Self {
        Self {
            max_tokens,
            max_cost_usd,
            warn_threshold: warn_threshold.clamp(0.0, 1.0),
        }
    }

    /// Create unlimited budget (for testing or privileged users).
    #[must_use]
    pub fn unlimited() -> Self {
        Self {
            max_tokens: u32::MAX,
            max_cost_usd: f64::MAX,
            warn_threshold: 0.99,
        }
    }
}

/// Token usage for a single LLM call.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Prompt/input tokens consumed.
    pub prompt_tokens: u32,
    /// Completion/output tokens generated.
    pub completion_tokens: u32,
    /// Estimated cost in USD for this call.
    pub cost_usd: f64,
}

impl TokenUsage {
    /// Calculate total tokens.
    #[must_use]
    pub fn total_tokens(&self) -> u32 {
        self.prompt_tokens.saturating_add(self.completion_tokens)
    }
}

/// Token counter for a task.
///
/// Tracks cumulative token usage and cost across multiple LLM calls.
#[derive(Debug, Clone, Default)]
pub struct TokenCounter {
    /// Cumulative prompt tokens.
    pub prompt_tokens: u32,
    /// Cumulative completion tokens.
    pub completion_tokens: u32,
    /// Cumulative cost in USD.
    pub total_cost_usd: f64,
    /// Number of LLM calls made.
    pub call_count: u32,
}

impl TokenCounter {
    /// Calculate total tokens used.
    #[must_use]
    pub fn total_tokens(&self) -> u32 {
        self.prompt_tokens.saturating_add(self.completion_tokens)
    }

    /// Add usage from an LLM call.
    pub fn add_usage(&mut self, usage: &TokenUsage) {
        self.prompt_tokens = self.prompt_tokens.saturating_add(usage.prompt_tokens);
        self.completion_tokens = self
            .completion_tokens
            .saturating_add(usage.completion_tokens);
        self.total_cost_usd += usage.cost_usd;
        self.call_count = self.call_count.saturating_add(1);
    }

    /// Check if usage exceeds limits.
    #[must_use]
    pub fn exceeds_limits(&self, limits: &BudgetLimits) -> bool {
        self.total_tokens() > limits.max_tokens || self.total_cost_usd > limits.max_cost_usd
    }

    /// Check if usage exceeds warning threshold.
    #[must_use]
    pub fn exceeds_warning(&self, limits: &BudgetLimits) -> bool {
        let token_pct = self.total_tokens() as f64 / limits.max_tokens as f64;
        let cost_pct = self.total_cost_usd / limits.max_cost_usd;
        token_pct >= limits.warn_threshold || cost_pct >= limits.warn_threshold
    }
}

/// Budget enforcement middleware.
///
/// Tracks token usage per task and enforces configurable limits.
/// Thread-safe for concurrent task execution.
///
/// # Example
///
/// ```rust,ignore
/// let middleware = BudgetMiddleware::new(BudgetLimits::default());
///
/// // Check before execution
/// middleware.check_before("task-1", 5000).await?;
///
/// // Track after execution
/// let usage = TokenUsage {
///     prompt_tokens: 1200,
///     completion_tokens: 800,
///     cost_usd: 0.05,
/// };
/// middleware.track_after("task-1", usage).await?;
/// ```
pub struct BudgetMiddleware {
    /// Per-task token counters.
    counters: Arc<RwLock<HashMap<String, TokenCounter>>>,
    /// Default budget limits.
    default_limits: BudgetLimits,
}

impl std::fmt::Debug for BudgetMiddleware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BudgetMiddleware")
            .field("task_count", &self.counters.read().len())
            .field("default_limits", &self.default_limits)
            .finish()
    }
}

impl BudgetMiddleware {
    /// Create a new budget middleware with default limits.
    ///
    /// # Arguments
    ///
    /// * `default_limits` - Budget limits applied to all tasks
    #[must_use]
    pub fn new(default_limits: BudgetLimits) -> Self {
        tracing::info!(
            max_tokens = default_limits.max_tokens,
            max_cost = default_limits.max_cost_usd,
            warn_threshold = default_limits.warn_threshold,
            "📊 Budget middleware initialized"
        );

        Self {
            counters: Arc::new(RwLock::new(HashMap::new())),
            default_limits,
        }
    }

    /// Check budget before executing an LLM call.
    ///
    /// Validates that the task hasn't exceeded its budget and that the
    /// estimated new tokens won't push it over the limit.
    ///
    /// # Arguments
    ///
    /// * `task_id` - Task identifier
    /// * `estimated_tokens` - Estimated tokens for upcoming call
    ///
    /// # Errors
    ///
    /// Returns error if budget would be exceeded by the new call.
    pub async fn check_before(&self, task_id: &str, estimated_tokens: u32) -> Result<()> {
        let counters = self.counters.read();

        if let Some(counter) = counters.get(task_id) {
            let projected_total = counter.total_tokens().saturating_add(estimated_tokens);

            // Check if projected usage exceeds limits
            if projected_total > self.default_limits.max_tokens {
                tracing::error!(
                    task_id = task_id,
                    current_tokens = counter.total_tokens(),
                    estimated_tokens = estimated_tokens,
                    max_tokens = self.default_limits.max_tokens,
                    "❌ Token budget exceeded"
                );

                anyhow::bail!(
                    "Token budget exceeded for task {}: {} tokens used, \
                     {} requested, {} max allowed",
                    task_id,
                    counter.total_tokens(),
                    estimated_tokens,
                    self.default_limits.max_tokens
                );
            }

            // Check if projected usage exceeds warning threshold
            if counter.exceeds_warning(&self.default_limits) {
                tracing::warn!(
                    task_id = task_id,
                    current_tokens = counter.total_tokens(),
                    max_tokens = self.default_limits.max_tokens,
                    usage_pct = (counter.total_tokens() as f64
                        / self.default_limits.max_tokens as f64)
                        * 100.0,
                    "⚠️  Approaching token budget limit"
                );
            }
        }

        Ok(())
    }

    /// Track token usage after an LLM call.
    ///
    /// Updates the task's token counter and emits warnings if approaching limits.
    ///
    /// # Arguments
    ///
    /// * `task_id` - Task identifier
    /// * `usage` - Token usage from the completed LLM call
    ///
    /// # Errors
    ///
    /// Returns error if the usage causes budget to be exceeded.
    pub async fn track_after(&self, task_id: &str, usage: TokenUsage) -> Result<()> {
        let mut counters = self.counters.write();
        let counter = counters.entry(task_id.to_string()).or_default();

        counter.add_usage(&usage);

        tracing::debug!(
            task_id = task_id,
            prompt_tokens = usage.prompt_tokens,
            completion_tokens = usage.completion_tokens,
            total_tokens = counter.total_tokens(),
            cost_usd = usage.cost_usd,
            total_cost = counter.total_cost_usd,
            call_count = counter.call_count,
            "📈 Token usage tracked"
        );

        // Check if now exceeding limits
        if counter.exceeds_limits(&self.default_limits) {
            tracing::error!(
                task_id = task_id,
                total_tokens = counter.total_tokens(),
                max_tokens = self.default_limits.max_tokens,
                total_cost = counter.total_cost_usd,
                max_cost = self.default_limits.max_cost_usd,
                "❌ Budget limit exceeded after tracking"
            );

            anyhow::bail!(
                "Budget exceeded for task {}: {} tokens / ${:.4} used, limits: {} tokens / ${:.2}",
                task_id,
                counter.total_tokens(),
                counter.total_cost_usd,
                self.default_limits.max_tokens,
                self.default_limits.max_cost_usd
            );
        }

        // Emit warning if approaching limit
        if counter.exceeds_warning(&self.default_limits) {
            let token_pct =
                (counter.total_tokens() as f64 / self.default_limits.max_tokens as f64) * 100.0;
            let cost_pct = (counter.total_cost_usd / self.default_limits.max_cost_usd) * 100.0;

            tracing::warn!(
                task_id = task_id,
                total_tokens = counter.total_tokens(),
                max_tokens = self.default_limits.max_tokens,
                token_usage_pct = format!("{:.1}%", token_pct),
                total_cost = counter.total_cost_usd,
                max_cost = self.default_limits.max_cost_usd,
                cost_usage_pct = format!("{:.1}%", cost_pct),
                "⚠️  Approaching budget limit"
            );
        }

        Ok(())
    }

    /// Get current usage statistics for a task.
    ///
    /// # Arguments
    ///
    /// * `task_id` - Task identifier
    ///
    /// # Returns
    ///
    /// Token counter for the task, or None if task not tracked.
    #[must_use]
    pub fn get_usage(&self, task_id: &str) -> Option<TokenCounter> {
        let counters = self.counters.read();
        counters.get(task_id).cloned()
    }

    /// Clear usage tracking for a task.
    ///
    /// Should be called after task completion to free memory.
    ///
    /// # Arguments
    ///
    /// * `task_id` - Task identifier
    pub fn clear_task(&self, task_id: &str) {
        let mut counters = self.counters.write();
        if counters.remove(task_id).is_some() {
            tracing::debug!(task_id = task_id, "🗑️  Cleared budget tracking");
        }
    }

    /// Get the number of tasks currently tracked.
    #[must_use]
    pub fn task_count(&self) -> usize {
        self.counters.read().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_budget_middleware_creation() {
        let middleware = BudgetMiddleware::new(BudgetLimits::default());
        assert_eq!(middleware.task_count(), 0);
    }

    #[tokio::test]
    async fn test_track_usage() {
        let middleware = BudgetMiddleware::new(BudgetLimits::default());

        let usage = TokenUsage {
            prompt_tokens: 100,
            completion_tokens: 50,
            cost_usd: 0.01,
        };

        middleware.track_after("task-1", usage).await.unwrap();

        let counter = middleware.get_usage("task-1").unwrap();
        assert_eq!(counter.total_tokens(), 150);
        assert_eq!(counter.call_count, 1);
    }

    #[tokio::test]
    async fn test_budget_exceeded() {
        let limits = BudgetLimits::new(1000, 1.0, 0.8);
        let middleware = BudgetMiddleware::new(limits);

        // Use up most of budget
        let usage = TokenUsage {
            prompt_tokens: 900,
            completion_tokens: 50,
            cost_usd: 0.5,
        };
        middleware.track_after("task-1", usage).await.unwrap();

        // Try to exceed budget
        let result = middleware.check_before("task-1", 200).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("budget exceeded"));
    }

    #[tokio::test]
    async fn test_warning_threshold() {
        let limits = BudgetLimits::new(1000, 1.0, 0.8);
        let middleware = BudgetMiddleware::new(limits);

        // Use 85% of token budget (should trigger warning)
        let usage = TokenUsage {
            prompt_tokens: 700,
            completion_tokens: 150,
            cost_usd: 0.4,
        };
        middleware.track_after("task-1", usage).await.unwrap();

        let counter = middleware.get_usage("task-1").unwrap();
        assert!(counter.exceeds_warning(&limits));
    }

    #[tokio::test]
    async fn test_clear_task() {
        let middleware = BudgetMiddleware::new(BudgetLimits::default());

        let usage = TokenUsage {
            prompt_tokens: 100,
            completion_tokens: 50,
            cost_usd: 0.01,
        };
        middleware.track_after("task-1", usage).await.unwrap();

        assert_eq!(middleware.task_count(), 1);

        middleware.clear_task("task-1");
        assert_eq!(middleware.task_count(), 0);
    }

    #[tokio::test]
    async fn test_unlimited_budget() {
        let limits = BudgetLimits::unlimited();
        let middleware = BudgetMiddleware::new(limits);

        // Try to use massive amounts
        let usage = TokenUsage {
            prompt_tokens: 1_000_000,
            completion_tokens: 1_000_000,
            cost_usd: 100.0,
        };
        let result = middleware.track_after("task-1", usage).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_token_counter_saturation() {
        let mut counter = TokenCounter::default();

        let usage = TokenUsage {
            prompt_tokens: u32::MAX,
            completion_tokens: 1,
            cost_usd: 0.0,
        };

        counter.add_usage(&usage);
        // Should saturate at u32::MAX, not overflow
        assert_eq!(counter.total_tokens(), u32::MAX);
    }
}
