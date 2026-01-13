/**
 * @-mention parser for referencing files, agents, knowledge bases, and tools
 */

export type MentionType = 'file' | 'agent' | 'knowledge' | 'tool';

export interface Mention {
  type: MentionType;
  id: string;
  name: string;
  content?: string; // For files
}

export class MentionParser {
  /**
   * Parse @-mentions from text
   * Format: @type:identifier
   * Examples:
   * - @file:src/main.ts
   * - @agent:research-agent
   * - @knowledge:project-docs
   * - @tool:web-search
   */
  static parse(text: string): Mention[] {
    const mentions: Mention[] = [];

    // Match @type:identifier pattern
    const pattern = /@(file|agent|knowledge|tool):([^\s,]+)/g;

    let match: RegExpExecArray | null;
    // biome-ignore lint/suspicious/noAssignInExpressions: Required for regex exec pattern
    while ((match = pattern.exec(text)) !== null) {
      mentions.push({
        type: match[1] as MentionType,
        id: match[2],
        name: match[2],
      });
    }

    return mentions;
  }

  /**
   * Extract text without mentions
   */
  static stripMentions(text: string): string {
    return text.replace(/@(file|agent|knowledge|tool):[^\s,]+/g, '').trim();
  }

  /**
   * Check if text contains mentions
   */
  static hasMentions(text: string): boolean {
    return /@(file|agent|knowledge|tool):[^\s,]+/.test(text);
  }

  /**
   * Replace mentions with formatted text
   */
  static formatMentions(text: string, formatter: (mention: Mention) => string): string {
    return text.replace(/@(file|agent|knowledge|tool):([^\s,]+)/g, (_, type, id) => {
      return formatter({
        type: type as MentionType,
        id,
        name: id,
      });
    });
  }
}
