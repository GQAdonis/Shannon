/**
 * Add MCP Server Dialog
 *
 * Dialog for adding new MCP server configurations, with template support.
 */

'use client';

import { useState, useEffect } from 'react';
import { mcpService } from '@/lib/mcp/mcp-service';
import type { MCPServerConfig, MCPServerTemplate } from '@/lib/mcp/types';
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { Switch } from '@/components/ui/switch';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

interface AddServerDialogProps {
  onClose: () => void;
  onAdd: (config: MCPServerConfig) => void;
}

export function AddServerDialog({ onClose, onAdd }: AddServerDialogProps) {
  const [templates, setTemplates] = useState<MCPServerTemplate[]>([]);
  const [selectedTemplate, setSelectedTemplate] = useState<MCPServerTemplate | null>(null);

  // Form state
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [command, setCommand] = useState('');
  const [args, setArgs] = useState('');
  const [env, setEnv] = useState('');
  const [autoStart, setAutoStart] = useState(false);

  useEffect(() => {
    const loadTemplates = async () => {
      try {
        const result = await mcpService.getTemplates();
        setTemplates(result);
      } catch (error) {
        console.error('Failed to load templates:', error);
      }
    };

    loadTemplates();
  }, []);

  const handleTemplateSelect = (template: MCPServerTemplate) => {
    setSelectedTemplate(template);
    setName(template.name);
    setDescription(template.description);
    setCommand(template.command);
    setArgs(template.args.join(' '));
    setEnv(template.requiredEnv.map(key => `${key}=`).join('\n'));
  };

  const handleSubmit = () => {
    // Parse args and env
    const argsArray = args.trim().split(/\s+/).filter(Boolean);
    const envObject: Record<string, string> = {};

    env.split('\n').forEach(line => {
      const [key, value] = line.split('=');
      if (key && value) {
        envObject[key.trim()] = value.trim();
      }
    });

    const config: MCPServerConfig = {
      id: `mcp-${Date.now()}`,
      name,
      description,
      command,
      args: argsArray,
      env: envObject,
      autoStart,
      status: { type: 'Stopped' },
      tools: [],
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };

    onAdd(config);
  };

  const isFormValid = name.trim() && command.trim();

  return (
    <Dialog open={true} onOpenChange={onClose}>
      <DialogContent className="max-w-2xl max-h-[80vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Add MCP Server</DialogTitle>
          <DialogDescription>
            Add a new Model Context Protocol server from a template or custom configuration
          </DialogDescription>
        </DialogHeader>

        <Tabs defaultValue="templates" className="w-full">
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="templates">Templates</TabsTrigger>
            <TabsTrigger value="custom">Custom</TabsTrigger>
          </TabsList>

          {/* Templates Tab */}
          <TabsContent value="templates" className="space-y-4">
            {templates.length === 0 ? (
              <div className="text-center py-8 text-muted-foreground">
                Loading templates...
              </div>
            ) : (
              <div className="grid gap-3">
                {templates.map((template) => (
                  <Card
                    key={template.id}
                    className="cursor-pointer hover:bg-accent transition-colors"
                    onClick={() => handleTemplateSelect(template)}
                  >
                    <CardHeader>
                      <CardTitle className="text-base">{template.name}</CardTitle>
                      <CardDescription>{template.description}</CardDescription>
                    </CardHeader>
                    <CardContent>
                      <div className="text-sm text-muted-foreground">
                        <div>Command: {template.command} {template.args.join(' ')}</div>
                        {template.requiredEnv.length > 0 && (
                          <div className="mt-1">
                            Required: {template.requiredEnv.join(', ')}
                          </div>
                        )}
                      </div>
                    </CardContent>
                  </Card>
                ))}
              </div>
            )}
          </TabsContent>

          {/* Custom Tab */}
          <TabsContent value="custom" className="space-y-4">
            <div className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="name">Name *</Label>
                <Input
                  id="name"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="My MCP Server"
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="description">Description</Label>
                <Textarea
                  id="description"
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  placeholder="What does this server provide?"
                  rows={2}
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="command">Command *</Label>
                <Input
                  id="command"
                  value={command}
                  onChange={(e) => setCommand(e.target.value)}
                  placeholder="npx or python or node"
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="args">Arguments</Label>
                <Input
                  id="args"
                  value={args}
                  onChange={(e) => setArgs(e.target.value)}
                  placeholder="-y @modelcontextprotocol/server-github"
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="env">Environment Variables</Label>
                <Textarea
                  id="env"
                  value={env}
                  onChange={(e) => setEnv(e.target.value)}
                  placeholder="KEY=value (one per line)"
                  rows={4}
                />
              </div>

              <div className="flex items-center space-x-2">
                <Switch
                  id="autoStart"
                  checked={autoStart}
                  onCheckedChange={setAutoStart}
                />
                <Label htmlFor="autoStart">Auto-start on app launch</Label>
              </div>
            </div>
          </TabsContent>
        </Tabs>

        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            Cancel
          </Button>
          <Button onClick={handleSubmit} disabled={!isFormValid}>
            Add Server
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
