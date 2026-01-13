/**
 * Browser Panel Component
 *
 * Provides a UI for browser automation with navigation and content display.
 */

'use client';

import { useState } from 'react';
import { browserService, PageSnapshot } from '@/lib/actions/browser-service';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Globe, Loader2 } from 'lucide-react';
import { toast } from 'sonner';

export function BrowserPanel() {
  const [url, setUrl] = useState('');
  const [snapshot, setSnapshot] = useState<PageSnapshot | null>(null);
  const [loading, setLoading] = useState(false);

  const handleNavigate = async () => {
    if (!url) {
      toast.error('Please enter a URL');
      return;
    }

    setLoading(true);
    try {
      const result = await browserService.navigate(url);
      setSnapshot(result);
      toast.success('Page loaded successfully');
    } catch (error) {
      toast.error(`Navigation failed: ${error}`);
      console.error('Navigation error:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleNavigate();
    }
  };

  return (
    <Card className="w-full">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Globe className="h-5 w-5" />
          Browser Automation
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Address Bar */}
        <div className="flex gap-2">
          <Input
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            onKeyPress={handleKeyPress}
            placeholder="Enter URL (e.g., https://example.com)"
            disabled={loading}
            className="flex-1"
          />
          <Button onClick={handleNavigate} disabled={loading || !url}>
            {loading ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                Loading
              </>
            ) : (
              'Navigate'
            )}
          </Button>
        </div>

        {/* Content Display */}
        {snapshot && (
          <Tabs defaultValue="screenshot" className="w-full">
            <TabsList className="grid w-full grid-cols-3">
              <TabsTrigger value="screenshot">Screenshot</TabsTrigger>
              <TabsTrigger value="content">HTML</TabsTrigger>
              <TabsTrigger value="info">Info</TabsTrigger>
            </TabsList>

            <TabsContent value="screenshot" className="space-y-4">
              <div className="border rounded-md overflow-hidden bg-muted">
                <img
                  src={browserService.screenshotToDataUrl(snapshot.screenshot)}
                  alt={snapshot.title}
                  className="w-full h-auto"
                />
              </div>
            </TabsContent>

            <TabsContent value="content" className="space-y-4">
              <div className="border rounded-md p-4 bg-muted max-h-[500px] overflow-auto">
                <pre className="text-xs whitespace-pre-wrap break-words">
                  {snapshot.content.slice(0, 5000)}
                  {snapshot.content.length > 5000 && '\n... (truncated)'}
                </pre>
              </div>
            </TabsContent>

            <TabsContent value="info" className="space-y-4">
              <div className="space-y-2">
                <div>
                  <span className="font-medium">Title:</span>{' '}
                  <span className="text-muted-foreground">{snapshot.title}</span>
                </div>
                <div>
                  <span className="font-medium">URL:</span>{' '}
                  <span className="text-muted-foreground break-all">
                    {snapshot.url}
                  </span>
                </div>
                <div>
                  <span className="font-medium">Content Length:</span>{' '}
                  <span className="text-muted-foreground">
                    {snapshot.content.length.toLocaleString()} characters
                  </span>
                </div>
                <div>
                  <span className="font-medium">Screenshot Size:</span>{' '}
                  <span className="text-muted-foreground">
                    {(snapshot.screenshot.length / 1024).toFixed(2)} KB
                  </span>
                </div>
              </div>
            </TabsContent>
          </Tabs>
        )}

        {!snapshot && !loading && (
          <div className="text-center py-12 text-muted-foreground">
            Enter a URL to navigate to a web page
          </div>
        )}
      </CardContent>
    </Card>
  );
}
