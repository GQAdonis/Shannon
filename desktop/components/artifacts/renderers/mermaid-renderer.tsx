/**
 * Mermaid Diagram Renderer
 *
 * Renders Mermaid diagrams with theme support and export capabilities
 */

'use client';

import { useEffect, useRef, useState } from 'react';
import { useTheme } from 'next-themes';
import mermaid from 'mermaid';
import { Artifact } from '@/lib/artifacts/types';
import { Button } from '@/components/ui/button';
import { Download, Copy, Maximize2, AlertCircle } from 'lucide-react';
import { toast } from 'sonner';

interface MermaidRendererProps {
  artifact: Artifact;
}

export function MermaidRenderer({ artifact }: MermaidRendererProps) {
  const { theme } = useTheme();
  const svgRef = useRef<HTMLDivElement>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    if (!svgRef.current) return;

    // Initialize Mermaid with theme
    mermaid.initialize({
      startOnLoad: false,
      theme: artifact.metadata.theme === 'dark' || (artifact.metadata.theme === 'auto' && theme === 'dark')
        ? 'dark'
        : 'default',
      securityLevel: 'loose',
      fontFamily: 'ui-sans-serif, system-ui, sans-serif',
    });

    // Render diagram
    const renderDiagram = async () => {
      try {
        setIsLoading(true);
        setError(null);

        const { svg } = await mermaid.render(
          `mermaid-${artifact.id}`,
          artifact.content
        );

        if (svgRef.current) {
          svgRef.current.innerHTML = svg;
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to render diagram');
        console.error('Mermaid render error:', err);
      } finally {
        setIsLoading(false);
      }
    };

    renderDiagram();
  }, [artifact.content, artifact.id, artifact.metadata.theme, theme]);

  const handleExport = () => {
    if (!svgRef.current) return;

    const svgElement = svgRef.current.querySelector('svg');
    if (!svgElement) return;

    const svgData = new XMLSerializer().serializeToString(svgElement);
    const blob = new Blob([svgData], { type: 'image/svg+xml' });
    const url = URL.createObjectURL(blob);

    const link = document.createElement('a');
    link.href = url;
    link.download = `${artifact.title.replace(/\s+/g, '-')}.svg`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);

    toast.success('Diagram exported as SVG');
  };

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(artifact.content);
      toast.success('Mermaid code copied to clipboard');
    } catch (err) {
      toast.error('Failed to copy to clipboard');
    }
  };

  const handleFullscreen = () => {
    if (!svgRef.current) return;

    if (document.fullscreenElement) {
      document.exitFullscreen();
    } else {
      svgRef.current.requestFullscreen();
    }
  };

  return (
    <div className="mermaid-renderer border rounded-lg overflow-hidden">
      <div className="bg-muted px-4 py-2 border-b flex items-center justify-between">
        <h4 className="font-medium text-sm">{artifact.title}</h4>
        <div className="flex items-center gap-2">
          <Button
            variant="ghost"
            size="sm"
            onClick={handleCopy}
            title="Copy Mermaid code"
          >
            <Copy className="h-4 w-4" />
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onClick={handleExport}
            title="Export as SVG"
            disabled={!!error}
          >
            <Download className="h-4 w-4" />
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onClick={handleFullscreen}
            title="Fullscreen"
            disabled={!!error}
          >
            <Maximize2 className="h-4 w-4" />
          </Button>
        </div>
      </div>

      <div className="p-4 bg-background">
        {isLoading && (
          <div className="flex items-center justify-center py-8">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary" />
          </div>
        )}

        {error && (
          <div className="flex items-start gap-3 p-4 bg-destructive/10 text-destructive rounded-md">
            <AlertCircle className="h-5 w-5 mt-0.5 flex-shrink-0" />
            <div>
              <p className="font-medium">Failed to render diagram</p>
              <p className="text-sm mt-1">{error}</p>
            </div>
          </div>
        )}

        {!isLoading && !error && (
          <div
            ref={svgRef}
            className="flex items-center justify-center min-h-[200px]"
            style={{
              maxWidth: artifact.metadata.width || '100%',
              maxHeight: artifact.metadata.height || 'auto',
            }}
          />
        )}
      </div>
    </div>
  );
}
