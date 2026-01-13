/**
 * MCP Server Management Page
 *
 * Allows users to add, configure, and manage MCP servers that provide tools for agents.
 */

'use client';

import { useState, useEffect, useCallback } from 'react';
import { mcpService } from '@/lib/mcp/mcp-service';
import type { MCPServerConfig } from '@/lib/mcp/types';
import { ServerCard } from '@/components/mcp/server-card';
import { AddServerDialog } from '@/components/mcp/add-server-dialog';
import { Button } from '@/components/ui/button';
import { Plus, RefreshCw } from 'lucide-react';
import { toast } from 'sonner';

export default function MCPServersPage() {
  const [servers, setServers] = useState<MCPServerConfig[]>([]);
  const [loading, setLoading] = useState(true);
  const [showAddDialog, setShowAddDialog] = useState(false);


  const loadServers = useCallback(async () => {
    try {
      const result = await mcpService.listServers();
      setServers(result);
    } catch (error) {
      console.error('Failed to load MCP servers:', error);
      toast.error('Error', {
        description: 'Failed to load MCP servers',
      });
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadServers();

    // Auto-refresh every 5 seconds to update server status
    const interval = setInterval(loadServers, 5000);
    return () => clearInterval(interval);
  }, [loadServers]);

  const handleStart = async (id: string) => {
    try {
      await mcpService.startServer(id);
      toast.success('Success', {
        description: 'Server started successfully',
      });
      await loadServers();
    } catch (error) {
      console.error('Failed to start server:', error);
      toast.error('Error', {
        description: `Failed to start server: ${error}`,
      });
    }
  };

  const handleStop = async (id: string) => {
    try {
      await mcpService.stopServer(id);
      toast.success('Success', {
        description: 'Server stopped successfully',
      });
      await loadServers();
    } catch (error) {
      console.error('Failed to stop server:', error);
      toast.error('Error', {
        description: `Failed to stop server: ${error}`,
      });
    }
  };

  const handleRemove = async (id: string) => {
    try {
      await mcpService.removeServer(id);
      toast.success('Success', {
        description: 'Server removed successfully',
      });
      await loadServers();
    } catch (error) {
      console.error('Failed to remove server:', error);
      toast.error('Error', {
        description: `Failed to remove server: ${error}`,
      });
    }
  };

  const handleAdd = async (config: MCPServerConfig) => {
    try {
      await mcpService.addServer(config);
      toast.success('Success', {
        description: 'Server added successfully',
      });
      setShowAddDialog(false);
      await loadServers();
    } catch (error) {
      console.error('Failed to add server:', error);
      toast.error('Error', {
        description: `Failed to add server: ${error}`,
      });
    }
  };

  return (
    <div className="container mx-auto p-6 space-y-6">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold">MCP Servers</h1>
          <p className="text-muted-foreground mt-1">
            Manage Model Context Protocol servers that provide tools for agents
          </p>
        </div>
        <div className="flex gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={loadServers}
            disabled={loading}
          >
            <RefreshCw className={`h-4 w-4 mr-2 ${loading ? 'animate-spin' : ''}`} />
            Refresh
          </Button>
          <Button onClick={() => setShowAddDialog(true)}>
            <Plus className="h-4 w-4 mr-2" />
            Add Server
          </Button>
        </div>
      </div>

      {/* Server Grid */}
      {loading && servers.length === 0 ? (
        <div className="text-center py-12">
          <RefreshCw className="h-8 w-8 animate-spin mx-auto text-muted-foreground" />
          <p className="text-muted-foreground mt-2">Loading servers...</p>
        </div>
      ) : servers.length === 0 ? (
        <div className="text-center py-12 border-2 border-dashed rounded-lg">
          <p className="text-muted-foreground">No MCP servers configured</p>
          <Button
            variant="outline"
            className="mt-4"
            onClick={() => setShowAddDialog(true)}
          >
            <Plus className="h-4 w-4 mr-2" />
            Add Your First Server
          </Button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {servers.map((server) => (
            <ServerCard
              key={server.id}
              server={server}
              onStart={handleStart}
              onStop={handleStop}
              onRemove={handleRemove}
            />
          ))}
        </div>
      )}

      {/* Add Server Dialog */}
      {showAddDialog && (
        <AddServerDialog
          onClose={() => setShowAddDialog(false)}
          onAdd={handleAdd}
        />
      )}
    </div>
  );
}
