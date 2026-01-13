/**
 * React Component Renderer using Sandpack
 *
 * Renders interactive React components with live editing capabilities
 */

'use client';

import { Sandpack } from '@codesandbox/sandpack-react';
import { githubLight, sandpackDark } from '@codesandbox/sandpack-themes';
import { useTheme } from 'next-themes';
import { Artifact } from '../../lib/artifacts/types';

interface ReactRendererProps {
  artifact: Artifact;
  editable?: boolean;
  onCodeChange?: (code: string) => void;
}

export function ReactRenderer({ artifact, editable = false, onCodeChange }: ReactRendererProps) {
  const { theme } = useTheme();

  // Prepare files for Sandpack
  const files = {
    '/App.tsx': artifact.content,
    '/index.tsx': `import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';

const root = createRoot(document.getElementById('root')!);
root.render(<App />);
`,
    '/styles.css': `body {
  margin: 0;
  padding: 20px;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
}
`,
  };

  // Add dependencies if specified
  const customSetup = artifact.metadata.dependencies
    ? {
        dependencies: artifact.metadata.dependencies.reduce(
          (acc, dep) => {
            const [name, version] = dep.split('@');
            acc[name] = version || 'latest';
            return acc;
          },
          {} as Record<string, string>
        ),
      }
    : {};

  return (
    <div className="react-renderer border rounded-lg overflow-hidden">
      <Sandpack
        template="react-ts"
        files={files}
        theme={theme === 'dark' ? sandpackDark : githubLight}
        customSetup={customSetup}
        options={{
          showNavigator: true,
          showTabs: true,
          showLineNumbers: true,
          editorHeight: artifact.metadata.height || 500,
          showConsole: true,
          showConsoleButton: true,
          readOnly: !editable,
          autoReload: true,
          recompileMode: 'delayed',
          recompileDelay: 300,
        }}
        {...(onCodeChange && {
          onCodeUpdate: (newCode) => {
            const appCode = newCode['/App.tsx'];
            if (appCode) {
              onCodeChange(appCode);
            }
          },
        })}
      />
    </div>
  );
}
