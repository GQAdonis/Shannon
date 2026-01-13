/**
 * Artifacts Library Page
 *
 * Browse, search, and manage all artifacts
 */

'use client';

import { useEffect, useState } from 'react';
import { Artifact, ArtifactType, ArtifactFilter } from '@/lib/artifacts/types';
import { artifactService } from '@/lib/artifacts/database';
import { ArtifactRenderer } from '@/components/artifacts/artifact-renderer';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import {
  Search,
  Filter,
  Download,
  Upload,
  Trash2,
  Grid3x3,
  List,
  Eye,
} from 'lucide-react';
import { toast } from 'sonner';
import { cn } from '@/lib/utils';

const ARTIFACT_TYPES: ArtifactType[] = [
  'code',
  'react',
  'html',
  'svg',
  'mermaid',
  'chart',
  'markdown',
  'image',
  'video',
  'audio',
  'pdf',
];

export default function ArtifactsPage() {
  const [artifacts, setArtifacts] = useState<Artifact[]>([]);
  const [filter, setFilter] = useState<ArtifactFilter>({});
  const [search, setSearch] = useState('');
  const [selectedType, setSelectedType] = useState<string>('all');
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid');
  const [selectedArtifact, setSelectedArtifact] = useState<Artifact | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [stats, setStats] = useState<{ total: number; byType: Record<string, number> }>({
    total: 0,
    byType: {},
  });

  // Load artifacts
  const loadArtifacts = async () => {
    setIsLoading(true);
    try {
      const filterWithType: ArtifactFilter = {
        ...filter,
        search: search || undefined,
        type: selectedType !== 'all' ? (selectedType as ArtifactType) : undefined,
      };

      const results = await artifactService.list(filterWithType);
      setArtifacts(results);
    } catch (error) {
      toast.error('Failed to load artifacts');
      console.error('Failed to load artifacts:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const loadStats = async () => {
    try {
      const statsData = await artifactService.getStats();
      setStats(statsData);
    } catch (error) {
      console.error('Failed to load stats:', error);
    }
  };

  useEffect(() => {
    loadArtifacts();
    loadStats();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [filter, search, selectedType]);

  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this artifact?')) {
      return;
    }

    try {
      await artifactService.delete(id);
      toast.success('Artifact deleted');
      loadArtifacts();
      loadStats();
    } catch (error) {
      toast.error('Failed to delete artifact');
    }
  };

  const handleExport = async () => {
    try {
      const json = await artifactService.export();
      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);

      const link = document.createElement('a');
      link.href = url;
      link.download = `artifacts-${new Date().toISOString()}.json`;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      URL.revokeObjectURL(url);

      toast.success('Artifacts exported');
    } catch (error) {
      toast.error('Failed to export artifacts');
    }
  };

  const handleImport = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    try {
      const text = await file.text();
      const count = await artifactService.import(text);
      toast.success(`Imported ${count} artifacts`);
      loadArtifacts();
      loadStats();
    } catch (error) {
      toast.error('Failed to import artifacts');
    }
  };

  return (
    <div className="container mx-auto py-6 px-4">
      {/* Header */}
      <div className="mb-6">
        <h1 className="text-3xl font-bold mb-2">Artifact Library</h1>
        <p className="text-muted-foreground">
          Browse and manage all your generated artifacts
        </p>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
        <div className="border rounded-lg p-4">
          <p className="text-sm text-muted-foreground">Total Artifacts</p>
          <p className="text-2xl font-bold">{stats.total}</p>
        </div>
        {Object.entries(stats.byType).slice(0, 3).map(([type, count]) => (
          <div key={type} className="border rounded-lg p-4">
            <p className="text-sm text-muted-foreground capitalize">{type}</p>
            <p className="text-2xl font-bold">{count}</p>
          </div>
        ))}
      </div>

      {/* Filters */}
      <div className="flex flex-col md:flex-row gap-4 mb-6">
        <div className="flex-1 relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Search artifacts..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="pl-10"
          />
        </div>

        <Select value={selectedType} onValueChange={setSelectedType}>
          <SelectTrigger className="w-full md:w-48">
            <Filter className="h-4 w-4 mr-2" />
            <SelectValue placeholder="Filter by type" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All Types</SelectItem>
            {ARTIFACT_TYPES.map((type) => (
              <SelectItem key={type} value={type}>
                {type.charAt(0).toUpperCase() + type.slice(1)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

        <div className="flex gap-2">
          <Button
            variant={viewMode === 'grid' ? 'default' : 'outline'}
            size="sm"
            onClick={() => setViewMode('grid')}
          >
            <Grid3x3 className="h-4 w-4" />
          </Button>
          <Button
            variant={viewMode === 'list' ? 'default' : 'outline'}
            size="sm"
            onClick={() => setViewMode('list')}
          >
            <List className="h-4 w-4" />
          </Button>
        </div>

        <Button variant="outline" size="sm" onClick={handleExport}>
          <Download className="h-4 w-4 mr-2" />
          Export
        </Button>

        <label>
          <Button variant="outline" size="sm" asChild>
            <span>
              <Upload className="h-4 w-4 mr-2" />
              Import
            </span>
          </Button>
          <input
            type="file"
            accept=".json"
            className="hidden"
            onChange={handleImport}
          />
        </label>
      </div>

      {/* Artifacts Grid/List */}
      {isLoading ? (
        <div className="flex items-center justify-center py-12">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary" />
        </div>
      ) : artifacts.length === 0 ? (
        <div className="text-center py-12 text-muted-foreground">
          <p>No artifacts found</p>
        </div>
      ) : (
        <div
          className={cn(
            viewMode === 'grid'
              ? 'grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4'
              : 'space-y-4'
          )}
        >
          {artifacts.map((artifact) => (
            <div
              key={artifact.id}
              className="border rounded-lg overflow-hidden hover:shadow-lg transition-shadow"
            >
              <div className="bg-muted px-4 py-3 border-b flex items-center justify-between">
                <div className="flex-1 min-w-0">
                  <h3 className="font-medium truncate">{artifact.title}</h3>
                  <p className="text-xs text-muted-foreground">
                    {new Date(artifact.createdAt).toLocaleDateString()}
                  </p>
                </div>
                <span className="text-xs px-2 py-1 bg-primary/10 text-primary rounded ml-2">
                  {artifact.type}
                </span>
              </div>

              <div className="p-4">
                <p className="text-sm text-muted-foreground line-clamp-2 mb-4">
                  {artifact.content.substring(0, 100)}...
                </p>

                <div className="flex gap-2">
                  <Button
                    variant="default"
                    size="sm"
                    onClick={() => setSelectedArtifact(artifact)}
                  >
                    <Eye className="h-4 w-4 mr-2" />
                    View
                  </Button>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => handleDelete(artifact.id)}
                  >
                    <Trash2 className="h-4 w-4" />
                  </Button>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* View Dialog */}
      <Dialog open={!!selectedArtifact} onOpenChange={(open) => !open && setSelectedArtifact(null)}>
        <DialogContent className="max-w-4xl max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>{selectedArtifact?.title}</DialogTitle>
          </DialogHeader>
          {selectedArtifact && <ArtifactRenderer artifact={selectedArtifact} />}
        </DialogContent>
      </Dialog>
    </div>
  );
}
