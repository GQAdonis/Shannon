/**
 * MCP Server Card Component
 *
 * Displays an MCP server configuration with status and controls.
 */

'use client';

import type { MCPServerConfig, ServerStatus } from '@/lib/mcp/types';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Play, Square, Trash2, Wrench } from 'lucide-react';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog';

interface ServerCardProps {
  server: MCPServerConfig;
  onStart: (id: string) => void;
  onStop: (id: string) => void;
  onRemove: (id: string) => void;
}

function getStatusColor(status: ServerStatus): string {
  if (status.type === 'Running') return 'bg-green-500';
  if (status.type === 'Starting') return 'bg-yellow-500';
  if (status.type === 'Error') return 'bg-red-500';
  return 'bg-gray-500';
}

function getStatusLabel(status: ServerStatus): string {
  if (status.type === 'Error') {
    return `Error: ${status.message}`;
  }
  return status.type;
}

export function ServerCard({ server, onStart, onStop, onRemove }: ServerCardProps) {
  const isRunning = server.status.type === 'Running';
  const isStarting = server.status.type === 'Starting';
  const canStart = !isRunning && !isStarting;
  const canStop = isRunning || isStarting;

  return (
    <Card className="flex flex-col">
      <CardHeader>
        <div className="flex items-start justify-between">
          <div className="flex-1">
            <CardTitle className="flex items-center gap-2">
              {server.name}
              <div className={`h-2 w-2 rounded-full ${getStatusColor(server.status)}`} />
            </CardTitle>
            <CardDescription className="mt-1">
              {server.description || 'No description'}
            </CardDescription>
          </div>
        </div>
      </CardHeader>

      <CardContent className="flex-1 space-y-3">
        {/* Status Badge */}
        <div>
          <Badge variant={isRunning ? 'default' : 'secondary'}>
            {getStatusLabel(server.status)}
          </Badge>
        </div>

        {/* Command Info */}
        <div className="text-sm space-y-1">
          <div className="text-muted-foreground">
            <span className="font-medium">Command:</span> {server.command}
          </div>
          {server.args.length > 0 && (
            <div className="text-muted-foreground text-xs">
              Args: {server.args.join(' ')}
            </div>
          )}
        </div>

        {/* Tools Count */}
        {server.tools.length > 0 && (
          <div className="flex items-center gap-2 text-sm">
            <Wrench className="h-4 w-4 text-muted-foreground" />
            <span className="text-muted-foreground">
              {server.tools.length} tool{server.tools.length !== 1 ? 's' : ''} available
            </span>
          </div>
        )}

        {/* Auto-start indicator */}
        {server.autoStart && (
          <div className="text-xs text-muted-foreground">
            Auto-starts on launch
          </div>
        )}
      </CardContent>

      <CardFooter className="flex gap-2">
        {canStart && (
          <Button
            variant="default"
            size="sm"
            onClick={() => onStart(server.id)}
            className="flex-1"
          >
            <Play className="h-4 w-4 mr-2" />
            Start
          </Button>
        )}
        {canStop && (
          <Button
            variant="secondary"
            size="sm"
            onClick={() => onStop(server.id)}
            className="flex-1"
          >
            <Square className="h-4 w-4 mr-2" />
            Stop
          </Button>
        )}
        <AlertDialog>
          <AlertDialogTrigger asChild>
            <Button variant="destructive" size="sm">
              <Trash2 className="h-4 w-4" />
            </Button>
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>Remove MCP Server?</AlertDialogTitle>
              <AlertDialogDescription>
                Are you sure you want to remove &quot;{server.name}&quot;? This action cannot be undone.
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>Cancel</AlertDialogCancel>
              <AlertDialogAction onClick={() => onRemove(server.id)}>
                Remove
              </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      </CardFooter>
    </Card>
  );
}
