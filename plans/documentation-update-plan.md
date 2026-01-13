# Shannon Documentation Update Plan - Production Quality

**Date**: 2026-01-13  
**Status**: All 10 phases implemented - Documentation sync required  
**Objective**: Update README and all docs/ to reflect completed features for production documentation website

---

## Executive Summary

With all 10 implementation phases complete (100%), the documentation needs comprehensive updates to reflect:
- New dual-mode chat system
- Agent repository with 12 templates
- UI artifacts (11 types)
- Advanced context management (4 strategies)
- MCP tools ecosystem
- RAG knowledge bases with streaming citations
- Settings system
- Tab system and keyboard shortcuts
- Action engine (browser + filesystem)

---

## Documentation Structure for Website

```
docs/
├── getting-started/
│   ├── README.md (Quick Start)
│   ├── installation.md
│   ├── first-agent.md
│   └── deployment-modes.md
├── features/
│   ├── README.md (Overview)
│   ├── dual-mode-chat.md (NEW)
│   ├── agents.md (NEW)
│   ├── artifacts.md (NEW)
│   ├── context-management.md (UPDATE)
│   ├── mcp-tools.md (NEW)
│   ├── knowledge-bases.md (NEW)
│   ├── settings.md (NEW)
│   └── actions.md (NEW)
├── api-reference/
│   ├── REST/
│   │   ├── chat.md
│   │   ├── agents.md (NEW)
│   │   ├── knowledge.md (NEW)
│   │   ├── settings.md (NEW)
│   │   └── actions.md (NEW)
│   ├── websocket.md
│   └── events.md
├── configuration/
│   ├── README.md
│   ├── providers.md (UPDATE)
│   ├── models.md
│   ├── context-strategies.md (NEW)
│   ├── chunking-strategies.md (NEW)
│   └── deployment-modes.md
├── guides/
│   ├── context-management.md (NEW)
│   ├── rag-setup.md (NEW)
│   ├── custom-agents.md (NEW)
│   ├── mcp-integration.md (NEW)
│   └── browser-automation.md (NEW)
└── architecture/
    ├── overview.md (UPDATE)
    ├── workflow-engine.md (UPDATE)
    ├── vector-store.md (NEW)
    └── multi-tenant.md (UPDATE)
```

---

## README.md Updates

### New Sections to Add

#### 1. Feature Highlights (After "Why Shannon?")

```markdown
## 🌟 Feature Highlights

### Dual-Mode Chat System
- **Quick Chat**: Lightning-fast responses (<500ms) for simple queries
- **Task Chat**: Multi-agent workflows for complex research and analysis
- AI-powered mode detection automatically selects the optimal approach

### Custom Agents
- Create custom AI agents with specific prompts, tools, and knowledge
- 12 pre-configured templates (Code Expert, Research Analyst, etc.)
- Save, share, and reuse agent configurations

### UI Artifacts
- Interactive code execution with Sandpack (React, JavaScript, Python)
- Mermaid diagrams, charts, SVG graphics
- E2B Python interpreter for secure code execution
- 11 supported artifact types

### Advanced Context Management
- **Hierarchical Memory**: 3-tier system (verbatim, summarized, key facts)
- **Sliding Window**: Keep recent messages within budget
- **Progressive Summarization**: Compress older messages
- **Keep First & Last**: Preserve instructions + recent context
- Token-aware with automatic pruning

### Knowledge Bases & RAG
- Upload PDFs, DOCX, PPTX with Mistral or Unstructured.io
- 4 chunking strategies (Semantic chunking default)
- Vector search with sub-millisecond response
- Real-time citation streaming shows sources used
- Attach knowledge bases to agents or conversations

### MCP Tools Integration
- Full Model Context Protocol server management
- Per-conversation tool selection
- Pre-configured templates (GitHub, Slack, PostgreSQL, Calendar, Search)
- E2B code execution as built-in tool

### Action Engine (Manus.ai Parity)
- Browser automation with headless Chrome
- Sandboxed filesystem operations
- Permission-based security model
- Integrated with AI workflows
```

#### 2. Update Architecture Diagram

```markdown
### Shannon Architecture (2026)

```
┌────────────────────────────────────────────────────────────┐
│                    Desktop Application                      │
│                   (Next.js 16 + Tauri 2)                    │
│                                                              │
│  ┌──────────────────────┐  ┌────────────────────────────┐ │
│  │  Frontend (React 19) │  │  Backend (Rust - Tauri)    │ │
│  │  ├─ Dual-Mode Chat   │  │  ├─ Embedded Shannon API   │ │
│  │  ├─ Agent Repository │  │  ├─ Durable Workflows      │ │
│  │  ├─ UI Artifacts     │  │  ├─ MCP Server Manager     │ │
│  │  ├─ Knowledge Bases  │  │  ├─ RAG Pipeline           │ │
│  │  ├─ Tab System       │  │  └─ Action Engine          │ │
│  │  └─ Settings         │  │                             │ │
│  └──────────────────────┘  └────────────────────────────┘ │
└────────────────────────────────────────────────────────────┘
                           │
                           │ HTTP/SSE
                           ▼
┌────────────────────────────────────────────────────────────┐
│              Shannon API (Rust) - Port 8080/8765           │
│  ├─ Gateway (Auth, Sessions, Rate Limiting)                │
│  ├─ Workflow Engine (Durable + Temporal modes)             │
│  ├─ Knowledge System (RAG, Vector Store, Citations)        │
│  ├─ MCP Tools Registry                                      │
│  ├─ Action Engine (Browser, Filesystem)                    │
│  └─ Context Manager (4 strategies)                         │
└────────────────────────────────────────────────────────────┘
```
```

#### 3. Add Deployment Modes Comparison

```markdown
### Deployment Mode Feature Matrix

| Feature | Embedded | Cloud | Hybrid | Mesh |
|---------|----------|-------|--------|------|
| **Dual-Mode Chat** | ✅ | ✅ | ✅ | ✅ |
| **Custom Agents** | ✅ | ✅ | ✅ | ✅ |
| **UI Artifacts** | ✅ | ✅ | ✅ | ✅ |
| **Context Strategies** | ✅ | ✅ | ✅ | ✅ |
| **MCP Tools** | ✅ | ✅ | ✅ | ✅ |
| **Knowledge Bases** | ✅ Local | ✅ Shared | ✅ Sync | ✅ P2P |
| **Browser Automation** | ✅ | ✅ | ✅ | ✅ |
| **Multi-Tenant** | ❌ | ✅ | ⚠️ | ❌ |
| **Offline Support** | ✅ Full | ❌ | ⚠️ Partial | ✅ Full |
| **Database** | SQLite | PostgreSQL | SQLite+Cloud | SQLite |
| **Workflow Engine** | Durable (Rust) | Temporal (Go) | Both | Durable |
```

---

## New Documentation Files Needed

### 1. features/dual-mode-chat.md
```markdown
# Dual-Mode Chat System

Shannon provides two chat modes optimized for different use cases:

## Quick Chat Mode

Lightning-fast responses for simple queries and conversations.

**When to use:**
- Simple questions
- Quick lookups
- Casual conversation
- <500ms response time

**How it works:**
- Direct LLM API calls (no workflow overhead)
- Streaming response chunks
- Inline tool execution
- Minimal state management

**Example:**
```bash
curl -X POST http://localhost:8080/api/chat/quick \
  -d '{"message":"What is 2+2?", "mode":"quick"}'
```

## Task Chat Mode

Multi-agent workflows for complex research and analysis.

**When to use:**
- Research tasks
- Multi-step analysis
- Complex queries requiring multiple agents
- When you need citations and sources

**How it works:**
- Submits to durable workflow engine
- Multi-stage reasoning (Scientific, Exploratory)
- Token budget enforcement
- Durable state with resumability

**Example:**
```bash
curl -X POST http://localhost:8080/api/tasks \
  -d '{"query":"Research quantum computing", "strategy":"scientific"}'
```

## Auto-Detection

Shannon automatically suggests the optimal mode:

**Quick Mode Indicators:**
- Keywords: "quick", "simple", "what is"
- Short queries (<100 words)
- Single questions

**Task Mode Indicators:**
- Keywords: "research", "analyze", "compare"
- Long queries (>100 words)
- Multi-step instructions
```

### 2. features/agents.md
```markdown
# Agent Repository

Create, store, and run custom AI agents with specific configurations.

## Pre-configured Templates (12)

1. **General Assistant** 🤖 - Versatile helper
2. **Code Expert** 💻 - Software engineering
3. **Research Analyst** 🔬 - Deep research
4. **Creative Writer** ✍️ - Content creation
5. **Business Advisor** 💼 - Strategy consulting
6. **Data Analyst** 📊 - Data analysis
7. **Education Tutor** 🎓 - Teaching
8. **Technical Support** 🛠️ - Troubleshooting
9. **Marketing Expert** 📢 - Marketing strategy
10. **Legal Advisor** ⚖️ - Legal information
11. **Health & Wellness** 💪 - Wellness education
12. **Python Specialist** 🐍 - Python programming

## Creating Custom Agents

### Agent Configuration

```yaml
name: My Custom Agent
description: Specialized agent for my use case
version: 1.0.0
author: Your Name

# System Instructions
systemPrompt: |
  You are a specialized assistant focused on...

# Model Configuration
model:
  provider: anthropic
  name: claude-sonnet-4-5
  temperature: 0.7
  maxTokens: 4096

# Capabilities
tools:
  - web_search
  - calculator
  - execute_python

knowledgeBases:
  - kb_documentation
  - kb_examples

allowedActions:
  - browser
  - filesystem

# Behavior
strategy: scientific
conversationStyle: technical

# Metadata
tags:
  - technical
  - analysis
category: development
icon: 💻
```

### API Usage

```bash
# Create agent
curl -X POST http://localhost:8080/api/agents \
  -H "Content-Type: application/json" \
  -d @agent-spec.json

# List agents
curl http://localhost:8080/api/agents

# Execute agent
curl -X POST http://localhost:8080/api/agents/{id}/execute \
  -d '{"message":"Hello", "conversationId":"conv-123"}'
```

### Storage

Agents are stored in SQLite with full CRUD operations:
- Create, read, update, delete
- Export to YAML for sharing
- Import from YAML
- Version control support
```

### 3. features/context-management.md
```markdown
# Context Management Strategies

Shannon provides 4 intelligent strategies for managing conversation history within token limits.

## Strategies Overview

### 1. Hierarchical Memory (Default) 🌟

**Best for**: Complex conversations requiring both recent detail and historical context

**How it works:**
- **Tier 1 (Verbatim)**: Last N turns kept exactly (default: 5 turns = 10 messages)
- **Tier 2 (Summarized)**: Older messages compressed via LLM
- **Tier 3 (Key Facts)**: Critical facts extracted from entire history

**Configuration:**
```yaml
strategy: hierarchical_memory
short_term_turns: 5        # Recent messages kept verbatim
mid_term_budget: 2000      # Tokens for summaries
long_term_budget: 500      # Tokens for key facts
summarization_model: claude-haiku-4-5
```

**Example:**
```
Tier 3: [Key Facts] User is building a web app. Tech stack: React, Node.js
Tier 2: [Summary] Previous discussion covered authentication options...
Tier 1: (Last 5 turns verbatim)
  User: What about JWT?
  Assistant: JWT works well for stateless auth...
  User: Show me an example
  Assistant: Here's how to implement JWT...
  User: Thanks! Now about refresh tokens?
```

### 2. Sliding Window

**Best for**: Real-time conversations, chat applications

**How it works:**
- Keeps only the most recent messages that fit within token budget
- Simple FIFO (First In, First Out)
- Always preserves pinned messages

### 3. Progressive Summarization

**Best for**: Long conversations with occasional reference to history

**How it works:**
- Recent messages: verbatim
- Older messages: summarized when exceeding budget
- Single-pass summarization

### 4. Keep First & Last

**Best for**: Task-oriented chats with system instructions

**How it works:**
- Preserves first message (system instructions)
- Keeps last N turns (recent context)
- Removes everything in the middle

## Configuration

```bash
# REST API
curl -X PUT http://localhost:8080/api/v2/settings/context \
  -d '{
    "strategy": "hierarchical_memory",
    "short_term_turns": 5,
    "mid_term_budget": 2000,
    "long_term_budget": 500,
    "summarization_model": "claude-haiku-4-5"
  }'
```

```typescript
// TypeScript/Desktop
import { updateSettingsSection } from '@/lib/shannon/settings-v2';

await updateSettingsSection('context', {
  strategy: 'hierarchical_memory',
  shortTermTurns: 5,
  midTermBudget: 2000,
  longTermBudget: 500,
  summarizationModel: 'claude-haiku-4-5'
});
```

## Token Budgets

| Setting | Default | Range | Description |
|---------|---------|-------|-------------|
| Short-term Turns | 5 | 1-20 | Number of recent conversation turns kept verbatim |
| Mid-term Budget | 2000 | 500-10000 | Tokens allocated for summaries |
| Long-term Budget | 500 | 100-5000 | Tokens for extracted key facts |

## Summarization Models

Fast, inexpensive models recommended:
- `claude-haiku-4-5` (recommended)
- `gpt-4o-mini`
- `gemini-1.5-flash`

## @-Mention System

Reference context items directly:
- `@file:path/to/file` - Include file content
- `@agent:agent-id` - Reference agent configuration
- `@knowledge:kb-id` - Include knowledge base
- `@tool:tool-name` - Reference tool documentation
```

### 4. features/knowledge-bases.md
```markdown
# Knowledge Bases & RAG

Upload documents and enable AI agents to answer questions grounded in your knowledge.

## Document Processors

### Mistral

AI-powered document parsing with built-in OCR.

**Supported formats:**
- PDF (application/pdf)
- DOCX (Microsoft Word)
- PPTX (PowerPoint)
- XLSX (Excel)
- Text files (TXT, MD)
- HTML
- Images (PNG, JPEG) with OCR

**Configuration:**
```yaml
processor: mistral
api_key: your-mistral-api-key
```

### Unstructured.io

Enterprise-grade document extraction service.

**Hosted version:**
```yaml
processor: unstructured_hosted
api_key: your-unstructured-api-key
```

**Self-hosted version:**
```yaml
processor: unstructured_self_hosted
api_url: http://your-server:8000
```

**Supported formats:**
- PDF, DOC/DOCX, PPT/PPTX, XLS/XLSX
- Text, HTML, Markdown, CSV
- EPUB
- Images (with OCR)
- RTF, EML

## Chunking Strategies

### 1. Fixed-Size Chunking

**Default**: 768 tokens, 15% overlap

**When to use**: Baseline strategy for diverse content

**Configuration:**
```yaml
strategy: fixed_size
chunk_size: 768
overlap_percent: 0.15
```

### 2. Semantic Chunking (Default) 🌟

**When to use**: Most documents, best for retrieval quality

**How it works:**
- Respects sentence boundaries
- Maintains thought coherence
- Range: 256-1024 tokens

**Configuration:**
```yaml
strategy: semantic
min_chunk_size: 256
max_chunk_size: 1024
respect_sentences: true
```

### 3. Structure-Aware Chunking

**When to use**: Technical docs, code, tables

**How it works:**
- Preserves markdown headers
- Keeps code blocks intact
- Maintains table structure

**Configuration:**
```yaml
strategy: structure_aware
preserve_code_blocks: true
preserve_tables: true
preserve_lists: true
```

### 4. Hierarchical Chunking

**When to use**: Long documents requiring precision + context

**How it works:**
- Creates parent chunks (large context)
- Creates child chunks (precise retrieval)
- Links parent-child relationships

**Configuration:**
```yaml
strategy: hierarchical
parent_chunk_size: 2048
child_chunk_size: 512
max_depth: 3
```

## Creating a Knowledge Base

### Via REST API

```bash
# Create knowledge base
curl -X POST http://localhost:8080/api/knowledge/bases \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Product Documentation",
    "description": "All product manuals and guides",
    "chunking_strategy": "semantic",
    "embedding_provider": "openai",
    "embedding_model": "text-embedding-3-small"
  }'

# Upload document
curl -X POST http://localhost:8080/api/knowledge/bases/{kb_id}/documents \
  -F "file=@manual.pdf" \
  -F "processor=mistral"

# Search
curl -X POST http://localhost:8080/api/knowledge/search \
  -d '{"knowledge_base_ids":["kb-123"],"query":"How to connect?","limit":5}'
```

### Via Desktop App

1. Navigate to Knowledge page
2. Click "Create Knowledge Base"
3. Configure chunking strategy
4. Upload documents (drag & drop)
5. Documents automatically process and chunk
6. Attach to conversations or agents

## RAG with Citations

When knowledge bases are attached, Shannon:
1. Searches vector store for relevant chunks
2. **Streams citations first** (before LLM response)
3. Augments prompt with context
4. LLM responds with grounded knowledge
5. Citations displayed with relevance scores

**Example citation event (SSE):**
```json
{
  "event": "citation",
  "data": {
    "index": 1,
    "document_title": "product_manual.pdf",
    "content": "To connect to Wi-Fi, press the settings button...",
    "relevance_score": 0.952,
    "tokens": 156
  }
}
```

## Attachment

### To Conversations
```typescript
// Select KBs for this chat
await attachKnowledgeBases(conversationId, [kb1, kb2]);
```

### To Agents
```yaml
# In agent configuration
knowledgeBases:
  - kb_product_docs
  - kb_technical_specs
```

Agent will automatically use these KBs in every conversation.

## Vector Search

**Technology**: USearch (HNSW algorithm)
**Performance**: <1ms per query
**Embedding**: OpenAI text-embedding-3-small/large
**Storage**: SQLite for metadata, USearch for vectors
```

---

## Documentation Files to Update

### HIGH PRIORITY - Core Features

1. **README.md** - Add all Phase 1-10 features
2. **docs/embedded-api-reference.md** - Add agents, knowledge, actions endpoints
3. **docs/api-reference.md** - Update with complete REST API
4. **docs/streaming-api.md** - Add citation events, update event types

### MEDIUM PRIORITY - Feature Guides

5. **docs/context-window-management.md** - Expand with 4 strategies
6. Create **docs/features/agents.md** - Agent system guide
7. Create **docs/features/artifacts.md** - UI artifacts guide
8. Create **docs/features/knowledge-bases.md** - RAG setup guide
9. Create **docs/features/mcp-tools.md** - MCP integration guide
10. Create **docs/features/actions.md** - Browser + filesystem guide

### LOW PRIORITY - Configuration & Architecture

11. **docs/providers-models.md** - Update with dual-mode model selection
12. Create **docs/configuration/chunking-strategies.md** - Complete chunking guide
13. **docs/rust-architecture.md** - Update with Phase 1-10 components
14. Create **docs/architecture/vector-store.md** - USearch + SQLite design
15. **docs/multi-agent-workflow-architecture.md** - Add Scientific/Exploratory strategies

---

## Implementation Checklist

### Phase 1: README Updates
- [ ] Add "Feature Highlights" section
- [ ] Update architecture diagram
- [ ] Add deployment mode feature matrix
- [ ] Update quick start with dual-mode example
- [ ] Add links to new feature docs

### Phase 2: API Reference Updates
- [ ] Update embedded-api-reference.md with all endpoints
- [ ] Add agents API section
- [ ] Add knowledge API section
- [ ] Add actions API section
- [ ] Add settings v2 API section
- [ ] Update event types with citations

### Phase 3: Feature Documentation
- [ ] Create dual-mode-chat.md
- [ ] Create agents.md with 12 templates
- [ ] Create artifacts.md with 11 types
- [ ] Create knowledge-bases.md with chunking strategies
- [ ] Create mcp-tools.md
- [ ] Create actions.md

### Phase 4: Configuration Guides
- [ ] Update context-window-management.md
- [ ] Create chunking-strategies.md
- [ ] Update providers-models.md
- [ ] Create rag-setup.md guide

### Phase 5: Architecture Documentation
- [ ] Update rust-architecture.md
- [ ] Create vector-store.md
- [ ] Update multi-agent-workflow-architecture.md
- [ ] Add Phase 1-10 component diagrams

---

## Documentation Quality Standards

### Writing Style
- **Clear and concise**: No jargon without explanation
- **Example-driven**: Code samples for every feature
- **Progressive disclosure**: Start simple, add complexity
- **Cross-referenced**: Link related docs
- **Up-to-date**: Reflect current implementation

### Structure
- **Overview**: What is this feature?
- **When to use**: Use cases and scenarios
- **How it works**: Technical explanation
- **Configuration**: All options with defaults
- **API Reference**: Complete endpoint docs
- **Examples**: Real-world usage
- **Troubleshooting**: Common issues

### Code Examples
- Include both REST API and SDK examples
- Show complete, runnable code
- Include expected responses
- Cover error cases

---

## Estimated Effort

**Total documentation updates**: 20-25 files
**New documentation**: 10-12 files
**Estimated time**: 3-4 sessions to complete all documentation

---

## Success Criteria

- [ ] README.md reflects all 10 phases
- [ ] All API endpoints documented
- [ ] All features have dedicated guides
- [ ] Configuration options explained
- [ ] Architecture diagrams updated
- [ ] Examples are complete and tested
- [ ] Cross-references are accurate
- [ ] Ready for documentation website deployment

---

**Next Step**: Begin documentation updates in code mode, starting with README.md and high-priority API references.
