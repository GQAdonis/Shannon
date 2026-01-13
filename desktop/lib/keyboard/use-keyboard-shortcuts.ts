/**
 * Keyboard Shortcuts Hook
 *
 * React hook for registering and managing keyboard shortcuts
 */

'use client';

import { useEffect, useCallback } from 'react';
import {
  type ShortcutKey,
  SHORTCUTS,
  findMatchingShortcut,
  shouldPreventDefault,
} from './shortcuts';

export interface UseKeyboardShortcutsOptions {
  enabled?: boolean;
  preventDefault?: boolean;
}

/**
 * Register keyboard shortcut handlers
 */
export function useKeyboardShortcuts(
  handlers: Partial<Record<ShortcutKey, () => void>>,
  options: UseKeyboardShortcutsOptions = {}
) {
  const { enabled = true, preventDefault = true } = options;

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (!enabled) return;

      // Don't trigger shortcuts when typing in input fields
      const target = e.target as HTMLElement;
      const isInput =
        target.tagName === 'INPUT' ||
        target.tagName === 'TEXTAREA' ||
        target.isContentEditable;

      // Allow some shortcuts even in input fields
      const allowInInput = ['cmd+k', 'cmd+,', 'esc'];
      const matchedShortcut = findMatchingShortcut(e);

      if (isInput && matchedShortcut) {
        const shortcutValue = SHORTCUTS[matchedShortcut];
        if (!allowInInput.includes(shortcutValue)) {
          return;
        }
      }

      if (matchedShortcut && handlers[matchedShortcut]) {
        if (preventDefault && shouldPreventDefault(e)) {
          e.preventDefault();
        }
        handlers[matchedShortcut]?.();
      }
    },
    [enabled, preventDefault, handlers]
  );

  useEffect(() => {
    if (!enabled) return;

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [enabled, handleKeyDown]);
}

/**
 * Register a single keyboard shortcut
 */
export function useKeyboardShortcut(
  shortcut: string,
  handler: () => void,
  options: UseKeyboardShortcutsOptions = {}
) {
  const { enabled = true, preventDefault = true } = options;

  useEffect(() => {
    if (!enabled) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      const target = e.target as HTMLElement;
      const isInput =
        target.tagName === 'INPUT' ||
        target.tagName === 'TEXTAREA' ||
        target.isContentEditable;

      // Allow cmd+k and esc in input fields
      const allowInInput = ['cmd+k', 'cmd+,', 'esc'];
      if (isInput && !allowInInput.includes(shortcut)) {
        return;
      }

      const matchedShortcut = findMatchingShortcut(e);
      if (matchedShortcut && SHORTCUTS[matchedShortcut] === shortcut) {
        if (preventDefault) {
          e.preventDefault();
        }
        handler();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [shortcut, handler, enabled, preventDefault]);
}

/**
 * Get all registered shortcuts by category
 */
export function useShortcutCategories() {
  return {
    navigation: [
      { key: 'newChat', label: 'New Chat', shortcut: SHORTCUTS.newChat },
      { key: 'newTab', label: 'New Tab', shortcut: SHORTCUTS.newTab },
      { key: 'closeTab', label: 'Close Tab', shortcut: SHORTCUTS.closeTab },
      { key: 'nextTab', label: 'Next Tab', shortcut: SHORTCUTS.nextTab },
      { key: 'prevTab', label: 'Previous Tab', shortcut: SHORTCUTS.prevTab },
    ],
    actions: [
      { key: 'send', label: 'Send Message', shortcut: SHORTCUTS.send },
      { key: 'clearChat', label: 'Clear Chat', shortcut: SHORTCUTS.clearChat },
      { key: 'search', label: 'Search', shortcut: SHORTCUTS.search },
      { key: 'settings', label: 'Settings', shortcut: SHORTCUTS.settings },
      { key: 'commandPalette', label: 'Command Palette', shortcut: SHORTCUTS.commandPalette },
    ],
    modes: [
      { key: 'quickMode', label: 'Quick Mode', shortcut: SHORTCUTS.quickMode },
      { key: 'taskMode', label: 'Task Mode', shortcut: SHORTCUTS.taskMode },
      { key: 'agentMode', label: 'Agent Mode', shortcut: SHORTCUTS.agentMode },
    ],
    ui: [
      { key: 'toggleSidebar', label: 'Toggle Sidebar', shortcut: SHORTCUTS.toggleSidebar },
      { key: 'toggleTheme', label: 'Toggle Theme', shortcut: SHORTCUTS.toggleTheme },
      { key: 'focusInput', label: 'Focus Input', shortcut: SHORTCUTS.focusInput },
    ],
    tools: [
      { key: 'selectTools', label: 'Select Tools', shortcut: SHORTCUTS.selectTools },
      { key: 'selectKnowledge', label: 'Select Knowledge', shortcut: SHORTCUTS.selectKnowledge },
      { key: 'selectAgents', label: 'Select Agents', shortcut: SHORTCUTS.selectAgents },
    ],
  };
}
