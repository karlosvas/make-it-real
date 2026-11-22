import { useCallback, useRef, useState, type ReactNode } from 'react';

interface Props {
  direction: 'row' | 'col';
  /** Tamaño inicial del primer panel, en % */
  initial: number;
  min?: number;
  max?: number;
  first: ReactNode;
  second: ReactNode;
  storageKey?: string;
}

/** Dos paneles con un separador arrastrable. */
export default function Split({ direction, initial, min = 15, max = 85, first, second, storageKey }: Props) {
  const [size, setSize] = useState(() => {
    if (storageKey) {
      try {
        const v = Number(localStorage.getItem(storageKey));
        if (v >= min && v <= max) return v;
      } catch {}
    }
    return initial;
  });
  const ref = useRef<HTMLDivElement>(null);
  const row = direction === 'row';

  const onDown = useCallback(
    (e: React.PointerEvent) => {
      e.preventDefault();
      const el = ref.current;
      if (!el) return;
      const rect = el.getBoundingClientRect();
      const move = (ev: PointerEvent) => {
        const pct = row ? ((ev.clientX - rect.left) / rect.width) * 100 : ((ev.clientY - rect.top) / rect.height) * 100;
        const clamped = Math.min(max, Math.max(min, pct));
        setSize(clamped);
        if (storageKey) {
          try {
            localStorage.setItem(storageKey, String(clamped));
          } catch {}
        }
      };
      const up = () => {
        window.removeEventListener('pointermove', move);
        window.removeEventListener('pointerup', up);
        document.body.style.cursor = '';
        document.body.style.userSelect = '';
      };
      window.addEventListener('pointermove', move);
      window.addEventListener('pointerup', up);
      document.body.style.cursor = row ? 'col-resize' : 'row-resize';
      document.body.style.userSelect = 'none';
    },
    [row, min, max, storageKey],
  );

  return (
    <div ref={ref} className={`flex h-full w-full min-h-0 min-w-0 ${row ? 'flex-row' : 'flex-col'}`}>
      <div className="min-h-0 min-w-0 overflow-hidden" style={{ flexBasis: `${size}%`, flexGrow: 0, flexShrink: 0 }}>
        {first}
      </div>
      <div
        onPointerDown={onDown}
        className={`group relative shrink-0 ${row ? 'w-1.5 cursor-col-resize' : 'h-1.5 cursor-row-resize'} bg-ink-900`}
      >
        <div className={`absolute bg-ink-700 transition-colors group-hover:bg-mir-500/70 ${row ? 'inset-y-0 left-1/2 w-px -translate-x-1/2' : 'inset-x-0 top-1/2 h-px -translate-y-1/2'}`} />
      </div>
      <div className="min-h-0 min-w-0 flex-1 overflow-hidden">{second}</div>
    </div>
  );
}
