/**
 * E2B Python Code Executor (Browser Stub)
 *
 * Stub implementation for browser builds to avoid Node.js module issues.
 * E2B code execution requires server-side environment and is disabled in browser builds.
 */

import { ExecutionResult } from './types';

export class E2BExecutor {
  private apiKey: string;

  constructor(apiKey?: string) {
    this.apiKey = apiKey || process.env.NEXT_PUBLIC_E2B_API_KEY || '';
  }

  /**
   * Execute Python code in E2B sandbox (disabled in browser builds)
   */
  async executePython(code: string): Promise<ExecutionResult> {
    return {
      success: false,
      output: '',
      error: 'E2B execution is not available in browser builds. This feature requires a server-side environment.',
      results: [],
    };
  }

  /**
   * Execute Python code with timeout (disabled in browser builds)
   */
  async executePythonWithTimeout(
    code: string,
    timeoutMs: number = 30000
  ): Promise<ExecutionResult> {
    return this.executePython(code);
  }

  /**
   * Check if E2B is configured (always false in browser builds)
   */
  isConfigured(): boolean {
    return false;
  }
}

/**
 * Singleton instance
 */
export const e2bExecutor = new E2BExecutor();
