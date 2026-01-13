import { ScrollArea } from '@/components/ui/scroll-area';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import { Bot, User } from 'lucide-react';
import type { ChatMessage } from '@/lib/chat/types';
import { cn } from '@/lib/utils';

interface MessageListProps {
  messages: ChatMessage[];
  isStreaming?: boolean;
  className?: string;
}

export function MessageList({ messages, isStreaming = false, className }: MessageListProps) {
  return (
    <ScrollArea className={cn('flex-1 p-4', className)}>
      <div className="space-y-4 max-w-3xl mx-auto">
        {messages.map((message, index) => (
          <MessageItem
            key={`${message.timestamp}-${index}`}
            message={message}
            isLatest={index === messages.length - 1}
            isStreaming={isStreaming && index === messages.length - 1}
          />
        ))}
      </div>
    </ScrollArea>
  );
}

interface MessageItemProps {
  message: ChatMessage;
  isLatest: boolean;
  isStreaming: boolean;
}

function MessageItem({ message, isLatest, isStreaming }: MessageItemProps) {
  const isUser = message.role === 'user';
  const isSystem = message.role === 'system';

  if (isSystem) {
    return (
      <div className="flex justify-center">
        <div className="text-xs text-muted-foreground bg-muted px-3 py-1 rounded-full">
          {message.content}
        </div>
      </div>
    );
  }

  return (
    <div
      className={cn(
        'flex gap-3 group',
        isUser ? 'flex-row-reverse' : 'flex-row'
      )}
    >
      <Avatar className={cn('h-8 w-8', isUser ? 'bg-primary' : 'bg-secondary')}>
        <AvatarFallback>
          {isUser ? (
            <User className="h-4 w-4" />
          ) : (
            <Bot className="h-4 w-4" />
          )}
        </AvatarFallback>
      </Avatar>

      <div
        className={cn(
          'flex-1 space-y-2 rounded-lg px-4 py-3',
          isUser
            ? 'bg-primary text-primary-foreground'
            : 'bg-muted'
        )}
      >
        <div className="flex items-center gap-2">
          <span className="text-xs font-medium">
            {isUser ? 'You' : 'Shannon'}
          </span>
          <span className="text-xs opacity-50">
            {new Date(message.timestamp).toLocaleTimeString()}
          </span>
          {isLatest && isStreaming && !isUser && (
            <span className="flex items-center gap-1 text-xs opacity-70">
              <span className="inline-block h-1 w-1 rounded-full bg-current animate-pulse" />
              <span className="inline-block h-1 w-1 rounded-full bg-current animate-pulse delay-75" />
              <span className="inline-block h-1 w-1 rounded-full bg-current animate-pulse delay-150" />
            </span>
          )}
        </div>

        <div className="text-sm whitespace-pre-wrap break-words">
          {message.content}
        </div>
      </div>
    </div>
  );
}
