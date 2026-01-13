/**
 * MCP (Model Context Protocol) service for managing MCP servers and tools.
 */

import { invoke } from '@tauri-apps/api/core';
import type {
  MCPServerConfig,
  MCPToolInfo,
  ConversationTool,
  MCPServerTemplate,
} from './types';

export class MCPService {
  /**
   * List all MCP server configurations.
   */
  async listServers(): Promise<MCPServerConfig[]> {
    return await invoke('list_mcp_servers');
  }

  /**
   * Add a new MCP server configuration.
   */
  async addServer(config: MCPServerConfig): Promise<string> {
    return await invoke('add_mcp_server', { config });
  }

  /**
   * Start an MCP server by ID.
   */
  async startServer(id: string): Promise<void> {
    await invoke('start_mcp_server', { id });
  }

  /**
   * Stop an MCP server by ID.
   */
  async stopServer(id: string): Promise<void> {
    await invoke('stop_mcp_server', { id });
  }

  /**
   * Remove an MCP server configuration.
   */
  async removeServer(id: string): Promise<void> {
    await invoke('remove_mcp_server', { id });
  }

  /**
   * List all available tools from running MCP servers.
   */
  async listTools(): Promise<MCPToolInfo[]> {
    return await invoke('list_mcp_tools');
  }

  /**
   * Execute a tool on an MCP server.
   */
  async executeTool(
    serverId: string,
    toolName: string,
    args: unknown
  ): Promise<unknown> {
    return await invoke('execute_mcp_tool', { serverId, toolName, args });
  }

  /**
   * Get tools enabled for a specific conversation.
   */
  async getConversationTools(conversationId: string): Promise<ConversationTool[]> {
    return await invoke('get_conversation_tools', { conversationId });
  }

  /**
   * Set tools for a specific conversation.
   */
  async setConversationTools(
    conversationId: string,
    tools: ConversationTool[]
  ): Promise<void> {
    await invoke('set_conversation_tools', { conversationId, tools });
  }

  /**
   * Get available MCP server templates.
   */
  async getTemplates(): Promise<MCPServerTemplate[]> {
    return await invoke('get_mcp_templates');
  }
}

// Singleton instance
export const mcpService = new MCPService();
