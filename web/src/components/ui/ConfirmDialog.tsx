import { useCallback, useEffect, useRef, useState, type ReactNode } from 'react';

export type DialogChoice = 'confirm' | 'cancel' | 'extra';

export interface DialogOptions {
  title: string;
  message: ReactNode;
  confirmLabel: string;
  cancelLabel?: string;
  /** Botón adicional opcional (p. ej. "Ver una pista"). */
  extraLabel?: string;
  tone?: 'default' | 'danger';
  icon?: ReactNode;
}

interface Pending extends DialogOptions {
  resolve: (c: DialogChoice) => void;
}

/**
 * Diálogo de confirmación propio (sustituye a window.confirm).
 * Uso: const [dialog, ask] = useConfirm();  …  const choice = await ask({...});  …  {dialog}
 */
export function useConfirm(): [ReactNode, (o: DialogOptions) => Promise<DialogChoice>] {
  const [pending, setPending] = useState<Pending | null>(null);

  const ask = useCallback((o: DialogOptions) => new Promise<DialogChoice>((resolve) => setPending({ ...o, resolve })), []);

  const close = useCallback(
    (c: DialogChoice) => {
      pending?.resolve(c);
      setPending(null);
    },
    [pending],
  );

  return [pending ? <Dialog {...pending} onClose={close} /> : null, ask];
}

function Dialog({ title, message, confirmLabel, cancelLabel = 'Cancelar', extraLabel, tone = 'default', icon, onClose }: DialogOptions & { onClose: (c: DialogChoice) => void }) {
  const confirmRef = useRef<HTMLButtonElement>(null);
  const [shown, setShown] = useState(false);

  useEffect(() => {
    const raf = requestAnimationFrame(() => setShown(true));
    confirmRef.current?.focus();
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose('cancel');
    };
    window.addEventListener('keydown', onKey);
    return () => {
      cancelAnimationFrame(raf);
      window.removeEventListener('keydown', onKey);
    };
  }, [onClose]);

  const danger = tone === 'danger';
  return (
    <div className="fixed inset-0 z-[100] flex items-center justify-center p-4" role="dialog" aria-modal="true" aria-labelledby="mir-dialog-title">
      <div
        className={`absolute inset-0 bg-ink-950/70 backdrop-blur-sm transition-opacity duration-200 ${shown ? 'opacity-100' : 'opacity-0'}`}
        onClick={() => onClose('cancel')}
      />
      <div
        className={`relative w-full max-w-lg overflow-hidden rounded-2xl border border-ink-600 bg-ink-850 shadow-[0_30px_80px_-20px_rgba(0,0,0,.8)] transition duration-200 ${shown ? 'translate-y-0 scale-100 opacity-100' : 'translate-y-2 scale-[0.98] opacity-0'}`}
      >
        <div className={`h-1 w-full ${danger ? 'bg-rose-500' : 'bg-mir-500'}`} />
        <div className="flex gap-4 p-6">
          <div className={`flex size-10 shrink-0 items-center justify-center rounded-xl ${danger ? 'bg-rose-500/15 text-rose-300' : 'bg-mir-500/15 text-mir-400'}`}>
            {icon ?? (
              <svg viewBox="0 0 24 24" className="size-5" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                {danger ? (
                  <>
                    <path d="M3 12a9 9 0 1 0 3-6.7" />
                    <path d="M3 4v5h5" />
                  </>
                ) : (
                  <>
                    <circle cx="12" cy="12" r="9" />
                    <path d="M12 8v5" />
                    <path d="M12 16.5v.01" />
                  </>
                )}
              </svg>
            )}
          </div>
          <div className="min-w-0">
            <h2 id="mir-dialog-title" className="font-display text-lg font-semibold text-ink-100">
              {title}
            </h2>
            <div className="mt-1.5 text-sm leading-relaxed text-ink-300">{message}</div>
          </div>
        </div>
        <div className="flex flex-wrap items-center justify-end gap-2 border-t border-ink-700 bg-ink-900/60 px-6 py-4">
          {extraLabel && (
            <button onClick={() => onClose('extra')} className="mr-auto rounded-lg px-3 py-2 text-sm font-medium text-mir-400 transition hover:bg-ink-800 hover:text-mir-300">
              {extraLabel}
            </button>
          )}
          <button onClick={() => onClose('cancel')} className="rounded-lg px-4 py-2 text-sm font-medium text-ink-300 transition hover:bg-ink-800 hover:text-ink-100">
            {cancelLabel}
          </button>
          <button
            ref={confirmRef}
            onClick={() => onClose('confirm')}
            className={`rounded-lg px-4 py-2 text-sm font-semibold transition focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-ink-850 ${danger ? 'bg-rose-500 text-white hover:bg-rose-400 focus-visible:ring-rose-400' : 'bg-mir-500 text-ink-950 hover:bg-mir-400 focus-visible:ring-mir-400'}`}
          >
            {confirmLabel}
          </button>
        </div>
      </div>
    </div>
  );
}
