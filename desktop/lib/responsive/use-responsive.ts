/**
 * Responsive Design Utilities
 *
 * Hooks for responsive layouts and media queries
 */

'use client';

import { useState, useEffect } from 'react';

export interface Breakpoints {
  sm: boolean;
  md: boolean;
  lg: boolean;
  xl: boolean;
  '2xl': boolean;
}

export interface ResponsiveLayout {
  isMobile: boolean;
  isTablet: boolean;
  isDesktop: boolean;
  isWidescreen: boolean;
  breakpoint: 'mobile' | 'tablet' | 'desktop' | 'widescreen';
}

/**
 * Hook to detect responsive breakpoints
 */
export function useBreakpoints(): Breakpoints {
  const [breakpoints, setBreakpoints] = useState<Breakpoints>({
    sm: false,
    md: false,
    lg: false,
    xl: false,
    '2xl': false,
  });

  useEffect(() => {
    const queries = {
      sm: window.matchMedia('(min-width: 640px)'),
      md: window.matchMedia('(min-width: 768px)'),
      lg: window.matchMedia('(min-width: 1024px)'),
      xl: window.matchMedia('(min-width: 1280px)'),
      '2xl': window.matchMedia('(min-width: 1536px)'),
    };

    const updateBreakpoints = () => {
      setBreakpoints({
        sm: queries.sm.matches,
        md: queries.md.matches,
        lg: queries.lg.matches,
        xl: queries.xl.matches,
        '2xl': queries['2xl'].matches,
      });
    };

    // Initial check
    updateBreakpoints();

    // Listen for changes
    Object.values(queries).forEach((query) => {
      query.addEventListener('change', updateBreakpoints);
    });

    return () => {
      Object.values(queries).forEach((query) => {
        query.removeEventListener('change', updateBreakpoints);
      });
    };
  }, []);

  return breakpoints;
}

/**
 * Hook to detect responsive layout
 */
export function useResponsiveLayout(): ResponsiveLayout {
  const breakpoints = useBreakpoints();

  const isMobile = !breakpoints.md;
  const isTablet = breakpoints.md && !breakpoints.lg;
  const isDesktop = breakpoints.lg && !breakpoints.xl;
  const isWidescreen = breakpoints.xl;

  let breakpoint: ResponsiveLayout['breakpoint'] = 'mobile';
  if (isWidescreen) breakpoint = 'widescreen';
  else if (isDesktop) breakpoint = 'desktop';
  else if (isTablet) breakpoint = 'tablet';

  return {
    isMobile,
    isTablet,
    isDesktop,
    isWidescreen,
    breakpoint,
  };
}

/**
 * Hook for custom media query
 */
export function useMediaQuery(query: string): boolean {
  const [matches, setMatches] = useState(false);

  useEffect(() => {
    const mediaQuery = window.matchMedia(query);

    const updateMatches = () => {
      setMatches(mediaQuery.matches);
    };

    // Initial check
    updateMatches();

    // Listen for changes
    mediaQuery.addEventListener('change', updateMatches);

    return () => {
      mediaQuery.removeEventListener('change', updateMatches);
    };
  }, [query]);

  return matches;
}

/**
 * Hook to detect screen orientation
 */
export function useOrientation(): 'portrait' | 'landscape' {
  const [orientation, setOrientation] = useState<'portrait' | 'landscape'>('portrait');

  useEffect(() => {
    const updateOrientation = () => {
      setOrientation(
        window.innerHeight > window.innerWidth ? 'portrait' : 'landscape'
      );
    };

    updateOrientation();

    window.addEventListener('resize', updateOrientation);
    window.addEventListener('orientationchange', updateOrientation);

    return () => {
      window.removeEventListener('resize', updateOrientation);
      window.removeEventListener('orientationchange', updateOrientation);
    };
  }, []);

  return orientation;
}

/**
 * Hook to get window size
 */
export function useWindowSize() {
  const [size, setSize] = useState({
    width: 0,
    height: 0,
  });

  useEffect(() => {
    const updateSize = () => {
      setSize({
        width: window.innerWidth,
        height: window.innerHeight,
      });
    };

    updateSize();

    window.addEventListener('resize', updateSize);

    return () => {
      window.removeEventListener('resize', updateSize);
    };
  }, []);

  return size;
}

/**
 * Hook to detect if device is touch-enabled
 */
export function useIsTouchDevice(): boolean {
  const [isTouch] = useState(() => {
    // Check if running in browser environment
    if (typeof window === 'undefined') {
      return false;
    }

    return 'ontouchstart' in window || navigator.maxTouchPoints > 0;
  });

  return isTouch;
}
