/**
 * Chat mode types for dual-mode chat system
 */

export type ChatMode = 'quick' | 'task';

export interface ChatMessage {
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: string;
}

export interface QuickChatConfig {
  provider: 'openai' | 'anthropic' | 'google';
  model: string;
  temperature: number;
  maxTokens: number;
  stream: boolean;
}

export interface TaskChatConfig {
  strategy: 'auto' | 'chain_of_thought' | 'scientific' | 'exploratory';
  requireApproval: boolean;
  maxAgents: number;
  tokenBudget: number;
  complexity: 'simple' | 'complex' | 'exploratory';
}

export interface TaskHandle {
  taskId: string;
  workflowId: string;
  state: 'pending' | 'running' | 'completed' | 'failed';
  progress: number;
  message?: string;
}

export interface TaskEventData {
  progress?: number;
  message?: string;
  content?: string;
  error?: string;
  state?: TaskHandle['state'];
}

export interface TaskEvent {
  type: 'state_changed' | 'progress' | 'partial_result' | 'completed' | 'failed';
  taskId: string;
  data: TaskEventData;
}

export const DEFAULT_QUICK_CHAT_CONFIG: QuickChatConfig = {
  provider: 'openai',
  model: 'gpt-4',
  temperature: 0.7,
  maxTokens: 2048,
  stream: true,
};

export const DEFAULT_TASK_CHAT_CONFIG: TaskChatConfig = {
  strategy: 'auto',
  requireApproval: false,
  maxAgents: 3,
  tokenBudget: 10000,
  complexity: 'simple',
};
