//! Agent management commands for Tauri.
//!
//! This module provides commands for managing AI agent specifications in the
//! embedded desktop application.

use shannon_api::database::{AgentFilter, AgentRepository, AgentSpec, ModelConfig};
use std::sync::Arc;
use tauri::State;

/// State for managing agents with the database backend.
pub struct AgentState {
    repo: Arc<dyn AgentRepository>,
}

impl AgentState {
    /// Create a new agent state with the given repository.
    pub fn new(repo: Arc<dyn AgentRepository>) -> Self {
        Self { repo }
    }
}

/// Create a new agent.
#[tauri::command]
pub async fn create_agent(spec: AgentSpec, state: State<'_, AgentState>) -> Result<String, String> {
    state
        .repo
        .create(&spec)
        .await
        .map_err(|e| format!("Failed to create agent: {}", e))
}

/// Get an agent by ID.
#[tauri::command]
pub async fn get_agent(id: String, state: State<'_, AgentState>) -> Result<AgentSpec, String> {
    state
        .repo
        .get(&id)
        .await
        .map_err(|e| format!("Failed to get agent: {}", e))?
        .ok_or_else(|| format!("Agent not found: {}", id))
}

/// List agents with optional filtering.
#[tauri::command]
pub async fn list_agents(
    category: Option<String>,
    tags: Option<Vec<String>>,
    search: Option<String>,
    state: State<'_, AgentState>,
) -> Result<Vec<AgentSpec>, String> {
    let filter = if category.is_some() || tags.is_some() || search.is_some() {
        Some(AgentFilter {
            category,
            tags,
            search,
        })
    } else {
        None
    };

    state
        .repo
        .list(filter)
        .await
        .map_err(|e| format!("Failed to list agents: {}", e))
}

/// Update an existing agent.
#[tauri::command]
pub async fn update_agent(
    id: String,
    spec: AgentSpec,
    state: State<'_, AgentState>,
) -> Result<(), String> {
    state
        .repo
        .update(&id, &spec)
        .await
        .map_err(|e| format!("Failed to update agent: {}", e))
}

/// Delete an agent.
#[tauri::command]
pub async fn delete_agent(id: String, state: State<'_, AgentState>) -> Result<bool, String> {
    state
        .repo
        .delete(&id)
        .await
        .map_err(|e| format!("Failed to delete agent: {}", e))
}

/// Export an agent to YAML format.
#[tauri::command]
pub async fn export_agent(id: String, state: State<'_, AgentState>) -> Result<String, String> {
    state
        .repo
        .export(&id)
        .await
        .map_err(|e| format!("Failed to export agent: {}", e))
}

/// Import an agent from YAML format.
#[tauri::command]
pub async fn import_agent(yaml: String, state: State<'_, AgentState>) -> Result<String, String> {
    state
        .repo
        .import(&yaml)
        .await
        .map_err(|e| format!("Failed to import agent: {}", e))
}
