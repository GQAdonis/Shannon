/**
 * Context management types for conversation history optimization
 */

export type ContextStrategy =
  | 'sliding_window'
  | 'progressive_summarization'
  | 'hierarchical_memory'
  | 'keep_first_last';

export interface ContextSettings {
  id: string;
  strategy: ContextStrategy;
  shortTermTurns: number;
  midTermBudget: number;
  longTermBudget: number;
  summarizationModel: string;
  createdAt: string;
  updatedAt: string;
}

export const STRATEGY_DESCRIPTIONS: Record<ContextStrategy, string> = {
  sliding_window: 'Keep only recent messages within token budget. Best for real-time conversations with limited history needs.',
  progressive_summarization: 'Summarize older messages progressively. Best for long conversations where history matters but token budget is tight.',
  hierarchical_memory: 'Three-tier system: verbatim recent, summarized mid-term, key facts long-term. Best for complex conversations requiring both detail and long-term memory.',
  keep_first_last: 'Preserve initial instructions + recent context, remove middle. Best for conversations with important system prompts and recent context.',
};

export const STRATEGY_NAMES: Record<ContextStrategy, string> = {
  sliding_window: 'Sliding Window',
  progressive_summarization: 'Progressive Summarization',
  hierarchical_memory: 'Hierarchical Memory',
  keep_first_last: 'Keep First & Last',
};

export const DEFAULT_SETTINGS: Omit<ContextSettings, 'id' | 'createdAt' | 'updatedAt'> = {
  strategy: 'hierarchical_memory',
  shortTermTurns: 5,
  midTermBudget: 2000,
  longTermBudget: 500,
  summarizationModel: 'claude-haiku-4-5@20251001',
};

export const SUMMARIZATION_MODELS = [
  { value: 'claude-haiku-4-5@20251001', label: 'Claude Haiku 4.5 (Recommended)' },
  { value: 'gpt-4o-mini', label: 'GPT-4o Mini' },
  { value: 'gemini-1.5-flash', label: 'Gemini 1.5 Flash' },
];
