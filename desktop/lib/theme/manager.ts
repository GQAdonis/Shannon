/**
 * Theme Manager
 *
 * Handles theme switching, custom themes, and system preference detection.
 */

import { AppearanceSettings, CustomTheme } from "@/lib/shannon/settings-v2";

export type ThemeMode = "light" | "dark" | "auto";

export class ThemeManager {
    private mediaQuery: MediaQuery | null = null;

    constructor() {
        if (typeof window !== "undefined") {
            this.mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
        }
    }

    /**
     * Apply theme based on settings
     */
    async setTheme(appearance: AppearanceSettings): Promise<void> {
        const theme = appearance.theme;

        if (typeof theme === "string") {
            await this.applyBasicTheme(theme);
        } else if (typeof theme === "object" && "custom" in theme) {
            await this.applyCustomTheme(appearance.custom_theme);
        }

        // Apply font settings
        this.applyFontSettings(appearance);
    }

    /**
     * Apply basic theme (light/dark/auto)
     */
    private async applyBasicTheme(mode: string): Promise<void> {
        const root = document.documentElement;

        switch (mode) {
            case "light":
                root.classList.remove("dark");
                break;
            case "dark":
                root.classList.add("dark");
                break;
            case "auto":
                const prefersDark = this.mediaQuery?.matches ?? false;
                root.classList.toggle("dark", prefersDark);
                break;
        }
    }

    /**
     * Apply custom theme with color overrides
     */
    private async applyCustomTheme(theme?: CustomTheme): Promise<void> {
        if (!theme) return;

        const root = document.documentElement;

        // Apply custom colors as CSS variables
        Object.entries(theme.colors).forEach(([key, value]) => {
            root.style.setProperty(`--${key}`, value);
        });

        // Apply custom fonts
        if (theme.fonts) {
            Object.entries(theme.fonts).forEach(([key, value]) => {
                root.style.setProperty(`--font-${key}`, value);
            });
        }

        // Apply custom spacing
        if (theme.spacing) {
            Object.entries(theme.spacing).forEach(([key, value]) => {
                root.style.setProperty(`--spacing-${key}`, value);
            });
        }
    }

    /**
     * Apply font settings
     */
    private applyFontSettings(appearance: AppearanceSettings): void {
        const root = document.documentElement;
        root.style.setProperty("--font-family", appearance.font_family);
        root.style.setProperty("--font-size", `${appearance.font_size}px`);
    }

    /**
     * Watch for system theme preference changes
     */
    watchSystemPreference(callback: (isDark: boolean) => void): () => void {
        if (!this.mediaQuery) {
            return () => {};
        }

        const handler = (e: MediaQueryListEvent) => {
            callback(e.matches);
        };

        this.mediaQuery.addEventListener("change", handler);

        return () => {
            this.mediaQuery?.removeEventListener("change", handler);
        };
    }

    /**
     * Get current system theme preference
     */
    getSystemPreference(): "light" | "dark" {
        return this.mediaQuery?.matches ? "dark" : "light";
    }

    /**
     * Create a custom theme from current CSS variables
     */
    extractCurrentTheme(name: string): CustomTheme {
        const root = document.documentElement;
        const styles = getComputedStyle(root);

        const colors: Record<string, string> = {};
        const fonts: Record<string, string> = {};
        const spacing: Record<string, string> = {};

        // Extract color variables
        ["primary", "secondary", "background", "foreground", "border", "accent"].forEach(key => {
            const value = styles.getPropertyValue(`--${key}`);
            if (value) colors[key] = value.trim();
        });

        // Extract font variables
        ["sans", "mono"].forEach(key => {
            const value = styles.getPropertyValue(`--font-${key}`);
            if (value) fonts[key] = value.trim();
        });

        return {
            name,
            colors,
            fonts,
            spacing,
        };
    }

    /**
     * Validate theme structure
     */
    validateTheme(theme: CustomTheme): boolean {
        if (!theme.name || !theme.colors) {
            return false;
        }

        const requiredColors = ["primary", "background", "foreground"];
        return requiredColors.every(key => key in theme.colors);
    }
}

// Singleton instance
let themeManagerInstance: ThemeManager | null = null;

export function getThemeManager(): ThemeManager {
    if (!themeManagerInstance) {
        themeManagerInstance = new ThemeManager();
    }
    return themeManagerInstance;
}

// React hook for theme management
export function useThemeManager() {
    return getThemeManager();
}
