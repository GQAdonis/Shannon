"use client";

import { useEffect, useState, useCallback } from "react";
import Link from "next/link";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Loader2, Cloud, Brain, Palette, Database, BookOpen, Wrench, Sliders, ArrowRight, CheckCircle2, AlertCircle } from "lucide-react";
import { getAppSettings } from "@/lib/shannon/settings-v2";

export default function GeneralPage() {
    const [loading, setLoading] = useState(true);
    const [stats, setStats] = useState({
        providersConfigured: 0,
        modelsSet: false,
        themeCustomized: false,
        knowledgeEnabled: false,
        mcpEnabled: false,
    });

    const loadStats = useCallback(async () => {
        try {
            const settings = await getAppSettings();
            setStats({
                providersConfigured: settings.providers.filter(p => p.api_key && p.enabled).length,
                modelsSet: !!settings.models.default_task_model,
                themeCustomized: settings.appearance.theme !== "auto",
                knowledgeEnabled: settings.knowledge.chunking_strategy !== "recursive",
                mcpEnabled: settings.mcp.enabled && settings.mcp.servers.length > 0,
            });
        } catch (error) {
            console.error("Failed to load settings stats:", error);
        } finally {
            setLoading(false);
        }
    }, []);

    useEffect(() => {
        loadStats();
    }, [loadStats]);

    if (loading) {
        return (
            <div className="flex items-center justify-center p-8">
                <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
            </div>
        );
    }

    const categories = [
        {
            title: "Providers",
            description: "Configure LLM provider API keys",
            icon: Cloud,
            href: "/settings/v2/providers",
            status: stats.providersConfigured > 0 ? "configured" : "pending",
            statusText: `${stats.providersConfigured} configured`,
        },
        {
            title: "Models",
            description: "Select default models for tasks",
            icon: Brain,
            href: "/settings/v2/models",
            status: stats.modelsSet ? "configured" : "default",
            statusText: stats.modelsSet ? "Configured" : "Using defaults",
        },
        {
            title: "Appearance",
            description: "Customize theme and UI",
            icon: Palette,
            href: "/settings/v2/appearance",
            status: stats.themeCustomized ? "configured" : "default",
            statusText: stats.themeCustomized ? "Customized" : "Default theme",
        },
        {
            title: "Context",
            description: "Manage context windows",
            icon: Database,
            href: "/settings/v2/context",
            status: "default",
            statusText: "Phase 5 integration",
        },
        {
            title: "Knowledge",
            description: "RAG and document processing",
            icon: BookOpen,
            href: "/settings/v2/knowledge",
            status: stats.knowledgeEnabled ? "configured" : "default",
            statusText: "Phase 7 integration",
        },
        {
            title: "MCP Servers",
            description: "Model Context Protocol tools",
            icon: Wrench,
            href: "/settings/v2/mcp",
            status: stats.mcpEnabled ? "configured" : "default",
            statusText: "Phase 6 integration",
        },
        {
            title: "Advanced",
            description: "System and performance settings",
            icon: Sliders,
            href: "/settings/v2/advanced",
            status: "default",
            statusText: "System configuration",
        },
    ];

    return (
        <div className="space-y-6">
            <div>
                <h1 className="text-3xl font-bold">Settings</h1>
                <p className="text-muted-foreground">
                    Manage your Shannon configuration
                </p>
            </div>

            {/* Quick Stats */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <Card>
                    <CardContent className="pt-6">
                        <div className="flex items-center justify-between">
                            <div>
                                <p className="text-sm font-medium text-muted-foreground">Providers</p>
                                <p className="text-2xl font-bold">{stats.providersConfigured}</p>
                            </div>
                            <Cloud className="h-8 w-8 text-muted-foreground" />
                        </div>
                    </CardContent>
                </Card>

                <Card>
                    <CardContent className="pt-6">
                        <div className="flex items-center justify-between">
                            <div>
                                <p className="text-sm font-medium text-muted-foreground">Theme</p>
                                <p className="text-2xl font-bold">
                                    {stats.themeCustomized ? "Custom" : "Default"}
                                </p>
                            </div>
                            <Palette className="h-8 w-8 text-muted-foreground" />
                        </div>
                    </CardContent>
                </Card>

                <Card>
                    <CardContent className="pt-6">
                        <div className="flex items-center justify-between">
                            <div>
                                <p className="text-sm font-medium text-muted-foreground">MCP Servers</p>
                                <p className="text-2xl font-bold">
                                    {stats.mcpEnabled ? "Enabled" : "Disabled"}
                                </p>
                            </div>
                            <Wrench className="h-8 w-8 text-muted-foreground" />
                        </div>
                    </CardContent>
                </Card>
            </div>

            {/* Settings Categories */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {categories.map((category) => {
                    const Icon = category.icon;
                    const isConfigured = category.status === "configured";
                    const isPending = category.status === "pending";

                    return (
                        <Card key={category.title} className="group hover:shadow-md transition-shadow">
                            <CardHeader>
                                <div className="flex items-center justify-between">
                                    <div className="flex items-center gap-3">
                                        <Icon className="h-5 w-5" />
                                        <CardTitle className="text-lg">{category.title}</CardTitle>
                                    </div>
                                    {isConfigured && (
                                        <Badge variant="outline" className="bg-green-500/10 text-green-600 border-green-500/20">
                                            <CheckCircle2 className="h-3 w-3 mr-1" />
                                            Configured
                                        </Badge>
                                    )}
                                    {isPending && (
                                        <Badge variant="outline" className="bg-amber-500/10 text-amber-600 border-amber-500/20">
                                            <AlertCircle className="h-3 w-3 mr-1" />
                                            Setup Required
                                        </Badge>
                                    )}
                                </div>
                                <CardDescription>{category.description}</CardDescription>
                            </CardHeader>
                            <CardContent>
                                <div className="flex items-center justify-between">
                                    <span className="text-sm text-muted-foreground">
                                        {category.statusText}
                                    </span>
                                    <Link href={category.href}>
                                        <Button variant="ghost" size="sm" className="group-hover:bg-muted">
                                            Configure
                                            <ArrowRight className="ml-2 h-4 w-4" />
                                        </Button>
                                    </Link>
                                </div>
                            </CardContent>
                        </Card>
                    );
                })}
            </div>
        </div>
    );
}
