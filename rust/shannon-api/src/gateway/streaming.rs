//! SSE and WebSocket streaming endpoints.

use std::convert::Infallible;
use std::time::Duration;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse,
    },
    routing::get,
    Router,
};
use futures::SinkExt;
use serde::{Deserialize, Serialize};

use crate::AppState;

/// Streaming routes.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/tasks/{id}/stream", get(stream_task_events))
        .route("/api/v1/tasks/{id}/ws", get(websocket_task_events))
        .route("/api/v1/stream/events", get(stream_global_events))
}

/// Task event for streaming.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEvent {
    /// Event type.
    #[serde(rename = "type")]
    pub event_type: String,
    /// Task ID.
    pub task_id: String,
    /// Timestamp.
    pub timestamp: String,
    /// Event data.
    pub data: serde_json::Value,
}

impl TaskEvent {
    pub fn new(task_id: &str, event_type: &str, data: serde_json::Value) -> Self {
        Self {
            event_type: event_type.to_string(),
            task_id: task_id.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            data,
        }
    }

    pub fn status(task_id: &str, status: &str, message: Option<&str>) -> Self {
        Self::new(
            task_id,
            "status",
            serde_json::json!({
                "status": status,
                "message": message
            }),
        )
    }

    pub fn progress(task_id: &str, percent: u8, current_step: Option<&str>) -> Self {
        Self::new(
            task_id,
            "progress",
            serde_json::json!({
                "percent": percent,
                "current_step": current_step
            }),
        )
    }

    pub fn content(task_id: &str, content: &str, is_final: bool) -> Self {
        Self::new(
            task_id,
            if is_final { "content_final" } else { "content" },
            serde_json::json!({
                "content": content,
                "is_final": is_final
            }),
        )
    }

    pub fn error(task_id: &str, error: &str, code: Option<&str>) -> Self {
        Self::new(
            task_id,
            "error",
            serde_json::json!({
                "error": error,
                "code": code
            }),
        )
    }

    pub fn done(task_id: &str) -> Self {
        Self::new(task_id, "done", serde_json::json!({}))
    }
}

/// Stream task events via SSE.
///
/// This endpoint subscribes to the EventBus for real-time workflow events
/// and streams them to the client via Server-Sent Events (SSE).
pub async fn stream_task_events(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> impl IntoResponse {
    // Get workflow_id from task - for now assume task_id maps to workflow_id
    // In production, you'd look this up from the task registry
    let workflow_id = format!("durable-{}", task_id);

    // Subscribe to workflow events from the event bus
    let mut event_rx = state.event_bus.subscribe(&workflow_id);

    // Create a stream of events for this task
    let stream = async_stream::stream! {
        // Send initial connection message
        yield Ok::<_, Infallible>(Event::default()
            .event("connected")
            .data(serde_json::json!({
                "task_id": task_id,
                "workflow_id": workflow_id,
                "message": "Connected to workflow event stream"
            }).to_string()));

        // Stream events from the event bus
        loop {
            match event_rx.recv().await {
                Ok(workflow_event) => {
                    // Convert WorkflowEvent to SSE Event
                    let event_type = match &workflow_event {
                        crate::workflow::embedded::event_bus::WorkflowEvent::WorkflowStarted { .. } => "workflow.started",
                        crate::workflow::embedded::event_bus::WorkflowEvent::WorkflowStatusChanged { .. } => "workflow.status_changed",
                        crate::workflow::embedded::event_bus::WorkflowEvent::WorkflowCompleted { .. } => "workflow.completed",
                        crate::workflow::embedded::event_bus::WorkflowEvent::WorkflowFailed { .. } => "workflow.failed",
                        crate::workflow::embedded::event_bus::WorkflowEvent::WorkflowPausing { .. } => "workflow.pausing",
                        crate::workflow::embedded::event_bus::WorkflowEvent::WorkflowPaused { .. } => "workflow.paused",
                        crate::workflow::embedded::event_bus::WorkflowEvent::WorkflowResuming { .. } => "workflow.resuming",
                        crate::workflow::embedded::event_bus::WorkflowEvent::WorkflowCancelling { .. } => "workflow.cancelling",
                        crate::workflow::embedded::event_bus::WorkflowEvent::WorkflowCancelled { .. } => "workflow.cancelled",
                        crate::workflow::embedded::event_bus::WorkflowEvent::ActivityScheduled { .. } => "activity.scheduled",
                        crate::workflow::embedded::event_bus::WorkflowEvent::ActivityStarted { .. } => "activity.started",
                        crate::workflow::embedded::event_bus::WorkflowEvent::ActivityCompleted { .. } => "activity.completed",
                        crate::workflow::embedded::event_bus::WorkflowEvent::ActivityFailed { .. } => "activity.failed",
                        crate::workflow::embedded::event_bus::WorkflowEvent::LlmRequest { .. } => "llm.request",
                        crate::workflow::embedded::event_bus::WorkflowEvent::LlmPartial { .. } => "llm.partial",
                        crate::workflow::embedded::event_bus::WorkflowEvent::LlmResponse { .. } => "llm.response",
                        crate::workflow::embedded::event_bus::WorkflowEvent::ToolExecutionStarted { .. } => "tool.started",
                        crate::workflow::embedded::event_bus::WorkflowEvent::ToolExecutionCompleted { .. } => "tool.completed",
                        crate::workflow::embedded::event_bus::WorkflowEvent::Progress { .. } => "progress",
                        crate::workflow::embedded::event_bus::WorkflowEvent::CheckpointCreated { .. } => "checkpoint.created",
                    };

                    yield Ok(Event::default()
                        .event(event_type)
                        .data(serde_json::to_string(&workflow_event).unwrap_or_default()));

                    // Check for terminal events
                    if matches!(workflow_event,
                        crate::workflow::embedded::event_bus::WorkflowEvent::WorkflowCompleted { .. } |
                        crate::workflow::embedded::event_bus::WorkflowEvent::WorkflowFailed { .. } |
                        crate::workflow::embedded::event_bus::WorkflowEvent::WorkflowCancelled { .. }
                    ) {
                        // Send done event and close
                        yield Ok(Event::default()
                            .event("done")
                            .data(serde_json::json!({"task_id": task_id, "workflow_id": workflow_id}).to_string()));
                        break;
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                    // Client fell behind, warn but continue
                    tracing::warn!("Client lagged, skipped {} events for workflow {}", skipped, workflow_id);
                    yield Ok(Event::default()
                        .event("warning")
                        .data(serde_json::json!({
                            "message": format!("Skipped {} events due to slow connection", skipped)
                        }).to_string()));
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                    // Channel closed, workflow ended
                    tracing::debug!("Event channel closed for workflow {}", workflow_id);
                    yield Ok(Event::default()
                        .event("done")
                        .data(serde_json::json!({"task_id": task_id, "workflow_id": workflow_id}).to_string()));
                    break;
                }
            }
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}

/// WebSocket handler for task events.
pub async fn websocket_task_events(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_websocket(socket, state, task_id))
}

/// Handle WebSocket connection for task events.
async fn handle_websocket(mut socket: WebSocket, state: AppState, task_id: String) {
    // Send initial connection message
    let connect_msg = serde_json::json!({
        "type": "connected",
        "task_id": task_id,
        "message": "Connected via WebSocket"
    });

    if socket
        .send(Message::Text(connect_msg.to_string().into()))
        .await
        .is_err()
    {
        return;
    }

    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut iteration = 0u8;

    loop {
        tokio::select! {
            // Handle incoming messages
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        // Handle client commands (ping, subscribe, etc.)
                        if let Ok(cmd) = serde_json::from_str::<serde_json::Value>(&text) {
                            if cmd["type"].as_str() == Some("ping") {
                                let pong = serde_json::json!({
                                    "type": "pong",
                                    "timestamp": chrono::Utc::now().to_rfc3339()
                                });
                                let _ = socket.send(Message::Text(pong.to_string().into())).await;
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        break;
                    }
                    _ => {}
                }
            }

            // Send periodic updates
            _ = interval.tick() => {
                iteration = iteration.saturating_add(1);

                if let Some(ref redis) = state.redis {
                    let mut redis = redis.clone();
                    let key = format!("task:{task_id}");

                    if let Ok(Some(data)) = redis::AsyncCommands::get::<_, Option<String>>(&mut redis, &key).await {
                        if let Ok(task) = serde_json::from_str::<serde_json::Value>(&data) {
                            let status = task["status"].as_str().unwrap_or("pending");

                            let update = serde_json::json!({
                                "type": "progress",
                                "task_id": task_id,
                                "status": status,
                                "progress": task["progress"],
                                "timestamp": chrono::Utc::now().to_rfc3339()
                            });

                            if socket.send(Message::Text(update.to_string().into())).await.is_err() {
                                break;
                            }

                            if status == "completed" || status == "failed" || status == "cancelled" {
                                let done = serde_json::json!({
                                    "type": "done",
                                    "task_id": task_id,
                                    "status": status,
                                    "output": task["output"],
                                    "error": task["error"]
                                });
                                let _ = socket.send(Message::Text(done.to_string().into())).await;
                                break;
                            }
                        }
                    } else {
                        let error = serde_json::json!({
                            "type": "error",
                            "message": "Task not found"
                        });
                        let _ = socket.send(Message::Text(error.to_string().into())).await;
                        break;
                    }
                }

                // Safety limit
                if iteration > 250 {
                    let timeout = serde_json::json!({
                        "type": "timeout",
                        "message": "Connection timeout reached"
                    });
                    let _ = socket.send(Message::Text(timeout.to_string().into())).await;
                    break;
                }
            }
        }
    }

    let _ = socket.close().await;
}

/// Stream global events (admin/monitoring).
pub async fn stream_global_events(State(state): State<AppState>) -> impl IntoResponse {
    let stream = async_stream::stream! {
        let mut interval = tokio::time::interval(Duration::from_secs(5));

        yield Ok::<_, Infallible>(Event::default()
            .event("connected")
            .data(serde_json::json!({
                "message": "Connected to global event stream"
            }).to_string()));

        loop {
            interval.tick().await;

            // Send heartbeat
            yield Ok(Event::default()
                .event("heartbeat")
                .data(serde_json::json!({
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "redis_connected": state.redis.is_some()
                }).to_string()));
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}
