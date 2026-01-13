/**
 * Command Palette Component
 *
 * Global command search and execution (cmd+k)
 */

'use client';

import { useState, useEffect } from 'react';
import { useKeyboardShortcut } from '@/lib/keyboard/use-keyboard-shortcuts';
import { getShortcutDisplay } from '@/lib/keyboard/shortcuts';
import { useTabManager } from '@/lib/tabs/tab-manager';
import { useTheme } from 'next-themes';
import { useRouter } from 'next/navigation';
import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
} from '@/components/ui/command';
import {
  MessageSquare,
  Bot,
  FileCode,
  Book,
  Settings,
  Search,
  Sun,
  Moon,
  Laptop,
  Trash2,
  Plus,
  Zap,
  Target,
  Sparkles,
} from 'lucide-react';

interface Command {
  id: string;
  label: string;
  description?: string;
  icon: React.ReactNode;
  shortcut?: string;
  action: () => void;
  category: 'navigation' | 'actions' | 'modes' | 'theme' | 'settings';
}

export function CommandPalette() {
  const [open, setOpen] = useState(false);
  const { createTab, closeAllTabs } = useTabManager();
  const { setTheme } = useTheme();
  const router = useRouter();

  // Register cmd+k to open palette
  useKeyboardShortcut('cmd+k', () => setOpen(true));

  // Close on escape
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        setOpen(false);
      }
    };

    if (open) {
      window.addEventListener('keydown', handleKeyDown);
      return () => window.removeEventListener('keydown', handleKeyDown);
    }
  }, [open]);

  const commands: Command[] = [
    // Navigation
    {
      id: 'new-chat',
      label: 'New Chat',
      description: 'Start a new conversation',
      icon: <MessageSquare className="h-4 w-4" />,
      shortcut: getShortcutDisplay('cmd+n'),
      action: () => {
        createTab({ type: 'chat' });
        setOpen(false);
      },
      category: 'navigation',
    },
    {
      id: 'new-agent-chat',
      label: 'New Agent Chat',
      description: 'Chat with a specific agent',
      icon: <Bot className="h-4 w-4" />,
      action: () => {
        createTab({ type: 'agent' });
        setOpen(false);
      },
      category: 'navigation',
    },
    {
      id: 'new-artifact',
      label: 'New Artifact',
      description: 'Create code or document',
      icon: <FileCode className="h-4 w-4" />,
      action: () => {
        createTab({ type: 'artifact' });
        setOpen(false);
      },
      category: 'navigation',
    },
    {
      id: 'new-knowledge',
      label: 'New Knowledge Base',
      description: 'Browse knowledge sources',
      icon: <Book className="h-4 w-4" />,
      action: () => {
        createTab({ type: 'knowledge' });
        setOpen(false);
      },
      category: 'navigation',
    },

    // Actions
    {
      id: 'search',
      label: 'Search Conversations',
      description: 'Find messages across all chats',
      icon: <Search className="h-4 w-4" />,
      shortcut: getShortcutDisplay('cmd+f'),
      action: () => {
        // TODO: Open search modal
        setOpen(false);
      },
      category: 'actions',
    },
    {
      id: 'clear-all',
      label: 'Close All Tabs',
      description: 'Close all open conversations',
      icon: <Trash2 className="h-4 w-4" />,
      action: () => {
        closeAllTabs();
        setOpen(false);
      },
      category: 'actions',
    },

    // Modes
    {
      id: 'quick-mode',
      label: 'Quick Mode',
      description: 'Fast responses for simple tasks',
      icon: <Zap className="h-4 w-4" />,
      shortcut: getShortcutDisplay('cmd+shift+q'),
      action: () => {
        // TODO: Switch to quick mode
        setOpen(false);
      },
      category: 'modes',
    },
    {
      id: 'task-mode',
      label: 'Task Mode',
      description: 'Detailed execution with steps',
      icon: <Target className="h-4 w-4" />,
      shortcut: getShortcutDisplay('cmd+shift+t'),
      action: () => {
        // TODO: Switch to task mode
        setOpen(false);
      },
      category: 'modes',
    },
    {
      id: 'agent-mode',
      label: 'Agent Mode',
      description: 'Multi-agent collaboration',
      icon: <Sparkles className="h-4 w-4" />,
      shortcut: getShortcutDisplay('cmd+shift+a'),
      action: () => {
        // TODO: Switch to agent mode
        setOpen(false);
      },
      category: 'modes',
    },

    // Theme
    {
      id: 'theme-light',
      label: 'Light Theme',
      description: 'Switch to light mode',
      icon: <Sun className="h-4 w-4" />,
      action: () => {
        setTheme('light');
        setOpen(false);
      },
      category: 'theme',
    },
    {
      id: 'theme-dark',
      label: 'Dark Theme',
      description: 'Switch to dark mode',
      icon: <Moon className="h-4 w-4" />,
      action: () => {
        setTheme('dark');
        setOpen(false);
      },
      category: 'theme',
    },
    {
      id: 'theme-system',
      label: 'System Theme',
      description: 'Use system preference',
      icon: <Laptop className="h-4 w-4" />,
      action: () => {
        setTheme('system');
        setOpen(false);
      },
      category: 'theme',
    },

    // Settings
    {
      id: 'settings',
      label: 'Settings',
      description: 'Configure Shannon',
      icon: <Settings className="h-4 w-4" />,
      shortcut: getShortcutDisplay('cmd+,'),
      action: () => {
        router.push('/settings');
        setOpen(false);
      },
      category: 'settings',
    },
  ];

  const groupedCommands = {
    navigation: commands.filter((c) => c.category === 'navigation'),
    actions: commands.filter((c) => c.category === 'actions'),
    modes: commands.filter((c) => c.category === 'modes'),
    theme: commands.filter((c) => c.category === 'theme'),
    settings: commands.filter((c) => c.category === 'settings'),
  };

  return (
    <CommandDialog open={open} onOpenChange={setOpen}>
      <CommandInput placeholder="Type a command or search..." />
      <CommandList>
        <CommandEmpty>No results found.</CommandEmpty>

        <CommandGroup heading="Navigation">
          {groupedCommands.navigation.map((command) => (
            <CommandItem key={command.id} onSelect={command.action}>
              {command.icon}
              <span className="ml-2">{command.label}</span>
              {command.description && (
                <span className="ml-2 text-xs text-muted-foreground">
                  {command.description}
                </span>
              )}
              {command.shortcut && (
                <span className="ml-auto text-xs text-muted-foreground">
                  {command.shortcut}
                </span>
              )}
            </CommandItem>
          ))}
        </CommandGroup>

        <CommandSeparator />

        <CommandGroup heading="Actions">
          {groupedCommands.actions.map((command) => (
            <CommandItem key={command.id} onSelect={command.action}>
              {command.icon}
              <span className="ml-2">{command.label}</span>
              {command.description && (
                <span className="ml-2 text-xs text-muted-foreground">
                  {command.description}
                </span>
              )}
              {command.shortcut && (
                <span className="ml-auto text-xs text-muted-foreground">
                  {command.shortcut}
                </span>
              )}
            </CommandItem>
          ))}
        </CommandGroup>

        <CommandSeparator />

        <CommandGroup heading="Modes">
          {groupedCommands.modes.map((command) => (
            <CommandItem key={command.id} onSelect={command.action}>
              {command.icon}
              <span className="ml-2">{command.label}</span>
              {command.description && (
                <span className="ml-2 text-xs text-muted-foreground">
                  {command.description}
                </span>
              )}
              {command.shortcut && (
                <span className="ml-auto text-xs text-muted-foreground">
                  {command.shortcut}
                </span>
              )}
            </CommandItem>
          ))}
        </CommandGroup>

        <CommandSeparator />

        <CommandGroup heading="Theme">
          {groupedCommands.theme.map((command) => (
            <CommandItem key={command.id} onSelect={command.action}>
              {command.icon}
              <span className="ml-2">{command.label}</span>
              {command.description && (
                <span className="ml-2 text-xs text-muted-foreground">
                  {command.description}
                </span>
              )}
            </CommandItem>
          ))}
        </CommandGroup>

        <CommandSeparator />

        <CommandGroup heading="Settings">
          {groupedCommands.settings.map((command) => (
            <CommandItem key={command.id} onSelect={command.action}>
              {command.icon}
              <span className="ml-2">{command.label}</span>
              {command.description && (
                <span className="ml-2 text-xs text-muted-foreground">
                  {command.description}
                </span>
              )}
              {command.shortcut && (
                <span className="ml-auto text-xs text-muted-foreground">
                  {command.shortcut}
                </span>
              )}
            </CommandItem>
          ))}
        </CommandGroup>
      </CommandList>
    </CommandDialog>
  );
}
