/**
 * Knowledge Base Selector for Chat
 *
 * Allows users to attach/detach knowledge bases to/from a conversation
 */

'use client';

import { useState, useEffect, useCallback } from 'react';
import { Database, Check } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';
import { kbService } from '@/lib/knowledge/kb-service';
import type { KnowledgeBase } from '@/lib/knowledge/types';

interface KBSelectorProps {
  conversationId: string;
  onKBsChanged?: (kbIds: string[]) => void;
}

export function KBSelector({ conversationId, onKBsChanged }: KBSelectorProps) {
  const [allKbs, setAllKbs] = useState<KnowledgeBase[]>([]);
  const [selectedKbs, setSelectedKbs] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const [open, setOpen] = useState(false);

  const loadKnowledgeBases = useCallback(async () => {
    try {
      setLoading(true);
      const [all, attached] = await Promise.all([
        kbService.list(),
        kbService.getConversationKnowledgeBases(conversationId),
      ]);
      setAllKbs(all);
      setSelectedKbs(attached.map(kb => kb.id));
    } catch (error) {
      console.error('Failed to load knowledge bases:', error);
    } finally {
      setLoading(false);
    }
  }, [conversationId]);

  // Load all KBs and selected KBs on mount
  useEffect(() => {
    loadKnowledgeBases();
  }, [loadKnowledgeBases]);

  const handleToggle = async (kbId: string) => {
    try {
      const updated = selectedKbs.includes(kbId)
        ? selectedKbs.filter(id => id !== kbId)
        : [...selectedKbs, kbId];

      // Update backend
      await kbService.attachToConversation(conversationId, updated);

      // Update state
      setSelectedKbs(updated);

      // Notify parent
      onKBsChanged?.(updated);
    } catch (error) {
      console.error('Failed to toggle knowledge base:', error);
    }
  };

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="sm"
          className="h-8 gap-2"
        >
          <Database className="h-4 w-4" />
          <span className="hidden sm:inline">Knowledge</span>
          {selectedKbs.length > 0 && (
            <span className="flex h-5 w-5 items-center justify-center rounded-full bg-primary text-[10px] text-primary-foreground">
              {selectedKbs.length}
            </span>
          )}
        </Button>
      </PopoverTrigger>

      <PopoverContent className="w-80" align="end">
        <div className="space-y-4">
          <div>
            <h4 className="font-medium text-sm mb-2">Knowledge Bases</h4>
            <p className="text-xs text-muted-foreground">
              Select knowledge bases to provide context for this conversation
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
                    checked={selectedKbs.includes(kb.id)}
                    onCheckedChange={() => handleToggle(kb.id)}
                    className="mt-0.5"
                  />
                  <div className="flex-1 space-y-1">
                    <div className="flex items-center justify-between">
                      <div className="font-medium text-sm">{kb.name}</div>
                      {selectedKbs.includes(kb.id) && (
                        <Check className="h-4 w-4 text-primary" />
                      )}
                    </div>
                    {kb.description && (
                      <p className="text-xs text-muted-foreground line-clamp-2">
                        {kb.description}
                      </p>
                    )}
                    <div className="flex items-center gap-3 text-xs text-muted-foreground">
                      <span>{kb.documentCount} docs</span>
                      <span>•</span>
                      <span>{kb.totalChunks} chunks</span>
                      <span>•</span>
                      <span className="capitalize">{kb.chunkingStrategy.replace('_', ' ')}</span>
                    </div>
                  </div>
                </button>
              ))}
            </div>
          )}
        </div>
      </PopoverContent>
    </Popover>
  );
}
