//! Mode Detector
//!
//! Automatically detects the appropriate chat mode (Quick vs Task) based on query analysis.
//!
//! # Detection Strategy
//! 1. Keyword analysis (quick markers vs complexity markers)
//! 2. Query length (longer queries suggest complexity)
//! 3. Intent classification (conversational vs analytical)
//! 4. Default to Quick mode for ambiguous cases (lower latency)
//!
//! # Examples
//! - "What is 2+2?" → Quick (simple question)
//! - "Research quantum computing advancements" → Task (research keyword)
//! - "Compare React vs Vue performance" → Task (analysis required)
//! - "Hello, how are you?" → Quick (conversational)

use tracing::debug;

use super::ChatMode;

/// Mode detector for automatic chat mode selection
pub struct ModeDetector;

impl ModeDetector {
    /// Detect appropriate chat mode based on query analysis
    ///
    /// # Arguments
    /// * `query` - User query to analyze
    ///
    /// # Returns
    /// Recommended chat mode (Quick or Task)
    ///
    /// # Algorithm
    /// 1. Check for explicit quick mode markers
    /// 2. Check for complexity markers indicating task mode
    /// 3. Analyze query length and structure
    /// 4. Default to quick mode for conversational queries
    pub fn detect_mode(query: &str) -> ChatMode {
        let query_lower = query.to_lowercase();
        let word_count = query.split_whitespace().count();

        debug!(
            query_len = query.len(),
            word_count = word_count,
            "Analyzing query for mode detection"
        );

        // Check for explicit quick mode markers
        if Self::has_quick_markers(&query_lower) {
            debug!("Detected quick mode markers");
            return ChatMode::Quick;
        }

        // Check for complexity markers indicating task mode
        if Self::has_complexity_markers(&query_lower) {
            debug!("Detected complexity markers - selecting Task mode");
            return ChatMode::Task;
        }

        // Check for research and analysis indicators
        if Self::has_research_markers(&query_lower) {
            debug!("Detected research markers - selecting Task mode");
            return ChatMode::Task;
        }

        // Length-based heuristics
        // Very long queries (>500 words) likely need workflow
        if word_count > 500 {
            debug!(
                word_count = word_count,
                "Long query detected - selecting Task mode"
            );
            return ChatMode::Task;
        }

        // Medium-length queries (100-500 words) with specific keywords
        if word_count > 100 && Self::has_analytical_intent(&query_lower) {
            debug!("Medium-length analytical query - selecting Task mode");
            return ChatMode::Task;
        }

        // Check for multi-step instructions
        if Self::has_multi_step_instructions(query) {
            debug!("Multi-step instructions detected - selecting Task mode");
            return ChatMode::Task;
        }

        // Check for code generation requests
        if Self::has_code_generation_markers(&query_lower) {
            debug!("Code generation request - selecting Task mode");
            return ChatMode::Task;
        }

        // Default to quick mode for conversational queries
        debug!("No strong indicators - defaulting to Quick mode");
        ChatMode::Quick
    }

    /// Check for quick mode markers
    ///
    /// These indicate simple, conversational queries that should use Quick mode.
    fn has_quick_markers(query: &str) -> bool {
        let quick_keywords = [
            "quick",
            "simple",
            "just",
            "what is",
            "who is",
            "when is",
            "where is",
            "how do i",
            "can you",
            "please",
            "hello",
            "hi ",
            "hey",
            "thanks",
            "thank you",
        ];

        quick_keywords.iter().any(|keyword| query.contains(keyword))
    }

    /// Check for complexity markers
    ///
    /// These indicate complex tasks that should use Task mode.
    fn has_complexity_markers(query: &str) -> bool {
        let complex_keywords = [
            "analyze",
            "analyse",
            "evaluate",
            "assess",
            "investigate",
            "examine",
            "compare",
            "contrast",
            "comprehensive",
            "detailed",
            "in-depth",
            "thorough",
            "multi-step",
            "complex",
            "elaborate",
            "extensive",
            "systematic",
        ];

        complex_keywords
            .iter()
            .any(|keyword| query.contains(keyword))
    }

    /// Check for research markers
    ///
    /// These indicate research-oriented tasks that need workflow execution.
    fn has_research_markers(query: &str) -> bool {
        let research_keywords = [
            "research",
            "find out",
            "gather information",
            "collect data",
            "survey",
            "review literature",
            "benchmark",
            "study",
            "explore",
            "discover",
            "investigate",
            "look into",
            "dig into",
            "search for",
        ];

        research_keywords
            .iter()
            .any(|keyword| query.contains(keyword))
    }

    /// Check for analytical intent
    ///
    /// These suggest the query requires analysis beyond simple Q&A.
    fn has_analytical_intent(query: &str) -> bool {
        let analytical_keywords = [
            "why",
            "how come",
            "explain",
            "reason",
            "cause",
            "impact",
            "effect",
            "implication",
            "consequence",
            "trade-off",
            "pros and cons",
            "advantage",
            "disadvantage",
            "benefit",
            "drawback",
        ];

        // Count analytical markers
        let marker_count = analytical_keywords
            .iter()
            .filter(|keyword| query.contains(*keyword))
            .count();

        // If multiple analytical markers, it's likely analytical
        marker_count >= 2
    }

    /// Check for multi-step instructions
    ///
    /// Queries with multiple steps or phases suggest workflow execution.
    fn has_multi_step_instructions(query: &str) -> bool {
        // Look for numbered steps or sequential markers
        let step_markers = [
            "1.",
            "2.",
            "3.",
            "step 1",
            "step 2",
            "first",
            "then",
            "next",
            "finally",
            "after that",
            "following",
            "subsequently",
        ];

        let step_count = step_markers
            .iter()
            .filter(|marker| query.contains(*marker))
            .count();

        // Multiple step markers suggest multi-step process
        step_count >= 2
    }

    /// Check for code generation markers
    ///
    /// Code generation often benefits from workflow-based execution.
    fn has_code_generation_markers(query: &str) -> bool {
        let code_keywords = [
            "write code",
            "generate code",
            "create a function",
            "implement",
            "build a",
            "develop a",
            "code for",
            "program",
            "script",
            "algorithm",
            "refactor",
        ];

        code_keywords.iter().any(|keyword| query.contains(keyword))
    }

    /// Provide confidence score for mode detection
    ///
    /// Returns a score from 0.0 (low confidence) to 1.0 (high confidence).
    pub fn confidence_score(query: &str) -> f32 {
        let query_lower = query.to_lowercase();
        let mut score = 0.5; // Start with neutral confidence

        // Strong indicators increase confidence
        if Self::has_quick_markers(&query_lower) {
            score += 0.3;
        }
        if Self::has_complexity_markers(&query_lower) {
            score += 0.3;
        }
        if Self::has_research_markers(&query_lower) {
            score += 0.2;
        }

        // Cap at 1.0
        score.min(1.0)
    }

    /// Get detailed analysis of query for debugging
    ///
    /// Returns a structured analysis useful for understanding mode selection.
    #[allow(dead_code)]
    pub fn analyze_query(query: &str) -> QueryAnalysis {
        let query_lower = query.to_lowercase();
        let word_count = query.split_whitespace().count();

        QueryAnalysis {
            word_count,
            has_quick_markers: Self::has_quick_markers(&query_lower),
            has_complexity_markers: Self::has_complexity_markers(&query_lower),
            has_research_markers: Self::has_research_markers(&query_lower),
            has_analytical_intent: Self::has_analytical_intent(&query_lower),
            has_multi_step: Self::has_multi_step_instructions(query),
            has_code_generation: Self::has_code_generation_markers(&query_lower),
            recommended_mode: Self::detect_mode(query),
            confidence: Self::confidence_score(query),
        }
    }
}

/// Detailed query analysis result
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct QueryAnalysis {
    /// Number of words in query
    pub word_count: usize,
    /// Has quick mode markers
    pub has_quick_markers: bool,
    /// Has complexity markers
    pub has_complexity_markers: bool,
    /// Has research markers
    pub has_research_markers: bool,
    /// Has analytical intent
    pub has_analytical_intent: bool,
    /// Has multi-step instructions
    pub has_multi_step: bool,
    /// Has code generation markers
    pub has_code_generation: bool,
    /// Recommended chat mode
    pub recommended_mode: ChatMode,
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_questions_use_quick_mode() {
        assert_eq!(ModeDetector::detect_mode("What is 2+2?"), ChatMode::Quick);
        assert_eq!(
            ModeDetector::detect_mode("Who is the president?"),
            ChatMode::Quick
        );
        assert_eq!(
            ModeDetector::detect_mode("When is Christmas?"),
            ChatMode::Quick
        );
    }

    #[test]
    fn test_conversational_queries_use_quick_mode() {
        assert_eq!(
            ModeDetector::detect_mode("Hello, how are you?"),
            ChatMode::Quick
        );
        assert_eq!(
            ModeDetector::detect_mode("Can you help me with something?"),
            ChatMode::Quick
        );
        assert_eq!(
            ModeDetector::detect_mode("Thanks for the help!"),
            ChatMode::Quick
        );
    }

    #[test]
    fn test_research_queries_use_task_mode() {
        assert_eq!(
            ModeDetector::detect_mode("Research quantum computing advancements in 2024"),
            ChatMode::Task
        );
        assert_eq!(
            ModeDetector::detect_mode("Find out the latest trends in AI"),
            ChatMode::Task
        );
        assert_eq!(
            ModeDetector::detect_mode("Gather information about climate change"),
            ChatMode::Task
        );
    }

    #[test]
    fn test_analytical_queries_use_task_mode() {
        assert_eq!(
            ModeDetector::detect_mode(
                "Compare React vs Vue performance and explain the trade-offs"
            ),
            ChatMode::Task
        );
        assert_eq!(
            ModeDetector::detect_mode("Analyze the pros and cons of microservices architecture"),
            ChatMode::Task
        );
        assert_eq!(
            ModeDetector::detect_mode("Evaluate the impact of AI on job markets"),
            ChatMode::Task
        );
    }

    #[test]
    fn test_complex_queries_use_task_mode() {
        assert_eq!(
            ModeDetector::detect_mode("Conduct a comprehensive analysis of market trends"),
            ChatMode::Task
        );
        assert_eq!(
            ModeDetector::detect_mode("Provide a detailed investigation into the issue"),
            ChatMode::Task
        );
    }

    #[test]
    fn test_multi_step_queries_use_task_mode() {
        assert_eq!(
            ModeDetector::detect_mode(
                "First, analyze the data. Then, create a report. Finally, summarize findings."
            ),
            ChatMode::Task
        );
    }

    #[test]
    fn test_code_generation_uses_task_mode() {
        assert_eq!(
            ModeDetector::detect_mode("Write code to implement a binary search tree"),
            ChatMode::Task
        );
        assert_eq!(
            ModeDetector::detect_mode("Generate a REST API with authentication"),
            ChatMode::Task
        );
    }

    #[test]
    fn test_long_queries_use_task_mode() {
        let long_query = "word ".repeat(600); // 600 words
        assert_eq!(ModeDetector::detect_mode(&long_query), ChatMode::Task);
    }

    #[test]
    fn test_confidence_scoring() {
        let score_simple = ModeDetector::confidence_score("What is 2+2?");
        assert!(score_simple > 0.5);

        let score_complex =
            ModeDetector::confidence_score("Research and analyze quantum computing");
        assert!(score_complex > 0.7);
    }

    #[test]
    fn test_query_analysis() {
        let analysis = ModeDetector::analyze_query("Research and compare AI frameworks");
        assert!(analysis.has_research_markers);
        assert!(analysis.has_complexity_markers);
        assert_eq!(analysis.recommended_mode, ChatMode::Task);
    }
}
