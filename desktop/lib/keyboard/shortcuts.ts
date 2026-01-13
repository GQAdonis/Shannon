/**
 * Keyboard Shortcuts System
 *
 * Centralized keyboard shortcut management for Shannon Desktop
 */

export interface KeyboardShortcut {
  key: string;
  label: string;
  description: string;
  category: 'navigation' | 'actions' | 'modes' | 'ui' | 'tools';
  handler: () => void;
  mac?: string; // macOS specific key combination
  windows?: string; // Windows/Linux specific key combination
}

export interface ShortcutCategory {
  name: string;
  shortcuts: KeyboardShortcut[];
}

/**
 * Keyboard shortcut definitions
 */
export const SHORTCUTS = {
  // Navigation
  newChat: 'cmd+n',
  newTab: 'cmd+t',
  closeTab: 'cmd+w',
  nextTab: 'cmd+shift+]',
  prevTab: 'cmd+shift+[',
  switchTab1: 'cmd+1',
  switchTab2: 'cmd+2',
  switchTab3: 'cmd+3',
  switchTab4: 'cmd+4',
  switchTab5: 'cmd+5',
  switchTab6: 'cmd+6',
  switchTab7: 'cmd+7',
  switchTab8: 'cmd+8',
  switchTab9: 'cmd+9',

  // Actions
  send: 'cmd+enter',
  clearChat: 'cmd+shift+backspace',
  search: 'cmd+f',
  settings: 'cmd+,',
  commandPalette: 'cmd+k',

  // Modes
  quickMode: 'cmd+shift+q',
  taskMode: 'cmd+shift+t',
  agentMode: 'cmd+shift+a',

  // UI
  toggleSidebar: 'cmd+b',
  toggleTheme: 'cmd+shift+l',
  focusInput: '/',
  focusChat: 'esc',

  // Tools
  selectTools: 'cmd+shift+w',
  selectKnowledge: 'cmd+shift+k',
  selectAgents: 'cmd+shift+g',

  // Developer
  toggleDebugConsole: 'cmd+shift+d',
  toggleDevTools: 'cmd+option+i',
} as const;

export type ShortcutKey = keyof typeof SHORTCUTS;

/**
 * Check if a key matches the platform (cmd on macOS, ctrl on Windows/Linux)
 */
export function getPlatformModifier(): 'cmd' | 'ctrl' {
  if (typeof window === 'undefined') return 'cmd';
  return navigator.platform.toLowerCase().includes('mac') ? 'cmd' : 'ctrl';
}

/**
 * Convert shortcut string to platform-specific display
 */
export function getShortcutDisplay(shortcut: string): string {
  const isMac = getPlatformModifier() === 'cmd';

  const display = shortcut
    .replace('cmd', isMac ? '⌘' : 'Ctrl')
    .replace('ctrl', 'Ctrl')
    .replace('shift', isMac ? '⇧' : 'Shift')
    .replace('option', isMac ? '⌥' : 'Alt')
    .replace('alt', 'Alt')
    .replace('enter', isMac ? '↵' : 'Enter')
    .replace('backspace', isMac ? '⌫' : 'Backspace')
    .replace('esc', 'Esc')
    .replace('+', isMac ? '' : '+');

  return display
    .split(isMac ? '' : '+')
    .map(key => key.charAt(0).toUpperCase() + key.slice(1))
    .join(isMac ? '' : '+');
}

/**
 * Format keyboard event to shortcut string
 */
export function formatKeyCombo(e: KeyboardEvent): string {
  const parts: string[] = [];

  const modifier = getPlatformModifier();
  if (e.metaKey || e.ctrlKey) parts.push(modifier);
  if (e.shiftKey) parts.push('shift');
  if (e.altKey) parts.push(e.metaKey ? 'option' : 'alt');

  // Normalize key name
  let key = e.key.toLowerCase();
  if (key === ' ') key = 'space';
  if (key === 'arrowup') key = 'up';
  if (key === 'arrowdown') key = 'down';
  if (key === 'arrowleft') key = 'left';
  if (key === 'arrowright') key = 'right';

  // Don't add modifier keys as the key itself
  if (!['control', 'meta', 'shift', 'alt'].includes(key)) {
    parts.push(key);
  }

  return parts.join('+');
}

/**
 * Check if two shortcut strings match
 */
export function matchesShortcut(combo: string, shortcut: string): boolean {
  // Normalize both strings
  const normalize = (s: string) => {
    const modifier = getPlatformModifier();
    return s
      .replace('cmd', modifier)
      .replace('ctrl', modifier)
      .toLowerCase()
      .split('+')
      .sort()
      .join('+');
  };

  return normalize(combo) === normalize(shortcut);
}

/**
 * Check if event matches any shortcut
 */
export function findMatchingShortcut(e: KeyboardEvent): ShortcutKey | null {
  const combo = formatKeyCombo(e);

  for (const [key, shortcut] of Object.entries(SHORTCUTS)) {
    if (matchesShortcut(combo, shortcut)) {
      return key as ShortcutKey;
    }
  }

  return null;
}

/**
 * Prevent default browser shortcuts
 */
export function shouldPreventDefault(e: KeyboardEvent): boolean {
  const combo = formatKeyCombo(e);

  // List of shortcuts we want to override browser defaults for
  const overrides = [
    SHORTCUTS.newTab,
    SHORTCUTS.closeTab,
    SHORTCUTS.newChat,
    SHORTCUTS.commandPalette,
    SHORTCUTS.search,
    SHORTCUTS.settings,
  ];

  return overrides.some(shortcut => matchesShortcut(combo, shortcut));
}
