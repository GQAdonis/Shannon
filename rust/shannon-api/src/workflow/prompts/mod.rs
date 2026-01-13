//! Prompt template rendering system.
//!
//! Provides a flexible template system for rendering prompts for different
//! cognitive patterns (CoT, Research, Debate, etc.).
//!
//! # Architecture
//!
//! - Templates are stored as embedded resources or loaded from `config/templates/`
//! - Handlebars is used for variable interpolation
//! - Each pattern has a system prompt and user prompt template
//!
//! # Usage
//!
//! ```rust,ignore
//! use shannon_api::workflow::prompts::PromptRenderer;
//!
//! let renderer = PromptRenderer::new()?;
//! let rendered = renderer.render("chain_of_thought", &context)?;
//! println!("System: {}", rendered.system);
//! println!("User: {}", rendered.user);
//! ```

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Rendered prompt with system and user components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedPrompt {
    /// System prompt (instructions for the model).
    pub system: String,
    /// User prompt (the actual query/task).
    pub user: String,
    /// Additional metadata about the rendering.
    pub metadata: PromptMetadata,
}

/// Metadata about a rendered prompt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptMetadata {
    /// Pattern name used.
    pub pattern: String,
    /// Variables that were substituted.
    pub variables: HashMap<String, String>,
    /// Estimated token count.
    pub estimated_tokens: usize,
}

/// Template for a cognitive pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    /// Pattern name (e.g., "chain_of_thought", "research").
    pub name: String,
    /// System prompt template.
    pub system_template: String,
    /// User prompt template.
    pub user_template: String,
    /// Description of the pattern.
    pub description: Option<String>,
    /// Required variables for this template.
    pub required_vars: Vec<String>,
    /// Optional variables with defaults.
    #[serde(default)]
    pub optional_vars: HashMap<String, String>,
}

/// Prompt renderer for cognitive patterns.
///
/// This renderer loads templates and renders them with context variables
/// using Handlebars templating.
#[derive(Clone, Debug)]
pub struct PromptRenderer {
    /// Handlebars template engine.
    engine: handlebars::Handlebars<'static>,
    /// Loaded templates by pattern name.
    templates: Arc<HashMap<String, PromptTemplate>>,
    /// Template directory path.
    template_dir: Option<PathBuf>,
}

impl PromptRenderer {
    /// Create a new prompt renderer.
    ///
    /// This loads all built-in templates and optionally loads custom templates
    /// from the config directory.
    pub fn new() -> anyhow::Result<Self> {
        let mut engine = handlebars::Handlebars::new();

        // Disable HTML escaping (we're generating prompts, not HTML)
        engine.register_escape_fn(handlebars::no_escape);

        // Register built-in templates
        let templates = Self::load_builtin_templates(&mut engine)?;

        tracing::info!("✅ Loaded {} built-in prompt templates", templates.len());

        Ok(Self {
            engine,
            templates: Arc::new(templates),
            template_dir: None,
        })
    }

    /// Create a renderer with a custom template directory.
    pub fn with_template_dir(template_dir: PathBuf) -> anyhow::Result<Self> {
        let mut renderer = Self::new()?;
        renderer.template_dir = Some(template_dir.clone());

        // Load custom templates from directory if it exists
        if template_dir.exists() {
            let custom_templates =
                Self::load_directory_templates(&mut renderer.engine, &template_dir)?;
            if !custom_templates.is_empty() {
                tracing::info!(
                    "✅ Loaded {} custom templates from {:?}",
                    custom_templates.len(),
                    template_dir
                );

                // Merge with built-in templates (custom templates override built-in)
                let mut all_templates = (*renderer.templates).clone();
                all_templates.extend(custom_templates);
                renderer.templates = Arc::new(all_templates);
            }
        }

        Ok(renderer)
    }

    /// Render a prompt template with the given context.
    pub fn render(&self, pattern: &str, context: &Value) -> anyhow::Result<RenderedPrompt> {
        let template = self
            .templates
            .get(pattern)
            .ok_or_else(|| anyhow::anyhow!("Template not found: {}", pattern))?;

        // Validate required variables
        self.validate_context(template, context)?;

        // Render system and user prompts
        let system = self
            .engine
            .render(&format!("{}_system", pattern), context)?;
        let user = self.engine.render(&format!("{}_user", pattern), context)?;

        // Extract variable values for metadata
        let mut variables = HashMap::new();
        if let Some(obj) = context.as_object() {
            for (key, value) in obj {
                if let Some(s) = value.as_str() {
                    variables.insert(key.clone(), s.to_string());
                }
            }
        }

        // Estimate token count (rough approximation: 1 token ≈ 4 characters)
        let estimated_tokens = (system.len() + user.len()) / 4;

        Ok(RenderedPrompt {
            system,
            user,
            metadata: PromptMetadata {
                pattern: pattern.to_string(),
                variables,
                estimated_tokens,
            },
        })
    }

    /// List available templates.
    pub fn list_templates(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }

    /// Get template information.
    pub fn get_template(&self, pattern: &str) -> Option<&PromptTemplate> {
        self.templates.get(pattern)
    }

    /// Validate that the context contains all required variables.
    fn validate_context(&self, template: &PromptTemplate, context: &Value) -> anyhow::Result<()> {
        let context_obj = context
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Context must be a JSON object"))?;

        for required_var in &template.required_vars {
            if !context_obj.contains_key(required_var) {
                anyhow::bail!("Missing required variable: {}", required_var);
            }
        }

        Ok(())
    }

    /// Load built-in templates.
    fn load_builtin_templates(
        engine: &mut handlebars::Handlebars<'static>,
    ) -> anyhow::Result<HashMap<String, PromptTemplate>> {
        let mut templates = HashMap::new();

        // Chain of Thought template
        let cot_template = PromptTemplate {
            name: "chain_of_thought".to_string(),
            system_template: include_str!("templates/chain_of_thought_system.txt").to_string(),
            user_template: include_str!("templates/chain_of_thought_user.txt").to_string(),
            description: Some("Step-by-step reasoning for complex problems".to_string()),
            required_vars: vec!["query".to_string()],
            optional_vars: HashMap::from([("context".to_string(), "".to_string())]),
        };
        engine
            .register_template_string("chain_of_thought_system", &cot_template.system_template)?;
        engine.register_template_string("chain_of_thought_user", &cot_template.user_template)?;
        templates.insert("chain_of_thought".to_string(), cot_template);

        // Research template
        let research_template = PromptTemplate {
            name: "research".to_string(),
            system_template: include_str!("templates/research_system.txt").to_string(),
            user_template: include_str!("templates/research_user.txt").to_string(),
            description: Some("Deep research with source verification".to_string()),
            required_vars: vec!["query".to_string()],
            optional_vars: HashMap::from([
                ("context".to_string(), "".to_string()),
                ("sources".to_string(), "".to_string()),
            ]),
        };
        engine.register_template_string("research_system", &research_template.system_template)?;
        engine.register_template_string("research_user", &research_template.user_template)?;
        templates.insert("research".to_string(), research_template);

        // Debate template
        let debate_template = PromptTemplate {
            name: "debate".to_string(),
            system_template: include_str!("templates/debate_system.txt").to_string(),
            user_template: include_str!("templates/debate_user.txt").to_string(),
            description: Some("Multi-perspective analysis and critique".to_string()),
            required_vars: vec!["query".to_string()],
            optional_vars: HashMap::from([
                ("context".to_string(), "".to_string()),
                ("perspectives".to_string(), "3".to_string()),
            ]),
        };
        engine.register_template_string("debate_system", &debate_template.system_template)?;
        engine.register_template_string("debate_user", &debate_template.user_template)?;
        templates.insert("debate".to_string(), debate_template);

        // Tree of Thoughts template
        let tot_template = PromptTemplate {
            name: "tree_of_thoughts".to_string(),
            system_template: include_str!("templates/tree_of_thoughts_system.txt").to_string(),
            user_template: include_str!("templates/tree_of_thoughts_user.txt").to_string(),
            description: Some("Explore multiple solution paths".to_string()),
            required_vars: vec!["query".to_string()],
            optional_vars: HashMap::from([
                ("context".to_string(), "".to_string()),
                ("branches".to_string(), "3".to_string()),
            ]),
        };
        engine
            .register_template_string("tree_of_thoughts_system", &tot_template.system_template)?;
        engine.register_template_string("tree_of_thoughts_user", &tot_template.user_template)?;
        templates.insert("tree_of_thoughts".to_string(), tot_template);

        Ok(templates)
    }

    /// Load templates from a directory.
    fn load_directory_templates(
        engine: &mut handlebars::Handlebars<'static>,
        dir: &PathBuf,
    ) -> anyhow::Result<HashMap<String, PromptTemplate>> {
        let mut templates = HashMap::new();

        if !dir.exists() {
            return Ok(templates);
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("yaml")
                || path.extension().and_then(|s| s.to_str()) == Some("yml")
            {
                let content = std::fs::read_to_string(&path)?;
                let template: PromptTemplate = serde_yaml::from_str(&content)?;

                // Register templates with engine
                engine.register_template_string(
                    &format!("{}_system", template.name),
                    &template.system_template,
                )?;
                engine.register_template_string(
                    &format!("{}_user", template.name),
                    &template.user_template,
                )?;

                templates.insert(template.name.clone(), template);
            }
        }

        Ok(templates)
    }
}

impl Default for PromptRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create default PromptRenderer")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_render_chain_of_thought() {
        let renderer = PromptRenderer::new().unwrap();
        let context = json!({
            "query": "What is the capital of France?",
            "context": ""
        });

        let rendered = renderer.render("chain_of_thought", &context).unwrap();

        assert!(!rendered.system.is_empty());
        assert!(!rendered.user.is_empty());
        assert!(rendered.user.contains("France"));
        assert_eq!(rendered.metadata.pattern, "chain_of_thought");
    }

    #[test]
    fn test_missing_required_variable() {
        let renderer = PromptRenderer::new().unwrap();
        let context = json!({});

        let result = renderer.render("chain_of_thought", &context);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing required variable"));
    }

    #[test]
    fn test_list_templates() {
        let renderer = PromptRenderer::new().unwrap();
        let templates = renderer.list_templates();

        assert!(templates.contains(&"chain_of_thought".to_string()));
        assert!(templates.contains(&"research".to_string()));
        assert!(templates.contains(&"debate".to_string()));
        assert!(templates.contains(&"tree_of_thoughts".to_string()));
    }
}
