//! # Sandboxed Filesystem Service
//!
//! Provides secure file operations within a sandboxed directory.
//! All operations are restricted to prevent path traversal attacks.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tokio::fs;
use tracing::{debug, info, warn};

/// Sandboxed filesystem service
#[derive(Clone)]
pub struct FilesystemService {
    sandbox_root: PathBuf,
}

impl FilesystemService {
    /// Create a new filesystem service with the specified sandbox root
    ///
    /// # Arguments
    ///
    /// * `sandbox_root` - The root directory for all file operations
    ///
    /// # Errors
    ///
    /// Returns an error if the sandbox directory cannot be created
    ///
    /// # Security
    ///
    /// All file operations are restricted to paths within the sandbox root.
    /// Path traversal attempts (.., symlinks) are blocked.
    pub fn new(sandbox_root: PathBuf) -> Result<Self> {
        // Ensure sandbox exists
        std::fs::create_dir_all(&sandbox_root).context("Failed to create sandbox directory")?;

        let canonical_root = sandbox_root
            .canonicalize()
            .context("Failed to canonicalize sandbox root")?;

        info!("Initialized filesystem sandbox at: {:?}", canonical_root);

        Ok(Self {
            sandbox_root: canonical_root,
        })
    }

    /// Validate and resolve a path within the sandbox
    ///
    /// # Security
    ///
    /// This function prevents directory traversal attacks by:
    /// 1. Resolving the full path relative to sandbox root
    /// 2. Canonicalizing the path (resolves .., symlinks)
    /// 3. Verifying the result is still within sandbox_root
    ///
    /// # Errors
    ///
    /// Returns an error if the path escapes the sandbox
    fn validate_path(&self, path: &Path) -> Result<PathBuf> {
        let full_path = self.sandbox_root.join(path);

        // Try to canonicalize, or use the full path if it doesn't exist yet
        let canonical = if full_path.exists() {
            full_path
                .canonicalize()
                .context("Failed to canonicalize path")?
        } else {
            // For non-existent paths, canonicalize the parent and append the filename
            if let Some(parent) = full_path.parent() {
                if parent.exists() {
                    let canonical_parent = parent
                        .canonicalize()
                        .context("Failed to canonicalize parent")?;

                    if let Some(filename) = full_path.file_name() {
                        canonical_parent.join(filename)
                    } else {
                        full_path
                    }
                } else {
                    full_path
                }
            } else {
                full_path
            }
        };

        // Security check: ensure path is within sandbox
        if !canonical.starts_with(&self.sandbox_root) {
            warn!("Path traversal attempt blocked: {:?}", path);
            anyhow::bail!("Path escapes sandbox: {:?}", path);
        }

        Ok(canonical)
    }

    /// Read the contents of a file
    ///
    /// # Arguments
    ///
    /// * `path` - The file path relative to sandbox root
    ///
    /// # Returns
    ///
    /// The file contents as a UTF-8 string
    ///
    /// # Errors
    ///
    /// Returns an error if the file doesn't exist, cannot be read, or contains invalid UTF-8
    pub async fn read_file(&self, path: &str) -> Result<String> {
        let full_path = self.validate_path(Path::new(path))?;

        debug!("Reading file: {:?}", full_path);

        let content = fs::read_to_string(&full_path)
            .await
            .context(format!("Failed to read file: {:?}", full_path))?;

        debug!("Read {} bytes from {:?}", content.len(), full_path);

        Ok(content)
    }

    /// Write content to a file
    ///
    /// # Arguments
    ///
    /// * `path` - The file path relative to sandbox root
    /// * `content` - The content to write
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written or the path is invalid
    pub async fn write_file(&self, path: &str, content: &str) -> Result<()> {
        let full_path = self.validate_path(Path::new(path))?;

        debug!("Writing {} bytes to {:?}", content.len(), full_path);

        // Ensure parent directory exists
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .await
                .context("Failed to create parent directory")?;
        }

        fs::write(&full_path, content)
            .await
            .context(format!("Failed to write file: {:?}", full_path))?;

        info!("Wrote file: {:?}", full_path);

        Ok(())
    }

    /// List files and directories in a directory
    ///
    /// # Arguments
    ///
    /// * `path` - The directory path relative to sandbox root (use "." for root)
    ///
    /// # Returns
    ///
    /// A vector of file information for each item in the directory
    ///
    /// # Errors
    ///
    /// Returns an error if the directory doesn't exist or cannot be read
    pub async fn list_directory(&self, path: &str) -> Result<Vec<FileInfo>> {
        let full_path = self.validate_path(Path::new(path))?;

        debug!("Listing directory: {:?}", full_path);

        let mut entries = fs::read_dir(&full_path)
            .await
            .context(format!("Failed to read directory: {:?}", full_path))?;

        let mut files = Vec::new();

        while let Some(entry) = entries
            .next_entry()
            .await
            .context("Failed to read directory entry")?
        {
            let metadata = entry
                .metadata()
                .await
                .context("Failed to read entry metadata")?;

            let file_path = entry.path();
            let relative_path = file_path
                .strip_prefix(&self.sandbox_root)
                .unwrap_or(&file_path);

            files.push(FileInfo {
                name: entry.file_name().to_string_lossy().to_string(),
                path: relative_path.to_string_lossy().to_string(),
                is_directory: metadata.is_dir(),
                size: metadata.len(),
                modified: metadata
                    .modified()
                    .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH))
                    .map(|d| d.as_secs())
                    .unwrap_or(0),
            });
        }

        // Sort by name
        files.sort_by(|a, b| a.name.cmp(&b.name));

        debug!("Found {} entries in {:?}", files.len(), full_path);

        Ok(files)
    }

    /// Delete a file or directory
    ///
    /// # Arguments
    ///
    /// * `path` - The path to delete relative to sandbox root
    ///
    /// # Errors
    ///
    /// Returns an error if the path doesn't exist or cannot be deleted
    pub async fn delete(&self, path: &str) -> Result<()> {
        let full_path = self.validate_path(Path::new(path))?;

        debug!("Deleting: {:?}", full_path);

        if full_path.is_dir() {
            fs::remove_dir_all(&full_path)
                .await
                .context(format!("Failed to delete directory: {:?}", full_path))?;
        } else {
            fs::remove_file(&full_path)
                .await
                .context(format!("Failed to delete file: {:?}", full_path))?;
        }

        info!("Deleted: {:?}", full_path);

        Ok(())
    }

    /// Create a directory
    ///
    /// # Arguments
    ///
    /// * `path` - The directory path to create relative to sandbox root
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be created or the path is invalid
    pub async fn create_directory(&self, path: &str) -> Result<()> {
        let full_path = self.validate_path(Path::new(path))?;

        debug!("Creating directory: {:?}", full_path);

        fs::create_dir_all(&full_path)
            .await
            .context(format!("Failed to create directory: {:?}", full_path))?;

        info!("Created directory: {:?}", full_path);

        Ok(())
    }

    /// Get information about a file or directory
    ///
    /// # Arguments
    ///
    /// * `path` - The path relative to sandbox root
    ///
    /// # Returns
    ///
    /// File information including size, type, and modification time
    ///
    /// # Errors
    ///
    /// Returns an error if the path doesn't exist
    pub async fn get_info(&self, path: &str) -> Result<FileInfo> {
        let full_path = self.validate_path(Path::new(path))?;

        let metadata = fs::metadata(&full_path)
            .await
            .context(format!("Failed to get metadata for: {:?}", full_path))?;

        let relative_path = full_path
            .strip_prefix(&self.sandbox_root)
            .unwrap_or(&full_path);

        Ok(FileInfo {
            name: full_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default(),
            path: relative_path.to_string_lossy().to_string(),
            is_directory: metadata.is_dir(),
            size: metadata.len(),
            modified: metadata
                .modified()
                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH))
                .map(|d| d.as_secs())
                .unwrap_or(0),
        })
    }
}

/// Information about a file or directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// File or directory name
    pub name: String,
    /// Path relative to sandbox root
    pub path: String,
    /// Whether this is a directory
    pub is_directory: bool,
    /// Size in bytes
    pub size: u64,
    /// Last modified time (Unix timestamp)
    pub modified: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_filesystem_service_creation() {
        let temp_dir = TempDir::new().unwrap();
        let service = FilesystemService::new(temp_dir.path().to_path_buf());
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_write_and_read_file() {
        let temp_dir = TempDir::new().unwrap();
        let service = FilesystemService::new(temp_dir.path().to_path_buf()).unwrap();

        let content = "Hello, world!";
        service.write_file("test.txt", content).await.unwrap();

        let read_content = service.read_file("test.txt").await.unwrap();
        assert_eq!(read_content, content);
    }

    #[tokio::test]
    async fn test_path_traversal_prevention() {
        let temp_dir = TempDir::new().unwrap();
        let service = FilesystemService::new(temp_dir.path().to_path_buf()).unwrap();

        // Try to escape sandbox
        let result = service.read_file("../etc/passwd").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_directory() {
        let temp_dir = TempDir::new().unwrap();
        let service = FilesystemService::new(temp_dir.path().to_path_buf()).unwrap();

        // Create some files
        service.write_file("file1.txt", "content1").await.unwrap();
        service.write_file("file2.txt", "content2").await.unwrap();
        service.create_directory("subdir").await.unwrap();

        let files = service.list_directory(".").await.unwrap();
        assert_eq!(files.len(), 3);
    }

    #[tokio::test]
    async fn test_delete_file() {
        let temp_dir = TempDir::new().unwrap();
        let service = FilesystemService::new(temp_dir.path().to_path_buf()).unwrap();

        service
            .write_file("to_delete.txt", "content")
            .await
            .unwrap();
        service.delete("to_delete.txt").await.unwrap();

        let result = service.read_file("to_delete.txt").await;
        assert!(result.is_err());
    }
}
