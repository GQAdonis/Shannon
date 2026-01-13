/**
 * Tab Component
 *
 * Individual tab with close button and context menu
 */

'use client';

import { cn } from '@/lib/utils';
import { X, Pin, Copy, Trash2 } from 'lucide-react';
import { motion } from 'framer-motion';
import type { ChatTab } from '@/lib/tabs/types';
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuTrigger,
} from '@/components/ui/context-menu';
import { Button } from '@/components/ui/button';

interface TabProps {
  tab: ChatTab;
  active: boolean;
  onSelect: () => void;
  onClose: () => void;
  onPin?: () => void;
  onUnpin?: () => void;
  onDuplicate?: () => void;
  onCloseOthers?: () => void;
}

export function Tab({
  tab,
  active,
  onSelect,
  onClose,
  onPin,
  onUnpin,
  onDuplicate,
  onCloseOthers,
}: TabProps) {
  const Icon = tab.icon;

  return (
    <ContextMenu>
      <ContextMenuTrigger>
        <motion.div
          initial={{ opacity: 0, scale: 0.95 }}
          animate={{ opacity: 1, scale: 1 }}
          exit={{ opacity: 0, scale: 0.95 }}
          transition={{ duration: 0.15 }}
          className={cn(
            'group relative flex items-center gap-2 px-3 py-2 min-w-[120px] max-w-[200px]',
            'border-r border-border hover:bg-accent/50 transition-colors cursor-pointer',
            active && 'bg-background border-b-2 border-b-primary'
          )}
          onClick={onSelect}
        >
          {/* Pinned indicator */}
          {tab.isPinned && (
            <Pin className="h-3 w-3 text-muted-foreground absolute left-1 top-1" />
          )}

          {/* Tab icon */}
          {Icon && <Icon className="h-4 w-4 text-muted-foreground shrink-0" />}

          {/* Tab title */}
          <span className={cn(
            'flex-1 truncate text-sm',
            active ? 'text-foreground font-medium' : 'text-muted-foreground'
          )}>
            {tab.title}
          </span>

          {/* Dirty indicator */}
          {tab.isDirty && !active && (
            <div className="h-2 w-2 rounded-full bg-primary shrink-0" />
          )}

          {/* Close button */}
          <Button
            variant="ghost"
            size="icon"
            className={cn(
              'h-5 w-5 shrink-0 opacity-0 group-hover:opacity-100 transition-opacity',
              'hover:bg-destructive/10 hover:text-destructive'
            )}
            onClick={(e) => {
              e.stopPropagation();
              onClose();
            }}
          >
            <X className="h-3 w-3" />
          </Button>
        </motion.div>
      </ContextMenuTrigger>

      <ContextMenuContent>
        {onPin && !tab.isPinned && (
          <ContextMenuItem onClick={onPin}>
            <Pin className="mr-2 h-4 w-4" />
            Pin Tab
          </ContextMenuItem>
        )}
        {onUnpin && tab.isPinned && (
          <ContextMenuItem onClick={onUnpin}>
            <Pin className="mr-2 h-4 w-4" />
            Unpin Tab
          </ContextMenuItem>
        )}
        {onDuplicate && (
          <ContextMenuItem onClick={onDuplicate}>
            <Copy className="mr-2 h-4 w-4" />
            Duplicate Tab
          </ContextMenuItem>
        )}
        <ContextMenuSeparator />
        {onCloseOthers && (
          <ContextMenuItem onClick={onCloseOthers}>
            <Trash2 className="mr-2 h-4 w-4" />
            Close Other Tabs
          </ContextMenuItem>
        )}
        <ContextMenuItem onClick={onClose} className="text-destructive">
          <X className="mr-2 h-4 w-4" />
          Close Tab
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}
