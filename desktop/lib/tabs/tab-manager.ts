/**
 * Tab Manager Service
 *
 * Manages multiple concurrent chat tabs with persistence
 */

import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import { MessageSquare, Bot, FileCode, Book } from 'lucide-react';
import type { ChatTab, TabType, TabConfig } from './types';

const MAX_TABS = 10;
const STORAGE_KEY = 'shannon_tabs';

interface TabStore {
  tabs: ChatTab[];
  activeTabId: string | null;

  // Actions
  createTab: (config: TabConfig) => string;
  closeTab: (tabId: string) => void;
  switchTab: (tabId: string) => void;
  updateTab: (tabId: string, updates: Partial<ChatTab>) => void;
  pinTab: (tabId: string) => void;
  unpinTab: (tabId: string) => void;
  closeAllTabs: () => void;
  closeOtherTabs: (tabId: string) => void;
  duplicateTab: (tabId: string) => string | null;

  // Queries
  getTab: (tabId: string) => ChatTab | undefined;
  getActiveTab: () => ChatTab | undefined;
  canCreateTab: () => boolean;
}

/**
 * Generate a default title for a tab based on its type
 */
function generateTitle(type: TabType, index: number): string {
  const titles: Record<TabType, string> = {
    chat: `Chat ${index}`,
    agent: `Agent ${index}`,
    artifact: `Artifact ${index}`,
    knowledge: `Knowledge ${index}`,
  };
  return titles[type] || `Tab ${index}`;
}

/**
 * Get icon for tab type
 */
function getIconForType(type: TabType) {
  const icons: Record<TabType, typeof MessageSquare> = {
    chat: MessageSquare,
    agent: Bot,
    artifact: FileCode,
    knowledge: Book,
  };
  return icons[type] || MessageSquare;
}

/**
 * Generate a unique ID
 */
function generateId(): string {
  return `tab_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

export const useTabManager = create<TabStore>()(
  persist(
    (set, get) => ({
      tabs: [],
      activeTabId: null,

      createTab: (config: TabConfig) => {
        const state = get();

        // Check if we can create more tabs
        if (!state.canCreateTab()) {
          // Close least recently used unpinned tab
          const unpinnedTabs = state.tabs
            .filter(t => !t.isPinned)
            .sort((a, b) => a.lastActive.getTime() - b.lastActive.getTime());

          if (unpinnedTabs.length > 0) {
            state.closeTab(unpinnedTabs[0].id);
          } else {
            console.warn('Cannot create more tabs: max tabs reached and all are pinned');
            return state.activeTabId || '';
          }
        }

        // Create new tab
        const tabIndex = state.tabs.length + 1;
        const newTab: ChatTab = {
          id: generateId(),
          title: config.title || generateTitle(config.type, tabIndex),
          type: config.type,
          conversationId: config.conversationId,
          agentId: config.agentId,
          icon: config.icon || getIconForType(config.type),
          isDirty: false,
          lastActive: new Date(),
          isPinned: false,
        };

        set(state => ({
          tabs: [...state.tabs, newTab],
          activeTabId: newTab.id,
        }));

        return newTab.id;
      },

      closeTab: (tabId: string) => {
        const state = get();
        const tabIndex = state.tabs.findIndex(t => t.id === tabId);

        if (tabIndex === -1) return;

        const newTabs = state.tabs.filter(t => t.id !== tabId);

        // Determine new active tab
        let newActiveTabId = state.activeTabId;
        if (state.activeTabId === tabId) {
          if (newTabs.length > 0) {
            // Try to activate the next tab, or the previous one
            newActiveTabId = newTabs[Math.min(tabIndex, newTabs.length - 1)].id;
          } else {
            newActiveTabId = null;
          }
        }

        set({
          tabs: newTabs,
          activeTabId: newActiveTabId,
        });
      },

      switchTab: (tabId: string) => {
        const state = get();
        const tab = state.tabs.find(t => t.id === tabId);

        if (!tab) return;

        set(state => ({
          tabs: state.tabs.map(t =>
            t.id === tabId
              ? { ...t, lastActive: new Date() }
              : t
          ),
          activeTabId: tabId,
        }));
      },

      updateTab: (tabId: string, updates: Partial<ChatTab>) => {
        set(state => ({
          tabs: state.tabs.map(t =>
            t.id === tabId
              ? { ...t, ...updates, lastActive: new Date() }
              : t
          ),
        }));
      },

      pinTab: (tabId: string) => {
        set(state => ({
          tabs: state.tabs.map(t =>
            t.id === tabId ? { ...t, isPinned: true } : t
          ),
        }));
      },

      unpinTab: (tabId: string) => {
        set(state => ({
          tabs: state.tabs.map(t =>
            t.id === tabId ? { ...t, isPinned: false } : t
          ),
        }));
      },

      closeAllTabs: () => {
        set({ tabs: [], activeTabId: null });
      },

      closeOtherTabs: (tabId: string) => {
        set(state => {
          const keepTab = state.tabs.find(t => t.id === tabId);
          if (!keepTab) return state;

          return {
            tabs: [keepTab],
            activeTabId: tabId,
          };
        });
      },

      duplicateTab: (tabId: string) => {
        const state = get();
        const tab = state.tabs.find(t => t.id === tabId);

        if (!tab || !state.canCreateTab()) return null;

        const newTab: ChatTab = {
          ...tab,
          id: generateId(),
          title: `${tab.title} (Copy)`,
          isDirty: false,
          lastActive: new Date(),
          isPinned: false,
        };

        set(state => ({
          tabs: [...state.tabs, newTab],
          activeTabId: newTab.id,
        }));

        return newTab.id;
      },

      getTab: (tabId: string) => {
        return get().tabs.find(t => t.id === tabId);
      },

      getActiveTab: () => {
        const state = get();
        return state.tabs.find(t => t.id === state.activeTabId);
      },

      canCreateTab: () => {
        return get().tabs.length < MAX_TABS;
      },
    }),
    {
      name: STORAGE_KEY,
      storage: createJSONStorage(() => localStorage),
      partialize: (state) => ({
        tabs: state.tabs.map(tab => ({
          ...tab,
          lastActive: tab.lastActive.toISOString(), // Serialize dates
        })),
        activeTabId: state.activeTabId,
      }),
      onRehydrateStorage: () => (state) => {
        if (state) {
          // Deserialize dates
          state.tabs = state.tabs.map(tab => ({
            ...tab,
            lastActive: new Date(tab.lastActive as unknown as string),
          }));
        }
      },
    }
  )
);

/**
 * Initialize the tab system with a default tab if none exist
 */
export function initializeTabs() {
  const { tabs, createTab } = useTabManager.getState();

  if (tabs.length === 0) {
    createTab({ type: 'chat', title: 'New Chat' });
  }
}
