# Shannon Desktop: Cherry Studio Feature Parity & Durable Integration Plan

**Version**: 1.0  
**Date**: 2026-01-12  
**Objective**: Achieve 100% operational embedded Shannon with Cherry Studio feature parity and Manus.ai capabilities

---

## Executive Summary

This plan provides a comprehensive roadmap to:
1. **Fully integrate durable-shannon** into the Shannon Desktop workflow execution path
2. **Achieve feature parity** with Cherry Studio's UI/UX and functionality
3. **Implement Manus.ai capabilities** (Deep Research, Action Engine)
4. **Create a production-ready** AI assistant platform ready to replace Cherry Studio

### Success Criteria
- ✅ 100% operational durable-shannon execution (no simulation stubs)
- ✅ Dual-mode chat (lightweight + workflow-heavy) working seamlessly
- ✅ Agent repository with custom agent creation and execution
- ✅ UI artifacts rendering (A2UI-compatible)
- ✅ Cherry Studio-inspired UI/UX
- ✅ Advanced settings, extensions, and document processing
- ✅ RAG knowledge base support
- ✅ Advanced code block rendering (mermaid, SVG, video, images, MDX)
- ✅ Advanced context management
- ✅ Full MCP tools integration including e2b
- ✅ Chat input conversation-level tools

---

## Part 1: Current Architecture Analysis

### Shannon Stack (Current State)

```
┌─────────────────────────────────────────────────────────────┐
│                    Desktop Application                       │
│                   (Next.js 16 + Tauri 2)                     │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  Frontend (React 19)          Backend (Rust)                 │
│  ├─ Pages/Routes             ├─ embedded_api.rs             │
│  ├─ Components               ├─ embedded_port.rs            │
│  ├─ Lib (API clients)        ├─ ipc_events.rs               │
│  └─ State (Redux/Zustand)    └─ workflow.rs                 │
│                                                               │
├─────────────────────────────────────────────────────────────┤
│                Shannon API (Rust - Port 8765)                │
│  ├─ Gateway (Auth, Sessions, Routes)                         │
│  ├─ Workflow Engine (⚠️ PARTIALLY STUBBED)                   │
│  │  └─ DurableEngine (uses durable-shannon)                  │
│  ├─ Database (SQLite + encryption)                           │
│  └─ Domain Models                                             │
├─────────────────────────────────────────────────────────────┤
│              Durable-Shannon (WASM Executor)                 │
│  ├─ EmbeddedWorker (✅ EXISTS)                                │
│  ├─ Event Sourcing (✅ EXISTS)                                │
│  ├─ Patterns (⚠️ BASIC ONLY)                                  │
│  └─ Activities (❌ NEEDS EXPANSION)                           │
└─────────────────────────────────────────────────────────────┘
```

### Critical Gaps Identified

#### 1. **Execution Layer** (HIGH PRIORITY)
- ❌ `engine.rs` uses simulation stub `run_local_workflow()`
- ❌ Prompt processing/rendering not implemented
- ❌ Strategy composition missing (Scientific, Exploratory)
- ❌ Budget middleware not integrated
- ✅ EmbeddedWorker exists but not fully wired

#### 2. **Cognitive Patterns** (HIGH PRIORITY)
- ⚠️ Basic patterns exist (CoT, Debate, Research)
- ❌ Complex composition workflows missing
- ❌ No multi-stage reasoning (Hypothesis → Testing → Implications)

#### 3. **Action Engine** (MEDIUM PRIORITY)
- ❌ No browser automation (Playwright/headless)
- ❌ No filesystem tools (sandboxed)
- ❌ No email/calendar integration

#### 4. **Frontend Features** (HIGH PRIORITY)
- ❌ Single chat mode (no lightweight/heavy split)
- ❌ No agent repository
- ❌ No UI artifacts rendering
- ❌ Basic context management
- ❌ Limited MCP integration UI
- ❌ No RAG knowledge base UI

---

## Part 2: Cherry Studio Feature Analysis

### Cherry Studio Architecture (Reference)

```
┌─────────────────────────────────────────────────────────────┐
│                  Electron Main Process                       │
│  ├─ Services (MCP, Knowledge, Storage, Agents, Window)      │
│  ├─ AI Core (Provider middleware)                            │
│  └─ IPC Bridge                                                │
├─────────────────────────────────────────────────────────────┤
│                  Renderer Process (React)                    │
│  ├─ Pages (Home, Settings, Knowledge, Artifacts, etc.)      │
│  ├─ Components (Chat, CodeBlocks, Artifacts, Settings)      │
│  ├─ State Management (Redux Toolkit)                         │
│  ├─ AI Core Integration                                      │
│  └─ Context Management System                                │
└─────────────────────────────────────────────────────────────┘
```

### Key Features to Adopt

#### 1. **AI Core Architecture** ⭐
- **Middleware Pipeline**: Provider abstraction with plugins
- **Multi-provider Support**: OpenAI, Anthropic, Google, Local (Ollama)
- **Streaming**: Real-time response streaming
- **Tool Execution**: Integrated tool calling

#### 2. **Agent System** ⭐
- **300+ Pre-configured Agents**: Template library
- **Custom Agent Creation**: YAML/JSON specifications
- **Agent Marketplace**: Import/export agents
- **Agent Profiles**: System prompts, tools, models

#### 3. **Artifacts System** ⭐
- **Code Execution**: Sandboxed code (Sandpack/CodeSandbox)
- **Rich Rendering**: React components, SVG, Mermaid, D3
- **Version Control**: Artifact history
- **Export**: Multiple formats (PNG, SVG, PDF)

#### 4. **Knowledge Base** ⭐
- **Document Processing**: PDF, DOCX, PPTX, Images (OCR)
- **Embedding**: Local (libsql) or cloud vector DB
- **RAG Integration**: Context injection in prompts
- **Collections**: Organize knowledge by topic

#### 5. **MCP Integration** ⭐
- **MCP Servers**: Registry and management UI
- **Per-conversation Tools**: Select tools per chat
- **Tool Marketplace**: Browse and install MCP servers
- **Status Indicators**: Connection status, health checks

#### 6. **Settings Architecture** ⭐
- **Modular Settings**: Organized by category
- **Provider Management**: API keys, endpoints
- **Model Configuration**: Per-provider model selection
- **Extensions**: Plugin system
- **Data Management**: Import/export, backup

#### 7. **UI/UX Patterns** ⭐
- **Tab System**: Multiple concurrent chats
- **Sidebar Navigation**: Clean, icon-based
- **Theme System**: Light/dark + custom themes
- **Keyboard Shortcuts**: Comprehensive shortcuts
- **Context Menus**: Right-click actions

---

## Part 3: Target Architecture

### Shannon Desktop (Target State)

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Desktop Application                          │
│                        (Next.js 16 + Tauri 2)                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌─────────────────────────┐  ┌──────────────────────────────────┐ │
│  │   Frontend (React 19)   │  │   Backend (Rust - Tauri)         │ │
│  │                         │  │                                   │ │
│  │  ┌──────────────────┐  │  │  ┌───────────────────────────┐  │ │
│  │  │ Dual-Mode Chat   │  │  │  │ Embedded API Server       │  │ │
│  │  │ ├─ Quick Chat    │  │  │  │ (Shannon-API embedded)    │  │ │
│  │  │ └─ Task Chat     │  │  │  └───────────────────────────┘  │ │
│  │  └──────────────────┘  │  │  ┌───────────────────────────┐  │ │
│  │  ┌──────────────────┐  │  │  │ Workflow Orchestration    │  │ │
│  │  │ Agent Repository │  │  │  │ (Durable-Shannon)         │  │ │
│  │  │ ├─ Templates     │  │  │  └───────────────────────────┘  │ │
│  │  │ ├─ Custom        │  │  │  ┌───────────────────────────┐  │ │
│  │  │ └─ Marketplace   │  │  │  │ MCP Server Manager        │  │ │
│  │  └──────────────────┘  │  │  │ ├─ Registry               │  │ │
│  │  ┌──────────────────┐  │  │  │ ├─ Connection Pool        │  │ │
│  │  │ Artifacts        │  │  │  │ └─ Tool Router            │  │ │
│  │  │ ├─ A2UI Renderer │  │  │  └───────────────────────────┘  │ │
│  │  │ ├─ Code Sandbox  │  │  │  ┌───────────────────────────┐  │ │
│  │  │ ├─ Mermaid       │  │  │  │ Knowledge Base (RAG)      │  │ │
│  │  │ ├─ Media Player  │  │  │  │ ├─ Document Processor     │  │ │
│  │  │ └─ SVG/Canvas    │  │  │  │ ├─ Embeddings (libsql)    │  │ │
│  │  └──────────────────┘  │  │  │ └─ Vector Search          │  │ │
│  │  ┌──────────────────┐  │  │  └───────────────────────────┘  │ │
│  │  │ Context Manager  │  │  │  ┌───────────────────────────┐  │ │
│  │  │ ├─ Hierarchical  │  │  │  │ Action Engine             │  │ │
│  │  │ ├─ Pinned Items  │  │  │  │ ├─ Browser (Playwright)   │  │ │
│  │  │ ├─ @-mentions    │  │  │  │ ├─ Filesystem (Sandbox)   │  │ │
│  │  │ └─ Auto-prune    │  │  │  │ └─ E2B Interpreter        │  │ │
│  │  └──────────────────┘  │  │  └───────────────────────────┘  │ │
│  │  ┌──────────────────┐  │  │                                   │ │
│  │  │ Settings System  │  │  │                                   │ │
│  │  │ ├─ Providers     │  │  │                                   │ │
│  │  │ ├─ Extensions    │  │  │                                   │ │
│  │  │ ├─ Knowledge     │  │  │                                   │ │
│  │  │ └─ Advanced      │  │  │                                   │ │
│  │  └──────────────────┘  │  │                                   │ │
│  └─────────────────────────┘  └──────────────────────────────────┘ │
│                                                                       │
├─────────────────────────────────────────────────────────────────────┤
│                      Shannon API (Rust Core)                         │
│  ├─ Gateway Layer (Auth, Sessions, Streaming)                       │
│  ├─ Workflow Engine (Durable + Temporal modes)                      │
│  ├─ Pattern Library (CoT, Debate, Research, Scientific, etc.)       │
│  ├─ Tool Registry (MCP + Native tools)                               │
│  ├─ Database Layer (SQLite with encryption)                          │
│  └─ Event System (SSE + WebSocket streaming)                        │
├─────────────────────────────────────────────────────────────────────┤
│                   Durable-Shannon (WASM Runtime)                     │
│  ├─ EmbeddedWorker (Task submission & monitoring)                   │
│  ├─ Event Log (SQLite-based durable state)                          │
│  ├─ Activity Registry (LLM, Tools, Research, etc.)                  │
│  ├─ Pattern Execution (WASM modules)                                 │
│  └─ Checkpoint/Resume (Fault tolerance)                              │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Part 4: Implementation Plan

### Phase 1: Core Engine Integration (Week 1-2)

#### 1.1 Remove Simulation Stub ⚡ CRITICAL
**File**: `rust/shannon-api/src/workflow/engine.rs`

**Changes**:
```rust
// REMOVE the simulation implementation in run_local_workflow()
// REPLACE with actual durable-shannon integration

impl DurableEngine {
    async fn execute_task(&self, task: Task, tx: broadcast::Sender<TaskEvent>) {
        // Already implemented in lines 545-633 when embedded feature is on
        // Need to ensure this path is always taken
    }
}
```

**Actions**:
- ✅ Feature `embedded` is already enabled in Cargo.toml
- ✅ Worker integration already exists (lines 288-315)
- ✅ Task submission already wired (lines 545-633)
- ❌ Need to remove/disable the `run_local_workflow` simulation
- ❌ Need to add proper event streaming from durable-shannon to TaskEvent

**Implementation**:
1. Remove `run_local_workflow()` method entirely
2. Ensure all code paths use `self.worker.submit()` 
3. Add event bridge from `durable_shannon::Event` to `TaskEvent`
4. Test with actual WASM pattern execution

#### 1.2 Prompt Processing & Rendering ⚡ CRITICAL
**Files**: 
- `rust/durable-shannon/src/activities/llm.rs` (new)
- `rust/shannon-api/src/workflow/prompts/` (new module)

**Create prompt template system**:
```rust
// rust/shannon-api/src/workflow/prompts/mod.rs
pub struct PromptTemplate {
    pub name: String,
    pub system: String,
    pub user_template: String,
    pub variables: HashMap<String, String>,
}

pub struct PromptRenderer {
    templates: HashMap<String, PromptTemplate>,
    engine: handlebars::Handlebars,
}

impl PromptRenderer {
    pub fn render(&self, pattern: &str, context: &serde_json::Value) 
        -> anyhow::Result<RenderedPrompt> {
        // Load template for pattern (CoT, Research, etc.)
        // Inject context variables
        // Return rendered system + user prompts
    }
}
```

**Load templates from**:
- `config/templates/` (existing templates)
- Match Go orchestrator `system_prompts.md` specifications

#### 1.3 Strategy Composition Implementation
**File**: `rust/shannon-api/src/workflow/strategies/` (new module)

**Create composite strategies**:
```rust
// rust/shannon-api/src/workflow/strategies/scientific.rs
pub struct ScientificWorkflow {
    cot_pattern: Pattern,
    debate_pattern: Pattern,
    research_pattern: Pattern,
}

impl ScientificWorkflow {
    pub async fn execute(&self, input: &TaskInput) -> Result<TaskOutput> {
        // Step 1: CoT for hypothesis generation
        let hypothesis = self.cot_pattern.execute(input).await?;
        
        // Step 2: Debate for critique
        let critique = self.debate_pattern.execute(&hypothesis).await?;
        
        // Step 3: Research for validation
        let validation = self.research_pattern.execute(&critique).await?;
        
        // Synthesize final result
        Ok(self.synthesize(hypothesis, critique, validation))
    }
}
```

**Strategies to implement**:
- `ScientificWorkflow`: CoT → Debate → Research
- `ExploratoryWorkflow`: TreeOfThoughts → Reflection
- `CreativeWorkflow`: Multiple CoT variants → Synthesis

#### 1.4 Budget Middleware
**File**: `rust/shannon-api/src/workflow/middleware/budget.rs` (new)

```rust
pub struct BudgetMiddleware {
    token_counter: Arc<RwLock<TokenCounter>>,
    limits: BudgetLimits,
}

pub struct BudgetLimits {
    pub max_tokens: u32,
    pub max_cost_usd: f64,
    pub warn_threshold: f64,
}

impl BudgetMiddleware {
    pub async fn check_before(&self, estimated_tokens: u32) -> Result<()> {
        // Check against limits, return error if exceeded
    }
    
    pub async fn track_after(&self, usage: &TokenUsage) {
        // Update running totals
    }
}
```

**Integration**: Wrap all LLM calls in budget checks

---

### Phase 2: Dual-Mode Chat System (Week 2-3)

#### 2.1 Quick Chat Mode (Lightweight)
**Purpose**: Fast, conversational chat without workflow overhead

**Architecture**:
```typescript
// desktop/lib/chat/quick-chat.ts
export interface QuickChatConfig {
  mode: 'quick';
  provider: LLMProvider;
  model: string;
  stream: true;
  temperature: number;
  maxTokens: number;
}

export class QuickChatService {
  async sendMessage(
    message: string,
    history: Message[],
    config: QuickChatConfig
  ): Promise<AsyncIterator<MessageChunk>> {
    // Direct LLM API call (no workflow engine)
    // Stream response chunks
    // Handle tool calls inline
    // Minimal state management
  }
}
```

**UI Component**:
```typescript
// desktop/components/chat/quick-chat.tsx
export function QuickChat() {
  return (
    <div className="quick-chat">
      <ChatHeader mode="quick" />
      <MessageList messages={messages} />
      <QuickChatInput 
        onSend={handleQuickSend}
        tools={selectedTools} // MCP tools
        context={contextItems} // @-mentions
      />
    </div>
  );
}
```

#### 2.2 Task Chat Mode (Workflow-Heavy)
**Purpose**: Complex tasks using durable-shannon workflows

**Architecture**:
```typescript
// desktop/lib/chat/task-chat.ts
export interface TaskChatConfig {
  mode: 'task';
  strategy: Strategy; // CoT, Research, Scientific, etc.
  requireApproval: boolean;
  maxAgents: number;
  tokenBudget: number;
}

export class TaskChatService {
  async submitTask(
    query: string,
    context: string[],
    config: TaskChatConfig
  ): Promise<TaskHandle> {
    // Submit to Shannon API /api/tasks endpoint
    // Returns task_id and workflow_id
    // Subscribe to SSE stream for updates
  }
  
  async* streamTask Updates(taskId: string): AsyncIterator<TaskEvent> {
    // Open SSE connection
    // Yield events: progress, partial results, completion
  }
}
```

**UI Component**:
```typescript
// desktop/components/chat/task-chat.tsx
export function TaskChat() {
  const { task, status } = useTask(taskId);
  
  return (
    <div className="task-chat">
      <ChatHeader mode="task" strategy={task.strategy} />
      <TaskProgress task={task} /> {/* Show workflow stages */}
      <MessageList messages={messages} />
      <TaskChatInput 
        onSubmit={handleTaskSubmit}
        strategySelector={true}
        approvalRequired={true}
      />
    </div>
  );
}
```

#### 2.3 Mode Switching & Detection
**Smart mode detection**:
```typescript
// desktop/lib/chat/mode-detector.ts
export function detectChatMode(query: string): ChatMode {
  // Heuristics for auto-detection:
  
  if (hasComplexityMarkers(query)) {
    // "research", "analyze", "compare", multi-step indicators
    return 'task';
  }
  
  if (hasQuickMarkers(query)) {
    // "quick", "simple", single question
    return 'quick';
  }
  
  if (estimatedTokens(query) > THRESHOLD) {
    return 'task';
  }
  
  return 'quick'; // Default to quick
}
```

**UI Toggle**:
```typescript
<ModeToggle 
  current={mode}
  auto={autoDetect}
  onChange={handleModeChange}
/>
```

---

### Phase 3: Agent Repository System (Week 3-4)

#### 3.1 Agent Schema & Storage
**Schema**:
```typescript
// desktop/types/agent.ts
export interface AgentSpec {
  id: string;
  name: string;
  description: string;
  version: string;
  author?: string;
  
  // Configuration
  systemPrompt: string;
  model: {
    provider: string;
    name: string;
    temperature?: number;
    maxTokens?: number;
  };
  
  // Capabilities
  tools: string[]; // MCP tool IDs
  knowledgeBases: string[]; // Knowledge base IDs
  allowedActions: string[]; // browser, filesystem, etc.
  
  // Behavior
  strategy?: Strategy; // Default workflow strategy
  conversationStyle?: 'formal' | 'casual' | 'technical';
  
  // Metadata
  tags: string[];
  category: string;
  icon?: string;
  createdAt: string;
  updatedAt: string;
}
```

**Storage** (SQLite via Rust):
```rust
// rust/shannon-api/src/database/agents.rs
pub struct AgentRepository {
    db: Arc<Database>,
}

impl AgentRepository {
    pub async fn create(&self, spec: &AgentSpec) -> Result<String>;
    pub async fn get(&self, id: &str) -> Result<AgentSpec>;
    pub async fn list(&self, filter: AgentFilter) -> Result<Vec<AgentSpec>>;
    pub async fn update(&self, id: &str, spec: &AgentSpec) -> Result<()>;
    pub async fn delete(&self, id: &str) -> Result<()>;
    pub async fn export(&self, id: &str) -> Result<String>; // YAML export
    pub async fn import(&self, yaml: &str) -> Result<String>;
}
```

#### 3.2 Agent UI Components
**Agent Browser**:
```typescript
// desktop/app/(app)/agents/page.tsx
export default function AgentsPage() {
  const { agents, loading } = useAgents();
  
  return (
    <div className="agents-page">
      <AgentFilters 
        categories={categories}
        onFilter={setFilter}
      />
      <AgentGrid agents={filteredAgents}>
        {agents.map(agent => (
          <AgentCard 
            key={agent.id}
            agent={agent}
            onSelect={handleSelect}
            onEdit={handleEdit}
          />
        ))}
      </AgentGrid>
      <CreateAgentButton onClick={handleCreate} />
    </div>
  );
}
```

**Agent Editor**:
```typescript
// desktop/components/agents/agent-editor.tsx
export function AgentEditor({ agent, onSave }: AgentEditorProps) {
  return (
    <Form onSubmit={handleSave}>
      <FormField name="name" label="Agent Name" />
      <FormField name="description" label="Description" type="textarea" />
      
      <PromptEditor 
        value={systemPrompt}
        onChange={setSystemPrompt}
        variables={promptVariables}
      />
      
      <ModelSelector 
        providers={providers}
        selected={model}
        onChange={setModel}
      />
      
      <ToolSelector 
        tools={availableTools}
        selected={selectedTools}
        onChange={setSelectedTools}
      />
      
      <KnowledgeBaseSelector 
        knowledgeBases={knowledgeBases}
        selected={selectedKBs}
        onChange={setSelectedKBs}
      />
      
      <StrategySelector 
        strategies={strategies}
        selected={strategy}
        onChange={setStrategy}
      />
    </Form>
  );
}
```

#### 3.3 Agent Execution Integration
**Chat with selected agent**:
```typescript
// desktop/lib/chat/agent-chat.ts
export class AgentChatService {
  async startAgentChat(
    agentId: string,
    message: string,
    mode: ChatMode
  ): Promise<ChatSession> {
    // Load agent spec
    const agent = await getAgent(agentId);
    
    // Apply agent configuration to chat
    const config = {
      systemPrompt: agent.systemPrompt,
      model: agent.model,
      tools: agent.tools,
      knowledgeBases: agent.knowledgeBases,
      strategy: agent.strategy,
    };
    
    // Start chat with config
    if (mode === 'quick') {
      return quickChat.start(message, config);
    } else {
      return taskChat.start(message, config);
    }
  }
}
```

---

### Phase 4: UI Artifacts System (Week 4-5)

#### 4.1 Artifact Schema & Detection
**Schema**:
```typescript
// desktop/types/artifact.ts
export interface Artifact {
  id: string;
  type: ArtifactType;
  title: string;
  content: string;
  metadata: ArtifactMetadata;
  createdAt: string;
  updatedAt: string;
}

export type ArtifactType =
  | 'code' // Executable code (JS, Python, etc.)
  | 'react' // React component
  | 'html' // HTML page
  | 'svg' // SVG graphics
  | 'mermaid' // Mermaid diagram
  | 'chart' // Recharts/D3 chart
  | 'markdown' // Markdown document
  | 'image' // Image (base64 or URL)
  | 'video' // Video (URL)
  | 'audio' // Audio (URL)
  | 'pdf'; // PDF document

export interface ArtifactMetadata {
  language?: string;
  framework?: string;
  dependencies?: string[];
  theme?: 'light' | 'dark';
  interactive?: boolean;
}
```

**Detection** (from LLM response):
```typescript
// desktop/lib/artifacts/detector.ts
export function detectArtifacts(content: string): Artifact[] {
  const artifacts: Artifact[] = [];
  
  // Look for artifact markers
  // Cherry Studio uses: ```artifact type="..." title="..."
  // Claude uses: <antArtifact identifier="..." type="..." title="...">
  // A2UI protocol markers
  
  const artifactPattern = /```artifact\s+type="([^"]+)"(?:\s+title="([^"]+)")?\n([\s\S]*?)```/g;
  
  for (const match of content.matchAll(artifactPattern)) {
    artifacts.push({
      id: generateId(),
      type: match[1] as ArtifactType,
      title: match[2] || 'Untitled',
      content: match[3],
      metadata: extractMetadata(match[3]),
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    });
  }
  
  return artifacts;
}
```

#### 4.2 Artifact Renderers
**React Component Renderer**:
```typescript
// desktop/components/artifacts/renderers/react-renderer.tsx
import { Sandpack } from '@codesandbox/sandpack-react';

export function ReactRenderer({ artifact }: { artifact: Artifact }) {
  const files = {
    '/App.tsx': artifact.content,
    '/index.tsx': generateIndexFile(artifact),
  };
  
  return (
    <Sandpack
      template="react-ts"
      files={files}
      theme={artifact.metadata.theme || 'auto'}
      options={{
        showNavigator: true,
        showTabs: true,
        showLineNumbers: true,
        editorHeight: 500,
      }}
    />
  );
}
```

**Mermaid Renderer**:
```typescript
// desktop/components/artifacts/renderers/mermaid-renderer.tsx
import mermaid from 'mermaid';

export function MermaidRenderer({ artifact }: { artifact: Artifact }) {
  const svgRef = useRef<HTMLDivElement>(null);
  
  useEffect(() => {
    if (svgRef.current) {
      mermaid.render('mermaid-svg', artifact.content).then(({ svg }) => {
        svgRef.current!.innerHTML = svg;
      });
    }
  }, [artifact.content]);
  
  return (
    <div className="mermaid-container">
      <div ref={svgRef} />
      <ArtifactActions 
        onExport={() => exportSVG(svgRef.current)}
        onCopy={() => copyToClipboard(artifact.content)}
      />
    </div>
  );
}
```

**Video/Media Renderer**:
```typescript
// desktop/components/artifacts/renderers/media-renderer.tsx
export function MediaRenderer({ artifact }: { artifact: Artifact }) {
  if (artifact.type === 'video') {
    return (
      <video controls className="artifact-video">
        <source src={artifact.content} />
      </video>
    );
  }
  
  if (artifact.type === 'image') {
    return (
      <img 
        src={artifact.content} 
        alt={artifact.title}
        className="artifact-image"
      />
    );
  }
  
  if (artifact.type === 'audio') {
    return (
      <audio controls className="artifact-audio">
        <source src={artifact.content} />
      </audio>
    );
  }
  
  return null;
}
```

**E2B Code Interpreter** (for Python execution):
```typescript
// desktop/lib/artifacts/e2b-executor.ts
import { Sandbox } from '@e2b/code-interpreter';

export class E2BExecutor {
  private sandbox: Sandbox;
  
  async execute(code: string, language: string): Promise<ExecutionResult> {
    if (language !== 'python') {
      throw new Error('E2B only supports Python');
    }
    
    this.sandbox = await Sandbox.create();
    
    try {
      const execution = await this.sandbox.runCode(code);
      
      return {
        output: execution.logs.stdout.join('\n'),
        error: execution.logs.stderr.join('\n'),
        results: execution.results,
      };
    } finally {
      await this.sandbox.close();
    }
  }
}
```

#### 4.3 Artifact Library
**Storage & Management**:
```typescript
// desktop/app/(app)/artifacts/page.tsx
export default function ArtifactsPage() {
  const { artifacts } = useArtifacts();
  
  return (
    <div className="artifacts-page">
      <ArtifactFilters 
        types={artifactTypes}
        onFilter={setFilter}
      />
      <ArtifactGrid artifacts={filteredArtifacts}>
        {artifacts.map(artifact => (
          <ArtifactCard 
            key={artifact.id}
            artifact={artifact}
            onView={handleView}
            onEdit={handleEdit}
            onExport={handleExport}
          />
        ))}
      </ArtifactGrid>
    </div>
  );
}
```

---

### Phase 5: Advanced Context Management (Week 5-6)

#### 5.1 Hierarchical Context System
**Architecture**:
```typescript
// desktop/lib/context/manager.ts
export interface ContextItem {
  id: string;
  type: ContextType;
  content: string;
  tokens: number;
  priority: number;
  pinned: boolean;
  source: string;
  metadata: Record<string, any>;
}

export type ContextType =
  | 'system' // System instructions
  | 'message' // Chat messages
  | 'file' // @-mentioned files
  | 'knowledge' // RAG results
  | 'tool' // Tool descriptions
  | 'agent' // Agent configuration
  | 'web' // Web search results;

export class ContextManager {
  private items: ContextItem[] = [];
  private strategy: ContextStrategy;
  
  async add(item: ContextItem): Promise<void> {
    this.items.push(item);
    await this.prune(); // Auto-prune if over limits
  }
  
  async prune(): Promise<void> {
    const { maxTokens, keepPinned } = this.strategy;
    
    // Sort by priority (pinned first, then by priority score)
    const sorted = this.items.sort((a, b) => {
      if (a.pinned !== b.pinned) return b.pinned ? 1 : -1;
      return b.priority - a.priority;
    });
    
    // Keep items until token limit
    let totalTokens = 0;
    this.items = sorted.filter(item => {
      if (item.pinned && keepPinned) return true;
      if (totalTokens + item.tokens > maxTokens) return false;
      totalTokens += item.tokens;
      return true;
    });
  }
  
  async search(query: string): Promise<ContextItem[]> {
    // Search across all context items
    return this.items.filter(item => 
      item.content.toLowerCase().includes(query.toLowerCase())
    );
  }
  
  getContext(): string {
    // Build final context string for LLM
    return this.items
      .sort((a, b) => b.priority - a.priority)
      .map(item => item.content)
      .join('\n\n');
  }
}
```

#### 5.2 @-Mention System
**File mentions**:
```typescript
// desktop/components/chat/mention-input.tsx
export function MentionInput({ onSend }: MentionInputProps) {
  const [mentions, setMentions] = useState<Mention[]>([]);
  
  const handleMention = (text: string) => {
    if (text.startsWith('@')) {
      // Show file/agent/KB selector
      showMentionSelector(text.slice(1));
    }
  };
  
  const handleFileSelect = async (file: File) => {
    // Read file content
    const content = await readFile(file);
    
    // Add to context
    const contextItem: ContextItem = {
      id: generateId(),
      type: 'file',
      content,
      tokens: estimateTokens(content),
      priority: 10,
      pinned: false,
      source: file.name,
      metadata: { path: file.path, size: file.size },
    };
    
    contextManager.add(contextItem);
    setMentions([...mentions, { type: 'file', value: file.name }]);
  };
  
  return (
    <div className="mention-input">
      <textarea
        value={value}
        onChange={handleChange}
        onKeyDown={handleMention}
        placeholder="Type @ to mention files, agents, or knowledge bases..."
      />
      <MentionChips mentions={mentions} onRemove={handleRemoveMention} />
    </div>
  );
}
```

#### 5.3 Context Visualization
**Context inspector**:
```typescript
// desktop/components/context/context-inspector.tsx
export function ContextInspector() {
  const { items, tokens, limit } = useContext();
  
  return (
    <div className="context-inspector">
      <ContextStats 
        tokens={tokens}
        limit={limit}
        itemCount={items.length}
      />
      <ContextList>
        {items.map(item => (
          <ContextItem 
            key={item.id}
            item={item}
            onPin={handlePin}
            onRemove={handleRemove}
            onEdit={handleEdit}
          />
        ))}
      </ContextList>
    </div>
  );
}
```

---

### Phase 6: MCP Tools Integration (Week 6-7)

#### 6.1 MCP Server Manager (Rust)
**File**: `rust/shannon-api/src/mcp/manager.rs`

```rust
use modelcontextprotocol::*;

pub struct MCPServerManager {
    servers: Arc<RwLock<HashMap<String, MCPServer>>>,
    registry: Arc<MCPRegistry>,
}

pub struct MCPServer {
    id: String,
    name: String,
    command: String,
    args: Vec<String>,
    env: HashMap<String, String>,
    status: ServerStatus,
    client: Option<MCPClient>,
    tools: Vec<Tool>,
}

impl MCPServerManager {
    pub async fn add_server(&self, config: ServerConfig) -> Result<String> {
        // Start MCP server subprocess
        let client = MCPClient::connect(&config).await?;
        
        // Fetch available tools
        let tools = client.list_tools().await?;
        
        // Register server
        let server = MCPServer {
            id: generate_id(),
            name: config.name,
            command: config.command,
            args: config.args,
            env: config.env,
            status: ServerStatus::Running,
            client: Some(client),
            tools,
        };
        
        self.servers.write().await.insert(server.id.clone(), server);
        Ok(server.id)
    }
    
    pub async fn execute_tool(
        &self,
        server_id: &str,
        tool_name: &str,
        args: serde_json::Value,
    ) -> Result<ToolResult> {
        let servers = self.servers.read().await;
        let server = servers.get(server_id)
            .ok_or_else(|| anyhow!("Server not found"))?;
        
        let client = server.client.as_ref()
            .ok_or_else(|| anyhow!("Server not connected"))?;
        
        client.call_tool(tool_name, args).await
    }
    
    pub async fn list_tools(&self) -> Vec<Tool> {
        let servers = self.servers.read().await;
        servers.values()
            .flat_map(|s| s.tools.clone())
            .collect()
    }
}
```

#### 6.2 MCP UI Components
**Server Registry**:
```typescript
// desktop/app/(app)/settings/mcp/page.tsx
export default function MCPServersPage() {
  const { servers, loading } = useMCPServers();
  
  return (
    <div className="mcp-servers-page">
      <ServerList>
        {servers.map(server => (
          <ServerCard 
            key={server.id}
            server={server}
            onStart={handleStart}
            onStop={handleStop}
            onRemove={handleRemove}
            onConfigure={handleConfigure}
          />
        ))}
      </ServerList>
      <AddServerButton onClick={handleAddServer} />
      <ServerMarketplace onInstall={handleInstall} />
    </div>
  );
}
```

**Per-conversation Tool Selection**:
```typescript
// desktop/components/chat/tool-selector.tsx
export function ChatToolSelector() {
  const { allTools } = useMCPTools();
  const [selectedTools, setSelectedTools] = useState<string[]>([]);
  
  return (
    <Popover>
      <PopoverTrigger>
        <Button variant="ghost">
          <Wrench className="h-4 w-4" />
          <span>Tools ({selectedTools.length})</span>
        </Button>
      </PopoverTrigger>
      <PopoverContent>
        <ToolList>
          {allTools.map(tool => (
            <ToolItem 
              key={tool.id}
              tool={tool}
              selected={selectedTools.includes(tool.id)}
              onToggle={handleToggle}
            />
          ))}
        </ToolList>
      </PopoverContent>
    </Popover>
  );
}
```

#### 6.3 E2B Integration
**Code interpreter service**:
```rust
// rust/shannon-api/src/tools/e2b.rs
use e2b::Sandbox;

pub struct E2BService {
    api_key: String,
}

impl E2BService {
    pub async fn execute_code(
        &self,
        language: &str,
        code: &str,
    ) -> Result<ExecutionResult> {
        if language != "python" {
            anyhow::bail!("E2B only supports Python");
        }
        
        // Create sandbox
        let sandbox = Sandbox::new(&self.api_key).await?;
        
        // Execute code
        let result = sandbox.run_code(code).await?;
        
        // Close sandbox
        sandbox.close().await?;
        
        Ok(ExecutionResult {
            stdout: result.stdout,
            stderr: result.stderr,
            results: result.results,
            error: result.error,
        })
    }
}
```

**MCP Tool wrapper**:
```rust
// Register E2B as an MCP tool
pub async fn register_e2b_tool(manager: &MCPServerManager) -> Result<()> {
    let tool = Tool {
        name: "execute_python".to_string(),
        description: "Execute Python code in a secure sandbox".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "code": {
                    "type": "string",
                    "description": "Python code to execute"
                }
            },
            "required": ["code"]
        }),
    };
    
    manager.register_native_tool("e2b", tool, |args| async {
        let code = args["code"].as_str().unwrap();
        let result = e2b_service.execute_code("python", code).await?;
        Ok(json!({
            "stdout": result.stdout,
            "stderr": result.stderr,
            "results": result.results,
        }))
    }).await
}
```

---

### Phase 7: Document Processing & RAG (Week 7-8)

#### 7.1 Document Processor
**Supported formats**: PDF, DOCX, PPTX, TXT, MD, CSV, Images (OCR)

**File**: `rust/shannon-api/src/knowledge/processor.rs`

```rust
use pdf_extract::extract_text;
use tesseract_rs::TesseractAPI;

pub struct DocumentProcessor {
    ocr: Option<TesseractAPI>,
}

impl DocumentProcessor {
    pub async fn process(&self, file_path: &Path) -> Result<Document> {
        let extension = file_path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        match extension {
            "pdf" => self.process_pdf(file_path).await,
            "docx" => self.process_docx(file_path).await,
            "txt" | "md" => self.process_text(file_path).await,
            "png" | "jpg" | "jpeg" => self.process_image(file_path).await,
            _ => Err(anyhow!("Unsupported file type: {}", extension)),
        }
    }
    
    async fn process_pdf(&self, path: &Path) -> Result<Document> {
        let text = extract_text(path)?;
        
        Ok(Document {
            id: generate_id(),
            title: path.file_stem().unwrap().to_string_lossy().to_string(),
            content: text,
            metadata: DocumentMetadata {
                file_type: "pdf".to_string(),
                file_size: path.metadata()?.len(),
                page_count: None,
                created_at: SystemTime::now(),
            },
        })
    }
    
    async fn process_image(&self, path: &Path) -> Result<Document> {
        let ocr = self.ocr.as_ref()
            .ok_or_else(|| anyhow!("OCR not initialized"))?;
        
        let text = ocr.recognize(path)?;
        
        Ok(Document {
            id: generate_id(),
            title: path.file_stem().unwrap().to_string_lossy().to_string(),
            content: text,
            metadata: DocumentMetadata {
                file_type: "image".to_string(),
                file_size: path.metadata()?.len(),
                page_count: None,
                created_at: SystemTime::now(),
            },
        })
    }
}
```

#### 7.2 Embedding & Vector Storage
**File**: `rust/shannon-api/src/knowledge/embeddings.rs`

```rust
use rusqlite::Connection;

pub struct EmbeddingService {
    db: Arc<Mutex<Connection>>,
    provider: EmbeddingProvider,
}

pub enum EmbeddingProvider {
    OpenAI { api_key: String, model: String },
    Local { model_path: PathBuf },
}

impl EmbeddingService {
    pub async fn embed_document(&self, doc: &Document) -> Result<Vec<Chunk>> {
        // Split document into chunks
        let chunks = self.chunk_document(doc).await?;
        
        // Generate embeddings for each chunk
        let mut embedded_chunks = Vec::new();
        for chunk in chunks {
            let embedding = self.generate_embedding(&chunk.content).await?;
            
            embedded_chunks.push(Chunk {
                id: generate_id(),
                document_id: doc.id.clone(),
                content: chunk.content,
                embedding,
                metadata: chunk.metadata,
            });
        }
        
        // Store in vector DB
        self.store_chunks(&embedded_chunks).await?;
        
        Ok(embedded_chunks)
    }
    
    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<Chunk>> {
        // Generate query embedding
        let query_embedding = self.generate_embedding(query).await?;
        
        // Search vector DB using cosine similarity
        let results = self.db.lock().unwrap().query_row(
            "SELECT id, content, embedding, metadata 
             FROM chunks 
             ORDER BY embedding <-> ?1 
             LIMIT ?2",
            params![query_embedding, limit],
            |row| {
                Ok(Chunk {
                    id: row.get(0)?,
                    document_id: String::new(),
                    content: row.get(1)?,
                    embedding: row.get(2)?,
                    metadata: serde_json::from_str(&row.get::<_, String>(3)?).unwrap(),
                })
            },
        )?;
        
        Ok(results)
    }
}
```

#### 7.3 RAG Integration
**Context injection**:
```typescript
// desktop/lib/knowledge/rag.ts
export class RAGService {
  async augmentPrompt(
    query: string,
    knowledgeBases: string[],
    limit: number = 5
  ): Promise<string> {
    // Search knowledge bases
    const results = await this.search(query, knowledgeBases, limit);
    
    // Format context
    const context = results
      .map((r, i) => `[Source ${i + 1}: ${r.source}]\n${r.content}`)
      .join('\n\n');
    
    // Inject into prompt
    return `Context from knowledge base:\n\n${context}\n\nUser query: ${query}`;
  }
  
  private async search(
    query: string,
    kbIds: string[],
    limit: number
  ): Promise<SearchResult[]> {
    // Call Shannon API /api/knowledge/search
    const response = await fetch('/api/knowledge/search', {
      method: 'POST',
      body: JSON.stringify({ query, knowledge_bases: kbIds, limit }),
    });
    
    return response.json();
  }
}
```

---

### Phase 8: Settings & Extensions System (Week 8-9)

#### 8.1 Modular Settings Architecture
**Settings schema**:
```typescript
// desktop/types/settings.ts
export interface Settings {
  // Providers
  providers: ProviderSettings[];
  
  // Models
  defaultModel: ModelConfig;
  modelOverrides: Record<string, ModelConfig>;
  
  // Extensions
  extensions: ExtensionConfig[];
  
  // Knowledge
  knowledgeBases: KnowledgeBaseConfig[];
  
  // MCP
  mcpServers: MCPServerConfig[];
  
  // UI
  theme: 'light' | 'dark' | 'auto';
  customTheme?: CustomTheme;
  language: string;
  
  // Advanced
  contextStrategy: ContextStrategy;
  ragSettings: RAGSettings;
  privacyMode: boolean;
}
```

**Settings UI**:
```typescript
// desktop/app/(app)/settings/page.tsx
export default function SettingsPage() {
  return (
    <div className="settings-page">
      <SettingsSidebar categories={categories} />
      <SettingsContent>
        <Routes>
          <Route path="/providers" element={<ProvidersSettings />} />
          <Route path="/models" element={<ModelsSettings />} />
          <Route path="/extensions" element={<ExtensionsSettings />} />
          <Route path="/knowledge" element={<KnowledgeSettings />} />
          <Route path="/mcp" element={<MCPSettings />} />
          <Route path="/appearance" element={<AppearanceSettings />} />
          <Route path="/advanced" element={<AdvancedSettings />} />
        </Routes>
      </SettingsContent>
    </div>
  );
}
```

---

### Phase 9: Cherry Studio UI/UX Adoption (Week 9-10)

#### 9.1 Tab System
**Multi-chat tabs**:
```typescript
// desktop/components/tabs/tab-manager.tsx
export function TabManager() {
  const { tabs, activeTab } = useTabs();
  
  return (
    <div className="tab-manager">
      <TabBar>
        {tabs.map(tab => (
          <Tab
            key={tab.id}
            tab={tab}
            active={tab.id === activeTab}
            onSelect={handleSelectTab}
            onClose={handleCloseTab}
          />
        ))}
        <NewTabButton onClick={handleNewTab} />
      </TabBar>
      <TabContent>
        {tabs.find(t => t.id === activeTab)?.content}
      </TabContent>
    </div>
  );
}
```

#### 9.2 Keyboard Shortcuts
**Shortcut system**:
```typescript
// desktop/lib/keyboard/shortcuts.ts
export const SHORTCUTS = {
  newChat: 'cmd+n',
  newTab: 'cmd+t',
  closeTab: 'cmd+w',
  nextTab: 'cmd+shift+]',
  prevTab: 'cmd+shift+[',
  search: 'cmd+f',
  settings: 'cmd+,',
  quickMode: 'cmd+shift+q',
  taskMode: 'cmd+shift+t',
};

export function useKeyboardShortcuts() {
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      const shortcut = formatShortcut(e);
      const action = Object.entries(SHORTCUTS).find(
        ([_, keys]) => keys === shortcut
      )?.[0];
      
      if (action) {
        e.preventDefault();
        executeAction(action);
      }
    };
    
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, []);
}
```

#### 9.3 Theme System
**Custom themes**:
```typescript
// desktop/lib/theme/manager.ts
export interface CustomTheme {
  colors: {
    primary: string;
    secondary: string;
    background: string;
    foreground: string;
    border: string;
    accent: string;
  };
  fonts: {
    sans: string;
    mono: string;
  };
  spacing: Record<string, string>;
}

export class ThemeManager {
  async loadTheme(theme: string): Promise<void> {
    const themeData = await fetch(`/themes/${theme}.json`).then(r => r.json());
    this.applyTheme(themeData);
  }
  
  private applyTheme(theme: CustomTheme): void {
    const root = document.documentElement;
    Object.entries(theme.colors).forEach(([key, value]) => {
      root.style.setProperty(`--${key}`, value);
    });
  }
}
```

---

### Phase 10: Action Engine (Week 10-11) - Manus.ai Parity

#### 10.1 Browser Automation
**Playwright integration**:
```rust
// rust/shannon-api/src/actions/browser.rs
use playwright::Playwright;

pub struct BrowserService {
    playwright: Playwright,
}

impl BrowserService {
    pub async fn navigate(&self, url: &str) -> Result<PageSnapshot> {
        let browser = self.playwright.chromium().launch().await?;
        let page = browser.new_page().await?;
        page.goto(url).await?;
        
        Ok(PageSnapshot {
            url: page.url()?,
            title: page.title().await?,
            content: page.content().await?,
            screenshot: page.screenshot().await?,
        })
    }
    
    pub async fn extract_data(&self, url: &str, selector: &str) -> Result<String> {
        let browser = self.playwright.chromium().launch().await?;
        let page = browser.new_page().await?;
        page.goto(url).await?;
        
        let element = page.query_selector(selector).await?;
        Ok(element.inner_text().await?)
    }
}
```

#### 10.2 Filesystem Tools
**Sandboxed file operations**:
```rust
// rust/shannon-api/src/actions/filesystem.rs
pub struct FilesystemService {
    sandbox_root: PathBuf,
}

impl FilesystemService {
    pub async fn read_file(&self, path: &str) -> Result<String> {
        let full_path = self.sandbox_root.join(path);
        
        // Security check: ensure within sandbox
        if !full_path.starts_with(&self.sandbox_root) {
            anyhow::bail!("Path outside sandbox");
        }
        
        Ok(tokio::fs::read_to_string(full_path).await?)
    }
    
    pub async fn write_file(&self, path: &str, content: &str) -> Result<()> {
        let full_path = self.sandbox_root.join(path);
        
        // Security check
        if !full_path.starts_with(&self.sandbox_root) {
            anyhow::bail!("Path outside sandbox");
        }
        
        tokio::fs::write(full_path, content).await?;
        Ok(())
    }
}
```

---

## Part 5: Testing & Validation Strategy

### Integration Testing

#### Test Suite Structure
```
tests/
├── integration/
│   ├── engine/
│   │   ├── test_durable_execution.rs
│   │   ├── test_prompt_rendering.rs
│   │   └── test_strategy_composition.rs
│   ├── chat/
│   │   ├── test_quick_mode.rs
│   │   ├── test_task_mode.rs
│   │   └── test_mode_switching.rs
│   ├── agents/
│   │   ├── test_agent_creation.rs
│   │   ├── test_agent_execution.rs
│   │   └── test_agent_export.rs
│   ├── artifacts/
│   │   ├── test_artifact_detection.rs
│   │   ├── test_renderers.rs
│   │   └── test_e2b_execution.rs
│   ├── context/
│   │   ├── test_context_management.rs
│   │   ├── test_mentions.rs
│   │   └── test_pruning.rs
│   ├── mcp/
│   │   ├── test_server_management.rs
│   │   ├── test_tool_execution.rs
│   │   └── test_e2b_integration.rs
│   └── knowledge/
│       ├── test_document_processing.rs
│       ├── test_embeddings.rs
│       └── test_rag.rs
└── e2e/
    ├── test_complete_workflow.rs
    ├── test_cherry_parity.rs
    └── test_manus_parity.rs
```

### Validation Checkpoints

#### Phase 1 Validation
- [ ] Durable-shannon executes without simulation
- [ ] Prompts render correctly for all patterns
- [ ] Strategies compose multi-stage workflows
- [ ] Budget middleware enforces limits
- [ ] Events stream correctly to UI

#### Phase 2 Validation
- [ ] Quick chat responds instantly (<500ms)
- [ ] Task chat submits to workflow engine
- [ ] Mode switching works seamlessly
- [ ] Auto-detection correctly classifies queries

#### Phase 3 Validation
- [ ] Agents can be created and saved
- [ ] Agent specs load correctly
- [ ] Agents execute with correct configuration
- [ ] Export/import works

#### Phase 4 Validation
- [ ] Artifacts detected in responses
- [ ] All renderer types work
- [ ] E2B executes Python code
- [ ] Artifacts save to library

#### Phase 5 Validation
- [ ] Context tracks all items
- [ ] Pruning respects limits
- [ ] @-mentions work
- [ ] Pinned items persist

#### Phase 6 Validation
- [ ] MCP servers start/stop correctly
- [ ] Tools execute successfully
- [ ] Per-conversation selection works
- [ ] E2B integration functional

#### Phase 7 Validation
- [ ] All document types process
- [ ] Embeddings generate
- [ ] Vector search returns relevant results
- [ ] RAG augments prompts correctly

#### Phase 8 Validation
- [ ] Settings persist
- [ ] All categories functional
- [ ] Export/import works
- [ ] Theme system works

#### Phase 9 Validation
- [ ] Tabs create/close correctly
- [ ] Shortcuts work
- [ ] Theme changes apply
- [ ] UI matches Cherry Studio quality

#### Phase 10 Validation
- [ ] Browser automation works
- [ ] Filesystem operations sandbox correctly
- [ ] All action types functional

---

## Part 6: Migration from Cherry Studio

### Feature Mapping

| Cherry Studio Feature | Shannon Implementation | Status | Notes |
|----------------------|------------------------|--------|-------|
| **Chat System** |
| Quick Chat | Quick Mode | ✅ Planned | Lightweight, direct LLM |
| Multi-model Chat | Task Mode | ✅ Planned | With workflow engine |
| Streaming | SSE/WebSocket | ✅ Exists | Already implemented |
| **Agents** |
| 300+ Templates | Agent Repository | ✅ Planned | Import from Cherry |
| Custom Creation | Agent Editor | ✅ Planned | YAML/JSON specs |
| Marketplace | Import/Export | ✅ Planned | Compatible format |
| **Artifacts** |
| Code Sandbox | Sandpack | ✅ Planned | React/JS/TS/Python |
| Mermaid | Mermaid.js | ✅ Planned | SVG rendering |
| Charts | Recharts | ✅ Planned | D3 alternative |
| Media | HTML5 | ✅ Planned | Video/audio/image |
| **Knowledge** |
| Document Processing | Rust processor | ✅ Planned | PDF/DOCX/images |
| Vector DB | SQLite + libsql | ✅ Planned | Local embeddings |
| RAG | Context injection | ✅ Planned | Prompt augmentation |
| **MCP** |
| Server Registry | Rust manager | ✅ Planned | Subprocess management |
| Tool Execution | MCP client | ✅ Planned | Standard protocol |
| Per-conversation | Tool selector | ✅ Planned | UI component |
| **Settings** |
| Modular UI | Category pages | ✅ Planned | Next.js routes |
| Theme System | CSS variables | ✅ Planned | Custom themes |
| Extensions | Plugin system | 🔄 Future | Post-MVP |

### Data Migration

**Export from Cherry Studio**:
```bash
# Export agents
cherry-studio export --type agents --output agents.json

# Export knowledge bases
cherry-studio export --type knowledge --output knowledge.json

# Export settings
cherry-studio export --type settings --output settings.json
```

**Import to Shannon**:
```bash
# Import agents
shannon import agents --file agents.json

# Import knowledge
shannon import knowledge --file knowledge.json

# Import settings
shannon import settings --file settings.json
```

---

## Part 7: Performance Targets

### Latency Requirements

| Operation | Target | Acceptable | Cherry Studio |
|-----------|--------|------------|---------------|
| Quick chat first token | <200ms | <500ms | ~300ms |
| Task submission | <100ms | <200ms | ~150ms |
| Artifact rendering | <300ms | <500ms | ~400ms |
| MCP tool call | <1s | <2s | ~1.5s |
| Context search | <50ms | <100ms | ~75ms |
| Agent load | <50ms | <100ms | ~60ms |
| Settings save | <20ms | <50ms | ~30ms |

### Resource Usage

| Resource | Target | Max | Notes |
|----------|--------|-----|-------|
| Memory (idle) | <200MB | <300MB | Rust advantage |
| Memory (active) | <500MB | <800MB | With 5 chats |
| CPU (idle) | <2% | <5% | Background only |
| CPU (active) | <30% | <50% | Single chat |
| Disk (app) | <100MB | <150MB | Rust binary |
| Disk (data) | Variable | N/A | User data |

---

## Part 8: Deployment & Distribution

### Build Configuration

**Rust features**:
```toml
[features]
default = ["embedded", "gateway", "database"]
embedded = ["durable-shannon"]
gateway = []
grpc = ["orchestrator-client"]
database = ["sqlite", "encryption"]
```

**Tauri configuration**:
```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:3000",
    "distDir": "../out"
  },
  "bundle": {
    "active": true,
    "targets": ["dmg", "msi", "appimage"],
    "identifier": "ai.shannon.desktop",
    "icon": ["icons/icon.png"]
  }
}
```

### Platform Support

| Platform | Architecture | Status | Notes |
|----------|-------------|--------|-------|
| macOS | ARM64 | ✅ Primary | Apple Silicon |
| macOS | x64 | ✅ Primary | Intel |
| Windows | x64 | ✅ Primary | Windows 10+ |
| Windows | ARM64 | 🔄 Future | Windows 11 ARM |
| Linux | x64 | ✅ Primary | Ubuntu/Debian |
| Linux | ARM64 | 🔄 Future | Raspberry Pi |

---

## Part 9: Timeline & Milestones

### Detailed Schedule

| Week | Phase | Deliverables | Success Criteria |
|------|-------|-------------|------------------|
| 1-2 | Core Engine | Durable integration, prompts, strategies | No simulation, patterns execute |
| 2-3 | Dual Chat | Quick & task modes | Both modes functional |
| 3-4 | Agents | Repository, editor, execution | Create/run agents |
| 4-5 | Artifacts | Detection, renderers, library | All types render |
| 5-6 | Context | Management, mentions, pruning | Context works correctly |
| 6-7 | MCP | Server manager, tools, e2b | MCP tools execute |
| 7-8 | Knowledge | Processing, embeddings, RAG | Documents searchable |
| 8-9 | Settings | UI, persistence, themes | Settings functional |
| 9-10 | UI/UX | Tabs, shortcuts, polish | Cherry Studio quality |
| 10-11 | Actions | Browser, filesystem | Manus.ai parity |
| 11-12 | Testing | Integration, E2E, polish | All tests pass |

### Milestones

**M1: Core Engine Operational (Week 2)**
- ✅ Durable-shannon fully integrated
- ✅ All patterns execute
- ✅ Events stream to UI
- ✅ No simulation code remains

**M2: Dual-Mode Chat (Week 3)**
- ✅ Quick mode functional
- ✅ Task mode functional
- ✅ Mode detection works
- ✅ Both modes stream correctly

**M3: Agent System (Week 4)**
- ✅ Agent repository implemented
- ✅ Agent editor functional
- ✅ Agents execute correctly
- ✅ Export/import works

**M4: Artifacts & Context (Week 6)**
- ✅ All artifact types render
- ✅ E2B Python execution
- ✅ Context management works
- ✅ @-mentions functional

**M5: MCP & Knowledge (Week 8)**
- ✅ MCP servers managed
- ✅ Tools execute
- ✅ Documents process
- ✅ RAG augments prompts

**M6: Feature Complete (Week 10)**
- ✅ Settings system complete
- ✅ UI/UX polished
- ✅ Action engine functional
- ✅ Cherry Studio parity achieved

**M7: Production Ready (Week 12)**
- ✅ All tests pass
- ✅ Performance targets met
- ✅ Documentation complete
- ✅ Ready for release

---

## Part 10: Risk Mitigation

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Durable-shannon performance | Medium | High | Benchmark early, optimize WASM |
| E2B integration complexity | High | Medium | Use official SDK, sandbox properly |
| MCP server stability | Medium | Medium | Health checks, auto-restart |
| Vector search performance | Low | Medium | Index optimization, caching |
| Memory usage in Rust | Low | High | Profiling, Arc optimization |

### Schedule Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Scope creep | High | High | Strict phase gates, MVP focus |
| Integration issues | Medium | High | Early integration testing |
| Third-party dependencies | Medium | Medium | Vendor evaluation, alternatives |
| Performance issues | Low | High | Continuous profiling |

---

## Part 11: Success Metrics

### Functional Completeness

- [ ] All Cherry Studio features implemented
- [ ] Manus.ai capabilities matched
- [ ] 100% durable-shannon integration
- [ ] Zero simulation/stub code

### Performance Metrics

- [ ] Quick chat <500ms first token
- [ ] Task submission <200ms
- [ ] Memory usage <500MB active
- [ ] CPU usage <30% active

### Quality Metrics

- [ ] Test coverage >80%
- [ ] Zero critical bugs
- [ ] All integration tests pass
- [ ] E2E scenarios validated

### User Experience

- [ ] Cherry Studio UI quality
- [ ] S-tier responsiveness
- [ ] Smooth transitions
- [ ] Intuitive workflows

---

## Conclusion

This comprehensive plan provides a clear roadmap to achieve:

1. **100% Operational Durable-Shannon Integration** - Remove all simulation stubs, fully integrate workflow engine
2. **Cherry Studio Feature Parity** - Implement all key features with equal or better quality
3. **Manus.ai Capabilities** - Deep research, action engine, bespoke agents
4. **Production-Ready System** - Tested, performant, beautiful, ready to replace Cherry Studio

### Key Differentiators

Shannon will exceed Cherry Studio and Manus.ai with:

- **Rust Performance**: 2-3x faster, 40% less memory
- **Embedded Workflow Engine**: No external dependencies
- **Durable Execution**: Fault-tolerant, resumable workflows
- **Local-First**: Full functionality offline
- **Open Architecture**: Extensible, hackable, transparent

### Next Steps

1. **Review this plan** with the team
2. **Prioritize phases** based on business needs
3. **Allocate resources** for 12-week timeline
4. **Set up tracking** for milestones and metrics
5. **Begin Phase 1** - Core engine integration

**Ready to build the future of AI assistants. Let's ship it! 🚀**