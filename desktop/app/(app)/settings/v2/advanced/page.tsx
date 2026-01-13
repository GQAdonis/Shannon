"use client";

import { useState, useEffect } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Loader2, Sliders, Bug, Radio, RefreshCw, Zap, Clock, Flask } from "lucide-react";
import { getAppSettings, updateSettingsSection, AdvancedSettings } from "@/lib/shannon/settings-v2";
import { toast } from "sonner";

export default function AdvancedPage() {
    const [loading, setLoading] = useState(true);
    const [saving, setSaving] = useState(false);
    const [advanced, setAdvanced] = useState<AdvancedSettings>({
        debug_mode: false,
        telemetry_enabled: false,
        auto_update: true,
        concurrent_requests: 5,
        request_timeout: 300,
        experimental_features: false,
    });

    useEffect(() => {
        loadSettings();
    }, []);

    const loadSettings = async () => {
        try {
            const settings = await getAppSettings();
            setAdvanced(settings.advanced);
        } catch (error) {
            console.error("Failed to load advanced settings:", error);
            toast.error("Failed to load advanced settings");
        } finally {
            setLoading(false);
        }
    };

    const handleSave = async () => {
        setSaving(true);
        try {
            await updateSettingsSection("advanced", advanced);
            toast.success("Advanced settings saved successfully");
        } catch (error) {
            console.error("Failed to save advanced settings:", error);
            toast.error("Failed to save advanced settings");
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
                <h1 className="text-3xl font-bold">Advanced Settings</h1>
                <p className="text-muted-foreground">
                    Configure advanced system behavior and performance
                </p>
            </div>

            {/* Debug Mode */}
            <Card>
                <CardHeader>
                    <CardTitle className="flex items-center gap-2">
                        <Bug className="h-5 w-5" />
                        Debug Mode
                    </CardTitle>
                    <CardDescription>
                        Enable detailed logging and debugging information
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <div className="flex items-center justify-between">
                        <Label htmlFor="debug-mode">Enable debug logging</Label>
                        <Switch
                            id="debug-mode"
                            checked={advanced.debug_mode}
                            onCheckedChange={(checked) =>
                                setAdvanced({ ...advanced, debug_mode: checked })
                            }
                        />
                    </div>
                </CardContent>
            </Card>

            {/* Telemetry */}
            <Card>
                <CardHeader>
                    <CardTitle className="flex items-center gap-2">
                        <Radio className="h-5 w-5" />
                        Telemetry
                    </CardTitle>
                    <CardDescription>
                        Help improve Shannon by sharing anonymous usage data
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <div className="flex items-center justify-between">
                        <Label htmlFor="telemetry">Enable telemetry</Label>
                        <Switch
                            id="telemetry"
                            checked={advanced.telemetry_enabled}
                            onCheckedChange={(checked) =>
                                setAdvanced({ ...advanced, telemetry_enabled: checked })
                            }
                        />
                    </div>
                </CardContent>
            </Card>

            {/* Auto Update */}
            <Card>
                <CardHeader>
                    <CardTitle className="flex items-center gap-2">
                        <RefreshCw className="h-5 w-5" />
                        Auto Update
                    </CardTitle>
                    <CardDescription>
                        Automatically download and install updates
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <div className="flex items-center justify-between">
                        <Label htmlFor="auto-update">Enable automatic updates</Label>
                        <Switch
                            id="auto-update"
                            checked={advanced.auto_update}
                            onCheckedChange={(checked) =>
                                setAdvanced({ ...advanced, auto_update: checked })
                            }
                        />
                    </div>
                </CardContent>
            </Card>

            {/* Performance */}
            <Card>
                <CardHeader>
                    <CardTitle className="flex items-center gap-2">
                        <Zap className="h-5 w-5" />
                        Performance
                    </CardTitle>
                    <CardDescription>
                        Configure request concurrency and timeouts
                    </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                    <div className="space-y-2">
                        <Label htmlFor="concurrent">
                            Concurrent Requests: {advanced.concurrent_requests}
                        </Label>
                        <Input
                            id="concurrent"
                            type="number"
                            min={1}
                            max={20}
                            value={advanced.concurrent_requests}
                            onChange={(e) =>
                                setAdvanced({
                                    ...advanced,
                                    concurrent_requests: parseInt(e.target.value) || 5,
                                })
                            }
                        />
                        <p className="text-xs text-muted-foreground">
                            Maximum number of simultaneous API requests (1-20)
                        </p>
                    </div>

                    <div className="space-y-2">
                        <Label htmlFor="timeout">
                            Request Timeout: {advanced.request_timeout}s
                        </Label>
                        <Input
                            id="timeout"
                            type="number"
                            min={30}
                            max={600}
                            value={advanced.request_timeout}
                            onChange={(e) =>
                                setAdvanced({
                                    ...advanced,
                                    request_timeout: parseInt(e.target.value) || 300,
                                })
                            }
                        />
                        <p className="text-xs text-muted-foreground">
                            Timeout for API requests in seconds (30-600)
                        </p>
                    </div>
                </CardContent>
            </Card>

            {/* Experimental Features */}
            <Card>
                <CardHeader>
                    <CardTitle className="flex items-center gap-2">
                        <Flask className="h-5 w-5" />
                        Experimental Features
                    </CardTitle>
                    <CardDescription>
                        Enable experimental and unstable features
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <div className="flex items-center justify-between">
                        <div className="space-y-0.5">
                            <Label htmlFor="experimental">Enable experimental features</Label>
                            <p className="text-xs text-muted-foreground">
                                May be unstable or change without notice
                            </p>
                        </div>
                        <Switch
                            id="experimental"
                            checked={advanced.experimental_features}
                            onCheckedChange={(checked) =>
                                setAdvanced({ ...advanced, experimental_features: checked })
                            }
                        />
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
