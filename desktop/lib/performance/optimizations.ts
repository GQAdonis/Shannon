/**
 * Performance Optimization Utilities
 *
 * Hooks and utilities for optimizing Shannon Desktop performance
 */

'use client';

import { useCallback, useEffect, useRef, useState } from 'react';
import { useDebouncedCallback } from 'use-debounce';

/**
 * Debounced search hook
 */
export function useDebouncedSearch<T>(
  searchFn: (query: string) => Promise<T[]>,
  delay: number = 300
) {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<T[]>([]);
  const [isSearching, setIsSearching] = useState(false);

  const debouncedSearch = useDebouncedCallback(
    async (searchQuery: string) => {
      if (!searchQuery.trim()) {
        setResults([]);
        setIsSearching(false);
        return;
      }

      setIsSearching(true);
      try {
        const searchResults = await searchFn(searchQuery);
        setResults(searchResults);
      } catch (error) {
        console.error('Search error:', error);
        setResults([]);
      } finally {
        setIsSearching(false);
      }
    },
    delay
  );

  const search = useCallback(
    (newQuery: string) => {
      setQuery(newQuery);
      debouncedSearch(newQuery);
    },
    [debouncedSearch]
  );

  return {
    query,
    results,
    isSearching,
    search,
    setQuery,
  };
}

/**
 * Intersection observer hook for lazy loading
 */
export function useIntersectionObserver(
  callback: () => void,
  options?: IntersectionObserverInit
) {
  const targetRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const target = targetRef.current;
    if (!target) return;

    const observer = new IntersectionObserver(([entry]) => {
      if (entry.isIntersecting) {
        callback();
      }
    }, options);

    observer.observe(target);

    return () => {
      observer.disconnect();
    };
  }, [callback, options]);

  return targetRef;
}

/**
 * Measure render performance
 */
export function useRenderPerformance(componentName: string) {
  const renderCount = useRef(0);
  const startTime = useRef<number>(0);

  useEffect(() => {
    // Lazy initialization: set startTime on first effect run
    if (startTime.current === 0) {
      startTime.current = performance.now();
    }

    renderCount.current += 1;
    const endTime = performance.now();
    const renderTime = endTime - startTime.current;

    if (process.env.NODE_ENV === 'development') {
      console.log(
        `[Performance] ${componentName} rendered ${renderCount.current} times, took ${renderTime.toFixed(2)}ms`
      );
    }

    startTime.current = endTime;
  });

  // Note: We don't return renderCount as it's only for internal logging
  // Returning ref.current during render violates React purity rules
  return {};
}

/**
 * Optimize large list rendering
 */
export function useVirtualizedList<T>(
  items: T[],
  itemHeight: number,
  containerHeight: number
) {
  const [scrollTop, setScrollTop] = useState(0);

  const startIndex = Math.floor(scrollTop / itemHeight);
  const endIndex = Math.min(
    items.length - 1,
    Math.ceil((scrollTop + containerHeight) / itemHeight)
  );

  const visibleItems = items.slice(startIndex, endIndex + 1);
  const offsetY = startIndex * itemHeight;

  const handleScroll = useCallback((e: React.UIEvent<HTMLDivElement>) => {
    setScrollTop(e.currentTarget.scrollTop);
  }, []);

  return {
    visibleItems,
    offsetY,
    totalHeight: items.length * itemHeight,
    handleScroll,
    startIndex,
    endIndex,
  };
}

/**
 * Throttle callback
 */
export function useThrottle<T extends (...args: unknown[]) => unknown>(
  callback: T,
  delay: number
): T {
  const lastRun = useRef(0);

  return useCallback(
    (...args: Parameters<T>) => {
      const now = Date.now();
      // Lazy initialization on first call
      if (lastRun.current === 0) {
        lastRun.current = now;
      }
      if (now - lastRun.current >= delay) {
        lastRun.current = now;
        return callback(...args);
      }
    },
    [callback, delay]
  ) as T;
}

/**
 * Detect if user prefers reduced motion
 */
export function usePrefersReducedMotion() {
  const [prefersReducedMotion, setPrefersReducedMotion] = useState(() => {
    // Lazy initialization to avoid accessing window during SSR
    if (typeof window === 'undefined') return false;
    return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  });

  useEffect(() => {
    const mediaQuery = window.matchMedia('(prefers-reduced-motion: reduce)');

    const handleChange = (e: MediaQueryListEvent) => {
      setPrefersReducedMotion(e.matches);
    };

    mediaQuery.addEventListener('change', handleChange);
    return () => mediaQuery.removeEventListener('change', handleChange);
  }, []);

  return prefersReducedMotion;
}

/**
 * Measure component size
 */
export function useComponentSize() {
  const ref = useRef<HTMLDivElement>(null);
  const [size, setSize] = useState({ width: 0, height: 0 });

  useEffect(() => {
    if (!ref.current) return;

    const observer = new ResizeObserver((entries) => {
      const entry = entries[0];
      if (entry) {
        setSize({
          width: entry.contentRect.width,
          height: entry.contentRect.height,
        });
      }
    });

    observer.observe(ref.current);

    return () => {
      observer.disconnect();
    };
  }, []);

  return { ref, ...size };
}

/**
 * Lazy load component
 * Once shouldLoad becomes true, the component stays loaded
 */
export function useLazyLoad(shouldLoad: boolean) {
  // Simply return shouldLoad - if the consumer wants "sticky" behavior,
  // they should manage that in their own state
  return shouldLoad;
}

/**
 * Monitor FPS
 */
export function useFPS() {
  const [fps, setFPS] = useState(60);
  const frameTimesRef = useRef<number[]>([]);
  const lastTimeRef = useRef<number>(0);
  const rafIdRef = useRef<number | undefined>(undefined);

  useEffect(() => {
    const measureFPS = () => {
      const now = performance.now();
      // Lazy initialization on first frame
      if (lastTimeRef.current === 0) {
        lastTimeRef.current = now;
        rafIdRef.current = requestAnimationFrame(measureFPS);
        return;
      }
      const delta = now - lastTimeRef.current;
      lastTimeRef.current = now;

      frameTimesRef.current.push(delta);
      if (frameTimesRef.current.length > 60) {
        frameTimesRef.current.shift();
      }

      const avgDelta =
        frameTimesRef.current.reduce((a, b) => a + b, 0) /
        frameTimesRef.current.length;
      const currentFPS = Math.round(1000 / avgDelta);
      setFPS(currentFPS);

      rafIdRef.current = requestAnimationFrame(measureFPS);
    };

    rafIdRef.current = requestAnimationFrame(measureFPS);

    return () => {
      if (rafIdRef.current) {
        cancelAnimationFrame(rafIdRef.current);
      }
    };
  }, []);

  return fps;
}
