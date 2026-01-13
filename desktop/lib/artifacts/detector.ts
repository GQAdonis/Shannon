/**
 * Artifact Detector
 *
 * Detects and extracts artifacts from LLM responses supporting:
 * - Cherry Studio format
 * - Anthropic Claude format
 * - A2UI protocol
 */

import { Artifact, ArtifactType, ArtifactPattern, ArtifactMetadata } from './types';
import { generateId } from '../utils';

export class ArtifactDetector {
  private patterns: ArtifactPattern[];

  constructor() {
    this.patterns = [
      this.cherryStudioPattern(),
      this.claudePattern(),
      this.a2uiPattern(),
      this.standardCodeBlockPattern(),
    ];
  }

  /**
   * Detect all artifacts in content
   */
  detect(
    content: string,
    messageId: string,
    conversationId: string
  ): Artifact[] {
    const artifacts: Artifact[] = [];
    const timestamp = new Date().toISOString();

    for (const pattern of this.patterns) {
      const matches = content.matchAll(pattern.regex);

      for (const match of matches) {
        try {
          const extracted = pattern.extract(match);

          const artifact: Artifact = {
            id: extracted.id || generateId('artifact'),
            type: extracted.type || 'code',
            title: extracted.title || 'Untitled Artifact',
            content: extracted.content || '',
            metadata: extracted.metadata || {},
            messageId,
            conversationId,
            createdAt: timestamp,
            updatedAt: timestamp,
          };

          artifacts.push(artifact);
        } catch (error) {
          console.error(`Failed to extract artifact from ${pattern.name}:`, error);
        }
      }
    }

    return artifacts;
  }

  /**
   * Cherry Studio format pattern
   * ```artifact type="react" title="My Component" language="typescript"
   * content here
   * ```
   */
  private cherryStudioPattern(): ArtifactPattern {
    return {
      name: 'Cherry Studio',
      regex: /```artifact\s+type="([^"]+)"(?:\s+title="([^"]+)")?(?:\s+language="([^"]+)")?(?:\s+([^`\n]+))?\n([\s\S]*?)```/g,
      extract: (match) => {
        const [, type, title, language, otherAttrs, content] = match;

        // Parse additional attributes
        const metadata: Record<string, string | undefined> = { language };
        if (otherAttrs) {
          const attrMatches = otherAttrs.matchAll(/(\w+)="([^"]+)"/g);
          for (const [, key, value] of attrMatches) {
            metadata[key] = value;
          }
        }

        return {
          type: type as ArtifactType,
          title: title || 'Untitled',
          content: content.trim(),
          metadata: metadata as unknown as ArtifactMetadata,
        };
      },
    };
  }

  /**
   * Anthropic Claude format pattern
   * <antArtifact identifier="unique-id" type="text/html" title="My Page">
   * content here
   * </antArtifact>
   */
  private claudePattern(): ArtifactPattern {
    return {
      name: 'Anthropic Claude',
      regex: /<antArtifact\s+identifier="([^"]+)"\s+type="([^"]+)"(?:\s+title="([^"]+)")?\s*>([\s\S]*?)<\/antArtifact>/g,
      extract: (match) => {
        const [, identifier, type, title, content] = match;

        // Map Claude MIME types to artifact types
        const artifactType = this.mapClaudeType(type);
        const language = this.inferLanguage(type, content);

        return {
          id: identifier,
          type: artifactType,
          title: title || 'Untitled',
          content: content.trim(),
          metadata: { language },
        };
      },
    };
  }

  /**
   * A2UI protocol pattern
   * [A2UI:react:My Component]
   * content here
   * [/A2UI]
   */
  private a2uiPattern(): ArtifactPattern {
    return {
      name: 'A2UI Protocol',
      regex: /\[A2UI:([^:]+):([^\]]+)\]([\s\S]*?)\[\/A2UI\]/g,
      extract: (match) => {
        const [, type, title, content] = match;

        return {
          type: type as ArtifactType,
          title: title.trim(),
          content: content.trim(),
          metadata: {},
        };
      },
    };
  }

  /**
   * Standard code block pattern (fallback)
   * ```typescript
   * code here
   * ```
   */
  private standardCodeBlockPattern(): ArtifactPattern {
    return {
      name: 'Standard Code Block',
      regex: /```(\w+)\n([\s\S]*?)```/g,
      extract: (match) => {
        const [, language, content] = match;

        // Only create artifacts for specific languages
        const artifactLanguages = ['typescript', 'javascript', 'python', 'html', 'react', 'tsx', 'jsx'];
        if (!artifactLanguages.includes(language.toLowerCase())) {
          return {}; // Skip
        }

        const type = this.inferTypeFromLanguage(language);

        return {
          type,
          title: `${language} Code`,
          content: content.trim(),
          metadata: { language },
        };
      },
    };
  }

  /**
   * Map Claude MIME type to artifact type
   */
  private mapClaudeType(mimeType: string): ArtifactType {
    const mapping: Record<string, ArtifactType> = {
      'text/html': 'html',
      'application/vnd.ant.react': 'react',
      'image/svg+xml': 'svg',
      'text/markdown': 'markdown',
      'application/vnd.ant.mermaid': 'mermaid',
      'application/pdf': 'pdf',
      'image/*': 'image',
      'video/*': 'video',
      'audio/*': 'audio',
    };

    for (const [key, value] of Object.entries(mapping)) {
      if (mimeType.includes(key) || key.includes('*') && mimeType.startsWith(key.split('/')[0])) {
        return value;
      }
    }

    return 'code';
  }

  /**
   * Infer language from MIME type or content
   */
  private inferLanguage(mimeType: string, content: string): string {
    if (mimeType.includes('javascript')) return 'javascript';
    if (mimeType.includes('typescript')) return 'typescript';
    if (mimeType.includes('python')) return 'python';
    if (mimeType.includes('html')) return 'html';

    // Analyze content for language hints
    if (content.includes('import React') || content.includes('jsx')) return 'typescript';
    if (content.includes('def ') || content.includes('import ')) return 'python';

    return 'javascript';
  }

  /**
   * Infer artifact type from language
   */
  private inferTypeFromLanguage(language: string): ArtifactType {
    const mapping: Record<string, ArtifactType> = {
      'react': 'react',
      'tsx': 'react',
      'jsx': 'react',
      'html': 'html',
      'svg': 'svg',
      'mermaid': 'mermaid',
      'markdown': 'markdown',
      'md': 'markdown',
    };

    return mapping[language.toLowerCase()] || 'code';
  }

  /**
   * Check if content contains any artifacts
   */
  hasArtifacts(content: string): boolean {
    return this.patterns.some(pattern => pattern.regex.test(content));
  }

  /**
   * Extract only artifact IDs from content
   */
  extractArtifactIds(content: string): string[] {
    const artifacts = this.detect(content, '', '');
    return artifacts.map(a => a.id);
  }
}

/**
 * Singleton instance
 */
export const artifactDetector = new ArtifactDetector();
