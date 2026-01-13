/**
 * Artifact Types and Schemas
 *
 * Comprehensive type definitions for the artifact system supporting multiple
 * rendering formats and A2UI protocol compatibility.
 */

/**
 * Supported artifact types for rendering
 */
export type ArtifactType =
  | 'code'      // Executable code (JS, Python, etc.)
  | 'react'     // React component
  | 'html'      // HTML page
  | 'svg'       // SVG graphics
  | 'mermaid'   // Mermaid diagram
  | 'chart'     // Recharts/D3 chart
  | 'markdown'  // Markdown document
  | 'image'     // Image (base64 or URL)
  | 'video'     // Video (URL)
  | 'audio'     // Audio (URL)
  | 'pdf';      // PDF document

/**
 * Metadata for artifact configuration
 */
export interface ArtifactMetadata {
  language?: string;
  framework?: string;
  dependencies?: string[];
  theme?: 'light' | 'dark' | 'auto';
  interactive?: boolean;
  width?: number;
  height?: number;
  version?: string;
  author?: string;
  description?: string;
  tags?: string[];
}

/**
 * Core artifact definition
 */
export interface Artifact {
  id: string;
  type: ArtifactType;
  title: string;
  content: string;
  metadata: ArtifactMetadata;
  messageId: string;      // Source message
  conversationId: string; // Source conversation
  createdAt: string;
  updatedAt: string;
}

/**
 * Artifact filter for library queries
 */
export interface ArtifactFilter {
  type?: ArtifactType | ArtifactType[];
  conversationId?: string;
  messageId?: string;
  dateFrom?: string;
  dateTo?: string;
  tags?: string[];
  search?: string;
}

/**
 * Execution result for code artifacts
 */
export interface ExecutionResult {
  success: boolean;
  output: string;
  error?: string;
  results: ExecutionOutput[];
  executionTime?: number;
  memoryUsed?: number;
}

/**
 * Output from code execution
 */
export interface ExecutionOutput {
  type: 'text' | 'image' | 'html' | 'json' | 'svg' | 'error';
  value: string;
  mimeType?: string;
}

/**
 * Artifact action types
 */
export type ArtifactAction =
  | 'view'
  | 'edit'
  | 'copy'
  | 'export'
  | 'delete'
  | 'fullscreen'
  | 'share'
  | 'execute';

/**
 * Export format options
 */
export type ExportFormat =
  | 'json'
  | 'html'
  | 'png'
  | 'svg'
  | 'pdf'
  | 'markdown';

/**
 * Artifact detection pattern
 */
export interface ArtifactPattern {
  name: string;
  regex: RegExp;
  extract: (match: RegExpMatchArray) => Partial<Artifact>;
}

/**
 * Chart configuration for chart artifacts
 */
export interface ChartConfig {
  type: 'line' | 'bar' | 'pie' | 'area' | 'scatter' | 'radar';
  data: Record<string, string | number>[];
  xAxisKey?: string;
  yAxisKey?: string;
  colors?: string[];
  legend?: boolean;
  grid?: boolean;
}
