/**
 * MCP (Model Context Protocol) type definitions.
 */

export type ServerStatus =
  | { type: 'Stopped' }
  | { type: 'Starting' }
  | { type: 'Running' }
  | { type: 'Error'; message: string };

export interface MCPTool {
  name: string;
  description: string;
  inputSchema: Record<string, unknown>;
}

export interface MCPServerConfig {
  id: string;
  name: string;
  description: string;
  command: string;
  args: string[];
  env: Record<string, string>;
  autoStart: boolean;
  status: ServerStatus;
  tools: MCPTool[];
  createdAt: string;
  updatedAt: string;
}

export interface MCPToolInfo {
  serverId: string;
  serverName: string;
  toolName: string;
  description: string;
  inputSchema: Record<string, unknown>;
}

export interface ConversationTool {
  serverId: string;
  toolName: string;
  enabled: boolean;
}

export interface MCPServerTemplate {
  id: string;
  name: string;
  description: string;
  command: string;
  args: string[];
  requiredEnv: string[];
}
