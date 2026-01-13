"use client";

import { useState } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Loader2, FileDown, Copy, Check } from "lucide-react";
import { exportSettings } from "@/lib/shannon/settings-v2";
import { toast } from "sonner";

export default function ExportPage() {
    const [loading, setLoading] = useState(false);
    const [yaml, setYaml] = useState("");
    const [copied, setCopied] = useState(false);

    const handleExport = async () => {
        setLoading(true);
        try {
            const exportedYaml = await exportSettings();
            setYaml(exportedYaml);
            toast.success("Settings exported successfully");
        } catch (error) {
            console.error("Failed to export settings:", error);
            toast.error("Failed to export settings");
        } finally {
            setLoading(false);
        }
    };

    const handleDownload = () => {
        const blob = new Blob([yaml], { type: "text/yaml" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `shannon-settings-${new Date().toISOString().split("T")[0]}.yaml`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
        toast.success("Settings downloaded");
    };

    const handleCopy = async () => {
        try {
            await navigator.clipboard.writeText(yaml);
            setCopied(true);
            setTimeout(() => setCopied(false), 2000);
            toast.success("Settings copied to clipboard");
        } catch (error) {
            toast.error("Failed to copy to clipboard");
        }
    };

    return (
        <div className="space-y-6">
            <div>
                <h1 className="text-3xl font-bold">Export Settings</h1>
                <p className="text-muted-foreground">
                    Export your Shannon configuration as YAML
                </p>
            </div>

            <Card>
                <CardHeader>
                    <CardTitle>Export Configuration</CardTitle>
                    <CardDescription>
                        Download or copy your settings for backup or sharing
                    </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                    {!yaml ? (
                        <Button onClick={handleExport} disabled={loading}>
                            {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                            <FileDown className="mr-2 h-4 w-4" />
                            Export Settings
                        </Button>
                    ) : (
                        <>
                            <Textarea
                                value={yaml}
                                readOnly
                                className="font-mono text-xs h-96"
                            />
                            <div className="flex gap-2">
                                <Button onClick={handleDownload}>
                                    <FileDown className="mr-2 h-4 w-4" />
                                    Download YAML
                                </Button>
                                <Button variant="outline" onClick={handleCopy}>
                                    {copied ? (
                                        <Check className="mr-2 h-4 w-4" />
                                    ) : (
                                        <Copy className="mr-2 h-4 w-4" />
                                    )}
                                    {copied ? "Copied!" : "Copy to Clipboard"}
                                </Button>
                            </div>
                        </>
                    )}
                </CardContent>
            </Card>

            <Card className="border-amber-500/20 bg-amber-500/5">
                <CardHeader>
                    <CardTitle className="text-amber-600">Note</CardTitle>
                </CardHeader>
                <CardContent>
                    <p className="text-sm text-muted-foreground">
                        Exported settings will have API keys redacted for security. You'll need to
                        re-enter them after importing.
                    </p>
                </CardContent>
            </Card>
        </div>
    );
}
