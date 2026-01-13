/**
 * Knowledge Base Multi-Selector for Agents
 *
 * Allows selecting multiple knowledge bases to attach to an agent
 */

'use client';

import { useState, useEffect, useCallback } from 'react';
import { Database, X } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Checkbox } from '@/components/ui/checkbox';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';
import { kbService } from '@/lib/knowledge/kb-service';
import type { KnowledgeBase } from '@/lib/knowledge/types';

interface KBMultiSelectorProps {
  selected: string[];
  onChange: (kbIds: string[]) => void;
}

export function KBMultiSelector({ selected, onChange }: KBMultiSelectorProps) {
  const [allKbs, setAllKbs] = useState<KnowledgeBase[]>([]);
  const [loading, setLoading] = useState(true);
  const [open, setOpen] = useState(false);

  const loadKnowledgeBases = useCallback(async () => {
    try {
      setLoading(true);
      const kbs = await kbService.list();
      setAllKbs(kbs);
    } catch (error) {
      console.error('Failed to load knowledge bases:', error);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadKnowledgeBases();
  }, [loadKnowledgeBases]);

  const handleToggle = (kbId: string) => {
    const updated = selected.includes(kbId)
      ? selected.filter(id => id !== kbId)
      : [...selected, kbId];
    onChange(updated);
  };

  const handleRemove = (kbId: string) => {
    onChange(selected.filter(id => id !== kbId));
  };

  const selectedKbs = allKbs.filter(kb => selected.includes(kb.id));

  return (
    <div className="space-y-2">
      {/* Selected KBs Display */}
      {selectedKbs.length > 0 && (
        <div className="flex flex-wrap gap-2">
          {selectedKbs.map(kb => (
            <Badge
              key={kb.id}
              variant="secondary"
              className="gap-1 pr-1"
            >
              <Database className="h-3 w-3" />
              {kb.name}
              <button
                type="button"
                onClick={() => handleRemove(kb.id)}
                className="ml-1 rounded-sm hover:bg-accent"
              >
                <X className="h-3 w-3" />
              </button>
            </Badge>
          ))}
        </div>
      )}

      {/* Selector Popover */}
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
          <Button
            type="button"
            variant="outline"
            size="sm"
            className="w-full justify-start"
          >
            <Database className="h-4 w-4 mr-2" />
            {selected.length === 0
              ? 'Select Knowledge Bases'
              : `${selected.length} selected`}
          </Button>
        </PopoverTrigger>

        <PopoverContent className="w-80" align="start">
          <div className="space-y-4">
            <div>
              <h4 className="font-medium text-sm mb-2">Knowledge Bases</h4>
              <p className="text-xs text-muted-foreground">
                Select knowledge bases for this agent to access
              </p>
            </div>

            {loading ? (
              <div className="text-sm text-muted-foreground text-center py-4">
                Loading...
              </div>
            ) : allKbs.length === 0 ? (
              <div className="text-sm text-muted-foreground text-center py-4">
                No knowledge bases available.
                <br />
                Create one in Settings.
              </div>
            ) : (
              <div className="space-y-2 max-h-[300px] overflow-y-auto">
                {allKbs.map(kb => (
                  <button
                    type="button"
                    key={kb.id}
                    className="flex items-start space-x-3 rounded-lg border p-3 hover:bg-accent transition-colors cursor-pointer w-full text-left"
                    onClick={() => handleToggle(kb.id)}
                  >
                    <Checkbox
                      checked={selected.includes(kb.id)}
                      onCheckedChange={() => handleToggle(kb.id)}
                      className="mt-0.5"
                    />
                    <div className="flex-1 space-y-1">
                      <div className="font-medium text-sm">{kb.name}</div>
                      {kb.description && (
                        <p className="text-xs text-muted-foreground line-clamp-2">
                          {kb.description}
                        </p>
                      )}
                      <div className="flex items-center gap-3 text-xs text-muted-foreground">
                        <span>{kb.documentCount} docs</span>
                        <span>•</span>
                        <span>{kb.totalChunks} chunks</span>
                      </div>
                    </div>
                  </button>
                ))}
              </div>
            )}
          </div>
        </PopoverContent>
      </Popover>
    </div>
  );
}
