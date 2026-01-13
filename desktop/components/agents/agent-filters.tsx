/**
 * Agent filters component for filtering and searching agents.
 */

import { AGENT_CATEGORIES } from '@/lib/agents/types';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Badge } from '@/components/ui/badge';
import { Search, X } from 'lucide-react';

interface AgentFiltersProps {
  selectedCategory?: string;
  selectedTags?: string[];
  searchQuery?: string;
  onCategoryChange?: (category: string | undefined) => void;
  onTagsChange?: (tags: string[]) => void;
  onSearchChange?: (query: string) => void;
  onClear?: () => void;
}

export function AgentFilters({
  selectedCategory,
  selectedTags = [],
  searchQuery = '',
  onCategoryChange,
  onTagsChange,
  onSearchChange,
  onClear,
}: AgentFiltersProps) {
  const hasActiveFilters = selectedCategory || selectedTags.length > 0 || searchQuery;

  return (
    <div className="space-y-4 p-4 border rounded-lg bg-card">
      <div className="flex items-center justify-between">
        <h3 className="font-semibold">Filters</h3>
        {hasActiveFilters && onClear && (
          <Button
            variant="ghost"
            size="sm"
            onClick={onClear}
          >
            <X className="h-4 w-4 mr-1" />
            Clear
          </Button>
        )}
      </div>

      {/* Search */}
      <div className="space-y-2">
        <Label htmlFor="search">Search</Label>
        <div className="relative">
          <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input
            id="search"
            placeholder="Search agents..."
            value={searchQuery}
            onChange={(e) => onSearchChange?.(e.target.value)}
            className="pl-8"
          />
        </div>
      </div>

      {/* Categories */}
      <div className="space-y-2">
        <Label>Category</Label>
        <div className="flex flex-wrap gap-2">
          <Badge
            variant={!selectedCategory ? 'default' : 'outline'}
            className="cursor-pointer"
            onClick={() => onCategoryChange?.(undefined)}
          >
            All
          </Badge>
          {AGENT_CATEGORIES.map((category) => (
            <Badge
              key={category}
              variant={selectedCategory === category ? 'default' : 'outline'}
              className="cursor-pointer capitalize"
              onClick={() => onCategoryChange?.(category)}
            >
              {category}
            </Badge>
          ))}
        </div>
      </div>

      {/* Common Tags */}
      {selectedTags.length > 0 && (
        <div className="space-y-2">
          <Label>Selected Tags</Label>
          <div className="flex flex-wrap gap-2">
            {selectedTags.map((tag) => (
              <Badge
                key={tag}
                variant="secondary"
                className="cursor-pointer"
                onClick={() => {
                  onTagsChange?.(selectedTags.filter((t) => t !== tag));
                }}
              >
                {tag}
                <X className="h-3 w-3 ml-1" />
              </Badge>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
