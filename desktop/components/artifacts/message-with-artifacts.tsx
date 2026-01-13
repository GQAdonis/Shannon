/**
 * Message with Artifacts Component
 *
 * Displays chat messages and automatically detects and renders artifacts
 */

'use client';

import { useEffect, useState } from 'react';
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
  const [cleanContent, setCleanContent] = useState(content);

  const detectAndSaveArtifacts = async () => {
    // Detect artifacts in message content
    const detected = artifactDetector.detect(content, messageId, conversationId);

    if (detected.length > 0) {
      // Save artifacts to database
      try {
        await artifactService.saveMany(detected);
        setArtifacts(detected);

        // Remove artifact markers from content for cleaner display
        let cleaned = content;

        // Remove Cherry Studio format
        cleaned = cleaned.replace(/```artifact[^`]*```/g, '');

        // Remove Claude format
        cleaned = cleaned.replace(/<antArtifact[^>]*>[\s\S]*?<\/antArtifact>/g, '');

        // Remove A2UI format
        cleaned = cleaned.replace(/\[A2UI:[^\]]+\][\s\S]*?\[\/A2UI\]/g, '');

        setCleanContent(cleaned.trim());
      } catch (error) {
        console.error('Failed to save artifacts:', error);
        // Still show artifacts even if saving fails
        setArtifacts(detected);
      }
    } else {
      setCleanContent(content);
    }
  };

  useEffect(() => {
    detectAndSaveArtifacts();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [content, messageId, conversationId]);

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
      {artifacts.length > 0 && (
        <div className="space-y-4 mt-4">
          {artifacts.map((artifact) => (
            <ArtifactRenderer key={artifact.id} artifact={artifact} />
          ))}
        </div>
      )}
    </div>
  );
}
