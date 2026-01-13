'use client';

import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button } from '@/components/ui/button';
import { Label } from '@/components/ui/label';
import { Input } from '@/components/ui/input';
import { Slider } from '@/components/ui/slider';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import {
  ContextSettings,
  ContextStrategy,
  STRATEGY_DESCRIPTIONS,
  STRATEGY_NAMES,
  DEFAULT_SETTINGS,
  SUMMARIZATION_MODELS,
} from '@/lib/context/types';
import { InfoIcon } from 'lucide-react';

export default function ContextSettingsPage() {
  const [settings, setSettings] = useState<ContextSettings | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [message, setMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  const loadSettings = useCallback(async () => {
    try {
      setLoading(true);
      const result = await invoke<ContextSettings>('get_context_settings', {
        id: 'default',
      });
      setSettings(result);
    } catch (error) {
      console.error('Failed to load context settings:', error);
      // Initialize with default settings
      const now = new Date().toISOString();
      setSettings({
        id: 'default',
        ...DEFAULT_SETTINGS,
        createdAt: now,
        updatedAt: now,
      });
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  const handleSave = async () => {
    if (!settings) return;

    try {
      setSaving(true);
      setMessage(null);

      await invoke('save_context_settings', {
        settings: {
          ...settings,
          updatedAt: new Date().toISOString(),
        },
      });

      setMessage({ type: 'success', text: 'Context settings saved successfully!' });
      setTimeout(() => setMessage(null), 3000);
    } catch (error) {
      console.error('Failed to save context settings:', error);
      setMessage({
        type: 'error',
        text: `Failed to save settings: ${error}`,
      });
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <div className="container max-w-4xl mx-auto p-6">
        <div className="flex items-center justify-center h-64">
          <div className="text-muted-foreground">Loading context settings...</div>
        </div>
      </div>
    );
  }

  if (!settings) {
    return (
      <div className="container max-w-4xl mx-auto p-6">
        <Alert variant="destructive">
          <AlertDescription>Failed to load context settings.</AlertDescription>
        </Alert>
      </div>
    );
  }

  return (
    <div className="container max-w-4xl mx-auto p-6 space-y-6">
      <div>
        <h1 className="text-3xl font-bold">Context Management</h1>
        <p className="text-muted-foreground mt-2">
          Configure how conversation history is managed and optimized within token budgets.
        </p>
      </div>

      {message && (
        <Alert variant={message.type === 'error' ? 'destructive' : 'default'}>
          <AlertDescription>{message.text}</AlertDescription>
        </Alert>
      )}

      <Card>
        <CardHeader>
          <CardTitle>Strategy Selection</CardTitle>
          <CardDescription>
            Choose how conversation context should be managed
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="strategy">Context Strategy</Label>
            <Select
              value={settings.strategy}
              onValueChange={(value: ContextStrategy) =>
                setSettings({ ...settings, strategy: value })
              }
            >
              <SelectTrigger id="strategy">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {Object.entries(STRATEGY_NAMES).map(([value, label]) => (
                  <SelectItem key={value} value={value}>
                    {label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <Alert>
            <InfoIcon className="h-4 w-4" />
            <AlertDescription>
              {STRATEGY_DESCRIPTIONS[settings.strategy]}
            </AlertDescription>
          </Alert>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Token Budgets</CardTitle>
          <CardDescription>
            Configure token limits for different context tiers
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label htmlFor="shortTermTurns">Short-term Turns</Label>
              <span className="text-sm text-muted-foreground">{settings.shortTermTurns}</span>
            </div>
            <Input
              id="shortTermTurns"
              type="number"
              min="1"
              max="20"
              value={settings.shortTermTurns}
              onChange={(e) =>
                setSettings({ ...settings, shortTermTurns: Number(e.target.value) })
              }
            />
            <p className="text-sm text-muted-foreground">
              Number of recent conversation turns to keep verbatim (1 turn = user + assistant)
            </p>
          </div>

          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label htmlFor="midTermBudget">Mid-term Budget (tokens)</Label>
              <span className="text-sm text-muted-foreground">{settings.midTermBudget}</span>
            </div>
            <Slider
              id="midTermBudget"
              min={500}
              max={10000}
              step={100}
              value={[settings.midTermBudget]}
              onValueChange={([value]) =>
                setSettings({ ...settings, midTermBudget: value })
              }
            />
            <p className="text-sm text-muted-foreground">
              Token budget for mid-term context (summarized messages)
            </p>
          </div>

          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label htmlFor="longTermBudget">Long-term Budget (tokens)</Label>
              <span className="text-sm text-muted-foreground">{settings.longTermBudget}</span>
            </div>
            <Slider
              id="longTermBudget"
              min={100}
              max={5000}
              step={50}
              value={[settings.longTermBudget]}
              onValueChange={([value]) =>
                setSettings({ ...settings, longTermBudget: value })
              }
            />
            <p className="text-sm text-muted-foreground">
              Token budget for long-term context (key facts and important details)
            </p>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Summarization</CardTitle>
          <CardDescription>
            Configure the model used for summarizing conversation history
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="summarizationModel">Summarization Model</Label>
            <Select
              value={settings.summarizationModel}
              onValueChange={(value) =>
                setSettings({ ...settings, summarizationModel: value })
              }
            >
              <SelectTrigger id="summarizationModel">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {SUMMARIZATION_MODELS.map((model) => (
                  <SelectItem key={model.value} value={model.value}>
                    {model.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <p className="text-sm text-muted-foreground">
              Choose a fast, cost-effective model for summarization
            </p>
          </div>
        </CardContent>
      </Card>

      <div className="flex items-center justify-between pt-4">
        <Button variant="outline" onClick={loadSettings} disabled={loading || saving}>
          Reset
        </Button>
        <Button onClick={handleSave} disabled={saving}>
          {saving ? 'Saving...' : 'Save Settings'}
        </Button>
      </div>
    </div>
  );
}
