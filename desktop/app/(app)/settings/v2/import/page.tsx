"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Label } from "@/components/ui/label";
import { Loader2, FileUp, Upload, AlertCircle } from "lucide-react";
import { importSettings } from "@/lib/shannon/settings-v2";
import { toast } from "sonner";

export default function ImportPage() {
    const router = useRouter();
    const [loading, setLoading] = useState(false);
    const [yaml, setYaml] = useState("");

    const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
        const file = e.target.files?.[0];
        if (!file) return;

        const reader = new FileReader();
        reader.onload = (event) => {
            const content = event.target?.result as string;
            setYaml(content);
        };
        reader.readAsText(file);
    };

    const handleImport = async () => {
        if (!yaml.trim()) {
            toast.error("Please provide settings YAML to import");
            return;
        }

        setLoading(true);
        try {
            await importSettings(yaml);
            toast.success("Settings imported successfully");
            router.push("/settings/v2/general");
        } catch (error) {
            console.error("Failed to import settings:", error);
            toast.error("Failed to import settings. Please check the YAML format.");
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="space-y-6">
            <div>
                <h1 className="text-3xl font-bold">Import Settings</h1>
                <p className="text-muted-foreground">
                    Restore Shannon configuration from YAML
                </p>
            </div>

            <Card>
                <CardHeader>
                    <CardTitle>Import Configuration</CardTitle>
                    <CardDescription>
                        Upload a YAML file or paste settings to restore
                    </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                    <div className="space-y-2">
                        <Label htmlFor="file-upload">Upload YAML File</Label>
                        <div className="flex items-center gap-2">
                            <Input
                                id="file-upload"
                                type="file"
                                accept=".yaml,.yml"
                                onChange={handleFileUpload}
                                className="cursor-pointer"
                            />
                            <Upload className="h-4 w-4 text-muted-foreground" />
                        </div>
                    </div>

                    <div className="space-y-2">
                        <Label htmlFor="yaml-input">Or Paste YAML</Label>
                        <Textarea
                            id="yaml-input"
                            value={yaml}
                            onChange={(e) => setYaml(e.target.value)}
                            placeholder="Paste your settings YAML here..."
                            className="font-mono text-xs h-96"
                        />
                    </div>

                    <Button onClick={handleImport} disabled={loading || !yaml.trim()}>
                        {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                        <FileUp className="mr-2 h-4 w-4" />
                        Import Settings
                    </Button>
                </CardContent>
            </Card>

            <Card className="border-amber-500/20 bg-amber-500/5">
                <CardHeader>
                    <CardTitle className="text-amber-600 flex items-center gap-2">
                        <AlertCircle className="h-5 w-5" />
                        Important Notes
                    </CardTitle>
                </CardHeader>
                <CardContent>
                    <ul className="list-disc list-inside space-y-2 text-sm text-muted-foreground">
                        <li>
                            Imported settings will replace your current configuration
                        </li>
                        <li>
                            API keys are redacted in exports and must be re-entered manually
                        </li>
                        <li>
                            Invalid YAML will be rejected - ensure proper formatting
                        </li>
                        <li>
                            Settings are applied immediately after successful import
                        </li>
                    </ul>
                </CardContent>
            </Card>
        </div>
    );
}

// Add Input component import if missing
function Input(props: React.InputHTMLAttributes<HTMLInputElement>) {
    return (
        <input
            {...props}
            className={`flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 ${props.className || ''}`}
        />
    );
}
