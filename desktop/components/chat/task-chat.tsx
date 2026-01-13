'use client';

import { useState } from 'react';
import { TaskChatService } from '@/lib/chat/task-chat';
import type { ChatMessage, TaskChatConfig, TaskHandle } from '@/lib/chat/types';
import { DEFAULT_TASK_CHAT_CONFIG } from '@/lib/chat/types';
import { MessageList } from './message-list';
import { TaskProgress } from './task-progress';
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { Send, Loader2 } from 'lucide-react';
import { toast } from 'sonner';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Label } from '@/components/ui/label';

interface TaskChatProps {
  config?: Partial<TaskChatConfig>;
  onTaskSubmitted?: (taskId: string) => void;
}

export function TaskChat({ config, onTaskSubmitted }: TaskChatProps) {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [currentTask, setCurrentTask] = useState<TaskHandle | null>(null);
  const [isExecuting, setIsExecuting] = useState(false);
  const [strategy, setStrategy] = useState<TaskChatConfig['strategy']>('auto');
  const service = new TaskChatService();

  const finalConfig: TaskChatConfig = {
    ...DEFAULT_TASK_CHAT_CONFIG,
    ...config,
    strategy,
  };

  const handleSubmit = async () => {
    if (!input.trim() || isExecuting) return;

    const userMessage: ChatMessage = {
      role: 'user',
      content: input.trim(),
      timestamp: new Date().toISOString(),
    };

    const userContent = input.trim();
    setInput('');
    setMessages(prev => [...prev, userMessage]);

    setIsExecuting(true);

    try {
      // Submit task
      const taskHandle = await service.submitTask(userContent, [], finalConfig);
      setCurrentTask(taskHandle);
      onTaskSubmitted?.(taskHandle.taskId);

      // Add system message
      setMessages(prev => [
        ...prev,
        {
          role: 'system',
          content: `Task submitted: ${taskHandle.taskId}`,
          timestamp: new Date().toISOString(),
        },
      ]);

      // Stream updates
      for await (const event of service.streamUpdates(taskHandle.taskId)) {
        if (event.type === 'progress') {
          setCurrentTask(prev =>
            prev
              ? {
                  ...prev,
                  progress: event.data.progress ?? prev.progress,
                  message: event.data.message ?? prev.message,
                }
              : null
          );
        }

        if (event.type === 'state_changed') {
          setCurrentTask(prev =>
            prev
              ? {
                  ...prev,
                  state: event.data.state ?? prev.state,
                  message: event.data.message ?? prev.message,
                }
              : null
          );
        }

        if (event.type === 'partial_result') {
          // Add partial result as assistant message
          if (event.data.content) {
            setMessages(prev => [
              ...prev,
              {
                role: 'assistant',
                content: event.data.content!,
                timestamp: new Date().toISOString(),
              },
            ]);
          }
        }

        if (event.type === 'completed') {
          // Add final result
          setMessages(prev => [
            ...prev,
            {
              role: 'assistant',
              content: event.data.message || 'Task completed successfully',
              timestamp: new Date().toISOString(),
            },
          ]);
          setCurrentTask(null);
          toast.success('Task completed');
        }

        if (event.type === 'failed') {
          setMessages(prev => [
            ...prev,
            {
              role: 'assistant',
              content: `Task failed: ${event.data.error}`,
              timestamp: new Date().toISOString(),
            },
          ]);
          setCurrentTask(null);
          toast.error('Task failed', {
            description: event.data.error,
          });
        }
      }
    } catch (error) {
      console.error('Task chat error:', error);
      toast.error('Failed to submit task', {
        description: error instanceof Error ? error.message : 'Unknown error',
      });

      setMessages(prev => [
        ...prev,
        {
          role: 'assistant',
          content: 'Sorry, I encountered an error submitting your task. Please try again.',
          timestamp: new Date().toISOString(),
        },
      ]);
    } finally {
      setIsExecuting(false);
    }
  };

  const handleCancel = async () => {
    if (!currentTask) return;

    try {
      await service.cancelTask(currentTask.taskId);
      setCurrentTask(null);
      toast.info('Task cancelled');

      setMessages(prev => [
        ...prev,
        {
          role: 'system',
          content: 'Task cancelled by user',
          timestamp: new Date().toISOString(),
        },
      ]);
    } catch (error) {
      console.error('Cancel task error:', error);
      toast.error('Failed to cancel task');
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  };

  return (
    <div className="flex flex-col h-full">
      {currentTask && (
        <div className="p-4 border-b">
          <TaskProgress task={currentTask} onCancel={handleCancel} />
        </div>
      )}

      <MessageList messages={messages} isStreaming={isExecuting} />

      <div className="border-t p-4 bg-background space-y-3">
        <div className="max-w-3xl mx-auto">
          <Label htmlFor="strategy" className="text-xs text-muted-foreground">
            Execution Strategy
          </Label>
          <Select
            value={strategy}
            onValueChange={(value: TaskChatConfig['strategy']) => setStrategy(value)}
            disabled={isExecuting}
          >
            <SelectTrigger id="strategy" className="w-full">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="auto">Auto (Recommended)</SelectItem>
              <SelectItem value="chain_of_thought">Chain of Thought</SelectItem>
              <SelectItem value="scientific">Scientific Research</SelectItem>
              <SelectItem value="exploratory">Exploratory Analysis</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div className="max-w-3xl mx-auto flex gap-2">
          <Textarea
            value={input}
            onChange={e => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Describe your task (Task mode)..."
            disabled={isExecuting}
            className="resize-none"
            rows={3}
          />
          <Button
            onClick={handleSubmit}
            disabled={!input.trim() || isExecuting}
            size="icon"
            className="h-full aspect-square"
          >
            {isExecuting ? (
              <Loader2 className="h-5 w-5 animate-spin" />
            ) : (
              <Send className="h-5 w-5" />
            )}
          </Button>
        </div>

        <div className="max-w-3xl mx-auto">
          <p className="text-xs text-muted-foreground">
            Press Enter to submit task, Shift+Enter for new line
          </p>
        </div>
      </div>
    </div>
  );
}
