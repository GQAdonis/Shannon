/**
 * Sandboxed Filesystem Service
 *
 * Provides secure filesystem operations within a sandboxed directory.
 * All operations are restricted to prevent path traversal attacks.
 */

import { invoke } from '@tauri-apps/api/core';

export interface FileInfo {
  name: string;
  path: string;
  is_directory: boolean;
  size: number;
  modified: number; // Unix timestamp
}

export class FilesystemService {
  /**
   * Read a file from the sandboxed filesystem
   */
  async readFile(path: string): Promise<string> {
    return await invoke<string>('fs_read', { path });
  }

  /**
   * Write content to a file in the sandboxed filesystem
   */
  async writeFile(path: string, content: string): Promise<void> {
    await invoke('fs_write', { path, content });
  }

  /**
   * List files and directories
   */
  async listDirectory(path: string = '.'): Promise<FileInfo[]> {
    return await invoke<FileInfo[]>('fs_list', { path });
  }

  /**
   * Delete a file or directory
   */
  async delete(path: string): Promise<void> {
    await invoke('fs_delete', { path });
  }

  /**
   * Create a directory
   */
  async createDirectory(path: string): Promise<void> {
    await invoke('fs_mkdir', { path });
  }

  /**
   * Get information about a file or directory
   */
  async getInfo(path: string): Promise<FileInfo> {
    return await invoke<FileInfo>('fs_info', { path });
  }

  /**
   * Format file size in human-readable format
   */
  formatFileSize(bytes: number): string {
    if (bytes === 0) return '0 B';

    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
  }

  /**
   * Format modified timestamp as a date string
   */
  formatModified(timestamp: number): string {
    const date = new Date(timestamp * 1000);
    return date.toLocaleString();
  }

  /**
   * Get file extension
   */
  getExtension(filename: string): string {
    const parts = filename.split('.');
    return parts.length > 1 ? parts[parts.length - 1].toLowerCase() : '';
  }

  /**
   * Get icon name for file type (for UI)
   */
  getFileIcon(info: FileInfo): string {
    if (info.is_directory) {
      return 'folder';
    }

    const ext = this.getExtension(info.name);
    const iconMap: Record<string, string> = {
      // Documents
      'txt': 'file-text',
      'md': 'file-text',
      'pdf': 'file-pdf',
      'doc': 'file-word',
      'docx': 'file-word',

      // Code
      'js': 'file-code',
      'ts': 'file-code',
      'jsx': 'file-code',
      'tsx': 'file-code',
      'py': 'file-code',
      'rs': 'file-code',
      'go': 'file-code',
      'java': 'file-code',
      'cpp': 'file-code',
      'c': 'file-code',

      // Images
      'png': 'file-image',
      'jpg': 'file-image',
      'jpeg': 'file-image',
      'gif': 'file-image',
      'svg': 'file-image',
      'webp': 'file-image',

      // Archives
      'zip': 'file-archive',
      'tar': 'file-archive',
      'gz': 'file-archive',
      'rar': 'file-archive',

      // Data
      'json': 'file-json',
      'xml': 'file-code',
      'yaml': 'file-code',
      'yml': 'file-code',
      'toml': 'file-code',
      'csv': 'file-spreadsheet',
    };

    return iconMap[ext] || 'file';
  }
}

// Singleton instance
export const filesystemService = new FilesystemService();
