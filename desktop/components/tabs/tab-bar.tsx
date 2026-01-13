/**
 * Tab Bar Component
 *
 * Displays all open tabs with scroll support and new tab button
 */

'use client';

import { useTabManager } from '@/lib/tabs/tab-manager';
import { Tab } from './tab';
import { Button } from '@/components/ui/button';
import { ScrollArea, ScrollBar } from '@/components/ui/scroll-area';
import { Plus } from 'lucide-react';
import { AnimatePresence } from 'framer-motion';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { MessageSquare, Bot, FileCode, Book } from 'lucide-react';

export function TabBar() {
  const {
    tabs,
    activeTabId,
    createTab,
    closeTab,
    switchTab,
    pinTab,
    unpinTab,
    duplicateTab,
    closeOtherTabs,
    canCreateTab,
  } = useTabManager();

  const handleCreateTab = (type: 'chat' | 'agent' | 'artifact' | 'knowledge') => {
    createTab({ type });
  };

  // Sort tabs: pinned first, then by last active
  const sortedTabs = [...tabs].sort((a, b) => {
    if (a.isPinned && !b.isPinned) return -1;
    if (!a.isPinned && b.isPinned) return 1;
    return b.lastActive.getTime() - a.lastActive.getTime();
  });

  return (
    <div className="flex items-center border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <ScrollArea className="flex-1">
        <div className="flex">
          <AnimatePresence mode="popLayout">
            {sortedTabs.map((tab) => (
              <Tab
                key={tab.id}
                tab={tab}
                active={tab.id === activeTabId}
                onSelect={() => switchTab(tab.id)}
                onClose={() => closeTab(tab.id)}
                onPin={() => pinTab(tab.id)}
                onUnpin={() => unpinTab(tab.id)}
                onDuplicate={() => duplicateTab(tab.id)}
                onCloseOthers={() => closeOtherTabs(tab.id)}
              />
            ))}
          </AnimatePresence>
        </div>
        <ScrollBar orientation="horizontal" />
      </ScrollArea>

      {/* New tab button */}
      <div className="flex items-center gap-1 px-2 border-l">
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant="ghost"
              size="sm"
              className="h-8 w-8 p-0"
              disabled={!canCreateTab()}
              title="New Tab"
            >
              <Plus className="h-4 w-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem onClick={() => handleCreateTab('chat')}>
              <MessageSquare className="mr-2 h-4 w-4" />
              New Chat
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => handleCreateTab('agent')}>
              <Bot className="mr-2 h-4 w-4" />
              New Agent Chat
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => handleCreateTab('artifact')}>
              <FileCode className="mr-2 h-4 w-4" />
              New Artifact
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => handleCreateTab('knowledge')}>
              <Book className="mr-2 h-4 w-4" />
              New Knowledge Base
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </div>
  );
}
