/**
 * Tab Types for Shannon Desktop
 *
 * Defines the tab system types for multi-chat management
 */

import { type LucideIcon } from 'lucide-react';

export type TabType = 'chat' | 'agent' | 'artifact' | 'knowledge';

export interface ChatTab {
  id: string;
  title: string;
  type: TabType;
  conversationId?: string;
  agentId?: string;
  icon?: LucideIcon;
  isDirty: boolean;
  lastActive: Date;
  isPinned?: boolean;
}

export interface TabManagerState {
  tabs: ChatTab[];
  activeTabId: string | null;
  maxTabs: number;
}

export interface TabConfig {
  type: TabType;
  title?: string;
  conversationId?: string;
  agentId?: string;
  icon?: LucideIcon;
}

export interface TabManagerEvents {
  onTabCreated: (tab: ChatTab) => void;
  onTabClosed: (tabId: string) => void;
  onTabSwitched: (tabId: string) => void;
  onTabUpdated: (tab: ChatTab) => void;
}
