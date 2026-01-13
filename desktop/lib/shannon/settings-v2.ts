/* eslint-disable @typescript-eslint/no-explicit-any */
"use client";

import { getAccessToken, getAPIKey } from "@/lib/auth";

// =============================================================================
// Base API URL Helper
// =============================================================================

function getApiBaseUrl(): string {
    const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;

    if (isTauri) {
        return (typeof window !== 'undefined' && window.__SHANNON_API_URL) || "";
    }

    return process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";
}

// =============================================================================
// Auth Headers Helper
// =============================================================================

function getAuthHeaders(): Record<string, string> {
    const headers: Record<string, string> = {};

    const apiKey = getAPIKey();
    if (apiKey) {
        headers["X-API-Key"] = apiKey;
        return headers;
    }

    const token = getAccessToken();
    if (token) {
        headers["Authorization"] = `Bearer ${token}`;
        return headers;
    }

    const userId = process.env.NEXT_PUBLIC_USER_ID;
    if (userId) {
        headers["X-User-Id"] = userId;
    }

    return headers;
}

// =============================================================================
// Enhanced Settings Types (Phase 8)
// =============================================================================

export interface AppSettings {
    id: string;
    providers: ProviderSettings[];
    models: ModelPreferences;
    appearance: AppearanceSettings;
    context: ContextSettings;
    knowledge: KnowledgeSettings;
    mcp: MCPSettings;
    advanced: AdvancedSettings;
    created_at: string;
    updated_at: string;
}

export interface ProviderSettings {
    provider: string;
    api_key: string;
    api_base?: string;
    enabled: boolean;
    default_model: string;
    config: Record<string, any>;
}

export interface ModelPreferences {
    default_quick_model: string;
    default_task_model: string;
    default_embedding_model: string;
    model_overrides: Record<string, string>;
    temperature_overrides: Record<string, number>;
}

export interface AppearanceSettings {
    theme: 'light' | 'dark' | 'auto' | { custom: string };
    custom_theme?: CustomTheme;
    language: string;
    font_family: string;
    font_size: number;
    message_density: string;
    sidebar_position: string;
}

export interface CustomTheme {
    name: string;
    colors: Record<string, string>;
    fonts: Record<string, string>;
    spacing: Record<string, string>;
}

export interface ContextSettings {
    max_context_tokens: number;
    retention_strategy: string;
    auto_summarize: boolean;
}

export interface KnowledgeSettings {
    chunking_strategy: string;
    chunk_size: number;
    chunk_overlap: number;
    embedding_provider: string;
    similarity_threshold: number;
    max_results: number;
}

export interface MCPSettings {
    enabled: boolean;
    servers: MCPServerConfig[];
    auto_discover: boolean;
}

export interface MCPServerConfig {
    name: string;
    command: string;
    args: string[];
    env: Record<string, string>;
    enabled: boolean;
}

export interface AdvancedSettings {
    debug_mode: boolean;
    telemetry_enabled: boolean;
    auto_update: boolean;
    concurrent_requests: number;
    request_timeout: number;
    experimental_features: boolean;
}

export type SettingsSection =
    | 'providers'
    | 'models'
    | 'appearance'
    | 'context'
    | 'knowledge'
    | 'mcp'
    | 'advanced';

// =============================================================================
// Settings V2 API Functions
// =============================================================================

/**
 * Get comprehensive application settings.
 */
export async function getAppSettings(): Promise<AppSettings> {
    const response = await fetch(`${getApiBaseUrl()}/api/v2/settings`, {
        method: "GET",
        headers: getAuthHeaders(),
    });

    if (!response.ok) {
        throw new Error(`Failed to get app settings: ${response.statusText}`);
    }

    return response.json();
}

/**
 * Update comprehensive application settings.
 */
export async function updateAppSettings(settings: AppSettings): Promise<void> {
    const response = await fetch(`${getApiBaseUrl()}/api/v2/settings`, {
        method: "PUT",
        headers: {
            "Content-Type": "application/json",
            ...getAuthHeaders(),
        },
        body: JSON.stringify({ settings }),
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to update settings: ${response.statusText}`);
    }
}

/**
 * Update a specific settings section.
 */
export async function updateSettingsSection(
    section: SettingsSection,
    value: any
): Promise<void> {
    const response = await fetch(`${getApiBaseUrl()}/api/v2/settings/${section}`, {
        method: "PUT",
        headers: {
            "Content-Type": "application/json",
            ...getAuthHeaders(),
        },
        body: JSON.stringify(value),
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to update ${section}: ${response.statusText}`);
    }
}

/**
 * Export settings as YAML.
 */
export async function exportSettings(): Promise<string> {
    const response = await fetch(`${getApiBaseUrl()}/api/v2/settings/export`, {
        method: "GET",
        headers: getAuthHeaders(),
    });

    if (!response.ok) {
        throw new Error(`Failed to export settings: ${response.statusText}`);
    }

    return response.text();
}

/**
 * Import settings from YAML.
 */
export async function importSettings(yaml: string): Promise<void> {
    const response = await fetch(`${getApiBaseUrl()}/api/v2/settings/import`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            ...getAuthHeaders(),
        },
        body: JSON.stringify({ yaml }),
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to import settings: ${response.statusText}`);
    }
}

// =============================================================================
// Default Settings
// =============================================================================

export const DEFAULT_APP_SETTINGS: AppSettings = {
    id: "default",
    providers: [],
    models: {
        default_quick_model: "gpt-4o-mini",
        default_task_model: "gpt-4o",
        default_embedding_model: "text-embedding-3-small",
        model_overrides: {},
        temperature_overrides: {},
    },
    appearance: {
        theme: "auto",
        language: "en",
        font_family: "Inter",
        font_size: 14,
        message_density: "normal",
        sidebar_position: "left",
    },
    context: {
        max_context_tokens: 128000,
        retention_strategy: "sliding_window",
        auto_summarize: true,
    },
    knowledge: {
        chunking_strategy: "recursive",
        chunk_size: 1000,
        chunk_overlap: 200,
        embedding_provider: "openai",
        similarity_threshold: 0.7,
        max_results: 5,
    },
    mcp: {
        enabled: true,
        servers: [],
        auto_discover: true,
    },
    advanced: {
        debug_mode: false,
        telemetry_enabled: false,
        auto_update: true,
        concurrent_requests: 5,
        request_timeout: 300,
        experimental_features: false,
    },
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
};

// =============================================================================
// Legacy Settings API Functions (for backward compatibility)
// =============================================================================

export interface UserSetting {
    user_id: string;
    setting_key: string;
    setting_value: string;
    setting_type: string;
    encrypted: boolean;
    created_at: string;
    updated_at: string;
}

export interface SetSettingRequest {
    key: string;
    value: string;
    setting_type?: string;
    encrypted?: boolean;
}

export interface ApiKeyInfo {
    provider: string;
    is_configured: boolean;
    masked_key: string | null;
    is_active: boolean;
    last_used_at: string | null;
    created_at: string | null;
}

export interface SetApiKeyResponse {
    provider: string;
    masked_key: string;
    message: string;
}

export async function getAllSettings(): Promise<UserSetting[]> {
    const response = await fetch(`${getApiBaseUrl()}/api/v1/settings`, {
        method: "GET",
        headers: getAuthHeaders(),
    });

    if (!response.ok) {
        throw new Error(`Failed to get settings: ${response.statusText}`);
    }

    return response.json();
}

export async function getSetting(key: string): Promise<UserSetting> {
    const response = await fetch(`${getApiBaseUrl()}/api/v1/settings/${encodeURIComponent(key)}`, {
        method: "GET",
        headers: getAuthHeaders(),
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to get setting: ${response.statusText}`);
    }

    return response.json();
}

export async function setSetting(
    key: string,
    value: string,
    type: string = "string",
    encrypted: boolean = false
): Promise<UserSetting> {
    const response = await fetch(`${getApiBaseUrl()}/api/v1/settings`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            ...getAuthHeaders(),
        },
        body: JSON.stringify({
            key,
            value,
            setting_type: type,
            encrypted,
        }),
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to set setting: ${response.statusText}`);
    }

    await response.json();

    try {
        return await getSetting(key);
    } catch {
        return {
            user_id: "",
            setting_key: key,
            setting_value: value,
            setting_type: type,
            encrypted,
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString(),
        };
    }
}

export async function deleteSetting(key: string): Promise<void> {
    const response = await fetch(`${getApiBaseUrl()}/api/v1/settings/${encodeURIComponent(key)}`, {
        method: "DELETE",
        headers: getAuthHeaders(),
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to delete setting: ${response.statusText}`);
    }
}

export async function listApiKeys(): Promise<ApiKeyInfo[]> {
    const response = await fetch(`${getApiBaseUrl()}/api/v1/settings/api-keys`, {
        method: "GET",
        headers: getAuthHeaders(),
    });

    if (!response.ok) {
        throw new Error(`Failed to list API keys: ${response.statusText}`);
    }

    return response.json();
}

export async function setApiKey(provider: string, apiKey: string): Promise<SetApiKeyResponse> {
    const response = await fetch(`${getApiBaseUrl()}/api/v1/settings/api-keys/${encodeURIComponent(provider)}`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            ...getAuthHeaders(),
        },
        body: JSON.stringify({
            api_key: apiKey,
        }),
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to set API key: ${response.statusText}`);
    }

    return response.json();
}

export async function deleteApiKey(provider: string): Promise<void> {
    const response = await fetch(`${getApiBaseUrl()}/api/v1/settings/api-keys/${encodeURIComponent(provider)}`, {
        method: "DELETE",
        headers: getAuthHeaders(),
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to delete API key: ${response.statusText}`);
    }
}
