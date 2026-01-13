'use client';

import { useState, useEffect, useCallback } from 'react';
import { ModeToggle } from './mode-toggle';
import { QuickChat } from './quick-chat';
import { TaskChat } from './task-chat';
import { ModeDetector } from '@/lib/chat/mode-detector';
import type { ChatMode } from '@/lib/chat/types';
import { Card } from '@/components/ui/card';

interface ChatInterfaceProps {
  defaultMode?: ChatMode;
  autoDetect?: boolean;
  className?: string;
}

export function ChatInterface({
  defaultMode = 'quick',
  autoDetect = true,
  className,
}: ChatInterfaceProps) {
  const [mode, setMode] = useState<ChatMode>(defaultMode);
  const [suggestedMode, setSuggestedMode] = useState<ChatMode>();
  const [confidence, setConfidence] = useState<number>();
  const [inputBuffer, setInputBuffer] = useState('');
  const detector = new ModeDetector();

  const handleInputChange = useCallback(
    (query: string) => {
      setInputBuffer(query);

      if (!autoDetect || query.length < 20) {
        setSuggestedMode(undefined);
        setConfidence(undefined);
        return;
      }

      // Debounce detection
      const timeoutId = setTimeout(() => {
        const detection = detector.detectModeSync(query);

        // Only suggest if different from current mode and confident
        if (detection.mode !== mode && detection.confidence > 0.7) {
          setSuggestedMode(detection.mode);
          setConfidence(detection.confidence);
        } else {
          setSuggestedMode(undefined);
          setConfidence(undefined);
        }
      }, 300);

      return () => clearTimeout(timeoutId);
    },
    [autoDetect, mode, detector]
  );

  const handleModeChange = (newMode: ChatMode) => {
    setMode(newMode);
    setSuggestedMode(undefined);
    setConfidence(undefined);
  };

  const handleMessageSent = () => {
    setInputBuffer('');
    setSuggestedMode(undefined);
    setConfidence(undefined);
  };

  return (
    <div className={className}>
      <Card className="h-full flex flex-col overflow-hidden">
        <div className="p-4 border-b bg-muted/50">
          <ModeToggle
            mode={mode}
            suggestedMode={suggestedMode}
            confidence={confidence}
            onChange={handleModeChange}
          />
        </div>

        <div className="flex-1 overflow-hidden">
          {mode === 'quick' ? (
            <QuickChat onMessageSent={handleMessageSent} />
          ) : (
            <TaskChat onTaskSubmitted={handleMessageSent} />
          )}
        </div>
      </Card>
    </div>
  );
}
