'use client';

import { useState } from 'react';
import { QuickChatService } from '@/lib/chat/quick-chat';
import type { ChatMessage, QuickChatConfig } from '@/lib/chat/types';
import { DEFAULT_QUICK_CHAT_CONFIG } from '@/lib/chat/types';
import { MessageList } from './message-list';
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { Send, Loader2 } from 'lucide-react';
import { toast } from 'sonner';

interface QuickChatProps {
  config?: Partial<QuickChatConfig>;
  onMessageSent?: (message: string) => void;
}

export function QuickChat({ config, onMessageSent }: QuickChatProps) {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [isStreaming, setIsStreaming] = useState(false);
  const service = new QuickChatService();

  const finalConfig: QuickChatConfig = {
    ...DEFAULT_QUICK_CHAT_CONFIG,
    ...config,
  };

  const handleSend = async () => {
    if (!input.trim() || isStreaming) return;

    const userMessage: ChatMessage = {
      role: 'user',
      content: input.trim(),
      timestamp: new Date().toISOString(),
    };

    const userContent = input.trim();
    setInput('');
    setMessages(prev => [...prev, userMessage]);
    onMessageSent?.(userContent);

    setIsStreaming(true);
    let assistantContent = '';

    try {
      const stream = await service.sendMessage(
        userContent,
        messages,
        finalConfig
      );

      // Create placeholder for assistant message
      setMessages(prev => [
        ...prev,
        {
          role: 'assistant',
          content: '',
          timestamp: new Date().toISOString(),
        },
      ]);

      for await (const chunk of stream) {
        assistantContent += chunk;

        // Update assistant message in real-time
        setMessages(prev => {
          const newMessages = [...prev];
          const lastMessage = newMessages[newMessages.length - 1];

          if (lastMessage?.role === 'assistant') {
            lastMessage.content = assistantContent;
          }

          return newMessages;
        });
      }
    } catch (error) {
      console.error('Quick chat error:', error);
      toast.error('Failed to send message', {
        description: error instanceof Error ? error.message : 'Unknown error',
      });

      // Add error message
      setMessages(prev => [
        ...prev,
        {
          role: 'assistant',
          content: 'Sorry, I encountered an error processing your message. Please try again.',
          timestamp: new Date().toISOString(),
        },
      ]);
    } finally {
      setIsStreaming(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <div className="flex flex-col h-full">
      <MessageList messages={messages} isStreaming={isStreaming} />

      <div className="border-t p-4 bg-background">
        <div className="max-w-3xl mx-auto flex gap-2">
          <Textarea
            value={input}
            onChange={e => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Ask anything (Quick mode)..."
            disabled={isStreaming}
            className="resize-none"
            rows={3}
          />
          <Button
            onClick={handleSend}
            disabled={!input.trim() || isStreaming}
            size="icon"
            className="h-full aspect-square"
          >
            {isStreaming ? (
              <Loader2 className="h-5 w-5 animate-spin" />
            ) : (
              <Send className="h-5 w-5" />
            )}
          </Button>
        </div>
        <div className="max-w-3xl mx-auto mt-2">
          <p className="text-xs text-muted-foreground">
            Press Enter to send, Shift+Enter for new line
          </p>
        </div>
      </div>
    </div>
  );
}
