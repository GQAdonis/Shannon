/**
 * Dual-Mode Chat System Components
 *
 * Phase 2.2 Implementation - Frontend Components
 */

// Main unified interface
export { ChatInterface } from './chat-interface';

// Individual mode components
export { QuickChat } from './quick-chat';
export { TaskChat } from './task-chat';

// UI components
export { ModeToggle } from './mode-toggle';
export { MessageList } from './message-list';
export { TaskProgress } from './task-progress';

// Services
export { QuickChatService } from '@/lib/chat/quick-chat';
export { TaskChatService } from '@/lib/chat/task-chat';
export { ModeDetector } from '@/lib/chat/mode-detector';

// Types
export type {
  ChatMode,
  ChatMessage,
  QuickChatConfig,
  TaskChatConfig,
  TaskHandle,
  TaskEvent,
  TaskEventData,
} from '@/lib/chat/types';

export {
  DEFAULT_QUICK_CHAT_CONFIG,
  DEFAULT_TASK_CHAT_CONFIG,
} from '@/lib/chat/types';
