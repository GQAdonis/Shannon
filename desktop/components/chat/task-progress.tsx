import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Progress } from '@/components/ui/progress';
import { Button } from '@/components/ui/button';
import {
  Loader2,
  CheckCircle2,
  XCircle,
  Clock,
  AlertCircle,
  X,
} from 'lucide-react';
import type { TaskHandle } from '@/lib/chat/types';
import { cn } from '@/lib/utils';

interface TaskProgressProps {
  task: TaskHandle;
  onCancel?: () => void;
  className?: string;
}

export function TaskProgress({ task, onCancel, className }: TaskProgressProps) {
  const { state, progress, message, taskId } = task;

  const getStatusIcon = () => {
    switch (state) {
      case 'pending':
        return <Clock className="h-5 w-5 text-muted-foreground animate-pulse" />;
      case 'running':
        return <Loader2 className="h-5 w-5 text-blue-500 animate-spin" />;
      case 'completed':
        return <CheckCircle2 className="h-5 w-5 text-green-500" />;
      case 'failed':
        return <XCircle className="h-5 w-5 text-destructive" />;
      default:
        return <AlertCircle className="h-5 w-5 text-muted-foreground" />;
    }
  };

  const getStatusLabel = () => {
    switch (state) {
      case 'pending':
        return 'Pending';
      case 'running':
        return 'Running';
      case 'completed':
        return 'Completed';
      case 'failed':
        return 'Failed';
      default:
        return 'Unknown';
    }
  };

  const getProgressColor = () => {
    switch (state) {
      case 'completed':
        return 'bg-green-500';
      case 'failed':
        return 'bg-destructive';
      case 'running':
        return 'bg-blue-500';
      default:
        return 'bg-muted-foreground';
    }
  };

  return (
    <Card className={cn('border-2', className)}>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <CardTitle className="text-sm font-medium flex items-center gap-2">
            {getStatusIcon()}
            <span>Task {getStatusLabel()}</span>
          </CardTitle>
          {onCancel && (state === 'pending' || state === 'running') && (
            <Button
              variant="ghost"
              size="sm"
              onClick={onCancel}
              className="h-6 w-6 p-0"
            >
              <X className="h-4 w-4" />
            </Button>
          )}
        </div>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="space-y-1.5">
          <div className="flex items-center justify-between text-xs">
            <span className="text-muted-foreground">Progress</span>
            <span className="font-medium">{progress}%</span>
          </div>
          <Progress value={progress} className={cn("h-2", getProgressColor())} />
        </div>

        {message && (
          <div className="text-xs text-muted-foreground bg-muted px-3 py-2 rounded-md">
            {message}
          </div>
        )}

        <div className="text-xs text-muted-foreground font-mono">
          ID: {taskId.slice(0, 8)}...
        </div>
      </CardContent>
    </Card>
  );
}
