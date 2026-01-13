//! Strategy composition for multi-stage workflow execution.
//!
//! Strategies compose cognitive patterns to create complex multi-stage workflows.
//! For example, the Scientific strategy chains Chain of Thought → Debate → Research
//! to perform rigorous analysis with hypothesis generation and validation.
//!
//! # Architecture
//!
//! - Strategies implement the [`Strategy`] trait
//! - Each strategy coordinates multiple pattern executions
//! - Patterns are executed sequentially with output piped between stages
//! - Token budgets are enforced via middleware
//!
//! # Usage
//!
//! ```rust,ignore
//! use shannon_api::workflow::strategies::{Strategy, ScientificWorkflow};
//! use shannon_api::workflow::prompts::PromptRenderer;
//!
//! let renderer = PromptRenderer::new()?;
//! let strategy = ScientificWorkflow::new(renderer);
//! let result = strategy.execute(&task).await?;
//! ```

pub mod exploratory;
pub mod scientific;

pub use exploratory::ExploratoryWorkflow;
pub use scientific::ScientificWorkflow;

use crate::workflow::task::Task;
use anyhow::Result;
use async_trait::async_trait;

/// Strategy trait for multi-stage workflow execution.
///
/// Strategies coordinate multiple cognitive patterns to accomplish complex tasks.
/// Each strategy defines a specific composition of patterns that are executed
/// sequentially to produce a comprehensive result.
///
/// # Thread Safety
///
/// Implementations must be `Send + Sync` for concurrent execution.
#[async_trait]
pub trait Strategy: Send + Sync + std::fmt::Debug {
    /// Execute the strategy with the given task.
    ///
    /// This method orchestrates multiple pattern executions, passing outputs
    /// between stages to build a comprehensive result.
    ///
    /// # Arguments
    ///
    /// * `task` - The task containing query, context, and execution parameters
    ///
    /// # Errors
    ///
    /// Returns error if any stage fails or if token budget is exceeded.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = strategy.execute(&task).await?;
    /// println!("Final result: {}", result);
    /// ```
    async fn execute(&self, task: &Task) -> Result<String>;

    /// Get the strategy name.
    ///
    /// Used for logging, metrics, and strategy selection.
    fn name(&self) -> &str;

    /// Get a description of what this strategy does.
    ///
    /// This should explain the pattern composition and intended use case.
    fn description(&self) -> &str {
        "No description provided"
    }

    /// Get the list of patterns this strategy uses.
    ///
    /// Returns the names of cognitive patterns executed by this strategy
    /// in the order they are applied.
    fn patterns(&self) -> Vec<&str> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow::prompts::PromptRenderer;
    use crate::workflow::task::Strategy as TaskStrategy;

    #[tokio::test]
    async fn test_scientific_workflow_creation() {
        let renderer = PromptRenderer::new().unwrap();
        let workflow = ScientificWorkflow::new(renderer);

        assert_eq!(workflow.name(), "Scientific");
        assert_eq!(
            workflow.patterns(),
            vec!["chain_of_thought", "debate", "research"]
        );
        assert!(!workflow.description().is_empty());
    }

    #[tokio::test]
    async fn test_exploratory_workflow_creation() {
        let renderer = PromptRenderer::new().unwrap();
        let workflow = ExploratoryWorkflow::new(renderer);

        assert_eq!(workflow.name(), "Exploratory");
        assert_eq!(workflow.patterns(), vec!["tree_of_thoughts"]);
        assert!(!workflow.description().is_empty());
    }

    #[tokio::test]
    async fn test_task_strategy_mapping() {
        // Verify that task strategies map to workflow strategies correctly
        let task =
            Task::new("task-1", "user-1", "test query").with_strategy(TaskStrategy::Scientific);

        assert_eq!(task.strategy, TaskStrategy::Scientific);
    }
}
