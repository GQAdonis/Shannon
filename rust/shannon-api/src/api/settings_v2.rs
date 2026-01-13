//! Enhanced settings API endpoints.
//!
//! This module provides REST API endpoints for comprehensive application settings
//! including providers, models, appearance, context, knowledge, MCP, and advanced settings.

use crate::database::{AppSettings, SettingsSection, SettingsV2Repository};
use crate::gateway::embedded_auth::AuthUser;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Create the settings v2 router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v2/settings", get(get_app_settings))
        .route("/api/v2/settings", put(update_app_settings))
        .route("/api/v2/settings/export", get(export_settings))
        .route("/api/v2/settings/import", post(import_settings))
        .route("/api/v2/settings/:section", put(update_settings_section))
}

/// Get comprehensive application settings.
///
/// # Errors
///
/// Returns an error if the database query fails.
async fn get_app_settings(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<AppSettings>, AppError> {
    let settings = state.db.get_app_settings(&user.user_id).await?;
    Ok(Json(settings))
}

/// Update request for app settings.
#[derive(Debug, Deserialize)]
struct UpdateAppSettingsRequest {
    settings: AppSettings,
}

/// Update comprehensive application settings.
///
/// # Errors
///
/// Returns an error if the database write fails.
async fn update_app_settings(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(req): Json<UpdateAppSettingsRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let mut settings = req.settings;
    settings.id = user.user_id.clone();

    state.db.save_app_settings(&settings).await?;

    // Emit settings updated event for hot-reload
    #[cfg(feature = "events")]
    {
        if let Some(event_tx) = &state.event_tx {
            let _ = event_tx.send(crate::events::ServerEvent::SettingsUpdated {
                user_id: user.user_id,
            });
        }
    }

    Ok(Json(MessageResponse {
        message: "Settings updated successfully".to_string(),
    }))
}

/// Export settings as YAML.
///
/// # Errors
///
/// Returns an error if serialization fails.
async fn export_settings(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Response, AppError> {
    let yaml = state.db.export_settings(&user.user_id).await?;

    Ok((
        StatusCode::OK,
        [
            ("Content-Type", "application/x-yaml"),
            (
                "Content-Disposition",
                "attachment; filename=\"shannon-settings.yaml\"",
            ),
        ],
        yaml,
    )
        .into_response())
}

/// Import request for settings.
#[derive(Debug, Deserialize)]
struct ImportSettingsRequest {
    yaml: String,
}

/// Import settings from YAML.
///
/// # Errors
///
/// Returns an error if deserialization or save fails.
async fn import_settings(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(req): Json<ImportSettingsRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    state.db.import_settings(&user.user_id, &req.yaml).await?;

    // Emit settings updated event
    #[cfg(feature = "events")]
    {
        if let Some(event_tx) = &state.event_tx {
            let _ = event_tx.send(crate::events::ServerEvent::SettingsUpdated {
                user_id: user.user_id,
            });
        }
    }

    Ok(Json(MessageResponse {
        message: "Settings imported successfully".to_string(),
    }))
}

/// Update a specific settings section.
///
/// # Errors
///
/// Returns an error if the update fails.
async fn update_settings_section(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(section): Path<String>,
    Json(value): Json<serde_json::Value>,
) -> Result<Json<MessageResponse>, AppError> {
    let section = match section.as_str() {
        "providers" => SettingsSection::Providers,
        "models" => SettingsSection::Models,
        "appearance" => SettingsSection::Appearance,
        "context" => SettingsSection::Context,
        "knowledge" => SettingsSection::Knowledge,
        "mcp" => SettingsSection::Mcp,
        "advanced" => SettingsSection::Advanced,
        _ => {
            return Err(AppError::BadRequest(format!(
                "Invalid section: {}",
                section
            )));
        }
    };

    state
        .db
        .update_section(&user.user_id, section, value)
        .await?;

    // Emit settings updated event
    #[cfg(feature = "events")]
    {
        if let Some(event_tx) = &state.event_tx {
            let _ = event_tx.send(crate::events::ServerEvent::SettingsUpdated {
                user_id: user.user_id,
            });
        }
    }

    Ok(Json(MessageResponse {
        message: format!("Settings section '{}' updated successfully", section),
    }))
}

/// Generic success message response.
#[derive(Debug, Serialize)]
struct MessageResponse {
    message: String,
}

/// API error type.
#[derive(Debug)]
enum AppError {
    Database(anyhow::Error),
    BadRequest(String),
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::Database(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::Database(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", err),
            ),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        (
            status,
            Json(serde_json::json!({
                "error": message,
            })),
        )
            .into_response()
    }
}
