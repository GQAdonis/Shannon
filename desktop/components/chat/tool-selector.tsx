/**
 * Chat Tool Selector Component
 *
 * Allows users to select which MCP tools are available for a specific conversation.
 */

'use client';

import { useState, useEffect } from 'react';
import { mcpService } from '@/lib/mcp/mcp-service';
import type { MCPToolInfo, ConversationTool } from '@/lib/mcp/types';
import { Popover, PopoverTrigger, PopoverContent } from '@/components/ui/popover';
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Wrench, Loader2 } from 'lucide-react';
import { Badge } from '@/components/ui/badge';

interface ChatToolSelectorProps {
  conversationId: string;
}

export function ChatToolSelector({ conversationId }: ChatToolSelectorProps) {
  const [allTools, setAllTools] = useState<MCPToolInfo[]>([]);
  const [selectedTools, setSelectedTools] = useState<ConversationTool[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    const loadTools = async () => {
      try {
        setLoading(true);
        const [available, selected] = await Promise.all([
          mcpService.listTools(),
          mcpService.getConversationTools(conversationId),
        ]);

        setAllTools(available);
        setSelectedTools(selected);
      } catch (error) {
        console.error('Failed to load tools:', error);
      } finally {
        setLoading(false);
      }
    };

    loadTools();
  }, [conversationId]);

  const handleToggle = async (tool: MCPToolInfo) => {
    const isEnabled = selectedTools.some(
      (t) => t.serverId === tool.serverId && t.toolName === tool.toolName && t.enabled
    );

    const updated = isEnabled
      ? selectedTools.filter(
          (t) => !(t.serverId === tool.serverId && t.toolName === tool.toolName)
        )
      : [
          ...selectedTools.filter(
            (t) => !(t.serverId === tool.serverId && t.toolName === tool.toolName)
          ),
          {
            serverId: tool.serverId,
            toolName: tool.toolName,
            enabled: true,
          },
        ];

    try {
      setSaving(true);
      await mcpService.setConversationTools(conversationId, updated);
      setSelectedTools(updated);
    } catch (error) {
      console.error('Failed to update tools:', error);
    } finally {
      setSaving(false);
    }
  };

  const enabledCount = selectedTools.filter((t) => t.enabled).length;

  // Group tools by server
  const toolsByServer = allTools.reduce((acc, tool) => {
    if (!acc[tool.serverName]) {
      acc[tool.serverName] = [];
    }
    acc[tool.serverName].push(tool);
    return acc;
  }, {} as Record<string, MCPToolInfo[]>);

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="ghost" size="sm" disabled={loading}>
          {loading ? (
            <Loader2 className="h-4 w-4 mr-2 animate-spin" />
          ) : (
            <Wrench className="h-4 w-4 mr-2" />
          )}
          Tools
          {enabledCount > 0 && (
            <Badge variant="secondary" className="ml-2">
              {enabledCount}
            </Badge>
          )}
        </Button>
      </PopoverTrigger>

      <PopoverContent className="w-96" align="end">
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold">Select Tools for This Chat</h3>
            {saving && <Loader2 className="h-4 w-4 animate-spin" />}
          </div>

          {allTools.length === 0 ? (
            <div className="text-center py-8 text-muted-foreground text-sm">
              No tools available. Start an MCP server to enable tools.
            </div>
          ) : (
            <ScrollArea className="h-[400px] pr-4">
              <div className="space-y-4">
                {Object.entries(toolsByServer).map(([serverName, tools]) => (
                  <div key={serverName} className="space-y-2">
                    <div className="font-medium text-sm text-muted-foreground">
                      {serverName}
                    </div>
                    {tools.map((tool) => {
                      const toolKey = `${tool.serverId}-${tool.toolName}`;
                      const isChecked = selectedTools.some(
                        (t) =>
                          t.serverId === tool.serverId &&
                          t.toolName === tool.toolName &&
                          t.enabled
                      );

                      return (
                        <label
                          key={toolKey}
                          htmlFor={toolKey}
                          className="flex items-start space-x-2 p-2 hover:bg-accent rounded cursor-pointer"
                        >
                          <Checkbox
                            id={toolKey}
                            checked={isChecked}
                            onCheckedChange={() => handleToggle(tool)}
                            className="mt-1"
                          />
                          <div className="flex-1 min-w-0">
                            <div className="font-medium text-sm">{tool.toolName}</div>
                            <div className="text-xs text-muted-foreground line-clamp-2">
                              {tool.description}
                            </div>
                          </div>
                        </label>
                      );
                    })}
                  </div>
                ))}
              </div>
            </ScrollArea>
          )}
        </div>
      </PopoverContent>
    </Popover>
  );
}
