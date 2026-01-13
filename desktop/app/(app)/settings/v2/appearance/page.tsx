"use client";

import { useState, useEffect, useCallback } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Slider } from "@/components/ui/slider";
import { Button } from "@/components/ui/button";
import { Loader2, Sun, Moon, Monitor, Palette } from "lucide-react";
import { getAppSettings, updateSettingsSection, AppearanceSettings } from "@/lib/shannon/settings-v2";
import { useToast } from "@/hooks/use-toast";

const THEMES = [
    { value: "light", label: "Light", icon: Sun },
    { value: "dark", label: "Dark", icon: Moon },
    { value: "auto", label: "Auto", icon: Monitor },
];

const LANGUAGES = [
    { value: "en", label: "English" },
    { value: "es", label: "Español" },
    { value: "fr", label: "Français" },
    { value: "de", label: "Deutsch" },
    { value: "zh", label: "中文" },
];

const FONT_FAMILIES = [
    { value: "Inter", label: "Inter" },
    { value: "system-ui", label: "System" },
    { value: "monospace", label: "Monospace" },
];

const MESSAGE_DENSITIES = [
    { value: "compact", label: "Compact" },
    { value: "normal", label: "Normal" },
    { value: "comfortable", label: "Comfortable" },
];

export default function AppearancePage() {
    const { toast } = useToast();
    const [loading, setLoading] = useState(true);
    const [saving, setSaving] = useState(false);
    const [appearance, setAppearance] = useState<AppearanceSettings>({
        theme: "auto",
        language: "en",
        font_family: "Inter",
        font_size: 14,
        message_density: "normal",
        sidebar_position: "left",
    });

    const loadSettings = useCallback(async () => {
        try {
            const settings = await getAppSettings();
            setAppearance(settings.appearance);
        } catch (error) {
            console.error("Failed to load appearance settings:", error);
            toast({
                title: "Error",
                description: "Failed to load appearance settings",
                variant: "destructive",
            });
        } finally {
            setLoading(false);
        }
    }, [toast]);

    useEffect(() => {
        loadSettings();
    }, [loadSettings]);

    const handleSave = async () => {
        setSaving(true);
        try {
            await updateSettingsSection("appearance", appearance);

            // Apply theme immediately
            applyTheme(appearance.theme);

            toast({
                title: "Success",
                description: "Appearance settings saved successfully",
            });
        } catch (error) {
            console.error("Failed to save appearance settings:", error);
            toast({
                title: "Error",
                description: "Failed to save appearance settings",
                variant: "destructive",
            });
        } finally {
            setSaving(false);
        }
    };

    const applyTheme = (theme: string | { custom: string }) => {
        const themeValue = typeof theme === "string" ? theme : theme.custom;

        if (themeValue === "auto") {
            const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
            document.documentElement.classList.toggle("dark", prefersDark);
        } else {
            document.documentElement.classList.toggle("dark", themeValue === "dark");
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
                <h1 className="text-3xl font-bold">Appearance</h1>
                <p className="text-muted-foreground">
                    Customize the look and feel of Shannon
                </p>
            </div>

            {/* Theme Selection */}
            <Card>
                <CardHeader>
                    <CardTitle>Theme</CardTitle>
                    <CardDescription>
                        Choose your preferred color scheme
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <RadioGroup
                        value={typeof appearance.theme === "string" ? appearance.theme : "custom"}
                        onValueChange={(value) =>
                            setAppearance({ ...appearance, theme: value as 'light' | 'dark' | 'auto' })
                        }
                    >
                        <div className="grid grid-cols-3 gap-4">
                            {THEMES.map((theme) => {
                                const Icon = theme.icon;
                                return (
                                    <Label
                                        key={theme.value}
                                        htmlFor={theme.value}
                                        className="flex cursor-pointer flex-col items-center gap-2 rounded-lg border-2 border-muted p-4 hover:bg-accent hover:text-accent-foreground [&:has([data-state=checked])]:border-primary"
                                    >
                                        <RadioGroupItem
                                            value={theme.value}
                                            id={theme.value}
                                            className="sr-only"
                                        />
                                        <Icon className="h-6 w-6" />
                                        <span className="text-sm font-medium">{theme.label}</span>
                                    </Label>
                                );
                            })}
                        </div>
                    </RadioGroup>
                </CardContent>
            </Card>

            {/* Language */}
            <Card>
                <CardHeader>
                    <CardTitle>Language</CardTitle>
                    <CardDescription>
                        Select your preferred language
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <Select
                        value={appearance.language}
                        onValueChange={(value) =>
                            setAppearance({ ...appearance, language: value })
                        }
                    >
                        <SelectTrigger>
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            {LANGUAGES.map((lang) => (
                                <SelectItem key={lang.value} value={lang.value}>
                                    {lang.label}
                                </SelectItem>
                            ))}
                        </SelectContent>
                    </Select>
                </CardContent>
            </Card>

            {/* Font Settings */}
            <Card>
                <CardHeader>
                    <CardTitle>Font</CardTitle>
                    <CardDescription>
                        Customize font family and size
                    </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                    <div className="space-y-2">
                        <Label>Font Family</Label>
                        <Select
                            value={appearance.font_family}
                            onValueChange={(value) =>
                                setAppearance({ ...appearance, font_family: value })
                            }
                        >
                            <SelectTrigger>
                                <SelectValue />
                            </SelectTrigger>
                            <SelectContent>
                                {FONT_FAMILIES.map((font) => (
                                    <SelectItem key={font.value} value={font.value}>
                                        {font.label}
                                    </SelectItem>
                                ))}
                            </SelectContent>
                        </Select>
                    </div>

                    <div className="space-y-2">
                        <Label>Font Size: {appearance.font_size}px</Label>
                        <Slider
                            value={[appearance.font_size]}
                            onValueChange={([value]) =>
                                setAppearance({ ...appearance, font_size: value })
                            }
                            min={10}
                            max={20}
                            step={1}
                        />
                    </div>
                </CardContent>
            </Card>

            {/* Message Density */}
            <Card>
                <CardHeader>
                    <CardTitle>Message Density</CardTitle>
                    <CardDescription>
                        Adjust spacing between messages
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <Select
                        value={appearance.message_density}
                        onValueChange={(value) =>
                            setAppearance({ ...appearance, message_density: value })
                        }
                    >
                        <SelectTrigger>
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            {MESSAGE_DENSITIES.map((density) => (
                                <SelectItem key={density.value} value={density.value}>
                                    {density.label}
                                </SelectItem>
                            ))}
                        </SelectContent>
                    </Select>
                </CardContent>
            </Card>

            {/* Save Button */}
            <div className="flex justify-end">
                <Button onClick={handleSave} disabled={saving}>
                    {saving && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                    Save Changes
                </Button>
            </div>
        </div>
    );
}
