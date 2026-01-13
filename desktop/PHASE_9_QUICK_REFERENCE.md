# Phase 9: UI/UX Polish - Quick Reference

## Quick Start

### 1. Install Dependencies
```bash
cd desktop
npm install
```

Dependencies include:
- `framer-motion` - Animations
- `@tanstack/react-virtual` - Virtual scrolling
- `use-debounce` - Debounced inputs

### 2. Tab System

```typescript
import { useTabManager } from '@/lib/tabs/tab-manager';
import { TabBar } from '@/components/tabs/tab-bar';

// In your layout
<TabBar />

// In your component
const { createTab, closeTab, switchTab } = useTabManager();
createTab({ type: 'chat', title: 'New Chat' });
```

### 3. Keyboard Shortcuts

```typescript
import { useKeyboardShortcuts } from '@/lib/keyboard/use-keyboard-shortcuts';

useKeyboardShortcuts({
  newChat: () => createTab({ type: 'chat' }),
  send: () => submitMessage(),
  settings: () => router.push('/settings'),
});
```

### 4. Command Palette

```typescript
import { CommandPalette } from '@/components/command-palette';

// Add to root layout
<CommandPalette />

// Opens with cmd+k automatically
```

### 5. Animations

```typescript
import { motion } from 'framer-motion';
import { fadeIn, messageAnimation } from '@/lib/animations/transitions';

<motion.div {...fadeIn}>
  Content fades in
</motion.div>

<motion.div {...messageAnimation}>
  Message appears with animation
</motion.div>
```

### 6. Responsive Design

```typescript
import { useResponsiveLayout } from '@/lib/responsive/use-responsive';

const { isMobile, isTablet, isDesktop } = useResponsiveLayout();

if (isMobile) {
  return <MobileView />;
}
return <DesktopView />;
```

### 7. Performance Optimizations

```typescript
import { 
  useDebouncedSearch, 
  useVirtualizedList 
} from '@/lib/performance/optimizations';

// Debounced search
const { search, results, isSearching } = useDebouncedSearch(
  async (query) => searchMessages(query),
  300
);

// Virtual scrolling
const { visibleItems, handleScroll, totalHeight } = useVirtualizedList(
  messages,
  100, // item height
  600  // container height
);
```

## Keyboard Shortcuts Reference

| Shortcut | Action |
|----------|--------|
| `cmd+k` | Open command palette |
| `cmd+n` | New chat |
| `cmd+t` | New tab |
| `cmd+w` | Close tab |
| `cmd+1-9` | Switch to tab 1-9 |
| `cmd+shift+]` | Next tab |
| `cmd+shift+[` | Previous tab |
| `cmd+enter` | Send message |
| `cmd+,` | Settings |
| `cmd+f` | Search |
| `/` | Focus input |
| `esc` | Focus chat |

## File Structure

```
desktop/
├── lib/
│   ├── tabs/
│   │   ├── types.ts              # Tab type definitions
│   │   └── tab-manager.ts        # Tab state management
│   ├── keyboard/
│   │   ├── shortcuts.ts          # Shortcut definitions
│   │   └── use-keyboard-shortcuts.ts
│   ├── animations/
│   │   └── transitions.ts        # Animation variants
│   ├── performance/
│   │   └── optimizations.ts      # Performance hooks
│   └── responsive/
│       └── use-responsive.ts     # Responsive utilities
└── components/
    ├── tabs/
    │   ├── tab.tsx               # Individual tab
    │   └── tab-bar.tsx           # Tab bar UI
    ├── command-palette.tsx       # Command palette (cmd+k)
    └── theme/
        └── theme-customizer.tsx  # Theme settings
```

## Integration Examples

### Example 1: Chat Page with Tabs

```typescript
// app/(app)/chat/page.tsx
'use client';

import { useEffect } from 'react';
import { TabBar } from '@/components/tabs/tab-bar';
import { initializeTabs, useTabManager } from '@/lib/tabs/tab-manager';
import { useKeyboardShortcuts } from '@/lib/keyboard/use-keyboard-shortcuts';

export default function ChatPage() {
  const { createTab, closeTab, activeTabId } = useTabManager();
  
  useEffect(() => {
    initializeTabs();
  }, []);
  
  useKeyboardShortcuts({
    newChat: () => createTab({ type: 'chat' }),
    closeTab: () => activeTabId && closeTab(activeTabId),
  });
  
  return (
    <div>
      <TabBar />
      <ChatInterface />
    </div>
  );
}
```

### Example 2: Animated Message List

```typescript
import { motion, AnimatePresence } from 'framer-motion';
import { messageAnimation } from '@/lib/animations/transitions';

export function MessageList({ messages }) {
  return (
    <div>
      <AnimatePresence mode="popLayout">
        {messages.map((message) => (
          <motion.div
            key={message.id}
            {...messageAnimation}
            layout
          >
            <Message message={message} />
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
}
```

### Example 3: Responsive Layout

```typescript
import { useResponsiveLayout } from '@/lib/responsive/use-responsive';

export function Layout({ children }) {
  const { isMobile, isDesktop } = useResponsiveLayout();
  
  return (
    <div className={cn(
      "grid gap-4",
      isMobile ? "grid-cols-1" : "grid-cols-2 lg:grid-cols-3"
    )}>
      {children}
    </div>
  );
}
```

### Example 4: Performance Optimized Search

```typescript
import { useDebouncedSearch } from '@/lib/performance/optimizations';

export function SearchBar() {
  const { search, results, isSearching } = useDebouncedSearch(
    async (query) => {
      const response = await fetch(`/api/search?q=${query}`);
      return response.json();
    },
    300
  );
  
  return (
    <div>
      <input onChange={(e) => search(e.target.value)} />
      {isSearching && <Spinner />}
      <SearchResults results={results} />
    </div>
  );
}
```

## Customization

### Custom Theme
```typescript
// Create custom theme JSON
{
  "colors": {
    "primary": "hsl(222.2, 47.4%, 11.2%)",
    "background": "hsl(0, 0%, 100%)"
  },
  "fonts": {
    "sans": "Inter, system-ui",
    "mono": "JetBrains Mono, monospace"
  },
  "spacing": { "scale": 1 },
  "radius": { "value": 0.5 }
}
```

### Custom Shortcuts
```typescript
// Add your own shortcuts
import { useKeyboardShortcut } from '@/lib/keyboard/use-keyboard-shortcuts';

useKeyboardShortcut('cmd+shift+p', () => {
  // Your custom action
});
```

### Custom Animations
```typescript
// Define custom animation variant
const myAnimation: Variants = {
  initial: { scale: 0, rotate: -180 },
  animate: { scale: 1, rotate: 0 },
  exit: { scale: 0, rotate: 180 },
};

<motion.div variants={myAnimation} />
```

## Performance Tips

1. **Virtual Scrolling**: Use for lists >100 items
2. **Debouncing**: Use for search/filter inputs (300ms recommended)
3. **Lazy Loading**: Load heavy components only when visible
4. **Memoization**: Use `React.memo()` for expensive components
5. **Reduced Motion**: Respect user preferences with `usePrefersReducedMotion()`

## Troubleshooting

### Tabs not persisting
- Check localStorage: `shannon_tabs` key
- Verify Zustand persist middleware is working

### Shortcuts not working
- Check if focus is in an input field
- Verify platform modifier (cmd on Mac, ctrl on Windows)

### Animations janky
- Check FPS with `useFPS()` hook
- Reduce animation complexity
- Use `transform` properties instead of `width/height`

### Search too slow
- Increase debounce delay (currently 300ms)
- Implement server-side pagination
- Add result caching

## See Also

- [Full Implementation Details](./PHASE_9_UX_POLISH_COMPLETE.md)
- [Shannon Desktop Cherry Parity Plan](../plans/shannon-desktop-cherry-parity-plan.md)
- [Phase 8: Settings System](./PHASE_8_SETTINGS_SYSTEM_SUMMARY.md)
