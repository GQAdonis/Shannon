/**
 * Code Renderer with Syntax Highlighting and Python Execution
 *
 * Renders code with syntax highlighting and supports Python execution via E2B
 */

'use client';

import { useState } from 'react';
import { Artifact, ExecutionResult } from '@/lib/artifacts/types';
import { e2bExecutor } from '@/lib/artifacts/e2b-executor';
import { Button } from '@/components/ui/button';
import { Play, Copy, Download, AlertCircle, CheckCircle2, Loader2 } from 'lucide-react';
import { toast } from 'sonner';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

interface CodeRendererProps {
  artifact: Artifact;
}

export function CodeRenderer({ artifact }: CodeRendererProps) {
  const [output, setOutput] = useState<ExecutionResult | null>(null);
  const [isExecuting, setIsExecuting] = useState(false);

  const isPython = artifact.metadata.language === 'python';
  const isExecutable = isPython && e2bExecutor.isConfigured();

  const handleExecute = async () => {
    if (!isPython) return;

    setIsExecuting(true);
    setOutput(null);

    try {
      const result = await e2bExecutor.executePythonWithTimeout(artifact.content, 60000);
      setOutput(result);

      if (result.success) {
        toast.success('Code executed successfully');
      } else {
        toast.error('Execution failed');
      }
    } catch (error) {
      setOutput({
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
        output: '',
        results: [],
      });
      toast.error('Failed to execute code');
    } finally {
      setIsExecuting(false);
    }
  };

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(artifact.content);
      toast.success('Code copied to clipboard');
    } catch (err) {
      toast.error('Failed to copy to clipboard');
    }
  };

  const handleDownload = () => {
    const ext = artifact.metadata.language || 'txt';
    const blob = new Blob([artifact.content], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);

    const link = document.createElement('a');
    link.href = url;
    link.download = `${artifact.title.replace(/\s+/g, '-')}.${ext}`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);

    toast.success('Code downloaded');
  };

  return (
    <div className="code-renderer border rounded-lg overflow-hidden">
      <div className="bg-muted px-4 py-2 border-b flex items-center justify-between">
        <div className="flex items-center gap-2">
          <h4 className="font-medium text-sm">{artifact.title}</h4>
          {artifact.metadata.language && (
            <span className="text-xs px-2 py-0.5 bg-primary/10 text-primary rounded">
              {artifact.metadata.language}
            </span>
          )}
        </div>
        <div className="flex items-center gap-2">
          {isExecutable && (
            <Button
              variant="default"
              size="sm"
              onClick={handleExecute}
              disabled={isExecuting}
            >
              {isExecuting ? (
                <>
                  <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                  Executing...
                </>
              ) : (
                <>
                  <Play className="h-4 w-4 mr-2" />
                  Run Code
                </>
              )}
            </Button>
          )}
          {!isExecutable && isPython && (
            <span className="text-xs text-muted-foreground">
              E2B API key required for execution
            </span>
          )}
          <Button variant="ghost" size="sm" onClick={handleCopy}>
            <Copy className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="sm" onClick={handleDownload}>
            <Download className="h-4 w-4" />
          </Button>
        </div>
      </div>

      <div className="bg-background">
        <pre className="p-4 overflow-x-auto">
          <code className={`language-${artifact.metadata.language || 'plaintext'}`}>
            {artifact.content}
          </code>
        </pre>
      </div>

      {output && (
        <div className="border-t">
          <Tabs defaultValue="output" className="w-full">
            <TabsList className="w-full justify-start rounded-none border-b bg-muted/50">
              <TabsTrigger value="output">Output</TabsTrigger>
              {output.error && <TabsTrigger value="error">Error</TabsTrigger>}
              {output.results.length > 0 && <TabsTrigger value="results">Results</TabsTrigger>}
            </TabsList>

            <TabsContent value="output" className="p-4">
              <div className="flex items-start gap-2 mb-2">
                {output.success ? (
                  <CheckCircle2 className="h-5 w-5 text-green-500 flex-shrink-0 mt-0.5" />
                ) : (
                  <AlertCircle className="h-5 w-5 text-destructive flex-shrink-0 mt-0.5" />
                )}
                <div className="flex-1">
                  <p className="font-medium text-sm">
                    {output.success ? 'Execution successful' : 'Execution failed'}
                  </p>
                  {output.executionTime && (
                    <p className="text-xs text-muted-foreground mt-1">
                      Completed in {output.executionTime}ms
                    </p>
                  )}
                </div>
              </div>

              {output.output && (
                <pre className="mt-3 p-3 bg-muted rounded text-sm overflow-x-auto">
                  {output.output}
                </pre>
              )}

              {!output.output && !output.error && (
                <p className="text-sm text-muted-foreground italic">No output</p>
              )}
            </TabsContent>

            {output.error && (
              <TabsContent value="error" className="p-4">
                <div className="bg-destructive/10 text-destructive p-3 rounded">
                  <pre className="text-sm whitespace-pre-wrap">{output.error}</pre>
                </div>
              </TabsContent>
            )}

            {output.results.length > 0 && (
              <TabsContent value="results" className="p-4 space-y-4">
                {output.results.map((result) => (
                  <div key={`${result.type}-${result.value.slice(0, 20)}`} className="border rounded-lg p-3">
                    <p className="text-xs text-muted-foreground mb-2">
                      {result.type} {result.mimeType && `(${result.mimeType})`}
                    </p>

                    {result.type === 'image' && (
                      <img
                        src={`data:${result.mimeType};base64,${result.value}`}
                        alt="Output"
                        className="max-w-full h-auto rounded"
                      />
                    )}

                    {result.type === 'svg' && (
                      <img
                        src={`data:image/svg+xml,${encodeURIComponent(result.value)}`}
                        alt="SVG Output"
                        className="max-w-full h-auto rounded"
                        data-security="sandboxed-svg"
                      />
                    )}

                    {result.type === 'html' && (
                      <iframe
                        title="HTML Output"
                        srcDoc={result.value}
                        className="w-full h-96 border rounded"
                        sandbox="allow-scripts"
                      />
                    )}

                    {(result.type === 'text' || result.type === 'json') && (
                      <pre className="p-3 bg-muted rounded text-sm overflow-x-auto">
                        {result.value}
                      </pre>
                    )}
                  </div>
                ))}
              </TabsContent>
            )}
          </Tabs>
        </div>
      )}
    </div>
  );
}
