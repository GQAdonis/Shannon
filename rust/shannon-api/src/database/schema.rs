//! Database schema definitions.
//!
//! Contains `SQLite` schema for embedded backends (desktop and mobile).

/// `SQLite` schema for mobile mode.
pub const SQLITE_SCHEMA: &str = r"
-- Runs table
CREATE TABLE IF NOT EXISTS runs (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    session_id TEXT,
    query TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    strategy TEXT NOT NULL DEFAULT 'standard',
    result TEXT,
    error TEXT,
    token_usage TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    completed_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_runs_user ON runs(user_id);
CREATE INDEX IF NOT EXISTS idx_runs_status ON runs(status);
CREATE INDEX IF NOT EXISTS idx_runs_session ON runs(session_id);

-- Memories table
CREATE TABLE IF NOT EXISTS memories (
    id TEXT PRIMARY KEY,
    conversation_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding BLOB,
    metadata TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_memories_conversation ON memories(conversation_id);

-- Workflow events table
CREATE TABLE IF NOT EXISTS workflow_events (
    workflow_id TEXT NOT NULL,
    event_idx INTEGER NOT NULL,
    event_type TEXT NOT NULL,
    data BLOB,
    created_at TEXT DEFAULT (datetime('now')),
    PRIMARY KEY (workflow_id, event_idx)
);

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    name TEXT,
    password_hash TEXT,
    api_keys TEXT,
    settings TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Sessions table
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    title TEXT,
    context TEXT,
    message_count INTEGER DEFAULT 0,
    token_usage TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);

-- Sync state table
CREATE TABLE IF NOT EXISTS sync_state (
    device_id TEXT PRIMARY KEY,
    last_sync_at TEXT,
    state_vector BLOB,
    created_at TEXT DEFAULT (datetime('now'))
);
";
