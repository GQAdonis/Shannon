//! Enhanced settings system with comprehensive configuration support.
//!
//! This module extends the basic settings with structured application settings
//! including providers, models, appearance, context, knowledge, MCP, and advanced settings.

use crate::database::encryption::KeyManager;
use crate::database::hybrid::HybridBackend;
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Comprehensive application settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// User ID or "default" for system-wide settings
    pub id: String,
    /// Provider configurations
    pub providers: Vec<ProviderSettings>,
    /// Model preferences
    pub models: ModelPreferences,
    /// Appearance and theme settings
    pub appearance: AppearanceSettings,
    /// Context management settings (from Phase 5)
    pub context: ContextSettings,
    /// Knowledge base settings (from Phase 7)
    pub knowledge: KnowledgeSettings,
    /// MCP server settings (from Phase 6)
    pub mcp: MCPSettings,
    /// Advanced settings
    pub advanced: AdvancedSettings,
    /// When the settings were created
    pub created_at: DateTime<Utc>,
    /// When the settings were last updated
    pub updated_at: DateTime<Utc>,
}

/// Provider configuration for LLM services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderSettings {
    /// Provider identifier (openai, anthropic, google, groq, xai)
    pub provider: String,
    /// API key (encrypted in storage)
    pub api_key: String,
    /// Custom API base URL (optional)
    pub api_base: Option<String>,
    /// Whether provider is enabled
    pub enabled: bool,
    /// Default model for this provider
    pub default_model: String,
    /// Additional provider-specific configuration
    pub config: HashMap<String, serde_json::Value>,
}

/// Model selection preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPreferences {
    /// Default model for quick queries
    pub default_quick_model: String,
    /// Default model for complex tasks
    pub default_task_model: String,
    /// Default embedding model
    pub default_embedding_model: String,
    /// Per-capability model overrides
    pub model_overrides: HashMap<String, String>,
    /// Model-specific temperature settings
    pub temperature_overrides: HashMap<String, f32>,
}

impl Default for ModelPreferences {
    fn default() -> Self {
        Self {
            default_quick_model: "gpt-4o-mini".to_string(),
            default_task_model: "gpt-4o".to_string(),
            default_embedding_model: "text-embedding-3-small".to_string(),
            model_overrides: HashMap::new(),
            temperature_overrides: HashMap::new(),
        }
    }
}

/// Appearance and theme configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    /// Theme mode
    pub theme: ThemeMode,
    /// Custom theme configuration
    pub custom_theme: Option<CustomTheme>,
    /// Language code (en, es, fr, etc.)
    pub language: String,
    /// Font family
    pub font_family: String,
    /// Font size in pixels
    pub font_size: u8,
    /// Message density (compact, normal, comfortable)
    pub message_density: String,
    /// Sidebar position (left, right)
    pub sidebar_position: String,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Auto,
            custom_theme: None,
            language: "en".to_string(),
            font_family: "Inter".to_string(),
            font_size: 14,
            message_density: "normal".to_string(),
            sidebar_position: "left".to_string(),
        }
    }
}

/// Theme mode selection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
    /// System preference
    Auto,
    /// Custom theme by name
    Custom(String),
}

/// Custom theme definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTheme {
    /// Theme name
    pub name: String,
    /// Color palette
    pub colors: HashMap<String, String>,
    /// Font definitions
    pub fonts: HashMap<String, String>,
    /// Spacing definitions
    pub spacing: HashMap<String, String>,
}

/// Context management settings (from Phase 5).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSettings {
    /// Maximum context window size
    pub max_context_tokens: usize,
    /// Context retention strategy
    pub retention_strategy: String,
    /// Whether to enable auto-summarization
    pub auto_summarize: bool,
}

impl Default for ContextSettings {
    fn default() -> Self {
        Self {
            max_context_tokens: 128_000,
            retention_strategy: "sliding_window".to_string(),
            auto_summarize: true,
        }
    }
}

/// Knowledge base settings (from Phase 7).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSettings {
    /// Default chunking strategy
    pub chunking_strategy: String,
    /// Chunk size for documents
    pub chunk_size: usize,
    /// Chunk overlap
    pub chunk_overlap: usize,
    /// Default embedding provider
    pub embedding_provider: String,
    /// Similarity threshold for retrieval
    pub similarity_threshold: f32,
    /// Maximum results to retrieve
    pub max_results: usize,
}

impl Default for KnowledgeSettings {
    fn default() -> Self {
        Self {
            chunking_strategy: "recursive".to_string(),
            chunk_size: 1000,
            chunk_overlap: 200,
            embedding_provider: "openai".to_string(),
            similarity_threshold: 0.7,
            max_results: 5,
        }
    }
}

/// MCP server settings (from Phase 6).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPSettings {
    /// Whether MCP is enabled
    pub enabled: bool,
    /// Configured MCP servers
    pub servers: Vec<MCPServerConfig>,
    /// Auto-discover MCP servers
    pub auto_discover: bool,
}

impl Default for MCPSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            servers: Vec::new(),
            auto_discover: true,
        }
    }
}

/// MCP server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPServerConfig {
    /// Server name
    pub name: String,
    /// Server command
    pub command: String,
    /// Server arguments
    pub args: Vec<String>,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Whether server is enabled
    pub enabled: bool,
}

/// Advanced system settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    /// Enable debug mode
    pub debug_mode: bool,
    /// Enable telemetry
    pub telemetry_enabled: bool,
    /// Enable auto-update
    pub auto_update: bool,
    /// Maximum concurrent requests
    pub concurrent_requests: u8,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Enable experimental features
    pub experimental_features: bool,
}

impl Default for AdvancedSettings {
    fn default() -> Self {
        Self {
            debug_mode: false,
            telemetry_enabled: false,
            auto_update: true,
            concurrent_requests: 5,
            request_timeout: 300,
            experimental_features: false,
        }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            id: "default".to_string(),
            providers: Vec::new(),
            models: ModelPreferences::default(),
            appearance: AppearanceSettings::default(),
            context: ContextSettings::default(),
            knowledge: KnowledgeSettings::default(),
            mcp: MCPSettings::default(),
            advanced: AdvancedSettings::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Repository trait for enhanced settings management.
#[async_trait]
pub trait SettingsV2Repository: Send + Sync {
    /// Get comprehensive settings for a user.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    async fn get_app_settings(&self, user_id: &str) -> Result<AppSettings>;

    /// Save comprehensive settings for a user.
    ///
    /// # Errors
    ///
    /// Returns an error if the database write fails.
    async fn save_app_settings(&self, settings: &AppSettings) -> Result<()>;

    /// Export settings as YAML.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    async fn export_settings(&self, user_id: &str) -> Result<String>;

    /// Import settings from YAML.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization or save fails.
    async fn import_settings(&self, user_id: &str, yaml: &str) -> Result<()>;

    /// Update a specific settings section.
    ///
    /// # Errors
    ///
    /// Returns an error if the update fails.
    async fn update_section(
        &self,
        user_id: &str,
        section: SettingsSection,
        value: serde_json::Value,
    ) -> Result<()>;
}

/// Settings section identifier for partial updates.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SettingsSection {
    /// Provider settings
    Providers,
    /// Model preferences
    Models,
    /// Appearance settings
    Appearance,
    /// Context settings
    Context,
    /// Knowledge settings
    Knowledge,
    /// MCP settings
    Mcp,
    /// Advanced settings
    Advanced,
}

#[async_trait]
impl SettingsV2Repository for HybridBackend {
    async fn get_app_settings(&self, user_id: &str) -> Result<AppSettings> {
        let user_id = user_id.to_string();
        let sqlite = self.sqlite.clone();

        tokio::task::spawn_blocking(move || -> Result<AppSettings> {
            let guard = sqlite.lock().unwrap();
            let conn = guard
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("SQLite not initialized"))?;

            // Try to load from app_settings table
            let mut stmt = conn.prepare(
                "SELECT settings_json, created_at, updated_at
                 FROM app_settings WHERE user_id = ?1",
            )?;

            let mut rows = stmt.query(params![user_id])?;

            if let Some(row) = rows.next()? {
                let json_str: String = row.get(0)?;
                let mut settings: AppSettings =
                    serde_json::from_str(&json_str).context("Failed to parse settings JSON")?;

                settings.id = user_id;
                settings.created_at = parse_datetime(row.get::<_, String>(1)?);
                settings.updated_at = parse_datetime(row.get::<_, String>(2)?);

                Ok(settings)
            } else {
                // Return default settings if none exist
                let mut settings = AppSettings::default();
                settings.id = user_id;
                Ok(settings)
            }
        })
        .await
        .context("Tokio spawn_blocking failed")?
    }

    async fn save_app_settings(&self, settings: &AppSettings) -> Result<()> {
        let user_id = settings.id.clone();
        let settings_json =
            serde_json::to_string(settings).context("Failed to serialize settings")?;
        let sqlite = self.sqlite.clone();
        let now = Utc::now().to_rfc3339();

        tokio::task::spawn_blocking(move || -> Result<()> {
            let guard = sqlite.lock().unwrap();
            let conn = guard
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("SQLite not initialized"))?;

            conn.execute(
                "INSERT INTO app_settings (user_id, settings_json, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?3)
                 ON CONFLICT(user_id) DO UPDATE SET
                   settings_json = excluded.settings_json,
                   updated_at = excluded.updated_at",
                params![user_id, settings_json, now],
            )?;
            Ok(())
        })
        .await
        .context("Tokio spawn_blocking failed")?
    }

    async fn export_settings(&self, user_id: &str) -> Result<String> {
        let settings = self.get_app_settings(user_id).await?;

        // Mask sensitive data before export
        let mut export_settings = settings.clone();
        for provider in &mut export_settings.providers {
            provider.api_key = "***REDACTED***".to_string();
        }

        serde_yaml::to_string(&export_settings).context("Failed to serialize settings to YAML")
    }

    async fn import_settings(&self, user_id: &str, yaml: &str) -> Result<()> {
        let mut settings: AppSettings =
            serde_yaml::from_str(yaml).context("Failed to parse settings YAML")?;

        settings.id = user_id.to_string();
        settings.updated_at = Utc::now();

        // Don't import redacted API keys
        settings.providers.retain(|p| p.api_key != "***REDACTED***");

        self.save_app_settings(&settings).await
    }

    async fn update_section(
        &self,
        user_id: &str,
        section: SettingsSection,
        value: serde_json::Value,
    ) -> Result<()> {
        let mut settings = self.get_app_settings(user_id).await?;

        match section {
            SettingsSection::Providers => {
                settings.providers =
                    serde_json::from_value(value).context("Invalid providers data")?;
            }
            SettingsSection::Models => {
                settings.models = serde_json::from_value(value).context("Invalid models data")?;
            }
            SettingsSection::Appearance => {
                settings.appearance =
                    serde_json::from_value(value).context("Invalid appearance data")?;
            }
            SettingsSection::Context => {
                settings.context = serde_json::from_value(value).context("Invalid context data")?;
            }
            SettingsSection::Knowledge => {
                settings.knowledge =
                    serde_json::from_value(value).context("Invalid knowledge data")?;
            }
            SettingsSection::Mcp => {
                settings.mcp = serde_json::from_value(value).context("Invalid MCP data")?;
            }
            SettingsSection::Advanced => {
                settings.advanced =
                    serde_json::from_value(value).context("Invalid advanced data")?;
            }
        }

        settings.updated_at = Utc::now();
        self.save_app_settings(&settings).await
    }
}

/// Parse datetime from RFC3339 string.
fn parse_datetime(value: String) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(&value)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

/// SQL schema for app_settings table.
pub const APP_SETTINGS_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS app_settings (
    user_id TEXT PRIMARY KEY,
    settings_json TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);
"#;
