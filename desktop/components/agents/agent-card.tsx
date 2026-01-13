/**
 * Agent card component for displaying agent information.
 */

import { AgentSpec } from '@/lib/agents/types';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { MessageSquare, Edit, Trash2, Download } from 'lucide-react';

interface AgentCardProps {
  agent: AgentSpec;
  onSelect?: (agent: AgentSpec) => void;
  onEdit?: (agent: AgentSpec) => void;
  onDelete?: (agent: AgentSpec) => void;
  onExport?: (agent: AgentSpec) => void;
}

export function AgentCard({ agent, onSelect, onEdit, onDelete, onExport }: AgentCardProps) {
  return (
    <Card className="hover:shadow-lg transition-shadow">
      <CardHeader>
        <div className="flex items-start justify-between">
          <div className="flex items-center gap-2">
            <span className="text-3xl">{agent.icon || '🤖'}</span>
            <div>
              <CardTitle className="text-lg">{agent.name}</CardTitle>
              <CardDescription className="text-sm mt-1">
                v{agent.version}
                {agent.author && ` • by ${agent.author}`}
              </CardDescription>
            </div>
          </div>
        </div>
      </CardHeader>

      <CardContent className="space-y-3">
        <p className="text-sm text-muted-foreground line-clamp-2">
          {agent.description}
        </p>

        <div className="flex flex-wrap gap-1">
          <Badge variant="secondary" className="text-xs">
            {agent.category}
          </Badge>
          {agent.tags.slice(0, 3).map((tag) => (
            <Badge key={tag} variant="outline" className="text-xs">
              {tag}
            </Badge>
          ))}
          {agent.tags.length > 3 && (
            <Badge variant="outline" className="text-xs">
              +{agent.tags.length - 3}
            </Badge>
          )}
        </div>

        <div className="text-xs text-muted-foreground space-y-1">
          <div className="flex items-center gap-2">
            <span className="font-medium">Model:</span>
            <span>{agent.model.provider}/{agent.model.name}</span>
          </div>
          {agent.tools.length > 0 && (
            <div className="flex items-center gap-2">
              <span className="font-medium">Tools:</span>
              <span>{agent.tools.length} enabled</span>
            </div>
          )}
          {agent.strategy && (
            <div className="flex items-center gap-2">
              <span className="font-medium">Strategy:</span>
              <span className="capitalize">{agent.strategy.replace(/_/g, ' ')}</span>
            </div>
          )}
        </div>
      </CardContent>

      <CardFooter className="flex gap-2">
        {onSelect && (
          <Button
            variant="default"
            size="sm"
            className="flex-1"
            onClick={() => onSelect(agent)}
          >
            <MessageSquare className="h-4 w-4 mr-1" />
            Chat
          </Button>
        )}
        {onEdit && (
          <Button
            variant="outline"
            size="sm"
            onClick={() => onEdit(agent)}
          >
            <Edit className="h-4 w-4" />
          </Button>
        )}
        {onExport && (
          <Button
            variant="outline"
            size="sm"
            onClick={() => onExport(agent)}
          >
            <Download className="h-4 w-4" />
          </Button>
        )}
        {onDelete && (
          <Button
            variant="outline"
            size="sm"
            onClick={() => onDelete(agent)}
          >
            <Trash2 className="h-4 w-4" />
          </Button>
        )}
      </CardFooter>
    </Card>
  );
}
