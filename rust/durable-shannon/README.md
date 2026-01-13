# Durable Shannon

Shannon-specific extensions to the Durable workflow engine.

## Features

- Event log persistence with SQLite backend
- WASM-based workflow execution with MicroSandbox
- Checkpoint/restore support for workflow state
- LRU caching for compiled WASM modules
- Embedded worker for desktop/mobile deployments

## Architecture

- **Event Log**: Immutable event sourcing with SQLite storage
- **MicroSandbox**: Secure WASM runtime with capability-based policies
- **Workflow Activities**: LLM calls, tool execution, agent coordination
- **Checkpoint Manager**: State compression and incremental snapshots

## Usage

```rust
use durable_shannon::{EmbeddedWorker, SqliteEventLog};

let event_log = SqliteEventLog::new("workflows.db").await?;
let worker = EmbeddedWorker::new(event_log, wasm_dir, 4).await?;

let result = worker.start_workflow("agent-task", "task-123", input).await?;
```

See the main Shannon repository for complete documentation.
