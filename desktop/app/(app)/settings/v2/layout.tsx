"use client";

import { useState } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { cn } from "@/lib/utils";
import {
    Settings,
    Cloud,
    Brain,
    Palette,
    Database,
    Wrench,
    BookOpen,
    Sliders,
    FileDown,
    FileUp,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";

interface SettingsCategory {
    id: string;
    label: string;
    icon: React.ComponentType<{ className?: string }>;
    path: string;
    description?: string;
}

const SETTINGS_CATEGORIES: SettingsCategory[] = [
    {
        id: "general",
        label: "General",
        icon: Settings,
        path: "/settings/v2/general",
        description: "Basic settings and preferences",
    },
    {
        id: "providers",
        label: "Providers",
        icon: Cloud,
        path: "/settings/v2/providers",
        description: "LLM provider configurations",
    },
    {
        id: "models",
        label: "Models",
        icon: Brain,
        path: "/settings/v2/models",
        description: "Model selection and preferences",
    },
    {
        id: "appearance",
        label: "Appearance",
        icon: Palette,
        path: "/settings/v2/appearance",
        description: "Theme and UI customization",
    },
    {
        id: "context",
        label: "Context",
        icon: Database,
        path: "/settings/v2/context",
        description: "Context window management",
    },
    {
        id: "knowledge",
        label: "Knowledge",
        icon: BookOpen,
        path: "/settings/v2/knowledge",
        description: "RAG and document processing",
    },
    {
        id: "mcp",
        label: "MCP Servers",
        icon: Wrench,
        path: "/settings/v2/mcp",
        description: "Model Context Protocol",
    },
    {
        id: "advanced",
        label: "Advanced",
        icon: Sliders,
        path: "/settings/v2/advanced",
        description: "Advanced system settings",
    },
];

export default function SettingsV2Layout({
    children,
}: {
    children: React.ReactNode;
}) {
    const pathname = usePathname();
    const [collapsed, setCollapsed] = useState(false);

    return (
        <div className="flex h-screen overflow-hidden">
            {/* Sidebar */}
            <div
                className={cn(
                    "border-r bg-muted/10 transition-all duration-300",
                    collapsed ? "w-16" : "w-64"
                )}
            >
                <div className="flex h-full flex-col">
                    {/* Header */}
                    <div className="flex items-center justify-between border-b p-4">
                        {!collapsed && (
                            <h2 className="text-lg font-semibold">Settings</h2>
                        )}
                        <Button
                            variant="ghost"
                            size="icon"
                            onClick={() => setCollapsed(!collapsed)}
                            className="h-8 w-8"
                        >
                            <Settings className="h-4 w-4" />
                        </Button>
                    </div>

                    {/* Navigation */}
                    <nav className="flex-1 space-y-1 overflow-y-auto p-2">
                        {SETTINGS_CATEGORIES.map((category) => {
                            const Icon = category.icon;
                            const isActive = pathname === category.path;

                            return (
                                <Link
                                    key={category.id}
                                    href={category.path}
                                    className={cn(
                                        "flex items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors",
                                        isActive
                                            ? "bg-primary text-primary-foreground"
                                            : "hover:bg-muted"
                                    )}
                                    title={collapsed ? category.label : undefined}
                                >
                                    <Icon className="h-4 w-4 flex-shrink-0" />
                                    {!collapsed && (
                                        <div className="flex-1 truncate">
                                            <div className="font-medium">{category.label}</div>
                                            {category.description && (
                                                <div className="text-xs text-muted-foreground">
                                                    {category.description}
                                                </div>
                                            )}
                                        </div>
                                    )}
                                </Link>
                            );
                        })}
                    </nav>

                    {/* Footer Actions */}
                    {!collapsed && (
                        <>
                            <Separator />
                            <div className="space-y-1 p-2">
                                <Link href="/settings/v2/export">
                                    <Button
                                        variant="outline"
                                        size="sm"
                                        className="w-full justify-start"
                                    >
                                        <FileDown className="mr-2 h-4 w-4" />
                                        Export Settings
                                    </Button>
                                </Link>
                                <Link href="/settings/v2/import">
                                    <Button
                                        variant="outline"
                                        size="sm"
                                        className="w-full justify-start"
                                    >
                                        <FileUp className="mr-2 h-4 w-4" />
                                        Import Settings
                                    </Button>
                                </Link>
                            </div>
                        </>
                    )}
                </div>
            </div>

            {/* Content Area */}
            <div className="flex-1 overflow-auto">
                <div className="container max-w-4xl py-6">{children}</div>
            </div>
        </div>
    );
}
