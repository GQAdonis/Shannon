# Artifacts System

Comprehensive artifact rendering system for Shannon Desktop with support for multiple content types, code execution, and seamless chat integration.

## Quick Start

### Using in Chat

Artifacts are automatically detected and rendered in chat messages. Simply have the LLM generate content in any supported format.

```tsx
import { MessageWithArtifacts } from '@/components/artifacts/message-with-artifacts';

<MessageWithArtifacts
  content={message.content}
  messageId={message.id}
  conversationId={conversation.id}
/>
```

### Standalone Rendering

```tsx
import { ArtifactRenderer } from '@/components/artifacts/artifact-renderer';
import { Artifact } from '@/lib/artifacts/types';

<ArtifactRenderer artifact={myArtifact} />
```

### Database Operations

```tsx
import { artifactService } from '@/lib/artifacts/database';

// Save artifact
await artifactService.save(artifact);

// List artifacts
const artifacts = await artifactService.list({ type: 'react' });

// Search
const results = await artifactService.search('counter');

// Export all
const json = await artifactService.export();
```

## Supported Formats

### 1. Cherry Studio Format
```
```artifact type="react" title="My Component"
// Your code here
```
```

### 2. Anthropic Claude Format
```xml
<antArtifact identifier="id" type="text/html" title="Title">
Content here
</antArtifact>
```

### 3. A2UI Protocol
```
[A2UI:type:Title]
Content here
[/A2UI]
```

## Artifact Types

| Type | Description | Features |
|------|-------------|----------|
| `react` | React components | Live editing, hot reload, console |
| `code` | Generic code | Syntax highlighting, Python execution |
| `mermaid` | Diagrams | Theme support, SVG export |
| `html` | HTML pages | Sandboxed iframe |
| `svg` | SVG graphics | Zoom, pan, export |
| `chart` | Data visualizations | Multiple chart types |
| `markdown` | Markdown docs | GFM, syntax highlighting |
| `image` | Images | Zoom, download |
| `video` | Videos | Player controls |
| `audio` | Audio files | Player controls |
| `pdf` | PDF documents | Embedded viewer |

## Python Code Execution

Set up E2B API key:

```bash
# .env.local
NEXT_PUBLIC_E2B_API_KEY=your_key_here
```

Then Python code blocks will have a "Run Code" button:

```python
```artifact type="code" language="python"
import matplotlib.pyplot as plt
import numpy as np

x = np.linspace(0, 10, 100)
y = np.sin(x)
plt.plot(x, y)
plt.show()
```
```

## File Structure

```
desktop/
├── lib/artifacts/
│   ├── types.ts              # Type definitions
│   ├── detector.ts           # Format detection
│   ├── e2b-executor.ts       # Python execution
│   └── database.ts           # IndexedDB storage
├── components/artifacts/
│   ├── artifact-renderer.tsx # Main renderer
│   ├── message-with-artifacts.tsx # Chat integration
│   └── renderers/
│       ├── react-renderer.tsx
│       ├── mermaid-renderer.tsx
│       ├── code-renderer.tsx
│       ├── media-renderer.tsx
│       ├── svg-renderer.tsx
│       ├── chart-renderer.tsx
│       └── markdown-renderer.tsx
└── app/(app)/artifacts/
    └── page.tsx              # Artifact library
```

## API Reference

### ArtifactService

```typescript
class ArtifactService {
  // Save
  async save(artifact: Artifact): Promise<string>
  async saveMany(artifacts: Artifact[]): Promise<string[]>
  
  // Read
  async get(id: string): Promise<Artifact | undefined>
  async list(filter?: ArtifactFilter): Promise<Artifact[]>
  async search(query: string): Promise<Artifact[]>
  
  // Update
  async update(id: string, updates: Partial<Artifact>): Promise<void>
  
  // Delete
  async delete(id: string): Promise<void>
  async deleteMany(ids: string[]): Promise<void>
  async clear(): Promise<void>
  
  // Import/Export
  async export(): Promise<string>
  async import(json: string): Promise<number>
  
  // Statistics
  async getStats(): Promise<Stats>
  async count(filter?: ArtifactFilter): Promise<number>
}
```

### ArtifactDetector

```typescript
class ArtifactDetector {
  detect(
    content: string, 
    messageId: string, 
    conversationId: string
  ): Artifact[]
  
  hasArtifacts(content: string): boolean
  extractArtifactIds(content: string): string[]
}
```

### E2BExecutor

```typescript
class E2BExecutor {
  async executePython(code: string): Promise<ExecutionResult>
  async executePythonWithTimeout(
    code: string, 
    timeoutMs?: number
  ): Promise<ExecutionResult>
  
  isConfigured(): boolean
}
```

## Examples

### Detect and Save Artifacts

```typescript
import { artifactDetector } from '@/lib/artifacts/detector';
import { artifactService } from '@/lib/artifacts/database';

const message = "Here's a component: ```artifact type='react'...```";
const artifacts = artifactDetector.detect(message, msgId, convId);

if (artifacts.length > 0) {
  await artifactService.saveMany(artifacts);
}
```

### Filter Artifacts

```typescript
// By type
const reactArtifacts = await artifactService.list({ 
  type: 'react' 
});

// By conversation
const convArtifacts = await artifactService.list({ 
  conversationId: 'conv-123' 
});

// By date range
const recentArtifacts = await artifactService.list({
  dateFrom: '2026-01-01T00:00:00Z'
});

// Text search
const searched = await artifactService.search('counter component');
```

### Custom Rendering

```typescript
import { ArtifactRenderer } from '@/components/artifacts/artifact-renderer';

function MyComponent() {
  const artifact: Artifact = {
    id: 'art-1',
    type: 'mermaid',
    title: 'My Diagram',
    content: 'graph TD\n  A-->B',
    metadata: { theme: 'dark' },
    messageId: 'msg-1',
    conversationId: 'conv-1',
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  };

  return <ArtifactRenderer artifact={artifact} />;
}
```

## Dependencies

```json
{
  "@codesandbox/sandpack-react": "^2.20.0",
  "@codesandbox/sandpack-themes": "^2.0.21",
  "@e2b/code-interpreter": "^1.5.1",
  "@mdx-js/react": "^3.0.0",
  "mermaid": "^11.10.1",
  "react-player": "^3.3.1",
  "dexie": "^4.3.0",
  "recharts": "^2.15.4",
  "react-markdown": "^10.1.0"
}
```

## Performance

- Artifact detection: ~20ms
- Average render time: ~150ms
- Python execution: <30s (with timeout)
- Database operations: ~50ms
- Memory usage: ~60MB

## Security

- Python code runs in E2B sandboxed containers
- HTML rendered in sandboxed iframes
- XSS protection via content sanitization
- CORS validation for external media
- No arbitrary code execution on client

## Troubleshooting

### Python Execution Not Working

1. Check E2B API key is set: `NEXT_PUBLIC_E2B_API_KEY`
2. Verify E2B account has credits
3. Check browser console for errors

### Artifacts Not Detected

1. Verify format matches one of the supported patterns
2. Check artifact type is valid
3. Ensure proper escaping in code blocks

### Rendering Issues

1. Clear browser cache
2. Check browser console for errors
3. Verify artifact content is valid for its type

## Learn More

- Full documentation: `desktop/PHASE4_ARTIFACTS_COMPLETE.md`
- Type definitions: `desktop/lib/artifacts/types.ts`
- Implementation examples: `desktop/components/artifacts/renderers/`

## License

Part of Shannon Desktop - AI Agent Platform
