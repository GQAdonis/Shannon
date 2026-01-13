/**
 * Theme Customizer Component
 *
 * Advanced theme customization with color picker and preview
 */

'use client';

import { useState } from 'react';
import { useTheme } from 'next-themes';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Label } from '@/components/ui/label';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Slider } from '@/components/ui/slider';
import { Sun, Moon, Laptop, Download, Upload, RotateCcw } from 'lucide-react';

interface ThemeColors {
  primary: string;
  secondary: string;
  accent: string;
  background: string;
  foreground: string;
}

interface ThemeConfig {
  colors: ThemeColors;
  fonts: {
    sans: string;
    mono: string;
  };
  spacing: {
    scale: number;
  };
  radius: {
    value: number;
  };
}

const DEFAULT_THEME: ThemeConfig = {
  colors: {
    primary: 'hsl(222.2, 47.4%, 11.2%)',
    secondary: 'hsl(210, 40%, 96.1%)',
    accent: 'hsl(210, 40%, 96.1%)',
    background: 'hsl(0, 0%, 100%)',
    foreground: 'hsl(222.2, 47.4%, 11.2%)',
  },
  fonts: {
    sans: 'system-ui',
    mono: 'monospace',
  },
  spacing: {
    scale: 1,
  },
  radius: {
    value: 0.5,
  },
};

const SYSTEM_FONTS = [
  { label: 'System Default', value: 'system-ui' },
  { label: 'Inter', value: 'Inter, system-ui' },
  { label: 'Helvetica', value: 'Helvetica, system-ui' },
  { label: 'Arial', value: 'Arial, system-ui' },
  { label: 'SF Pro', value: '-apple-system, BlinkMacSystemFont, system-ui' },
];

const MONO_FONTS = [
  { label: 'System Monospace', value: 'monospace' },
  { label: 'JetBrains Mono', value: 'JetBrains Mono, monospace' },
  { label: 'Fira Code', value: 'Fira Code, monospace' },
  { label: 'Consolas', value: 'Consolas, monospace' },
  { label: 'Monaco', value: 'Monaco, monospace' },
];

export function ThemeCustomizer() {
  const { theme, setTheme } = useTheme();
  const [customTheme, setCustomTheme] = useState<ThemeConfig>(DEFAULT_THEME);

  const updateThemeColor = (key: keyof ThemeColors, value: string) => {
    setCustomTheme((prev) => ({
      ...prev,
      colors: {
        ...prev.colors,
        [key]: value,
      },
    }));
  };

  const updateThemeFont = (type: 'sans' | 'mono', value: string) => {
    setCustomTheme((prev) => ({
      ...prev,
      fonts: {
        ...prev.fonts,
        [type]: value,
      },
    }));
  };

  const updateSpacing = (scale: number) => {
    setCustomTheme((prev) => ({
      ...prev,
      spacing: { scale },
    }));
  };

  const updateRadius = (value: number) => {
    setCustomTheme((prev) => ({
      ...prev,
      radius: { value },
    }));
  };

  const applyTheme = () => {
    // Apply custom CSS variables
    const root = document.documentElement;
    root.style.setProperty('--font-sans', customTheme.fonts.sans);
    root.style.setProperty('--font-mono', customTheme.fonts.mono);
    root.style.setProperty('--spacing-scale', customTheme.spacing.scale.toString());
    root.style.setProperty('--radius', `${customTheme.radius.value}rem`);

    // Save to localStorage
    localStorage.setItem('shannon_custom_theme', JSON.stringify(customTheme));
  };

  const resetTheme = () => {
    setCustomTheme(DEFAULT_THEME);
    localStorage.removeItem('shannon_custom_theme');

    // Reset CSS variables
    const root = document.documentElement;
    root.style.removeProperty('--font-sans');
    root.style.removeProperty('--font-mono');
    root.style.removeProperty('--spacing-scale');
    root.style.removeProperty('--radius');
  };

  const exportTheme = () => {
    const themeJson = JSON.stringify(customTheme, null, 2);
    const blob = new Blob([themeJson], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'shannon-theme.json';
    a.click();
    URL.revokeObjectURL(url);
  };

  const importTheme = () => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = 'application/json';
    input.onchange = (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (file) {
        const reader = new FileReader();
        reader.onload = (event) => {
          try {
            const imported = JSON.parse(event.target?.result as string);
            setCustomTheme(imported);
          } catch (error) {
            console.error('Failed to import theme:', error);
          }
        };
        reader.readAsText(file);
      }
    };
    input.click();
  };

  return (
    <div className="space-y-6">
      {/* Theme Mode Selector */}
      <Card>
        <CardHeader>
          <CardTitle>Theme Mode</CardTitle>
          <CardDescription>Choose between light, dark, or system theme</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex gap-2">
            <Button
              variant={theme === 'light' ? 'default' : 'outline'}
              onClick={() => setTheme('light')}
              className="flex-1"
            >
              <Sun className="mr-2 h-4 w-4" />
              Light
            </Button>
            <Button
              variant={theme === 'dark' ? 'default' : 'outline'}
              onClick={() => setTheme('dark')}
              className="flex-1"
            >
              <Moon className="mr-2 h-4 w-4" />
              Dark
            </Button>
            <Button
              variant={theme === 'system' ? 'default' : 'outline'}
              onClick={() => setTheme('system')}
              className="flex-1"
            >
              <Laptop className="mr-2 h-4 w-4" />
              System
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Advanced Customization */}
      <Card>
        <CardHeader>
          <CardTitle>Advanced Customization</CardTitle>
          <CardDescription>Customize fonts, spacing, and more</CardDescription>
        </CardHeader>
        <CardContent>
          <Tabs defaultValue="fonts" className="w-full">
            <TabsList className="grid w-full grid-cols-3">
              <TabsTrigger value="fonts">Fonts</TabsTrigger>
              <TabsTrigger value="spacing">Spacing</TabsTrigger>
              <TabsTrigger value="radius">Radius</TabsTrigger>
            </TabsList>

            <TabsContent value="fonts" className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="sans-font">Sans-serif Font</Label>
                <Select
                  value={customTheme.fonts.sans}
                  onValueChange={(value) => updateThemeFont('sans', value)}
                >
                  <SelectTrigger id="sans-font">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {SYSTEM_FONTS.map((font) => (
                      <SelectItem key={font.value} value={font.value}>
                        {font.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label htmlFor="mono-font">Monospace Font</Label>
                <Select
                  value={customTheme.fonts.mono}
                  onValueChange={(value) => updateThemeFont('mono', value)}
                >
                  <SelectTrigger id="mono-font">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {MONO_FONTS.map((font) => (
                      <SelectItem key={font.value} value={font.value}>
                        {font.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </TabsContent>

            <TabsContent value="spacing" className="space-y-4">
              <div className="space-y-2">
                <Label>Spacing Scale: {customTheme.spacing.scale.toFixed(2)}x</Label>
                <Slider
                  value={[customTheme.spacing.scale]}
                  onValueChange={([value]) => updateSpacing(value)}
                  min={0.5}
                  max={2}
                  step={0.1}
                />
                <p className="text-xs text-muted-foreground">
                  Adjust the overall spacing scale of the interface
                </p>
              </div>
            </TabsContent>

            <TabsContent value="radius" className="space-y-4">
              <div className="space-y-2">
                <Label>Border Radius: {customTheme.radius.value.toFixed(2)}rem</Label>
                <Slider
                  value={[customTheme.radius.value]}
                  onValueChange={([value]) => updateRadius(value)}
                  min={0}
                  max={1.5}
                  step={0.1}
                />
                <p className="text-xs text-muted-foreground">
                  Adjust the roundness of UI elements
                </p>
              </div>
            </TabsContent>
          </Tabs>

          {/* Action Buttons */}
          <div className="mt-6 flex flex-wrap gap-2">
            <Button onClick={applyTheme} className="flex-1">
              Apply Theme
            </Button>
            <Button variant="outline" onClick={resetTheme}>
              <RotateCcw className="mr-2 h-4 w-4" />
              Reset
            </Button>
            <Button variant="outline" onClick={exportTheme}>
              <Download className="mr-2 h-4 w-4" />
              Export
            </Button>
            <Button variant="outline" onClick={importTheme}>
              <Upload className="mr-2 h-4 w-4" />
              Import
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
