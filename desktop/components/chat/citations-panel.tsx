'use client';

import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { FileText, ChevronDown, ChevronUp } from 'lucide-react';
import type { KnowledgeCitation } from '@/lib/shannon/citations';

interface CitationsPanelProps {
  citations: KnowledgeCitation[];
  className?: string;
}

export function CitationsPanel({ citations, className }: CitationsPanelProps) {
  const [expanded, setExpanded] = useState<Set<number>>(new Set());

  if (citations.length === 0) {
    return null;
  }

  return (
    <Card className={`citations-panel mb-4 border-l-4 border-l-primary ${className || ''}`}>
      <CardHeader className="pb-3">
        <CardTitle className="text-sm flex items-center gap-2">
          <FileText className="h-4 w-4" />
          Knowledge Base Sources ({citations.length})
        </CardTitle>
      </CardHeader>

      <CardContent className="space-y-2">
        {citations.map((citation) => (
          <CitationCard
            key={citation.index}
            citation={citation}
            expanded={expanded.has(citation.index)}
            onToggle={() => {
              const newExpanded = new Set(expanded);
              if (newExpanded.has(citation.index)) {
                newExpanded.delete(citation.index);
              } else {
                newExpanded.add(citation.index);
              }
              setExpanded(newExpanded);
            }}
          />
        ))}
      </CardContent>
    </Card>
  );
}

interface CitationCardProps {
  citation: KnowledgeCitation;
  expanded: boolean;
  onToggle: () => void;
}

function CitationCard({ citation, expanded, onToggle }: CitationCardProps) {
  return (
    <div className="citation-card border rounded-lg p-3 hover:bg-accent/50 transition-colors">
      <div
        className="flex items-center justify-between cursor-pointer"
        onClick={onToggle}
        onKeyDown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            onToggle();
          }
        }}
        role="button"
        tabIndex={0}
        aria-expanded={expanded}
      >
        <div className="flex-1">
          <div className="flex items-center gap-2 flex-wrap">
            <Badge variant="outline" className="text-xs">
              Source {citation.index}
            </Badge>
            <span className="font-medium text-sm">{citation.document_title}</span>
            <Badge variant="secondary" className="text-xs">
              {Math.round(citation.relevance_score * 100)}% relevant
            </Badge>
          </div>

          {!expanded && (
            <p className="text-xs text-muted-foreground mt-1 line-clamp-2">
              {citation.content}
            </p>
          )}
        </div>

        {expanded ? (
          <ChevronUp className="h-4 w-4 text-muted-foreground flex-shrink-0 ml-2" />
        ) : (
          <ChevronDown className="h-4 w-4 text-muted-foreground flex-shrink-0 ml-2" />
        )}
      </div>

      {expanded && (
        <div className="mt-3 pt-3 border-t">
          <p className="text-sm whitespace-pre-wrap">
            {citation.content}
          </p>

          <div className="flex gap-2 mt-2 text-xs text-muted-foreground flex-wrap">
            <span>{citation.tokens} tokens</span>
            {citation.metadata.page != null && (
              <span>• Page {String(citation.metadata.page)}</span>
            )}
            {citation.metadata.section != null && (
              <span>• {String(citation.metadata.section)}</span>
            )}
            {citation.chunk_id && (
              <span className="font-mono">• ID: {citation.chunk_id.slice(0, 8)}</span>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
