//! Context settings database module for managing conversation context strategies
//!
//! This module provides persistent storage for context management settings,
//! including strategy selection and configuration parameters.

use anyhow::{Context as AnyhowContext, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Context management strategy types
///
/// Each strategy has different trade-offs between context preservation,
/// token usage, and summarization overhead.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContextStrategy {
    /// Keep only recent messages within token budget
    ///
    /// Best for: Real-time conversations with limited history needs
    SlidingWindow,

    /// Summarize older messages progressively
    ///
    /// Best for: Long conversations where history matters but token budget is tight
    ProgressiveSummarization,

    /// Three-tier system: verbatim recent, summarized mid-term, key facts long-term
    ///
    /// Best for: Complex conversations requiring both detail and long-term memory
    HierarchicalMemory,

    /// Preserve initial instructions + recent context, remove middle
    ///
    /// Best for: Conversations with important system prompts and recent context
    KeepFirstLast,
}

impl std::fmt::Display for ContextStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SlidingWindow => write!(f, "sliding_window"),
            Self::ProgressiveSummarization => write!(f, "progressive_summarization"),
            Self::HierarchicalMemory => write!(f, "hierarchical_memory"),
            Self::KeepFirstLast => write!(f, "keep_first_last"),
        }
    }
}

impl std::str::FromStr for ContextStrategy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "sliding_window" => Ok(Self::SlidingWindow),
            "progressive_summarization" => Ok(Self::ProgressiveSummarization),
            "hierarchical_memory" => Ok(Self::HierarchicalMemory),
            "keep_first_last" => Ok(Self::KeepFirstLast),
            _ => anyhow::bail!("Unknown context strategy: {}", s),
        }
    }
}

/// Context management settings
///
/// Controls how conversation history is managed, summarized, and preserved
/// within token budgets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSettings {
    /// Unique identifier (user or session ID)
    pub id: String,

    /// Active context management strategy
    pub strategy: ContextStrategy,

    /// Number of recent conversation turns to keep verbatim
    ///
    /// One turn = user message + assistant response
    pub short_term_turns: u32,

    /// Token budget for mid-term context (summarized)
    pub mid_term_budget: u32,

    /// Token budget for long-term context (key facts)
    pub long_term_budget: u32,

    /// Model to use for summarization
    ///
    /// Should be a fast, cost-effective model like "claude-haiku-4-5@20251001"
    pub summarization_model: String,

    /// Creation timestamp (RFC 3339 format)
    pub created_at: String,

    /// Last update timestamp (RFC 3339 format)
    pub updated_at: String,
}

impl Default for ContextSettings {
    fn default() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: "default".to_string(),
            strategy: ContextStrategy::HierarchicalMemory,
            short_term_turns: 5,
            mid_term_budget: 2000,
            long_term_budget: 500,
            summarization_model: "claude-haiku-4-5@20251001".to_string(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

/// Repository for context settings persistence
///
/// Provides CRUD operations for context management configuration.
#[derive(Debug, Clone)]
pub struct ContextSettingsRepository {
    db_path: PathBuf,
}

impl ContextSettingsRepository {
    /// Create a new context settings repository
    ///
    /// # Arguments
    ///
    /// * `db_path` - Path to SQLite database file
    pub fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    /// Initialize the database schema
    ///
    /// Creates the `context_settings` table if it doesn't exist.
    /// Safe to call multiple times (idempotent).
    pub fn init_db(&self) -> Result<()> {
        let conn =
            Connection::open(&self.db_path).context("Failed to open context settings database")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS context_settings (
                id TEXT PRIMARY KEY,
                strategy TEXT NOT NULL,
                short_term_turns INTEGER NOT NULL DEFAULT 5,
                mid_term_budget INTEGER NOT NULL DEFAULT 2000,
                long_term_budget INTEGER NOT NULL DEFAULT 500,
                summarization_model TEXT NOT NULL DEFAULT 'claude-haiku-4-5@20251001',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )
        .context("Failed to create context_settings table")?;

        Ok(())
    }

    /// Get context settings by ID
    ///
    /// Returns default settings if not found.
    ///
    /// # Arguments
    ///
    /// * `id` - Settings identifier (user or session ID)
    pub async fn get(&self, id: &str) -> Result<ContextSettings> {
        let db_path = self.db_path.clone();
        let id = id.to_string();

        tokio::task::spawn_blocking(move || {
            let conn =
                Connection::open(&db_path).context("Failed to open context settings database")?;

            let mut stmt = conn
                .prepare(
                    "SELECT id, strategy, short_term_turns, mid_term_budget,
                     long_term_budget, summarization_model, created_at, updated_at
                     FROM context_settings WHERE id = ?1",
                )
                .context("Failed to prepare select statement")?;

            let result = stmt.query_row(params![id], |row| {
                let strategy_str: String = row.get(1)?;
                let strategy = match strategy_str.parse::<ContextStrategy>() {
                    Ok(s) => s,
                    Err(_) => return Err(rusqlite::Error::InvalidQuery),
                };

                Ok(ContextSettings {
                    id: row.get(0)?,
                    strategy,
                    short_term_turns: row.get(2)?,
                    mid_term_budget: row.get(3)?,
                    long_term_budget: row.get(4)?,
                    summarization_model: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            });

            match result {
                Ok(settings) => Ok(settings),
                Err(rusqlite::Error::QueryReturnedNoRows) => {
                    // Return default settings if not found
                    let mut default = ContextSettings::default();
                    default.id = id;
                    Ok(default)
                }
                Err(e) => Err(e).context("Failed to query context settings"),
            }
        })
        .await
        .context("Task join error")?
    }

    /// Insert or update context settings
    ///
    /// Uses UPSERT to insert new settings or update existing ones.
    ///
    /// # Arguments
    ///
    /// * `settings` - Settings to save
    pub async fn upsert(&self, settings: &ContextSettings) -> Result<()> {
        let db_path = self.db_path.clone();
        let settings = settings.clone();

        tokio::task::spawn_blocking(move || {
            let conn =
                Connection::open(&db_path).context("Failed to open context settings database")?;

            let now = chrono::Utc::now().to_rfc3339();
            let strategy_str = settings.strategy.to_string();

            conn.execute(
                "INSERT INTO context_settings 
                 (id, strategy, short_term_turns, mid_term_budget, long_term_budget, 
                  summarization_model, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                 ON CONFLICT(id) DO UPDATE SET
                    strategy = excluded.strategy,
                    short_term_turns = excluded.short_term_turns,
                    mid_term_budget = excluded.mid_term_budget,
                    long_term_budget = excluded.long_term_budget,
                    summarization_model = excluded.summarization_model,
                    updated_at = excluded.updated_at",
                params![
                    settings.id,
                    strategy_str,
                    settings.short_term_turns,
                    settings.mid_term_budget,
                    settings.long_term_budget,
                    settings.summarization_model,
                    settings.created_at,
                    now,
                ],
            )
            .context("Failed to upsert context settings")?;

            Ok(())
        })
        .await
        .context("Task join error")?
    }

    /// Delete context settings by ID
    ///
    /// # Arguments
    ///
    /// * `id` - Settings identifier to delete
    pub async fn delete(&self, id: &str) -> Result<()> {
        let db_path = self.db_path.clone();
        let id = id.to_string();

        tokio::task::spawn_blocking(move || {
            let conn =
                Connection::open(&db_path).context("Failed to open context settings database")?;

            conn.execute("DELETE FROM context_settings WHERE id = ?1", params![id])
                .context("Failed to delete context settings")?;

            Ok(())
        })
        .await
        .context("Task join error")?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_context_settings_crud() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let repo = ContextSettingsRepository::new(db_path);

        repo.init_db().unwrap();

        // Test insert
        let settings = ContextSettings {
            id: "user123".to_string(),
            strategy: ContextStrategy::SlidingWindow,
            short_term_turns: 10,
            ..Default::default()
        };

        repo.upsert(&settings).await.unwrap();

        // Test get
        let retrieved = repo.get("user123").await.unwrap();
        assert_eq!(retrieved.id, "user123");
        assert_eq!(retrieved.strategy, ContextStrategy::SlidingWindow);
        assert_eq!(retrieved.short_term_turns, 10);

        // Test update
        let mut updated = retrieved;
        updated.strategy = ContextStrategy::HierarchicalMemory;
        repo.upsert(&updated).await.unwrap();

        let retrieved = repo.get("user123").await.unwrap();
        assert_eq!(retrieved.strategy, ContextStrategy::HierarchicalMemory);

        // Test delete
        repo.delete("user123").await.unwrap();
        let retrieved = repo.get("user123").await.unwrap();
        assert_eq!(retrieved.id, "user123"); // Returns default
    }

    #[test]
    fn test_strategy_serialization() {
        let strategy = ContextStrategy::HierarchicalMemory;
        let serialized = serde_json::to_string(&strategy).unwrap();
        assert_eq!(serialized, "\"hierarchical_memory\"");

        let deserialized: ContextStrategy = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, strategy);
    }
}
