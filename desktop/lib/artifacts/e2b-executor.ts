/**
 * E2B Python Code Executor
 *
 * Securely executes Python code using E2B's sandboxed interpreter
 */

import { Sandbox } from '@e2b/code-interpreter';
import { ExecutionResult, ExecutionOutput } from './types';

export class E2BExecutor {
  private apiKey: string;

  constructor(apiKey?: string) {
    this.apiKey = apiKey || process.env.NEXT_PUBLIC_E2B_API_KEY || '';
  }

  /**
   * Execute Python code in E2B sandbox
   */
  async executePython(code: string): Promise<ExecutionResult> {
    if (!this.apiKey) {
      return {
        success: false,
        output: '',
        error: 'E2B API key not configured. Set NEXT_PUBLIC_E2B_API_KEY in your environment.',
        results: [],
      };
    }

    const startTime = Date.now();
    let sandbox: Sandbox | null = null;

    try {
      // Create sandbox instance
      sandbox = await Sandbox.create({ apiKey: this.apiKey });

      // Execute code
      const execution = await sandbox.runCode(code);
      const executionTime = Date.now() - startTime;

      // Process results
      const results: ExecutionOutput[] = [];

      for (const result of execution.results) {
        if (result.text) {
          results.push({
            type: 'text',
            value: result.text,
          });
        }

        if (result.png) {
          results.push({
            type: 'image',
            value: result.png,
            mimeType: 'image/png',
          });
        }

        if (result.svg) {
          results.push({
            type: 'svg',
            value: result.svg,
            mimeType: 'image/svg+xml',
          });
        }

        if (result.html) {
          results.push({
            type: 'html',
            value: result.html,
            mimeType: 'text/html',
          });
        }

        if (result.json) {
          results.push({
            type: 'json',
            value: JSON.stringify(result.json, null, 2),
            mimeType: 'application/json',
          });
        }
      }

      return {
        success: !execution.error,
        output: execution.logs.stdout.join('\n'),
        error: execution.logs.stderr.join('\n') || execution.error?.value,
        results,
        executionTime,
      };
    } catch (error) {
      const executionTime = Date.now() - startTime;

      return {
        success: false,
        output: '',
        error: error instanceof Error ? error.message : String(error),
        results: [],
        executionTime,
      };
    } finally {
      // Clean up sandbox
      if (sandbox) {
        try {
          await sandbox.close();
        } catch (error) {
          console.error('Failed to close E2B sandbox:', error);
        }
      }
    }
  }

  /**
   * Execute Python code with timeout
   */
  async executePythonWithTimeout(
    code: string,
    timeoutMs: number = 30000
  ): Promise<ExecutionResult> {
    return Promise.race([
      this.executePython(code),
      new Promise<ExecutionResult>((_, reject) =>
        setTimeout(() => reject(new Error('Execution timeout')), timeoutMs)
      ),
    ]).catch((error) => ({
      success: false,
      output: '',
      error: error instanceof Error ? error.message : String(error),
      results: [],
    }));
  }

  /**
   * Check if E2B is configured
   */
  isConfigured(): boolean {
    return !!this.apiKey;
  }
}

/**
 * Singleton instance
 */
export const e2bExecutor = new E2BExecutor();
