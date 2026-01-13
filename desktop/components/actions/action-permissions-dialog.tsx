/**
 * Action Permissions Dialog
 *
 * Displays a dialog for approving browser and filesystem actions.
 * Supports one-time, session, and always-allow permissions.
 */

'use client';

import { useState } from 'react';
import {
  AlertDialog,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Checkbox } from '@/components/ui/checkbox';
import { Globe, FileText, AlertTriangle } from 'lucide-react';

export interface ActionPermissionRequest {
  type: 'browser' | 'filesystem';
  action: string;
  description: string;
  details?: Record<string, string | number | boolean | null>;
}

export interface ActionPermissionDialogProps {
  open: boolean;
  request: ActionPermissionRequest | null;
  onApprove: (scope: 'once' | 'session' | 'always') => void;
  onDeny: () => void;
}

export function ActionPermissionsDialog({
  open,
  request,
  onApprove,
  onDeny,
}: ActionPermissionDialogProps) {
  const [rememberChoice, setRememberChoice] = useState(false);
  const [alwaysAllow, setAlwaysAllow] = useState(false);

  if (!request) return null;

  const isHighRisk = request.action === 'delete' || request.action === 'write';

  const handleApprove = () => {
    if (alwaysAllow) {
      onApprove('always');
    } else if (rememberChoice) {
      onApprove('session');
    } else {
      onApprove('once');
    }

    // Reset state
    setRememberChoice(false);
    setAlwaysAllow(false);
  };

  const handleDeny = () => {
    onDeny();
    setRememberChoice(false);
    setAlwaysAllow(false);
  };

  const getIcon = () => {
    if (request.type === 'browser') {
      return <Globe className="h-5 w-5 text-blue-500" />;
    }
    return <FileText className="h-5 w-5 text-green-500" />;
  };

  const getCategoryBadge = () => {
    const color = request.type === 'browser' ? 'bg-blue-500' : 'bg-green-500';
    return (
      <Badge className={color}>
        {request.type === 'browser' ? 'Browser' : 'Filesystem'}
      </Badge>
    );
  };

  return (
    <AlertDialog open={open}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <div className="flex items-center gap-2 mb-2">
            {getIcon()}
            <AlertDialogTitle>Action Permission Required</AlertDialogTitle>
          </div>
          <AlertDialogDescription>
            The agent wants to perform the following action:
          </AlertDialogDescription>
        </AlertDialogHeader>

        <div className="space-y-4 py-4">
          {/* Action Category */}
          <div className="flex items-center gap-2">
            {getCategoryBadge()}
            <span className="text-sm font-medium">{request.action}</span>
          </div>

          {/* Action Description */}
          <div className="bg-muted p-3 rounded-md">
            <p className="text-sm">{request.description}</p>
          </div>

          {/* Action Details */}
          {request.details && (
            <div className="space-y-2">
              {Object.entries(request.details).map(([key, value]) => (
                <div key={key} className="flex items-start gap-2 text-sm">
                  <span className="font-medium text-muted-foreground">
                    {key}:
                  </span>
                  <span className="font-mono break-all">
                    {String(value)}
                  </span>
                </div>
              ))}
            </div>
          )}

          {/* High Risk Warning */}
          {isHighRisk && (
            <div className="flex items-start gap-2 p-3 bg-amber-50 dark:bg-amber-950 rounded-md border border-amber-200 dark:border-amber-800">
              <AlertTriangle className="h-4 w-4 text-amber-600 mt-0.5" />
              <div className="text-sm text-amber-800 dark:text-amber-200">
                <p className="font-medium">High-risk action</p>
                <p className="text-xs mt-1">
                  This action can modify or delete data. Review carefully before approving.
                </p>
              </div>
            </div>
          )}

          {/* Permission Options */}
          <div className="space-y-3 pt-2">
            <div className="flex items-center space-x-2">
              <Checkbox
                id="remember"
                checked={rememberChoice}
                onCheckedChange={(checked) => {
                  setRememberChoice(checked as boolean);
                  if (!checked) setAlwaysAllow(false);
                }}
              />
              <label
                htmlFor="remember"
                className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
              >
                Remember for this session
              </label>
            </div>

            {rememberChoice && !isHighRisk && (
              <div className="flex items-center space-x-2 ml-6">
                <Checkbox
                  id="always"
                  checked={alwaysAllow}
                  onCheckedChange={(checked) => setAlwaysAllow(checked as boolean)}
                />
                <label
                  htmlFor="always"
                  className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                >
                  Always allow (not recommended)
                </label>
              </div>
            )}
          </div>
        </div>

        <AlertDialogFooter>
          <Button variant="outline" onClick={handleDeny}>
            Deny
          </Button>
          <Button onClick={handleApprove}>
            Approve
          </Button>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
