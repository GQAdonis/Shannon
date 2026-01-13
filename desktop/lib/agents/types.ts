/**
 * Agent repository types for Shannon Desktop.
 *
 * Defines agent specifications, model configurations, and filtering options.
 */

/**
 * Model configuration for an agent.
 */
export interface ModelConfig {
  /** Provider name (e.g., "openai", "anthropic", "google") */
  provider: string;
  /** Model name (e.g., "gpt-4", "claude-3-opus") */
  name: string;
  /** Temperature parameter (0.0-2.0) */
  temperature?: number;
  /** Maximum tokens to generate */
  maxTokens?: number;
}

/**
 * Complete agent specification.
 */
export interface AgentSpec {
  /** Unique agent ID */
  id: string;
  /** Agent name */
  name: string;
  /** Agent description */
  description: string;
  /** Version string (e.g., "1.0.0") */
  version: string;
  /** Author name (optional) */
  author?: string;

  /** System prompt for the agent */
  systemPrompt: string;
  /** Model configuration */
  model: ModelConfig;

  /** MCP tool IDs enabled for this agent */
  tools: string[];
  /** Knowledge base IDs for RAG */
  knowledgeBases: string[];
  /** Allowed actions (browser, filesystem, etc.) */
  allowedActions: string[];

  /** Default workflow strategy */
  strategy?: string;
  /** Conversation style */
  conversationStyle?: 'formal' | 'casual' | 'technical';

  /** Tags for categorization */
  tags: string[];
  /** Category (e.g., "general", "code", "research") */
  category: string;
  /** Icon identifier (emoji or icon name) */
  icon?: string;

  /** When the agent was created (ISO 8601) */
  createdAt: string;
  /** When the agent was last updated (ISO 8601) */
  updatedAt: string;
}

/**
 * Filter for listing agents.
 */
export interface AgentFilter {
  /** Filter by category */
  category?: string;
  /** Filter by tags (any match) */
  tags?: string[];
  /** Search in name and description */
  search?: string;
}

/**
 * Agent categories for organization.
 */
export const AGENT_CATEGORIES = [
  'general',
  'code',
  'research',
  'creative',
  'business',
  'education',
  'health',
  'legal',
  'support',
] as const;

export type AgentCategory = typeof AGENT_CATEGORIES[number];

/**
 * Conversation styles for agents.
 */
export const CONVERSATION_STYLES = [
  'formal',
  'casual',
  'technical',
] as const;

export type ConversationStyle = typeof CONVERSATION_STYLES[number];

/**
 * Workflow strategies available.
 */
export const WORKFLOW_STRATEGIES = [
  'chain_of_thought',
  'scientific',
  'exploratory',
  'debate',
  'tree_of_thoughts',
] as const;

export type WorkflowStrategy = typeof WORKFLOW_STRATEGIES[number];
