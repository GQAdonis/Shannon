/**
 * Browser Automation Service
 *
 * Provides browser automation capabilities through Tauri IPC commands.
 * Supports navigation, data extraction, element clicking, and form filling.
 */

import { invoke } from '@tauri-apps/api/core';

export interface PageSnapshot {
  url: string;
  title: string;
  content: string;
  screenshot: number[]; // Vec<u8> from Rust
}

export interface FormField {
  selector: string;
  value: string;
}

export class BrowserService {
  /**
   * Navigate to a URL and capture a page snapshot
   */
  async navigate(url: string): Promise<PageSnapshot> {
    return await invoke<PageSnapshot>('browser_navigate', { url });
  }

  /**
   * Extract text data from a web page using a CSS selector
   */
  async extract(url: string, selector: string): Promise<string> {
    return await invoke<string>('browser_extract', { url, selector });
  }

  /**
   * Click an element on a web page
   */
  async click(url: string, selector: string): Promise<void> {
    await invoke('browser_click', { url, selector });
  }

  /**
   * Fill form fields on a web page
   */
  async fillForm(url: string, fields: FormField[]): Promise<void> {
    await invoke('browser_fill_form', { url, fields });
  }

  /**
   * Convert screenshot bytes to a data URL for display
   */
  screenshotToDataUrl(screenshot: number[]): string {
    const bytes = new Uint8Array(screenshot);
    const base64 = btoa(String.fromCharCode(...bytes));
    return `data:image/png;base64,${base64}`;
  }
}

// Singleton instance
export const browserService = new BrowserService();
