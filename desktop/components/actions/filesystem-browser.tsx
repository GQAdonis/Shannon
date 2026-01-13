/**
 * Filesystem Browser Component
 *
 * Provides a UI for browsing and managing files in the sandboxed filesystem.
 */

'use client';

import { useState, useEffect, useCallback } from 'react';
import { filesystemService, FileInfo } from '@/lib/actions/filesystem-service';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Folder, File, Trash2, FolderPlus, FilePlus, Eye } from 'lucide-react';
import { toast } from 'sonner';

export function FilesystemBrowser() {
  const [currentPath, setCurrentPath] = useState('.');
  const [files, setFiles] = useState<FileInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [selectedFile, setSelectedFile] = useState<FileInfo | null>(null);
  const [fileContent, setFileContent] = useState<string>('');
  const [showContentDialog, setShowContentDialog] = useState(false);
  const [newItemName, setNewItemName] = useState('');
  const [showNewDialog, setShowNewDialog] = useState<'file' | 'folder' | null>(null);

  const loadDirectory = useCallback(async (path: string) => {
    setLoading(true);
    try {
      const result = await filesystemService.listDirectory(path);
      setFiles(result);
      setCurrentPath(path);
    } catch (error) {
      toast.error(`Failed to load directory: ${error}`);
      console.error('Load directory error:', error);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadDirectory('.');
  }, [loadDirectory]);

  const handleNavigate = (file: FileInfo) => {
    if (file.is_directory) {
      const newPath = currentPath === '.' ? file.name : `${currentPath}/${file.name}`;
      loadDirectory(newPath);
    }
  };

  const handleBack = () => {
    if (currentPath === '.') return;

    const parts = currentPath.split('/');
    parts.pop();
    const parentPath = parts.length > 0 ? parts.join('/') : '.';
    loadDirectory(parentPath);
  };

  const handleView = async (file: FileInfo) => {
    if (file.is_directory) return;

    try {
      const content = await filesystemService.readFile(file.path);
      setFileContent(content);
      setSelectedFile(file);
      setShowContentDialog(true);
    } catch (error) {
      toast.error(`Failed to read file: ${error}`);
    }
  };

  const handleDelete = async (file: FileInfo) => {
    if (!confirm(`Are you sure you want to delete "${file.name}"?`)) {
      return;
    }

    try {
      await filesystemService.delete(file.path);
      toast.success(`Deleted ${file.name}`);
      loadDirectory(currentPath);
    } catch (error) {
      toast.error(`Failed to delete: ${error}`);
    }
  };

  const handleCreateItem = async () => {
    if (!newItemName) {
      toast.error('Please enter a name');
      return;
    }

    const path = currentPath === '.' ? newItemName : `${currentPath}/${newItemName}`;

    try {
      if (showNewDialog === 'folder') {
        await filesystemService.createDirectory(path);
        toast.success(`Created folder ${newItemName}`);
      } else if (showNewDialog === 'file') {
        await filesystemService.writeFile(path, '');
        toast.success(`Created file ${newItemName}`);
      }

      setShowNewDialog(null);
      setNewItemName('');
      loadDirectory(currentPath);
    } catch (error) {
      toast.error(`Failed to create: ${error}`);
    }
  };

  return (
    <Card className="w-full">
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center gap-2">
            <Folder className="h-5 w-5" />
            Filesystem Browser
          </CardTitle>
          <div className="flex gap-2">
            <Button
              size="sm"
              variant="outline"
              onClick={() => setShowNewDialog('folder')}
            >
              <FolderPlus className="h-4 w-4 mr-1" />
              New Folder
            </Button>
            <Button
              size="sm"
              variant="outline"
              onClick={() => setShowNewDialog('file')}
            >
              <FilePlus className="h-4 w-4 mr-1" />
              New File
            </Button>
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Breadcrumb */}
        <div className="flex items-center gap-2 text-sm">
          <Button
            size="sm"
            variant="ghost"
            onClick={handleBack}
            disabled={currentPath === '.' || loading}
          >
            ← Back
          </Button>
          <span className="text-muted-foreground">
            {currentPath === '.' ? 'Root' : currentPath}
          </span>
        </div>

        {/* File List */}
        <div className="border rounded-md">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead>Size</TableHead>
                <TableHead>Modified</TableHead>
                <TableHead className="text-right">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {loading ? (
                <TableRow>
                  <TableCell colSpan={4} className="text-center py-8 text-muted-foreground">
                    Loading...
                  </TableCell>
                </TableRow>
              ) : files.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={4} className="text-center py-8 text-muted-foreground">
                    No files or folders
                  </TableCell>
                </TableRow>
              ) : (
                files.map((file) => (
                  <TableRow
                    key={file.path}
                    className="cursor-pointer hover:bg-muted/50"
                    onClick={() => handleNavigate(file)}
                  >
                    <TableCell className="flex items-center gap-2">
                      {file.is_directory ? (
                        <Folder className="h-4 w-4 text-blue-500" />
                      ) : (
                        <File className="h-4 w-4 text-gray-500" />
                      )}
                      <span className="font-medium">{file.name}</span>
                    </TableCell>
                    <TableCell>
                      {file.is_directory ? '-' : filesystemService.formatFileSize(file.size)}
                    </TableCell>
                    <TableCell className="text-muted-foreground text-sm">
                      {filesystemService.formatModified(file.modified)}
                    </TableCell>
                    <TableCell className="text-right">
                      <div className="flex justify-end gap-1">
                        {!file.is_directory && (
                          <Button
                            size="sm"
                            variant="ghost"
                            onClick={(e) => {
                              e.stopPropagation();
                              handleView(file);
                            }}
                          >
                            <Eye className="h-4 w-4" />
                          </Button>
                        )}
                        <Button
                          size="sm"
                          variant="ghost"
                          onClick={(e) => {
                            e.stopPropagation();
                            handleDelete(file);
                          }}
                        >
                          <Trash2 className="h-4 w-4 text-red-500" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))
              )}
            </TableBody>
          </Table>
        </div>
      </CardContent>

      {/* View File Dialog */}
      <Dialog open={showContentDialog} onOpenChange={setShowContentDialog}>
        <DialogContent className="max-w-4xl max-h-[80vh]">
          <DialogHeader>
            <DialogTitle>{selectedFile?.name}</DialogTitle>
            <DialogDescription>
              {selectedFile && filesystemService.formatFileSize(selectedFile.size)} •{' '}
              {selectedFile && filesystemService.formatModified(selectedFile.modified)}
            </DialogDescription>
          </DialogHeader>
          <div className="border rounded-md p-4 bg-muted overflow-auto max-h-[60vh]">
            <pre className="text-xs whitespace-pre-wrap break-words">
              {fileContent}
            </pre>
          </div>
          <DialogFooter>
            <Button onClick={() => setShowContentDialog(false)}>Close</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Create Item Dialog */}
      <Dialog open={showNewDialog !== null} onOpenChange={() => setShowNewDialog(null)}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              Create New {showNewDialog === 'folder' ? 'Folder' : 'File'}
            </DialogTitle>
            <DialogDescription>
              Enter a name for the new {showNewDialog === 'folder' ? 'folder' : 'file'}
            </DialogDescription>
          </DialogHeader>
          <Input
            value={newItemName}
            onChange={(e) => setNewItemName(e.target.value)}
            placeholder={showNewDialog === 'folder' ? 'folder-name' : 'filename.txt'}
            onKeyPress={(e) => e.key === 'Enter' && handleCreateItem()}
          />
          <DialogFooter>
            <Button variant="outline" onClick={() => setShowNewDialog(null)}>
              Cancel
            </Button>
            <Button onClick={handleCreateItem}>Create</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </Card>
  );
}
