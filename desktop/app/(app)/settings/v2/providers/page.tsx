"use client";

import { useState, useEffect, useCallback } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Switch } from "@/components/ui/switch";
import { Badge } from "@/components/ui/badge";
import { Loader2, Cloud, Check, X, Eye, EyeOff, Trash2 } from "lucide-react";
import { getAppSettings, updateSettingsSection, ProviderSettings } from "@/lib/shannon/settings-v2";
import { toast } from "sonner";

const PROVIDERS = [
    { id: "openai", name: "OpenAI", defaultModel: "gpt-4o" },
    { id: "anthropic", name: "Anthropic", defaultModel: "claude-3-5-sonnet-20241022" },
    { id: "google", name: "Google", defaultModel: "gemini-2.0-flash-exp" },
    { id: "groq", name: "Groq", defaultModel: "llama-3.3-70b-versatile" },
    { id: "xai", name: "xAI", defaultModel: "grok-2-latest" },
];

export default function ProvidersPage() {
    const [loading, setLoading] = useState(true);
    const [saving, setSaving] = useState(false);
    const [providers, setProviders] = useState<ProviderSettings[]>([]);
    const [showKeys, setShowKeys] = useState<Record<string, boolean>>({});

    const loadSettings = useCallback(async () => {
        try {
            const settings = await getAppSettings();
            setProviders(settings.providers);
        } catch (error) {
            console.error("Failed to load provider settings:", error);
            toast.error("Failed to load provider settings");
        } finally {
            setLoading(false);
        }
    }, []);

    useEffect(() => {
        loadSettings();
    }, [loadSettings]);

    const handleSave = async () => {
        setSaving(true);
        try {
            await updateSettingsSection("providers", providers);
            toast.success("Provider settings saved successfully");
        } catch (error) {
            console.error("Failed to save provider settings:", error);
            toast.error("Failed to save provider settings");
        } finally {
            setSaving(false);
        }
    };

    const updateProvider = (providerId: string, updates: Partial<ProviderSettings>) => {
        setProviders(prev => {
            const existing = prev.find(p => p.provider === providerId);
            if (existing) {
                return prev.map(p =>
                    p.provider === providerId ? { ...p, ...updates } : p
                );
            } else {
                const providerInfo = PROVIDERS.find(p => p.id === providerId);
                return [...prev, {
                    provider: providerId,
                    api_key: "",
                    enabled: true,
                    default_model: providerInfo?.defaultModel || "",
                    config: {},
                    ...updates,
                }];
            }
        });
    };

    const removeProvider = (providerId: string) => {
        setProviders(prev => prev.filter(p => p.provider !== providerId));
    };

    const toggleKeyVisibility = (providerId: string) => {
        setShowKeys(prev => ({ ...prev, [providerId]: !prev[providerId] }));
    };

    if (loading) {
        return (
            <div className="flex items-center justify-center p-8">
                <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
            </div>
        );
    }

    return (
        <div className="space-y-6">
            <div>
                <h1 className="text-3xl font-bold">Providers</h1>
                <p className="text-muted-foreground">
                    Configure LLM provider API keys and settings
                </p>
            </div>

            {PROVIDERS.map((providerInfo) => {
                const provider = providers.find(p => p.provider === providerInfo.id);
                const isConfigured = provider && provider.api_key;

                return (
                    <Card key={providerInfo.id}>
                        <CardHeader>
                            <div className="flex items-center justify-between">
                                <div className="flex items-center gap-3">
                                    <Cloud className="h-5 w-5" />
                                    <div>
                                        <CardTitle>{providerInfo.name}</CardTitle>
                                        <CardDescription>
                                            Configure {providerInfo.name} integration
                                        </CardDescription>
                                    </div>
                                </div>
                                <div className="flex items-center gap-2">
                                    {isConfigured && (
                                        <Badge variant="outline" className="bg-green-500/10 text-green-600 border-green-500/20">
                                            <Check className="h-3 w-3 mr-1" />
                                            Configured
                                        </Badge>
                                    )}
                                    <Switch
                                        checked={provider?.enabled || false}
                                        onCheckedChange={(enabled) =>
                                            updateProvider(providerInfo.id, { enabled })
                                        }
                                    />
                                </div>
                            </div>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <div className="space-y-2">
                                <Label htmlFor={`${providerInfo.id}-key`}>API Key</Label>
                                <div className="flex gap-2">
                                    <div className="relative flex-1">
                                        <Input
                                            id={`${providerInfo.id}-key`}
                                            type={showKeys[providerInfo.id] ? "text" : "password"}
                                            value={provider?.api_key || ""}
                                            onChange={(e) =>
                                                updateProvider(providerInfo.id, {
                                                    api_key: e.target.value,
                                                })
                                            }
                                            placeholder={`Enter ${providerInfo.name} API key`}
                                        />
                                        <Button
                                            type="button"
                                            variant="ghost"
                                            size="icon"
                                            className="absolute right-0 top-0 h-full"
                                            onClick={() => toggleKeyVisibility(providerInfo.id)}
                                        >
                                            {showKeys[providerInfo.id] ? (
                                                <EyeOff className="h-4 w-4" />
                                            ) : (
                                                <Eye className="h-4 w-4" />
                                            )}
                                        </Button>
                                    </div>
                                    {isConfigured && (
                                        <Button
                                            variant="outline"
                                            size="icon"
                                            onClick={() => removeProvider(providerInfo.id)}
                                        >
                                            <Trash2 className="h-4 w-4" />
                                        </Button>
                                    )}
                                </div>
                            </div>

                            <div className="space-y-2">
                                <Label htmlFor={`${providerInfo.id}-model`}>Default Model</Label>
                                <Input
                                    id={`${providerInfo.id}-model`}
                                    value={provider?.default_model || providerInfo.defaultModel}
                                    onChange={(e) =>
                                        updateProvider(providerInfo.id, {
                                            default_model: e.target.value,
                                        })
                                    }
                                    placeholder="Model name"
                                />
                            </div>

                            <div className="space-y-2">
                                <Label htmlFor={`${providerInfo.id}-base`}>
                                    API Base URL (Optional)
                                </Label>
                                <Input
                                    id={`${providerInfo.id}-base`}
                                    value={provider?.api_base || ""}
                                    onChange={(e) =>
                                        updateProvider(providerInfo.id, {
                                            api_base: e.target.value || undefined,
                                        })
                                    }
                                    placeholder="https://api.example.com"
                                />
                            </div>
                        </CardContent>
                    </Card>
                );
            })}

            <div className="flex justify-end">
                <Button onClick={handleSave} disabled={saving}>
                    {saving && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                    Save Changes
                </Button>
            </div>
        </div>
    );
}
