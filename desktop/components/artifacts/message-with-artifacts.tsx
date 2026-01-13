/**
 * Message with Artifacts Component
 *
 * Displays chat messages and automatically detects and renders artifacts
 */

'use client';

import { useEffect, useMemo, useState } from 'react';
import { artifactDetector } from '@/lib/artifacts/detector';
import { artifactService } from '@/lib/artifacts/database';
import { Artifact } from '@/lib/artifacts/types';
import { ArtifactRenderer } from './artifact-renderer';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

interface MessageWithArtifactsProps {
  content: string;
  messageId: string;
  conversationId: string;
  className?: string;
}

export function MessageWithArtifacts({
  content,
  messageId,
  conversationId,
  className,
}: MessageWithArtifactsProps) {
  const [artifacts, setArtifacts] = useState<Artifact[]>([]);

  // Detect artifacts from content (pure computation)
  const detectedArtifacts = useMemo(() => {
    return artifactDetector.detect(content, messageId, conversationId);
  }, [content, messageId, conversationId]);

  // Clean content by removing artifact markers
  const cleanContent = useMemo(() => {
    if (detectedArtifacts.length === 0) {
      return content;
    }

    let cleaned = content;

    // Remove Cherry Studio format
    cleaned = cleaned.replace(/```artifact[^`]*```/g, '');

    // Remove Claude format
    cleaned = cleaned.replace(/<antArtifact[^>]*>[\s\S]*?<\/antArtifact>/g, '');

    // Remove A2UI format
    cleaned = cleaned.replace(/\[A2UI:[^\]]+\][\s\S]*?\[\/A2UI\]/g, '');

    return cleaned.trim();
  }, [content, detectedArtifacts.length]);

  // Save artifacts to database (side effect)
  useEffect(() => {
    if (detectedArtifacts.length > 0) {
      artifactService.saveMany(detectedArtifacts)
        .then(() => {
          setArtifacts(detectedArtifacts);
        })
        .catch((error) => {
          console.error('Failed to save artifacts:', error);
          // Still show artifacts even if saving fails
          setArtifacts(detectedArtifacts);
        });
    }
  }, [detectedArtifacts]);

  // Display detected artifacts immediately, even before they're saved
  const displayArtifacts = detectedArtifacts.length > 0 ? detectedArtifacts : artifacts;

  return (
    <div className={className}>
      {/* Regular message content */}
      {cleanContent && (
        <div className="prose prose-sm dark:prose-invert max-w-none mb-4">
          <ReactMarkdown remarkPlugins={[remarkGfm]}>
            {cleanContent}
          </ReactMarkdown>
        </div>
      )}

      {/* Rendered artifacts */}
      {displayArtifacts.length > 0 && (
        <div className="space-y-4 mt-4">
          {displayArtifacts.map((artifact) => (
            <ArtifactRenderer key={artifact.id} artifact={artifact} />
          ))}
        </div>
      )}
    </div>
  );
}
