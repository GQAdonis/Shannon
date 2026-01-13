# TODO Audit Report - Shannon Project

**Date**: 2026-01-13  
**Scan Coverage**: Rust (rust/), TypeScript (desktop/)  
**Total TODOs Found**: 47  

## Overview

This document catalogs all TODO, FIXME, XXX, and HACK comments found in the Shannon codebase, categorized by priority and actionability.

## Priority Classification

- **BLOCKING**: Prevents core functionality from working properly
- **HIGH**: Important for production readiness and security
- **MEDIUM**: Improves functionality and user experience
- **LOW**: Future enhancements and optimizations

---

## Rust TODOs (40 items)

### BLOCKING Priority (6 items)

#### 1. Knowledge Base API - Database Integration
**Files**: `rust/shannon-api/src/api/knowledge.rs`  
**Lines**: 217, 242, 255, 271, 291, 393, 406, 421, 429, 448  
**Issue**: Multiple knowledge base endpoints have placeholder implementations with "TODO: Fetch from database", "TODO: Store in database", "TODO: Verify ownership"

**Impact**: Knowledge base features non-functional without database integration

**Action Required**:
```rust
// Current placeholder pattern:
// TODO: Store in database via repository

// Needs implementation:
let kb = state.knowledge_repo.create_knowledge_base(user_id, &req).await?;
```

**Estimate**: 4-6 hours for full integration

---

#### 2. Auth Context Extraction - User ID Retrieval
**Files**: 
- `rust/shannon-api/src/gateway/schedules.rs:106`
- `rust/shannon-api/src/gateway/tasks.rs:507`
- `rust/shannon-api/src/api/knowledge.rs:198, 202`

**Issue**: Hardcoded user IDs instead of extracting from auth context

```rust
user_id: "embedded_user".to_string(), // TODO: Extract from auth context
user_id: "default".to_string(), // TODO: Get from auth context
let user_id = extract_user_id(None); // TODO: Pass actual claims
```

**Impact**: Multi-user scenarios will fail; security vulnerability

**Action Required**:
```rust
// Extract from JWT claims or API key
let claims = extract_jwt_claims(&req)?;
let user_id = claims.sub;
```

**Estimate**: 2-3 hours

---

#### 3. RAG Service Integration - Processing Pipeline
**Files**: `rust/shannon-api/src/api/knowledge.rs`  
**Lines**: 358, 481, 509, 512  

**Issue**: RAG (Retrieval Augmented Generation) pipeline has stub implementations

```rust
// TODO: Process document with RAG service
// TODO: Search knowledge bases
// TODO: Augment prompt with context
// TODO: Stream LLM response
```

**Impact**: RAG-enhanced chat functionality is non-functional

**Action Required**: Implement full RAG pipeline using knowledge base and vector search

**Estimate**: 6-8 hours

---

#### 4. Workflow Event Streaming
**Files**: 
- `rust/shannon-api/src/workflow/engine.rs:665, 682, 699`
- `rust/shannon-api/src/workflow/engine.rs:955`

**Issue**: Workflow control events (pause/resume/cancel) not emitted

```rust
// TODO: Emit WORKFLOW_CANCELLING and WORKFLOW_CANCELLED events (T123-T124)
// TODO: Emit WORKFLOW_PAUSING and WORKFLOW_PAUSED events (T120-T121)
// TODO: Emit WORKFLOW_RESUMED event (T122)
// TODO: Implement streaming subscription via gRPC streaming
```

**Impact**: UI cannot track workflow state changes in real-time

**Action Required**: Implement event emission using SSE/WebSocket

**Estimate**: 3-4 hours

---

#### 5. SurrealDB Connection (Repository)
**Files**: `rust/shannon-api/src/database/repository.rs:522`

**Issue**: Placeholder SurrealDB connection

```rust
// TODO: Implement actual SurrealDB connection
```

**Impact**: Database features non-functional in embedded mode

**Action Required**: Implement SurrealDB client initialization and connection pooling

**Estimate**: 2-3 hours

---

#### 6. Gateway Auth - Redis & Hybrid Backend
**Files**: `rust/shannon-api/src/gateway/auth.rs:144, 150`

**Issue**: Auth validation stubs for production deployment

```rust
// TODO: Implement Redis-based API key validation
// TODO: Implement user lookup for Hybrid backend
```

**Impact**: API key authentication incomplete for cloud deployment

**Action Required**: Wire up Redis and database lookups

**Estimate**: 3-4 hours

---

### HIGH Priority (10 items)

#### 7. LLM Pattern Integration - Workflow Strategies
**Files**:
- `rust/workflow-patterns/src/tree_of_thoughts.rs:86`
- `rust/workflow-patterns/src/chain_of_thought.rs:104`
- `rust/shannon-api/src/workflow/strategies/scientific.rs:97, 129, 161`
- `rust/shannon-api/src/workflow/strategies/exploratory.rs:124`

**Issue**: Workflow strategies have placeholder LLM calls

```rust
// TODO: Implement actual LLM-based evaluation
// TODO: Call LLM activity here
// TODO: Replace with actual LLM call via pattern registry
```

**Impact**: Advanced workflow patterns return mock data

**Action Required**: Integrate with LLM service layer

**Estimate**: 4-5 hours

---

#### 8. Control State Sync (Durable Shannon)
**Files**: `rust/durable-shannon/src/worker/mod.rs:482-490`

**Issue**: Workflow pause/resume control state checking disabled

```rust
// TODO: Wire up control state checking when EventLog is HybridBackend
// For now, this is a placeholder that always returns None
// This means pause/resume API endpoints work, but the workflow
// doesn't check them during execution.
```

**Impact**: Pause/resume controls don't actually pause running workflows

**Action Required**: Implement control state callback mechanism

**Estimate**: 3-4 hours

---

#### 9. Tracing Context Injection
**Files**: `rust/agent-core/src/llm_client.rs:209`

**Issue**: Distributed tracing not propagated to LLM calls

```rust
// crate::tracing::inject_current_trace_context(&mut headers); // TODO: Fix tracing import
```

**Impact**: Observability gaps in distributed traces

**Action Required**: Fix import and enable trace context propagation

**Estimate**: 1 hour

---

#### 10. Cloud vs Embedded Compatibility Test
**Files**: `rust/shannon-api/tests/workflow/compatibility/cloud_comparison_test.rs:183`

**Issue**: Integration test stub for cloud/embedded parity

```rust
// TODO: Implement when both cloud and embedded instances available
```

**Impact**: Cannot verify feature parity between deployment modes

**Action Required**: Implement dual-mode test harness

**Estimate**: 4-6 hours

---

### MEDIUM Priority (3 items)

#### 11. Workflow Capability Configuration
**Files**: `rust/durable-shannon/src/worker/mod.rs:379`

**Issue**: Sandbox capabilities hardcoded per workflow

```rust
// Define Capabilities (TODO: make configurable per workflow)
let caps = SandboxCapabilities {
    timeout_ms: 30_000,
    max_memory_mb: 512,
    ..Default::default()
};
```

**Impact**: Cannot customize security policies per workflow type

**Action Required**: Load capabilities from config file or workflow metadata

**Estimate**: 2-3 hours

---

#### 12. WASI Memory Limits
**Files**: `rust/agent-core/src/wasi_sandbox.rs:373`

**Issue**: Memory limits not enforced due to lifetime issues

```rust
// TODO: Add memory limits via StoreLimits once lifetime issues are resolved
```

**Impact**: WASM modules can consume unbounded memory

**Action Required**: Resolve Wasmtime lifetime issues and add StoreLimits

**Estimate**: 2-4 hours

---

### LOW Priority (21 items)

#### 13. TypeScript Integration Tests (3 items)
**Files**:
- `desktop/tests/integration/port_fallback.test.tsx:5`
- `desktop/tests/integration/debug_console_logs.test.tsx:5`
- `desktop/tests/integration/ready_gate.test.tsx:5`

**Issue**: Integration tests have stub implementations

```typescript
// TODO: Simulate port 1906 in use and assert UI connects to fallback port.
// TODO: Mount debug console and assert buffered logs are rendered.
// TODO: Mount the app shell and assert disabled input before readiness.
```

**Impact**: Limited test coverage for edge cases

**Action Required**: Implement full test scenarios

**Estimate**: 2-3 hours total

---

#### 14. Command Palette Features (4 items)
**Files**: `desktop/components/command-palette.tsx`  
**Lines**: 130, 155, 167, 179

**Issue**: Command palette actions are stubs

```typescript
// TODO: Open search modal
// TODO: Switch to quick mode
// TODO: Switch to task mode
// TODO: Switch to agent mode
```

**Impact**: UI commands don't trigger actions yet

**Action Required**: Wire up command handlers

**Estimate**: 2-3 hours

---

## TypeScript TODOs (7 items)

All TypeScript TODOs fall into LOW or MEDIUM priority:
- 3 integration test stubs (LOW)
- 4 command palette features (MEDIUM)

---

## Summary Statistics

| Priority | Count | % of Total | Estimated Hours |
|----------|-------|------------|-----------------|
| BLOCKING | 6 | 12.8% | 20-28 hours |
| HIGH | 10 | 21.3% | 15-20 hours |
| MEDIUM | 3 | 6.4% | 4-7 hours |
| LOW | 28 | 59.6% | 4-6 hours |
| **TOTAL** | **47** | **100%** | **43-61 hours** |

---

## Critical Path: BLOCKING Items

### Immediate Implementation Required

1. **Knowledge Base Database Integration** (Priority 1)
   - Affects: 10 endpoints in knowledge.rs
   - Blocks: RAG functionality, document management
   - Estimate: 4-6 hours

2. **Auth Context Extraction** (Priority 2)
   - Affects: Schedules, tasks, knowledge endpoints
   - Blocks: Multi-user support, security
   - Estimate: 2-3 hours

3. **RAG Service Pipeline** (Priority 3)
   - Affects: Augmented chat, document search
   - Blocks: Core AI features
   - Estimate: 6-8 hours

4. **Workflow Event Streaming** (Priority 4)
   - Affects: UI real-time updates
   - Blocks: Workflow monitoring
   - Estimate: 3-4 hours

5. **SurrealDB Connection** (Priority 5)
   - Affects: Embedded mode database
   - Blocks: Desktop app persistence
   - Estimate: 2-3 hours

6. **Gateway Auth Production** (Priority 6)
   - Affects: Cloud deployment
   - Blocks: API key validation
   - Estimate: 3-4 hours

**Total Critical Path**: 20-28 hours

---

## Implementation Strategy

### Phase 1: Core Functionality (BLOCKING)
**Duration**: 3-4 days  
**Focus**: Items 1-6 above  
**Outcome**: All core features functional

### Phase 2: Production Readiness (HIGH)
**Duration**: 2-3 days  
**Focus**: LLM integration, control state, tracing  
**Outcome**: Production-grade reliability

### Phase 3: Enhancement (MEDIUM)
**Duration**: 1-2 days  
**Focus**: Configuration, security hardening  
**Outcome**: Operational flexibility

### Phase 4: Polish (LOW)
**Duration**: 1 day  
**Focus**: Tests, UI commands  
**Outcome**: Complete feature set

---

## Recommendations

### Immediate Actions
1. ✅ Create this TODO audit (DONE)
2. 🔄 Prioritize BLOCKING items for sprint planning
3. ⏳ Assign owners to each BLOCKING TODO
4. ⏳ Set up task tracking (GitHub issues/JIRA)

### Process Improvements
1. **Pre-commit Hook**: Flag new TODOs without priority labels
2. **Code Review**: Require justification for new TODOs
3. **Sprint Planning**: Allocate 20% capacity to TODO resolution
4. **Technical Debt**: Track TODO age and prioritize old items

### Code Quality Gates
1. **No BLOCKING TODOs** in main branch
2. **Max 5 new TODOs** per PR
3. **All TODOs categorized** with priority label
4. **Quarterly TODO review** to prevent accumulation

---

## Appendix A: TODO Pattern Examples

### Good TODO (Actionable)
```rust
// TODO(priority=HIGH, owner=@alice, ticket=SHAN-123): 
// Implement Redis-based API key validation for cloud mode
// Estimated: 3 hours
// Blocks: Cloud deployment
```

### Bad TODO (Vague)
```rust
// TODO: Fix this later
```

### Converting Placeholders
```rust
// BEFORE
// TODO: Store in database

// AFTER
// TODO(priority=BLOCKING, ticket=SHAN-456):
// Replace stub with actual repository.create_knowledge_base() call
// See: docs/knowledge-base-schema.md
```

---

## Appendix B: Files with Multiple TODOs

| File | TODO Count | Category |
|------|------------|----------|
| `rust/shannon-api/src/api/knowledge.rs` | 15 | BLOCKING (Auth + DB) |
| `rust/shannon-api/src/workflow/engine.rs` | 4 | BLOCKING (Events) |
| `rust/shannon-api/src/workflow/strategies/*.rs` | 4 | HIGH (LLM) |
| `desktop/components/command-palette.tsx` | 4 | LOW (UI) |
| `desktop/tests/integration/*.test.tsx` | 3 | LOW (Tests) |

---

## Appendix C: Cross-References

- **Clippy Report**: `docs/QA_CLEAN_BUILD_REPORT.md`
- **Rust Standards**: `docs/coding-standards/RUST.md`
- **Knowledge Base Spec**: `docs/knowledge-base-api.md` (if exists)
- **Workflow Architecture**: `docs/workflow-engine.md` (if exists)

---

**Next Review**: After BLOCKING items are implemented  
**Report Maintainer**: Development Team  
**Last Updated**: 2026-01-13
