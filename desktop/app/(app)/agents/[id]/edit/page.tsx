/**
 * Agent editor page - create/edit AI agents.
 */

'use client';

import { useState, useEffect } from 'react';
import { useRouter, useParams } from 'next/navigation';
import { agentService } from '@/lib/agents/agent-service';
import { AgentSpec, AGENT_CATEGORIES, CONVERSATION_STYLES, WORKFLOW_STRATEGIES } from '@/lib/agents/types';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Save, ArrowLeft } from 'lucide-react';

export default function AgentEditorPage() {
  const router = useRouter();
  const params = useParams();
  const isEdit = params?.id && params.id !== 'new';

  const [spec, setSpec] = useState<AgentSpec>(agentService.createDefault());
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (isEdit) {
      loadAgent(params.id as string);
    }
  }, [isEdit, params?.id]);

  const loadAgent = async (id: string) => {
    try {
      setLoading(true);
      const agent = await agentService.get(id);
      setSpec(agent);
    } catch (error) {
      console.error('Failed to load agent:', error);
      alert('Failed to load agent');
      router.push('/agents');
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    try {
      setSaving(true);
      if (isEdit) {
        await agentService.update(spec.id, spec);
        alert('Agent updated successfully');
      } else {
        await agentService.create(spec);
        alert('Agent created successfully');
      }
      router.push('/agents');
    } catch (error) {
      console.error('Failed to save agent:', error);
      alert('Failed to save agent');
    } finally {
      setSaving(false);
    }
  };

  const updateSpec = (updates: Partial<AgentSpec>) => {
    setSpec({ ...spec, ...updates, updatedAt: new Date().toISOString() });
  };

  if (loading) {
    return (
      <div className="container mx-auto p-6">
        <div className="text-center py-12">Loading...</div>
      </div>
    );
  }

  return (
    <div className="container mx-auto p-6 max-w-4xl space-y-6">
      {/* Header */}
      <div className="flex items-center gap-4">
        <Button variant="ghost" size="icon" onClick={() => router.back()}>
          <ArrowLeft className="h-4 w-4" />
        </Button>
        <div className="flex-1">
          <h1 className="text-3xl font-bold">
            {isEdit ? 'Edit Agent' : 'Create Agent'}
          </h1>
          <p className="text-muted-foreground mt-1">
            {isEdit ? 'Modify agent configuration' : 'Configure a new AI agent'}
          </p>
        </div>
        <Button onClick={handleSave} disabled={saving}>
          <Save className="h-4 w-4 mr-2" />
          {saving ? 'Saving...' : 'Save'}
        </Button>
      </div>

      {/* Form */}
      <div className="space-y-6 bg-card p-6 rounded-lg border">
        {/* Basic Info */}
        <div className="space-y-4">
          <h2 className="text-xl font-semibold">Basic Information</h2>

          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="name">Name *</Label>
              <Input
                id="name"
                value={spec.name}
                onChange={(e) => updateSpec({ name: e.target.value })}
                placeholder="My Agent"
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="icon">Icon</Label>
              <Input
                id="icon"
                value={spec.icon || ''}
                onChange={(e) => updateSpec({ icon: e.target.value })}
                placeholder="🤖"
                maxLength={2}
              />
            </div>
          </div>

          <div className="space-y-2">
            <Label htmlFor="description">Description</Label>
            <Textarea
              id="description"
              value={spec.description}
              onChange={(e) => updateSpec({ description: e.target.value })}
              placeholder="Describe what this agent does..."
              rows={3}
            />
          </div>

          <div className="grid grid-cols-3 gap-4">
            <div className="space-y-2">
              <Label htmlFor="version">Version</Label>
              <Input
                id="version"
                value={spec.version}
                onChange={(e) => updateSpec({ version: e.target.value })}
                placeholder="1.0.0"
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="author">Author</Label>
              <Input
                id="author"
                value={spec.author || ''}
                onChange={(e) => updateSpec({ author: e.target.value })}
                placeholder="Your name"
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="category">Category *</Label>
              <Select
                value={spec.category}
                onValueChange={(value) => updateSpec({ category: value })}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {AGENT_CATEGORIES.map((cat) => (
                    <SelectItem key={cat} value={cat} className="capitalize">
                      {cat}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>
        </div>

        {/* System Prompt */}
        <div className="space-y-4">
          <h2 className="text-xl font-semibold">System Prompt</h2>
          <Textarea
            value={spec.systemPrompt}
            onChange={(e) => updateSpec({ systemPrompt: e.target.value })}
            placeholder="You are a helpful AI assistant..."
            rows={6}
            className="font-mono text-sm"
          />
        </div>

        {/* Model Configuration */}
        <div className="space-y-4">
          <h2 className="text-xl font-semibold">Model Configuration</h2>

          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="provider">Provider</Label>
              <Select
                value={spec.model.provider}
                onValueChange={(value) =>
                  updateSpec({ model: { ...spec.model, provider: value } })
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="openai">OpenAI</SelectItem>
                  <SelectItem value="anthropic">Anthropic</SelectItem>
                  <SelectItem value="google">Google</SelectItem>
                  <SelectItem value="groq">Groq</SelectItem>
                  <SelectItem value="xai">xAI</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label htmlFor="modelName">Model</Label>
              <Input
                id="modelName"
                value={spec.model.name}
                onChange={(e) =>
                  updateSpec({ model: { ...spec.model, name: e.target.value } })
                }
                placeholder="gpt-4"
              />
            </div>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="temperature">Temperature</Label>
              <Input
                id="temperature"
                type="number"
                min="0"
                max="2"
                step="0.1"
                value={spec.model.temperature || ''}
                onChange={(e) =>
                  updateSpec({
                    model: {
                      ...spec.model,
                      temperature: e.target.value ? parseFloat(e.target.value) : undefined,
                    },
                  })
                }
                placeholder="0.7"
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="maxTokens">Max Tokens</Label>
              <Input
                id="maxTokens"
                type="number"
                min="1"
                value={spec.model.maxTokens || ''}
                onChange={(e) =>
                  updateSpec({
                    model: {
                      ...spec.model,
                      maxTokens: e.target.value ? parseInt(e.target.value) : undefined,
                    },
                  })
                }
                placeholder="2000"
              />
            </div>
          </div>
        </div>

        {/* Behavior */}
        <div className="space-y-4">
          <h2 className="text-xl font-semibold">Behavior</h2>

          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="strategy">Workflow Strategy</Label>
              <Select
                value={spec.strategy || 'none'}
                onValueChange={(value) =>
                  updateSpec({ strategy: value === 'none' ? undefined : value })
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="none">None</SelectItem>
                  {WORKFLOW_STRATEGIES.map((strategy) => (
                    <SelectItem key={strategy} value={strategy} className="capitalize">
                      {strategy.replace(/_/g, ' ')}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label htmlFor="conversationStyle">Conversation Style</Label>
              <Select
                value={spec.conversationStyle || 'none'}
                onValueChange={(value) =>
                  updateSpec({
                    conversationStyle: value === 'none' ? undefined : (value as any),
                  })
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="none">None</SelectItem>
                  {CONVERSATION_STYLES.map((style) => (
                    <SelectItem key={style} value={style} className="capitalize">
                      {style}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>

          <div className="space-y-2">
            <Label htmlFor="tags">Tags (comma-separated)</Label>
            <Input
              id="tags"
              value={spec.tags.join(', ')}
              onChange={(e) =>
                updateSpec({
                  tags: e.target.value.split(',').map((t) => t.trim()).filter(Boolean),
                })
              }
              placeholder="coding, python, expert"
            />
          </div>
        </div>
      </div>
    </div>
  );
}
