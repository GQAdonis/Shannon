"use client";

import { useState, useEffect, useCallback } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Loader2, Brain, Zap, Target, Database } from "lucide-react";
import { getAppSettings, updateSettingsSection, ModelPreferences } from "@/lib/shannon/settings-v2";
import { toast } from "sonner";

const QUICK_MODELS = [
    { value: "gpt-4o-mini", label: "GPT-4o Mini (OpenAI)" },
    { value: "gpt-3.5-turbo", label: "GPT-3.5 Turbo (OpenAI)" },
    { value: "claude-3-5-haiku-20241022", label: "Claude 3.5 Haiku (Anthropic)" },
    { value: "gemini-2.0-flash-exp", label: "Gemini 2.0 Flash (Google)" },
];

const TASK_MODELS = [
    { value: "gpt-4o", label: "GPT-4o (OpenAI)" },
    { value: "o1", label: "o1 (OpenAI)" },
    { value: "claude-3-5-sonnet-20241022", label: "Claude 3.5 Sonnet (Anthropic)" },
    { value: "gemini-2.0-flash-thinking-exp-01-21", label: "Gemini 2.0 Flash Thinking (Google)" },
    { value: "llama-3.3-70b-versatile", label: "Llama 3.3 70B (Groq)" },
];

const EMBEDDING_MODELS = [
    { value: "text-embedding-3-small", label: "Text Embedding 3 Small (OpenAI)" },
    { value: "text-embedding-3-large", label: "Text Embedding 3 Large (OpenAI)" },
    { value: "text-embedding-ada-002", label: "Ada 002 (OpenAI)" },
];

export default function ModelsPage() {
    const [loading, setLoading] = useState(true);
    const [saving, setSaving] = useState(false);
    const [models, setModels] = useState<ModelPreferences>({
        default_quick_model: "gpt-4o-mini",
        default_task_model: "gpt-4o",
        default_embedding_model: "text-embedding-3-small",
        model_overrides: {},
        temperature_overrides: {},
    });

    const loadSettings = useCallback(async () => {
        try {
            const settings = await getAppSettings();
            setModels(settings.models);
        } catch (error) {
            console.error("Failed to load model settings:", error);
            toast.error("Failed to load model settings");
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
            await updateSettingsSection("models", models);
            toast.success("Model preferences saved successfully");
        } catch (error) {
            console.error("Failed to save model settings:", error);
            toast.error("Failed to save model settings");
        } finally {
            setSaving(false);
        }
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
                <h1 className="text-3xl font-bold">Models</h1>
                <p className="text-muted-foreground">
                    Configure default models for different task types
                </p>
            </div>

            {/* Quick Model */}
            <Card>
                <CardHeader>
                    <CardTitle className="flex items-center gap-2">
                        <Zap className="h-5 w-5" />
                        Quick Model
                    </CardTitle>
                    <CardDescription>
                        Fast model for simple queries and quick responses
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <Select
                        value={models.default_quick_model}
                        onValueChange={(value) =>
                            setModels({ ...models, default_quick_model: value })
                        }
                    >
                        <SelectTrigger>
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            {QUICK_MODELS.map((model) => (
                                <SelectItem key={model.value} value={model.value}>
                                    {model.label}
                                </SelectItem>
                            ))}
                        </SelectContent>
                    </Select>
                </CardContent>
            </Card>

            {/* Task Model */}
            <Card>
                <CardHeader>
                    <CardTitle className="flex items-center gap-2">
                        <Target className="h-5 w-5" />
                        Task Model
                    </CardTitle>
                    <CardDescription>
                        Powerful model for complex tasks and reasoning
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <Select
                        value={models.default_task_model}
                        onValueChange={(value) =>
                            setModels({ ...models, default_task_model: value })
                        }
                    >
                        <SelectTrigger>
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            {TASK_MODELS.map((model) => (
                                <SelectItem key={model.value} value={model.value}>
                                    {model.label}
                                </SelectItem>
                            ))}
                        </SelectContent>
                    </Select>
                </CardContent>
            </Card>

            {/* Embedding Model */}
            <Card>
                <CardHeader>
                    <CardTitle className="flex items-center gap-2">
                        <Database className="h-5 w-5" />
                        Embedding Model
                    </CardTitle>
                    <CardDescription>
                        Model for generating embeddings and semantic search
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <Select
                        value={models.default_embedding_model}
                        onValueChange={(value) =>
                            setModels({ ...models, default_embedding_model: value })
                        }
                    >
                        <SelectTrigger>
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            {EMBEDDING_MODELS.map((model) => (
                                <SelectItem key={model.value} value={model.value}>
                                    {model.label}
                                </SelectItem>
                            ))}
                        </SelectContent>
                    </Select>
                </CardContent>
            </Card>

            {/* Custom Model Override */}
            <Card>
                <CardHeader>
                    <CardTitle className="flex items-center gap-2">
                        <Brain className="h-5 w-5" />
                        Custom Model Override
                    </CardTitle>
                    <CardDescription>
                        Specify a custom model name for advanced configurations
                    </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                    <div className="space-y-2">
                        <Label>Custom Model Name</Label>
                        <Input
                            placeholder="e.g., gpt-4-turbo-preview"
                            onChange={(e) => {
                                // This would be expanded to support multiple overrides
                                console.log("Custom model:", e.target.value);
                            }}
                        />
                        <p className="text-xs text-muted-foreground">
                            For provider-specific model names not in the dropdown
                        </p>
                    </div>
                </CardContent>
            </Card>

            <div className="flex justify-end">
                <Button onClick={handleSave} disabled={saving}>
                    {saving && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                    Save Changes
                </Button>
            </div>
        </div>
    );
}
