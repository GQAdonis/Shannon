'use client';

import { ChatInterface } from '@/components/chat';

export default function ChatPage() {
  return (
    <div className="h-full">
      <ChatInterface defaultMode="quick" autoDetect={true} className="h-full" />
    </div>
  );
}
