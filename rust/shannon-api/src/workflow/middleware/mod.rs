//! Workflow middleware components.
//!
//! Provides cross-cutting concerns for workflow execution including:
//! - Budget enforcement and token tracking
//! - Rate limiting (future)
//! - Audit logging (future)
//! - Performance monitoring (future)

pub mod budget;

pub use budget::{BudgetLimits, BudgetMiddleware, TokenCounter, TokenUsage};
