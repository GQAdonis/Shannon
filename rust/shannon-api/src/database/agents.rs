//! Agent repository for storing and managing AI agent specifications.
//!
//! This module provides storage and retrieval of agent specifications including
//! prompts, models, tools, knowledge bases, and behavior configuration.

use crate::database::hybrid::HybridBackend;
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Model configuration for an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Provider name (e.g., "openai", "anthropic", "google")
    pub provider: String,
    /// Model name (e.g., "gpt-4", "claude-3-opus")
    pub name: String,
    /// Temperature parameter (0.0-2.0)
    pub temperature: Option<f32>,
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
}

/// Agent specification domain object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    /// Unique agent ID
    pub id: String,
    /// Agent name
    pub name: String,
    /// Agent description
    pub description: String,
    /// Version string (e.g., "1.0.0")
    pub version: String,
    /// Author name (optional)
    pub author: Option<String>,
    
    /// System prompt for the agent
    pub system_prompt: String,
    /// Model configuration
    pub model: ModelConfig,
    
    /// MCP tool IDs enabled for this agent
    pub tools: Vec<String>,
    /// Knowledge base IDs for RAG
    pub knowledge_bases: Vec<String>,
    /// Allowed actions (browser, filesystem, etc.)
    pub allowed_actions: Vec<String>,
    
    /// Default workflow strategy (chain_of_thought, scientific, exploratory)
    pub strategy: Option<String>,
    /// Conversation style (formal, casual, technical)
    pub conversation_style: Option<String>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Category (e.g., "general", "code", "research")
    pub category: String,
    /// Icon identifier (emoji or icon name)
    pub icon: Option<String>,
    
    /// When the agent was created
    pub created_at: DateTime<Utc>,
    /// When the agent was last updated
    pub updated_at: DateTime<Utc>,
}

/// Filter for listing agents.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentFilter {
    /// Filter by category
    pub category: Option<String>,
    /// Filter by tags (any match)
    pub tags: Option<Vec<String>>,
    /// Search in name and description
    pub search: Option<String>,
}

/// Repository trait for agent management.
#[async_trait]
pub trait AgentRepository: Send + Sync {
    /// Create a new agent.
    async fn create(&self, spec: &AgentSpec) -> Result<String>;
    
    /// Get an agent by ID.
    async fn get(&self, id: &str) -> Result<Option<AgentSpec>>;
    
    /// List agents with optional filtering.
    async fn list(&self, filter: Option<AgentFilter>) -> Result<Vec<AgentSpec>>;
    
    /// Update an existing agent.
    async fn update(&self, id: &str, spec: &AgentSpec) -> Result<()>;
    
    /// Delete an agent.
    async fn delete(&self, id: &str) -> Result<bool>;
    
    /// Export agent to YAML format.
    async fn export(&self, id: &str) -> Result<String>;
    
    /// Import agent from YAML format.
    async fn import(&self, yaml: &str) -> Result<String>;
}

#[async_trait]
impl AgentRepository for HybridBackend {
    async fn create(&self, spec: &AgentSpec) -> Result<String> {
        let spec = spec.clone();
        let sqlite = self.sqlite.clone();
        
        tokio::task::spawn_blocking(move || -> Result<String> {
            let guard = sqlite.lock().unwrap();
            let conn = guard
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("SQLite not initialized"))?;
            
            conn.execute(
                "INSERT INTO agents (
                    id, name, description, version, author,
                    system_prompt, model_provider, model_name, model_temperature, model_max_tokens,
                    tools, knowledge_bases, allowed_actions,
                    strategy, conversation_style,
                    tags, category, icon,
                    created_at, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
                params![
                    spec.id,
                    spec.name,
                    spec.description,
                    spec.version,
                    spec.author,
                    spec.system_prompt,
                    spec.model.provider,
                    spec.model.name,
                    spec.model.temperature,
                    spec.model.max_tokens,
                    serde_json::to_string(&spec.tools)?,
                    serde_json::to_string(&spec.knowledge_bases)?,
                    serde_json::to_string(&spec.allowed_actions)?,
                    spec.strategy,
                    spec.conversation_style,
                    serde_json::to_string(&spec.tags)?,
                    spec.category,
                    spec.icon,
                    spec.created_at.to_rfc3339(),
                    spec.updated_at.to_rfc3339(),
                ],
            )?;
            
            Ok(spec.id)
        })
        .await
        .context("Tokio spawn_blocking failed")?
    }
    
    async fn get(&self, id: &str) -> Result<Option<AgentSpec>> {
        let id = id.to_string();
        let sqlite = self.sqlite.clone();
        
        tokio::task::spawn_blocking(move || -> Result<Option<AgentSpec>> {
            let guard = sqlite.lock().unwrap();
            let conn = guard
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("SQLite not initialized"))?;
            
            let mut stmt = conn.prepare(
                "SELECT id, name, description, version, author,
                        system_prompt, model_provider, model_name, model_temperature, model_max_tokens,
                        tools, knowledge_bases, allowed_actions,
                        strategy, conversation_style,
                        tags, category, icon,
                        created_at, updated_at
                 FROM agents WHERE id = ?1"
            )?;
            
            let mut rows = stmt.query(params![id])?;
            
            if let Some(row) = rows.next()? {
                Ok(Some(AgentSpec {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    version: row.get(3)?,
                    author: row.get(4)?,
                    system_prompt: row.get(5)?,
                    model: ModelConfig {
                        provider: row.get(6)?,
                        name: row.get(7)?,
                        temperature: row.get(8)?,
                        max_tokens: row.get(9)?,
                    },
                    tools: serde_json::from_str(&row.get::<_, String>(10)?)?,
                    knowledge_bases: serde_json::from_str(&row.get::<_, String>(11)?)?,
                    allowed_actions: serde_json::from_str(&row.get::<_, String>(12)?)?,
                    strategy: row.get(13)?,
                    conversation_style: row.get(14)?,
                    tags: serde_json::from_str(&row.get::<_, String>(15)?)?,
                    category: row.get(16)?,
                    icon: row.get(17)?,
                    created_at: parse_datetime(row.get::<_, String>(18)?),
                    updated_at: parse_datetime(row.get::<_, String>(19)?),
                }))
            } else {
                Ok(None)
            }
        })
        .await
        .context("Tokio spawn_blocking failed")?
    }
    
    async fn list(&self, filter: Option<AgentFilter>) -> Result<Vec<AgentSpec>> {
        let sqlite = self.sqlite.clone();
        
        tokio::task::spawn_blocking(move || -> Result<Vec<AgentSpec>> {
            let guard = sqlite.lock().unwrap();
            let conn = guard
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("SQLite not initialized"))?;
            
            let mut query = "SELECT id, name, description, version, author,
                                    system_prompt, model_provider, model_name, model_temperature, model_max_tokens,
                                    tools, knowledge_bases, allowed_actions,
                                    strategy, conversation_style,
                                    tags, category, icon,
                                    created_at, updated_at
                             FROM agents WHERE 1=1".to_string();
            let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
            
            if let Some(f) = &filter {
                if let Some(cat) = &f.category {
                    query.push_str(" AND category = ?");
                    params.push(Box::new(cat.clone()));
                }
                if let Some(search) = &f.search {
                    query.push_str(" AND (name LIKE ? OR description LIKE ?)");
                    let pattern = format!("%{}%", search);
                    params.push(Box::new(pattern.clone()));
                    params.push(Box::new(pattern));
                }
            }
            
            query.push_str(" ORDER BY created_at DESC");
            
            let mut stmt = conn.prepare(&query)?;
            let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
            
            let rows = stmt.query_map(&param_refs[..], |row| {
                Ok(AgentSpec {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    version: row.get(3)?,
                    author: row.get(4)?,
                    system_prompt: row.get(5)?,
                    model: ModelConfig {
                        provider: row.get(6)?,
                        name: row.get(7)?,
                        temperature: row.get(8)?,
                        max_tokens: row.get(9)?,
                    },
                    tools: serde_json::from_str(&row.get::<_, String>(10)?).unwrap_or_default(),
                    knowledge_bases: serde_json::from_str(&row.get::<_, String>(11)?).unwrap_or_default(),
                    allowed_actions: serde_json::from_str(&row.get::<_, String>(12)?).unwrap_or_default(),
                    strategy: row.get(13)?,
                    conversation_style: row.get(14)?,
                    tags: serde_json::from_str(&row.get::<_, String>(15)?).unwrap_or_default(),
                    category: row.get(16)?,
                    icon: row.get(17)?,
                    created_at: parse_datetime(row.get::<_, String>(18)?),
                    updated_at: parse_datetime(row.get::<_, String>(19)?),
                })
            })?;
            
            let mut agents = Vec::new();
            for agent_result in rows {
                agents.push(agent_result?);
            }
            
            // Filter by tags if specified
            if let Some(f) = &filter {
                if let Some(filter_tags) = &f.tags {
                    agents.retain(|agent| {
                        filter_tags.iter().any(|tag| agent.tags.contains(tag))
                    });
                }
            }
            
            Ok(agents)
        })
        .await
        .context("Tokio spawn_blocking failed")?
    }
    
    async fn update(&self, id: &str, spec: &AgentSpec) -> Result<()> {
        let id = id.to_string();
        let spec = spec.clone();
        let sqlite = self.sqlite.clone();
        
        tokio::task::spawn_blocking(move || -> Result<()> {
            let guard = sqlite.lock().unwrap();
            let conn = guard
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("SQLite not initialized"))?;
            
            let count = conn.execute(
                "UPDATE agents SET
                    name = ?1, description = ?2, version = ?3, author = ?4,
                    system_prompt = ?5, model_provider = ?6, model_name = ?7,
                    model_temperature = ?8, model_max_tokens = ?9,
                    tools = ?10, knowledge_bases = ?11, allowed_actions = ?12,
                    strategy = ?13, conversation_style = ?14,
                    tags = ?15, category = ?16, icon = ?17,
                    updated_at = ?18
                 WHERE id = ?19",
                params![
                    spec.name,
                    spec.description,
                    spec.version,
                    spec.author,
                    spec.system_prompt,
                    spec.model.provider,
                    spec.model.name,
                    spec.model.temperature,
                    spec.model.max_tokens,
                    serde_json::to_string(&spec.tools)?,
                    serde_json::to_string(&spec.knowledge_bases)?,
                    serde_json::to_string(&spec.allowed_actions)?,
                    spec.strategy,
                    spec.conversation_style,
                    serde_json::to_string(&spec.tags)?,
                    spec.category,
                    spec.icon,
                    Utc::now().to_rfc3339(),
                    id,
                ],
            )?;
            
            if count == 0 {
                anyhow::bail!("Agent not found: {}", id);
            }
            
            Ok(())
        })
        .await
        .context("Tokio spawn_blocking failed")?
    }
    
    async fn delete(&self, id: &str) -> Result<bool> {
        let id = id.to_string();
        let sqlite = self.sqlite.clone();
        
        tokio::task::spawn_blocking(move || -> Result<bool> {
            let guard = sqlite.lock().unwrap();
            let conn = guard
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("SQLite not initialized"))?;
            
            let count = conn.execute("DELETE FROM agents WHERE id = ?1", params![id])?;
            Ok(count > 0)
        })
        .await
        .context("Tokio spawn_blocking failed")?
    }
    
    async fn export(&self, id: &str) -> Result<String> {
        let spec = self
            .get(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Agent not found: {}", id))?;
        
        serde_yaml::to_string(&spec).context("Failed to serialize agent to YAML")
    }
    
    async fn import(&self, yaml: &str) -> Result<String> {
        let mut spec: AgentSpec = serde_yaml::from_str(yaml)
            .context("Failed to parse agent YAML")?;
        
        // Generate new ID and timestamps for imported agent
        spec.id = uuid::Uuid::new_v4().to_string();
        spec.created_at = Utc::now();
        spec.updated_at = Utc::now();
        
        self.create(&spec).await
    }
}

/// Parse datetime from RFC3339 string.
fn parse_datetime(value: String) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(&value)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_agent() -> AgentSpec {
        AgentSpec {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Test Agent".to_string(),
            description: "A test agent".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Test Author".to_string()),
            system_prompt: "You are a helpful assistant.".to_string(),
            model: ModelConfig {
                provider: "openai".to_string(),
                name: "gpt-4".to_string(),
                temperature: Some(0.7),
                max_tokens: Some(2000),
            },
            tools: vec!["web_search".to_string()],
            knowledge_bases: vec![],
            allowed_actions: vec!["browser".to_string()],
            strategy: Some("chain_of_thought".to_string()),
            conversation_style: Some("casual".to_string()),
            tags: vec!["general".to_string(), "assistant".to_string()],
            category: "general".to_string(),
            icon: Some("🤖".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    #[test]
    fn test_agent_serialization() {
        let agent = create_test_agent();
        let yaml = serde_yaml::to_string(&agent).unwrap();
        let deserialized: AgentSpec = serde_yaml::from_str(&yaml).unwrap();
        
        assert_eq!(agent.id, deserialized.id);
        assert_eq!(agent.name, deserialized.name);
        assert_eq!(agent.model.provider, deserialized.model.provider);
    }
}
