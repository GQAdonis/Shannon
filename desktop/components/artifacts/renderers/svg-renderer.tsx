/**
 * SVG Renderer
 *
 * Renders interactive SVG graphics with zoom and export capabilities
 */

'use client';

import { useState, useRef } from 'react';
import { Artifact } from '@/lib/artifacts/types';
import { Button } from '@/components/ui/button';
import { Download, Copy, Maximize2, ZoomIn, ZoomOut, AlertCircle } from 'lucide-react';
import { toast } from 'sonner';

interface SVGRendererProps {
  artifact: Artifact;
}

export function SVGRenderer({ artifact }: SVGRendererProps) {
  const [error, setError] = useState<string | null>(null);
  const [zoom, setZoom] = useState(1);
  const svgRef = useRef<HTMLDivElement>(null);

  const handleExport = () => {
    const blob = new Blob([artifact.content], { type: 'image/svg+xml' });
    const url = URL.createObjectURL(blob);

    const link = document.createElement('a');
    link.href = url;
    link.download = `${artifact.title.replace(/\s+/g, '-')}.svg`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);

    toast.success('SVG exported');
  };

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(artifact.content);
      toast.success('SVG code copied to clipboard');
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

  const handleZoomIn = () => {
    setZoom((prev) => Math.min(prev + 0.2, 3));
  };

  const handleZoomOut = () => {
    setZoom((prev) => Math.max(prev - 0.2, 0.5));
  };

  const handleError = () => {
    setError('Failed to render SVG');
  };

  if (error) {
    return (
      <div className="svg-renderer border rounded-lg p-4">
        <div className="flex items-start gap-3 p-4 bg-destructive/10 text-destructive rounded-md">
          <AlertCircle className="h-5 w-5 mt-0.5 flex-shrink-0" />
          <div>
            <p className="font-medium">Failed to render SVG</p>
            <p className="text-sm mt-1">{error}</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="svg-renderer border rounded-lg overflow-hidden">
      <div className="bg-muted px-4 py-2 border-b flex items-center justify-between">
        <h4 className="font-medium text-sm">{artifact.title}</h4>
        <div className="flex items-center gap-2">
          <Button
            variant="ghost"
            size="sm"
            onClick={handleZoomOut}
            title="Zoom out"
          >
            <ZoomOut className="h-4 w-4" />
          </Button>
          <span className="text-xs text-muted-foreground min-w-12 text-center">
            {Math.round(zoom * 100)}%
          </span>
          <Button
            variant="ghost"
            size="sm"
            onClick={handleZoomIn}
            title="Zoom in"
          >
            <ZoomIn className="h-4 w-4" />
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onClick={handleCopy}
            title="Copy SVG code"
          >
            <Copy className="h-4 w-4" />
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onClick={handleExport}
            title="Export as SVG"
          >
            <Download className="h-4 w-4" />
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onClick={handleFullscreen}
            title="Fullscreen"
          >
            <Maximize2 className="h-4 w-4" />
          </Button>
        </div>
      </div>

      <div
        ref={svgRef}
        className="bg-background p-4 overflow-auto flex items-center justify-center min-h-[200px]"
        style={{
          maxHeight: artifact.metadata.height || 600,
        }}
      >
        <div
          style={{
            transform: `scale(${zoom})`,
            transformOrigin: 'center',
            transition: 'transform 0.2s ease-out',
          }}
          dangerouslySetInnerHTML={{ __html: artifact.content }}
          data-security="sandboxed-svg"
          onError={handleError}
        />
      </div>

      {artifact.metadata.description && (
        <div className="border-t px-4 py-2 text-sm text-muted-foreground">
          {artifact.metadata.description}
        </div>
      )}
    </div>
  );
}
