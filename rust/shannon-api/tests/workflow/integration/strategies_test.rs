//! Integration tests for strategy composition and budget middleware.
//!
//! Tests the full workflow execution pipeline with multi-stage strategies
//! and budget enforcement.

use shannon_api::workflow::task::Strategy as TaskStrategy;
use shannon_api::workflow::{
    BudgetLimits, BudgetMiddleware, ExploratoryWorkflow, PromptRenderer, ScientificWorkflow,
    Strategy, Task, TokenUsage,
};

#[tokio::test]
async fn test_scientific_workflow_execution() {
    // Setup
    let renderer = PromptRenderer::new().unwrap();
    let workflow = ScientificWorkflow::new(renderer);

    let task = Task::new(
        "task-scientific-1",
        "user-1",
        "How does photosynthesis work?",
    )
    .with_strategy(TaskStrategy::Scientific)
    .with_context(serde_json::json!({
        "background": "Biology question for educational purposes"
    }));

    // Execute
    let result = workflow.execute(&task).await.unwrap();

    // Validate
    assert!(!result.is_empty());
    assert!(result.contains("Scientific Analysis"));
    assert!(result.contains("Hypothesis Generation"));
    assert!(result.contains("Critical Analysis"));
    assert!(result.contains("Evidence Validation"));
    assert!(result.contains("Workflow Statistics"));

    // Verify all three stages executed
    assert!(result.contains("Stage 1:"));
    assert!(result.contains("Stage 2:"));
    assert!(result.contains("Stage 3:"));
}

#[tokio::test]
async fn test_exploratory_workflow_execution() {
    // Setup
    let renderer = PromptRenderer::new().unwrap();
    let workflow = ExploratoryWorkflow::new(renderer);

    let task = Task::new(
        "task-exploratory-1",
        "user-1",
        "What are the best strategies for team productivity?",
    )
    .with_strategy(TaskStrategy::TreeOfThoughts);

    // Execute
    let result = workflow.execute(&task).await.unwrap();

    // Validate
    assert!(!result.is_empty());
    assert!(result.contains("Exploratory Analysis"));
    assert!(result.contains("Solution Space Exploration"));
    assert!(result.contains("Best Path Selection"));
    assert!(result.contains("Branch 1"));
    assert!(result.contains("Branch 2"));
    assert!(result.contains("Branch 3"));
}

#[tokio::test]
async fn test_exploratory_workflow_custom_branches() {
    let renderer = PromptRenderer::new().unwrap();
    let workflow = ExploratoryWorkflow::new(renderer).with_branches(5);

    let task = Task::new("task-1", "user-1", "test query");
    let result = workflow.execute(&task).await.unwrap();

    assert!(result.contains("Explored 5 solution branches"));
    assert!(result.contains("Branch 5"));
}

#[tokio::test]
async fn test_budget_middleware_tracking() {
    let limits = BudgetLimits::new(10_000, 1.0, 0.8);
    let middleware = BudgetMiddleware::new(limits);

    // Simulate multi-stage workflow with budget tracking
    let task_id = "task-budget-1";

    // Stage 1: Chain of Thought
    let stage1_usage = TokenUsage {
        prompt_tokens: 1200,
        completion_tokens: 800,
        cost_usd: 0.05,
    };
    middleware.track_after(task_id, stage1_usage).await.unwrap();

    // Stage 2: Debate
    let stage2_usage = TokenUsage {
        prompt_tokens: 1500,
        completion_tokens: 1000,
        cost_usd: 0.06,
    };
    middleware.track_after(task_id, stage2_usage).await.unwrap();

    // Stage 3: Research
    let stage3_usage = TokenUsage {
        prompt_tokens: 2000,
        completion_tokens: 1500,
        cost_usd: 0.08,
    };
    middleware.track_after(task_id, stage3_usage).await.unwrap();

    // Verify total usage
    let counter = middleware.get_usage(task_id).unwrap();
    assert_eq!(counter.prompt_tokens, 4700);
    assert_eq!(counter.completion_tokens, 3300);
    assert_eq!(counter.total_tokens(), 8000);
    assert_eq!(counter.call_count, 3);
    assert!((counter.total_cost_usd - 0.19).abs() < 0.001);

    // Should still be under budget
    assert!(!counter.exceeds_limits(&BudgetLimits::new(10_000, 1.0, 0.8)));
}

#[tokio::test]
async fn test_budget_enforcement() {
    let limits = BudgetLimits::new(5000, 0.5, 0.8);
    let middleware = BudgetMiddleware::new(limits);

    let task_id = "task-budget-exceed";

    // Use up most of budget
    let usage1 = TokenUsage {
        prompt_tokens: 3000,
        completion_tokens: 1500,
        cost_usd: 0.3,
    };
    middleware.track_after(task_id, usage1).await.unwrap();

    // Try to exceed budget
    let result = middleware.check_before(task_id, 2000).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("budget exceeded"));
}

#[tokio::test]
async fn test_budget_warning_threshold() {
    let limits = BudgetLimits::new(10_000, 1.0, 0.8);
    let middleware = BudgetMiddleware::new(limits);

    let task_id = "task-budget-warning";

    // Use 85% of budget (should trigger warning)
    let usage = TokenUsage {
        prompt_tokens: 6000,
        completion_tokens: 2500,
        cost_usd: 0.7,
    };
    middleware.track_after(task_id, usage).await.unwrap();

    let counter = middleware.get_usage(task_id).unwrap();
    assert!(counter.exceeds_warning(&limits));
    assert!(!counter.exceeds_limits(&limits));
}

#[tokio::test]
async fn test_strategy_metadata() {
    let renderer = PromptRenderer::new().unwrap();

    // Scientific strategy
    let scientific = ScientificWorkflow::new(renderer.clone());
    assert_eq!(scientific.name(), "Scientific");
    assert_eq!(scientific.patterns().len(), 3);
    assert_eq!(
        scientific.patterns(),
        vec!["chain_of_thought", "debate", "research"]
    );
    assert!(scientific.description().contains("hypothesis"));

    // Exploratory strategy
    let exploratory = ExploratoryWorkflow::new(renderer);
    assert_eq!(exploratory.name(), "Exploratory");
    assert_eq!(exploratory.patterns().len(), 1);
    assert_eq!(exploratory.patterns(), vec!["tree_of_thoughts"]);
    assert!(exploratory.description().contains("Tree of Thoughts"));
}

#[tokio::test]
async fn test_workflow_with_context() {
    let renderer = PromptRenderer::new().unwrap();
    let workflow = ScientificWorkflow::new(renderer);

    // Test with array context
    let task = Task::new("task-1", "user-1", "test query").with_context(serde_json::json!([
        "Previous: The sky is blue",
        "Previous: Water is wet"
    ]));

    let result = workflow.execute(&task).await.unwrap();
    assert!(!result.is_empty());

    // Test with string context
    let task2 = Task::new("task-2", "user-1", "test query")
        .with_context(serde_json::json!("Single context string"));

    let result2 = workflow.execute(&task2).await.unwrap();
    assert!(!result2.is_empty());
}

#[tokio::test]
async fn test_budget_unlimited() {
    let middleware = BudgetMiddleware::new(BudgetLimits::unlimited());

    // Use massive amounts - should not fail
    let usage = TokenUsage {
        prompt_tokens: 1_000_000,
        completion_tokens: 1_000_000,
        cost_usd: 100.0,
    };

    let result = middleware.track_after("task-unlimited", usage).await;
    assert!(result.is_ok());

    let counter = middleware.get_usage("task-unlimited").unwrap();
    assert_eq!(counter.total_tokens(), 2_000_000);
}

#[tokio::test]
async fn test_multiple_tasks_isolation() {
    let middleware = BudgetMiddleware::new(BudgetLimits::default());

    // Task 1
    let usage1 = TokenUsage {
        prompt_tokens: 1000,
        completion_tokens: 500,
        cost_usd: 0.05,
    };
    middleware.track_after("task-1", usage1).await.unwrap();

    // Task 2
    let usage2 = TokenUsage {
        prompt_tokens: 2000,
        completion_tokens: 1000,
        cost_usd: 0.10,
    };
    middleware.track_after("task-2", usage2).await.unwrap();

    // Verify isolation
    let counter1 = middleware.get_usage("task-1").unwrap();
    assert_eq!(counter1.total_tokens(), 1500);

    let counter2 = middleware.get_usage("task-2").unwrap();
    assert_eq!(counter2.total_tokens(), 3000);

    assert_eq!(middleware.task_count(), 2);
}

#[tokio::test]
async fn test_clear_task_after_completion() {
    let middleware = BudgetMiddleware::new(BudgetLimits::default());

    let usage = TokenUsage {
        prompt_tokens: 1000,
        completion_tokens: 500,
        cost_usd: 0.05,
    };
    middleware.track_after("task-1", usage).await.unwrap();

    assert_eq!(middleware.task_count(), 1);

    middleware.clear_task("task-1");
    assert_eq!(middleware.task_count(), 0);
    assert!(middleware.get_usage("task-1").is_none());
}
