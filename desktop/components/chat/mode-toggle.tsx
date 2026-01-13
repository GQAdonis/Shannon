import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Zap, Workflow, Info } from 'lucide-react';
import type { ChatMode } from '@/lib/chat/types';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip';

interface ModeToggleProps {
  mode: ChatMode;
  suggestedMode?: ChatMode;
  confidence?: number;
  onChange: (mode: ChatMode) => void;
  disabled?: boolean;
}

export function ModeToggle({
  mode,
  suggestedMode,
  confidence,
  onChange,
  disabled = false,
}: ModeToggleProps) {
  return (
    <div className="flex items-center gap-3 p-3 border rounded-lg bg-background">
      <div className="flex items-center gap-2">
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant={mode === 'quick' ? 'default' : 'outline'}
                size="sm"
                onClick={() => onChange('quick')}
                disabled={disabled}
                className="gap-2"
              >
                <Zap className="h-4 w-4" />
                Quick Chat
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p className="max-w-xs">
                Fast, streaming responses for simple questions and conversations.
                Best for: definitions, explanations, quick answers.
              </p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>

        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant={mode === 'task' ? 'default' : 'outline'}
                size="sm"
                onClick={() => onChange('task')}
                disabled={disabled}
                className="gap-2"
              >
                <Workflow className="h-4 w-4" />
                Task Chat
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p className="max-w-xs">
                Orchestrated, multi-agent execution for complex tasks.
                Best for: research, analysis, multi-step workflows.
              </p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </div>

      {suggestedMode && mode !== suggestedMode && confidence && confidence > 0.7 && (
        <Badge variant="secondary" className="gap-1.5 text-xs">
          <Info className="h-3 w-3" />
          Suggested: {suggestedMode === 'quick' ? 'Quick' : 'Task'}
          <span className="text-muted-foreground">
            ({Math.round(confidence * 100)}%)
          </span>
        </Badge>
      )}

      <div className="flex-1" />

      <div className="text-xs text-muted-foreground">
        {mode === 'quick' ? (
          <span>⚡ Quick mode - Instant responses</span>
        ) : (
          <span>🔄 Task mode - Orchestrated execution</span>
        )}
      </div>
    </div>
  );
}
