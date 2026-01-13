import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  output: 'export',
  trailingSlash: true,
  images: {
    unoptimized: true,
  },
  // Transpile problematic packages for better compatibility
  transpilePackages: ['react-markdown', 'remark-gfm', 'rehype-highlight'],
  // Empty turbopack config to silence warning (Turbopack handles client-side exclusions automatically)
  turbopack: {},
  webpack: (config, { isServer }) => {
    if (!isServer) {
      // Ignore node:* and Node.js modules for client-side builds (e2b, sandpack dependencies)
      config.resolve.fallback = {
        ...config.resolve.fallback,
        fs: false,
        'node:fs': false,
        path: false,
        'node:path': false,
        crypto: false,
        'node:crypto': false,
        os: false,
        'node:os': false,
        stream: false,
        'node:stream': false,
        http: false,
        'node:http': false,
        https: false,
        'node:https': false,
        zlib: false,
        'node:zlib': false,
      };
    }
    return config;
  },
};

export default nextConfig;
