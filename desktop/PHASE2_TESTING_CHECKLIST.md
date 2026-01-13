# Phase 2.2 Testing Checklist

**Status**: Ready for Testing  
**Prerequisites**: Backend Phase 2.1 commands must be implemented in Tauri

## Pre-Testing Setup

1. **Start Development Server**
   ```bash
   cd desktop
   npm run dev
   ```

2. **Verify Backend Commands**
   - Check [`desktop/src-tauri/src/chat.rs`](desktop/src-tauri/src/chat.rs) exists
   - Verify commands registered in [`desktop/src-tauri/src/main.rs`](desktop/src-tauri/src/main.rs)
   - Ensure embedded API is running

3. **Navigate to Chat Page**
   - Open http://localhost:3000/chat
   - Or integrate into existing UI

---

## Test Suite 1: Quick Chat Mode

### Test 1.1: Basic Quick Chat
- [ ] Type "What is 2+2?" and send
- [ ] Verify Quick mode is active
- [ ] Response appears with streaming
- [ ] Answer is correct
- [ ] Latency < 500ms for first token ⚡

### Test 1.2: Streaming
- [ ] Send "Explain quantum computing"
- [ ] Characters appear progressively (not all at once)
- [ ] No lag or freezing
- [ ] Complete response is coherent

### Test 1.3: Follow-up Questions
- [ ] Send initial question
- [ ] Wait for response
- [ ] Send follow-up question
- [ ] Conversation context is maintained
- [ ] Previous messages visible in history

### Test 1.4: Error Handling
- [ ] Disconnect network mid-stream
- [ ] Verify error message appears
- [ ] Click "Retry stream" button
- [ ] Stream resumes successfully

### Test 1.5: Multi-turn Conversation
- [ ] Send 5+ messages back and forth
- [ ] Scroll works correctly
- [ ] No memory leaks (check dev tools)
- [ ] All messages display properly

---

## Test Suite 2: Task Chat Mode

### Test 2.1: Basic Task Submission
- [ ] Switch to Task mode
- [ ] Type "Research quantum computing applications"
- [ ] Select strategy (Auto, Chain of Thought, etc.)
- [ ] Submit task
- [ ] Task ID received and displayed

### Test 2.2: Progress Tracking
- [ ] Task progress bar appears
- [ ] Progress updates every ~1 second
- [ ] Progress percentage increases
- [ ] Status messages update
- [ ] Task completes successfully

### Test 2.3: Task Cancellation
- [ ] Submit complex task
- [ ] Click cancel button while running
- [ ] Task cancels within 2 seconds
- [ ] UI updates to show "Cancelled"
- [ ] Can submit new task after cancel

### Test 2.4: Strategy Selection
- [ ] Try each strategy:
  - [ ] Auto
  - [ ] Chain of Thought
  - [ ] Scientific Research
  - [ ] Exploratory Analysis
- [ ] Verify each uses different execution path
- [ ] Results vary appropriately

### Test 2.5: Complex Multi-Agent Task
- [ ] Submit: "Compare 5 AI frameworks in detail"
- [ ] Task runs for >30 seconds
- [ ] Multiple progress updates
- [ ] Final result is comprehensive
- [ ] Citations appear (if supported)

---

## Test Suite 3: Mode Detection & Switching

### Test 3.1: Auto-Detection - Quick Mode
- [ ] Type "What is Python?"
- [ ] Badge shows "Suggested: Quick" (if in Task mode)
- [ ] Confidence > 70%
- [ ] Suggestion disappears after switching

### Test 3.2: Auto-Detection - Task Mode
- [ ] Type "Research and analyze Python vs JavaScript for web development with pros, cons, use cases, and ecosystem comparison"
- [ ] Badge shows "Suggested: Task" (if in Quick mode)
- [ ] Confidence > 70%
- [ ] Reason indicates complex query

### Test 3.3: Manual Mode Switching
- [ ] Start in Quick mode
- [ ] Send a message
- [ ] Switch to Task mode
- [ ] Send another message
- [ ] Both messages appear in history
- [ ] Context is preserved

### Test 3.4: Auto-Detection Toggle
- [ ] Disable auto-detection via props
- [ ] Type various queries
- [ ] No suggestions appear
- [ ] Manual switching still works

---

## Test Suite 4: UI/UX

### Test 4.1: Responsive Design
- [ ] Resize window to mobile size (375px)
- [ ] All buttons remain accessible
- [ ] Text doesn't overflow
- [ ] Mode toggle works on mobile
- [ ] Messages scroll properly

### Test 4.2: Keyboard Shortcuts
- [ ] Enter sends message
- [ ] Shift+Enter creates new line
- [ ] Tab focuses next element
- [ ] Esc clears input (if implemented)

### Test 4.3: Visual Feedback
- [ ] Loading states show correctly
- [ ] Buttons disable during operations
- [ ] Progress bar animates smoothly
- [ ] Icons render correctly (Zap, Workflow, etc.)
- [ ] Color scheme matches design system

### Test 4.4: Accessibility
- [ ] Tab through all interactive elements
- [ ] Screen reader announces mode changes
- [ ] ARIA labels present
- [ ] Focus indicators visible
- [ ] Keyboard navigation works

---

## Test Suite 5: Integration

### Test 5.1: Component Isolation
- [ ] Import [`QuickChat`](desktop/components/chat/quick-chat.tsx) standalone
- [ ] Works without [`ChatInterface`](desktop/components/chat/chat-interface.tsx)
- [ ] Import [`TaskChat`](desktop/components/chat/task-chat.tsx) standalone
- [ ] Works independently

### Test 5.2: Custom Configuration
- [ ] Override Quick Chat config:
  ```tsx
  <QuickChat config={{ 
    provider: 'anthropic',
    model: 'claude-3',
    temperature: 0.5
  }} />
  ```
- [ ] Verify custom settings used
- [ ] Repeat for Task Chat config

### Test 5.3: Callbacks
- [ ] Test `onMessageSent` callback
- [ ] Test `onTaskSubmitted` callback
- [ ] Verify correct data passed
- [ ] No errors in console

### Test 5.4: State Management
- [ ] Open DevTools > Redux tab
- [ ] Send messages in both modes
- [ ] Verify state updates correctly
- [ ] No stale state issues
- [ ] State resets on mode switch (if designed to)

---

## Test Suite 6: Performance

### Test 6.1: Quick Chat Latency ⚡
- [ ] Send 10 quick questions
- [ ] Measure time to first token each time
- [ ] Average < 500ms target
- [ ] No degradation over time
- [ ] Memory usage stable

### Test 6.2: Task Chat Efficiency
- [ ] Submit task
- [ ] Monitor polling requests (Network tab)
- [ ] Polling interval ~1 second
- [ ] No excessive requests
- [ ] Polling stops after completion

### Test 6.3: Memory Leaks
- [ ] Send 50+ messages
- [ ] Check memory usage (Performance tab)
- [ ] No continuous growth
- [ ] Force garbage collection
- [ ] Memory returns to baseline

### Test 6.4: Concurrent Operations
- [ ] Open multiple chat instances
- [ ] Use both modes simultaneously
- [ ] No conflicts or race conditions
- [ ] Each instance independent

---

## Test Suite 7: Edge Cases

### Test 7.1: Empty Input
- [ ] Try sending empty message
- [ ] Button should be disabled
- [ ] No API call made
- [ ] No error message

### Test 7.2: Very Long Input
- [ ] Paste 5000+ character message
- [ ] Textarea expands appropriately
- [ ] Message sends successfully
- [ ] Response handles long input

### Test 7.3: Special Characters
- [ ] Send message with emojis 😀🎉
- [ ] Send code blocks with \`\`\`
- [ ] Send markdown syntax
- [ ] Send Unicode characters
- [ ] All render correctly

### Test 7.4: Rapid Submissions
- [ ] Send messages rapidly (spam Enter)
- [ ] System handles gracefully
- [ ] No crashes or errors
- [ ] Rate limiting works (if implemented)

### Test 7.5: Network Failures
- [ ] Start task
- [ ] Disable network mid-execution
- [ ] Re-enable network
- [ ] Verify recovery mechanism
- [ ] Error messages clear

---

## Test Suite 8: Cross-Browser Testing

### Test 8.1: Chrome/Edge
- [ ] All features work
- [ ] Streaming smooth
- [ ] No console errors

### Test 8.2: Firefox
- [ ] All features work
- [ ] Streaming smooth
- [ ] No console errors

### Test 8.3: Safari
- [ ] All features work
- [ ] Streaming smooth
- [ ] No console errors

### Test 8.4: Mobile Browsers
- [ ] iOS Safari
- [ ] Android Chrome
- [ ] Touch interactions work
- [ ] Virtual keyboard doesn't break layout

---

## Performance Benchmarks

Record actual measurements:

| Metric | Target | Actual | Pass/Fail |
|--------|--------|--------|-----------|
| Quick Chat - First Token | <500ms | ___ ms | ⬜ |
| Quick Chat - Full Response | <2s | ___ s | ⬜ |
| Task Submission | <200ms | ___ ms | ⬜ |
| Mode Switch Delay | <100ms | ___ ms | ⬜ |
| Progress Update Interval | ~1s | ___ s | ⬜ |
| Memory Usage (50 msgs) | <100MB | ___ MB | ⬜ |

---

## Known Issues to Document

During testing, document any issues found:

1. **Issue**: _____________
   - **Severity**: Critical / High / Medium / Low
   - **Steps to Reproduce**: _______________
   - **Expected**: _______________
   - **Actual**: _______________

---

## Sign-off

- [ ] All critical tests pass
- [ ] Performance benchmarks met
- [ ] No console errors in production build
- [ ] Documentation updated with findings
- [ ] Ready for Phase 3

**Tester**: _______________  
**Date**: _______________  
**Build**: _______________
