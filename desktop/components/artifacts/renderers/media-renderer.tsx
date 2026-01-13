/**
 * Media Renderer for Video, Images, and Audio
 *
 * Renders media artifacts with controls and download capabilities
 */

'use client';

import { useState } from 'react';
import ReactPlayer from 'react-player';
import { Artifact } from '@/lib/artifacts/types';
import { Button } from '@/components/ui/button';
import { Download, Maximize2, AlertCircle } from 'lucide-react';
import { toast } from 'sonner';

interface MediaRendererProps {
  artifact: Artifact;
}

export function MediaRenderer({ artifact }: MediaRendererProps) {
  const [error, setError] = useState<string | null>(null);
  const [isFullscreen, setIsFullscreen] = useState(false);

  const isVideo = artifact.type === 'video';
  const isAudio = artifact.type === 'audio';
  const isImage = artifact.type === 'image';

  const handleDownload = () => {
    const link = document.createElement('a');
    link.href = artifact.content;
    link.download = artifact.title;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    toast.success('Media download started');
  };

  const handleFullscreen = () => {
    setIsFullscreen(!isFullscreen);
  };

  const handleError = () => {
    setError('Failed to load media file');
    toast.error('Failed to load media');
  };

  if (error) {
    return (
      <div className="media-renderer border rounded-lg p-4">
        <div className="flex items-start gap-3 p-4 bg-destructive/10 text-destructive rounded-md">
          <AlertCircle className="h-5 w-5 mt-0.5 flex-shrink-0" />
          <div>
            <p className="font-medium">Failed to load media</p>
            <p className="text-sm mt-1">{error}</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="media-renderer border rounded-lg overflow-hidden">
      <div className="bg-muted px-4 py-2 border-b flex items-center justify-between">
        <h4 className="font-medium text-sm">{artifact.title}</h4>
        <div className="flex items-center gap-2">
          {(isVideo || isImage) && (
            <Button
              variant="ghost"
              size="sm"
              onClick={handleFullscreen}
              title="Toggle fullscreen"
            >
              <Maximize2 className="h-4 w-4" />
            </Button>
          )}
          <Button
            variant="ghost"
            size="sm"
            onClick={handleDownload}
            title="Download media"
          >
            <Download className="h-4 w-4" />
          </Button>
        </div>
      </div>

      <div className="bg-background p-4">
        {isImage && (
          <div className={isFullscreen ? 'fixed inset-0 z-50 bg-black flex items-center justify-center' : ''}>
            {isFullscreen && (
              <Button
                variant="ghost"
                size="sm"
                onClick={handleFullscreen}
                className="absolute top-4 right-4 text-white hover:bg-white/20"
              >
                Close
              </Button>
            )}
            <img
              src={artifact.content}
              alt={artifact.title}
              className={isFullscreen ? 'max-w-full max-h-full object-contain' : 'max-w-full h-auto rounded-lg'}
              onError={handleError}
              style={{
                maxWidth: artifact.metadata.width || '100%',
                maxHeight: artifact.metadata.height || 'auto',
              }}
            />
          </div>
        )}

        {isVideo && (
          <div className={isFullscreen ? 'fixed inset-0 z-50 bg-black flex items-center justify-center' : 'aspect-video'}>
            {isFullscreen && (
              <Button
                variant="ghost"
                size="sm"
                onClick={handleFullscreen}
                className="absolute top-4 right-4 z-10 text-white hover:bg-white/20"
              >
                Close
              </Button>
            )}
            <ReactPlayer
              src={artifact.content}
              controls
              width="100%"
              height="100%"
              onError={handleError}
            />
          </div>
        )}

        {isAudio && (
          <div className="w-full">
            <ReactPlayer
              src={artifact.content}
              controls
              width="100%"
              height="50px"
              onError={handleError}
            />
          </div>
        )}
      </div>

      {artifact.metadata.description && (
        <div className="border-t px-4 py-2 text-sm text-muted-foreground">
          {artifact.metadata.description}
        </div>
      )}
    </div>
  );
}
