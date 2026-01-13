/**
 * Animation Utilities
 *
 * Reusable animation variants for Framer Motion
 */

import type { Variants } from 'framer-motion';

/**
 * Fade in animation
 */
export const fadeIn: Variants = {
  initial: { opacity: 0 },
  animate: { opacity: 1 },
  exit: { opacity: 0 },
};

/**
 * Slide in from left
 */
export const slideInLeft: Variants = {
  initial: { x: -20, opacity: 0 },
  animate: { x: 0, opacity: 1 },
  exit: { x: 20, opacity: 0 },
};

/**
 * Slide in from right
 */
export const slideInRight: Variants = {
  initial: { x: 20, opacity: 0 },
  animate: { x: 0, opacity: 1 },
  exit: { x: -20, opacity: 0 },
};

/**
 * Slide in from top
 */
export const slideInTop: Variants = {
  initial: { y: -20, opacity: 0 },
  animate: { y: 0, opacity: 1 },
  exit: { y: 20, opacity: 0 },
};

/**
 * Slide in from bottom
 */
export const slideInBottom: Variants = {
  initial: { y: 20, opacity: 0 },
  animate: { y: 0, opacity: 1 },
  exit: { y: -20, opacity: 0 },
};

/**
 * Scale in animation
 */
export const scaleIn: Variants = {
  initial: { scale: 0.95, opacity: 0 },
  animate: { scale: 1, opacity: 1 },
  exit: { scale: 0.95, opacity: 0 },
};

/**
 * Pop in animation
 */
export const popIn: Variants = {
  initial: { scale: 0.8, opacity: 0 },
  animate: {
    scale: 1,
    opacity: 1,
    transition: {
      type: 'spring',
      stiffness: 300,
      damping: 20,
    },
  },
  exit: { scale: 0.8, opacity: 0 },
};

/**
 * Collapse animation
 */
export const collapse: Variants = {
  initial: { height: 0, opacity: 0 },
  animate: { height: 'auto', opacity: 1 },
  exit: { height: 0, opacity: 0 },
};

/**
 * Stagger children animation
 */
export const staggerContainer: Variants = {
  animate: {
    transition: {
      staggerChildren: 0.1,
    },
  },
};

/**
 * List item animation
 */
export const listItem: Variants = {
  initial: { x: -10, opacity: 0 },
  animate: { x: 0, opacity: 1 },
  exit: { x: 10, opacity: 0 },
};

/**
 * Transition presets
 */
export const transitions = {
  fast: { duration: 0.15 },
  default: { duration: 0.2 },
  slow: { duration: 0.3 },
  spring: {
    type: 'spring' as const,
    stiffness: 300,
    damping: 30,
  },
  bouncy: {
    type: 'spring' as const,
    stiffness: 400,
    damping: 20,
  },
};

/**
 * Message animation with streaming effect
 */
export const messageAnimation: Variants = {
  initial: {
    opacity: 0,
    y: 10,
    scale: 0.98,
  },
  animate: {
    opacity: 1,
    y: 0,
    scale: 1,
    transition: {
      duration: 0.2,
      ease: 'easeOut',
    },
  },
  exit: {
    opacity: 0,
    y: -10,
    scale: 0.98,
    transition: {
      duration: 0.15,
    },
  },
};

/**
 * Typing indicator animation
 */
export const typingIndicator: Variants = {
  animate: {
    opacity: [0.4, 1, 0.4],
    transition: {
      duration: 1.5,
      repeat: Infinity,
      ease: 'easeInOut',
    },
  },
};

/**
 * Shimmer loading animation
 */
export const shimmer: Variants = {
  animate: {
    backgroundPosition: ['200% 0', '-200% 0'],
    transition: {
      duration: 2,
      repeat: Infinity,
      ease: 'linear',
    },
  },
};

/**
 * Pulse animation
 */
export const pulse: Variants = {
  animate: {
    scale: [1, 1.05, 1],
    transition: {
      duration: 2,
      repeat: Infinity,
      ease: 'easeInOut',
    },
  },
};

/**
 * Notification animation
 */
export const notification: Variants = {
  initial: {
    x: 400,
    opacity: 0,
    scale: 0.9,
  },
  animate: {
    x: 0,
    opacity: 1,
    scale: 1,
    transition: {
      type: 'spring',
      stiffness: 300,
      damping: 25,
    },
  },
  exit: {
    x: 400,
    opacity: 0,
    scale: 0.9,
    transition: {
      duration: 0.2,
    },
  },
};

/**
 * Modal backdrop animation
 */
export const backdropAnimation: Variants = {
  initial: { opacity: 0 },
  animate: { opacity: 1 },
  exit: { opacity: 0 },
};

/**
 * Modal content animation
 */
export const modalAnimation: Variants = {
  initial: {
    scale: 0.95,
    opacity: 0,
    y: 20,
  },
  animate: {
    scale: 1,
    opacity: 1,
    y: 0,
    transition: {
      type: 'spring',
      stiffness: 300,
      damping: 30,
    },
  },
  exit: {
    scale: 0.95,
    opacity: 0,
    y: 20,
    transition: {
      duration: 0.2,
    },
  },
};
