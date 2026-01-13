import { toast as sonnerToast } from 'sonner'

interface ToastOptions {
  title?: string
  description?: string
  variant?: 'default' | 'destructive'
}

export function useToast() {
  return {
    toast: (opts: ToastOptions) => {
      const message = opts.title || opts.description || ''

      if (opts.variant === 'destructive') {
        sonnerToast.error(message, {
          description: opts.title && opts.description ? opts.description : undefined,
        })
      } else {
        sonnerToast.success(message, {
          description: opts.title && opts.description ? opts.description : undefined,
        })
      }
    }
  }
}
