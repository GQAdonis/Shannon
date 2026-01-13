import { invoke } from '@tauri-apps/api/core';
import type { ChatMode } from './types';

export interface ModeDetection {
  mode: ChatMode;
  confidence: number;
  reason?: string;
}

/**
 * Service for detecting appropriate chat mode based on user query
 */
export class ModeDetector {
  /**
   * Detect chat mode using backend AI-powered detection
   *
   * @param query - User query text
   * @returns Detected chat mode
   */
  async detectMode(query: string): Promise<ChatMode> {
    try {
      return await invoke<ChatMode>('detect_chat_mode', { query });
    } catch (error) {
      console.error('Mode detection error:', error);
      // Fallback to client-side detection
      return this.detectModeSync(query).mode;
    }
  }

  /**
   * Client-side heuristic-based mode detection for instant feedback
   *
   * @param query - User query text
   * @returns Detection result with mode, confidence, and reason
   */
  detectModeSync(query: string): ModeDetection {
    const queryLower = query.toLowerCase();
    const wordCount = query.split(/\s+/).length;
    const sentenceCount = query.split(/[.!?]+/).filter(s => s.trim().length > 0).length;

    // Quick chat indicators
    const quickKeywords = [
      'quick', 'simple', 'just', 'what is', 'who is', 'when was',
      'define', 'explain briefly', 'summarize', 'in short'
    ];

    // Complex task indicators
    const complexKeywords = [
      'research', 'analyze', 'compare', 'investigate', 'evaluate',
      'comprehensive', 'detailed', 'multi-step', 'deep dive',
      'thorough', 'examine', 'assess', 'review extensively'
    ];

    // Action verbs suggesting task orchestration
    const taskVerbs = [
      'create', 'build', 'develop', 'design', 'implement',
      'generate', 'write a report', 'produce', 'compile'
    ];

    const hasQuickMarkers = quickKeywords.some(k => queryLower.includes(k));
    const hasComplexMarkers = complexKeywords.some(k => queryLower.includes(k));
    const hasTaskVerbs = taskVerbs.some(k => queryLower.includes(k));

    // Strong quick chat indicators
    if (hasQuickMarkers && wordCount < 30) {
      return {
        mode: 'quick',
        confidence: 0.9,
        reason: 'Quick question keywords detected'
      };
    }

    // Strong task chat indicators
    if (hasComplexMarkers || (hasTaskVerbs && wordCount > 50)) {
      return {
        mode: 'task',
        confidence: 0.85,
        reason: 'Complex analysis or task creation requested'
      };
    }

    // Long, multi-sentence queries likely need orchestration
    if (wordCount > 100 || sentenceCount > 3) {
      return {
        mode: 'task',
        confidence: 0.75,
        reason: 'Long, complex query detected'
      };
    }

    // Questions with multiple sub-questions
    const questionMarks = (query.match(/\?/g) || []).length;
    if (questionMarks > 2) {
      return {
        mode: 'task',
        confidence: 0.7,
        reason: 'Multiple questions require orchestration'
      };
    }

    // Short queries default to quick mode
    if (wordCount < 20) {
      return {
        mode: 'quick',
        confidence: 0.6,
        reason: 'Short query suitable for quick response'
      };
    }

    // Medium-length queries - uncertain, lean toward quick
    return {
      mode: 'quick',
      confidence: 0.5,
      reason: 'Standard query, quick mode recommended'
    };
  }

  /**
   * Batch detect mode for multiple queries
   *
   * @param queries - Array of query strings
   * @returns Array of mode detections
   */
  detectModesSync(queries: string[]): ModeDetection[] {
    return queries.map(q => this.detectModeSync(q));
  }

  /**
   * Check if a query should switch modes based on conversation history
   *
   * @param query - Current query
   * @param currentMode - Current active mode
   * @param conversationLength - Number of messages in conversation
   * @returns Whether to suggest mode switch
   */
  shouldSwitchMode(
    query: string,
    currentMode: ChatMode,
    conversationLength: number
  ): boolean {
    const detection = this.detectModeSync(query);

    // Only suggest switch if confident and different from current
    if (detection.mode !== currentMode && detection.confidence > 0.7) {
      // Don't suggest switches in the middle of long conversations
      if (conversationLength < 3) {
        return true;
      }
    }

    return false;
  }
}
