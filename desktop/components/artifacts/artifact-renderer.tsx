/**
 * Artifact Renderer
 *
 * Main component that routes to specific renderers based on artifact type
 */

'use client';

import { Artifact } from '@/lib/artifacts/types';
import { ReactRenderer } from './renderers/react-renderer';
import { MermaidRenderer } from './renderers/mermaid-renderer';
import { CodeRenderer } from './renderers/code-renderer';
import { MediaRenderer } from './renderers/media-renderer';
import { SVGRenderer } from './renderers/svg-renderer';
import { ChartRenderer } from './renderers/chart-renderer';
import { MarkdownRenderer } from './renderers/markdown-renderer';
import { AlertCircle } from 'lucide-react';

interface ArtifactRendererProps {
  artifact: Artifact;
  editable?: boolean;
  onCodeChange?: (code: string) => void;
}

export function ArtifactRenderer({ artifact, editable, onCodeChange }: ArtifactRendererProps) {
  switch (artifact.type) {
    case 'react':
      return <ReactRenderer artifact={artifact} editable={editable} onCodeChange={onCodeChange} />;

    case 'mermaid':
      return <MermaidRenderer artifact={artifact} />;

    case 'code':
      return <CodeRenderer artifact={artifact} />;

    case 'html':
      // HTML can be rendered in an iframe for safety
      return (
        <div className="border rounded-lg overflow-hidden">
          <div className="bg-muted px-4 py-2 border-b">
            <h4 className="font-medium text-sm">{artifact.title}</h4>
          </div>
          <iframe
            title={artifact.title}
            srcDoc={artifact.content}
            className="w-full h-96 bg-background"
            sandbox="allow-scripts allow-same-origin"
          />
        </div>
      );

    case 'svg':
      return <SVGRenderer artifact={artifact} />;

    case 'chart':
      return <ChartRenderer artifact={artifact} />;

    case 'markdown':
      return <MarkdownRenderer artifact={artifact} />;

    case 'image':
    case 'video':
    case 'audio':
      return <MediaRenderer artifact={artifact} />;

    case 'pdf':
      // PDF rendering
      return (
        <div className="border rounded-lg overflow-hidden">
          <div className="bg-muted px-4 py-2 border-b">
            <h4 className="font-medium text-sm">{artifact.title}</h4>
          </div>
          <iframe
            title={artifact.title}
            src={artifact.content}
            className="w-full h-[600px] bg-background"
          />
        </div>
      );

    default:
      return (
        <div className="border rounded-lg p-4">
          <div className="flex items-start gap-3 p-4 bg-muted rounded-md">
            <AlertCircle className="h-5 w-5 mt-0.5 flex-shrink-0" />
            <div>
              <p className="font-medium">Unsupported artifact type</p>
              <p className="text-sm mt-1 text-muted-foreground">
                Type: {artifact.type}
              </p>
            </div>
          </div>
        </div>
      );
  }
}
