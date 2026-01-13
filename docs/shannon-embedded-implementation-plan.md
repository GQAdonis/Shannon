# Shannon Embedded: Implementation Plan

**Objective**: Reach parity with Upstream Go Orchestrator and enable Manus.ai-like capabilities in the Rust embedded engine.

## Phase 1: Engine Activation (Immediate)
*Focus: Removing the simulation stub and enabling actual execution.*

1.  **Integrate `EmbeddedWorker` in `shannon-api`**
    *   **Target**: `rust/shannon-api/src/workflow/engine.rs`
    *   **Action**: Replace `run_local_workflow` simulation loop.
    *   **Task**:
        *   Instantiate `durable_shannon::EmbeddedWorker` in `DurableEngine::new` (already partially done).
        *   In `execute_task`, call `self.worker.submit(strategy, input)`.
        *   Map `durable_shannon::Event` to `shannon_api::TaskEvent` for streaming to UI.
        *   Handle pattern loading from `wasm_dir`.

2.  **Prompt Processing Search & Fix**
    *   **Target**: `rust/durable-shannon/src/worker/` and `rust/shannon-api/src/workflow/patterns/`
    *   **Action**: Locate where prompts are rendered. If missing, implement Handlebars/Tera rendering for standard patterns (`CoT`, `Research`).
    *   **Verify**: Ensure prompts match the Go `system_prompts.md` specs.

## Phase 2: Strategy Parity (Short Term)
*Focus: Implement complex composition patterns.*

1.  **Implement `Strategy` Composition**
    *   **Target**: New module `rust/shannon-api/src/workflow/strategies/` or new WASM modules.
    *   **Action**: Create composable logic for:
        *   `ScientificWorkflow`: `CoT` (Hypothesis) -> `Debate` (Critique) -> `Research` (Validation).
        *   `ExploratoryWorkflow`: `TreeOfThoughts` (Expand) -> `Reflection` (Prune).
    *   **Update**: Update `router.rs` to map `Scientific`/`Exploratory` to these new composite handlers instead of single patterns.

2.  **Budget Middleware**
    *   **Target**: `rust/shannon-api/src/workflow/embedded/middleware.rs`
    *   **Action**: Port token counting and budget limits from Go.

## Phase 3: Manus.ai "Action Engine" (Mid Term)
*Focus: Expanding capabilities beyond text.*

1.  **Action Primitives (Tools)**
    *   **Target**: `rust/durable-shannon/src/activities/`
    *   **Action**: Implement `ToolActivity` that can execute:
        *   `browser_access` (via `headless_chrome` or `ferret`).
        *   `file_system` (sandboxed).
        *   `email_client` (SMTP/IMAP).

2.  **Deep Research Artifacts**
    *   **Target**: `rust/shannon-api/src/workflow/patterns/research.rs`
    *   **Action**: Add document generation step (Markdown -> PDF).

## Phase 4: Verification
1.  **Unit Tests**: Verify `DurableEngine` submits and receives events.
2.  **Integration**: Run `CoT` workflow and verify output against known LLM prompts.
3.  **Parity Check**: Run "Research quantum computing" on both Go and Rust engines and compare depth/quality.
