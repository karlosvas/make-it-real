import { useEffect, useRef, useState } from 'react';
import type { Trace } from '../../lib/types';
import { Viewer3D } from './Viewer3D';

interface Props {
  trace: Trace | null;
  /** Cambia para reiniciar y reproducir desde el principio. */
  playToken: number;
  autoplay: boolean;
  placeholder?: string;
  /** Instante inicial (para enlaces tipo ?t=2.5). */
  startAt?: number;
}

const SPEEDS = [0.5, 1, 2];

export default function ViewerPanel({ trace, playToken, autoplay, placeholder, startAt = 0 }: Props) {
  const host = useRef<HTMLDivElement>(null);
  const viewer = useRef<Viewer3D | null>(null);
  const [time, setTime] = useState(0);
  const [playing, setPlaying] = useState(false);
  const [speed, setSpeed] = useState(1);
  const timeRef = useRef(0);
  const duration = trace?.duration ?? 0;

  useEffect(() => {
    viewer.current = new Viewer3D(host.current!);
    return () => viewer.current?.dispose();
  }, []);

  useEffect(() => {
    viewer.current?.setTrace(trace);
    const t0 = trace ? Math.min(startAt, trace.duration) : 0;
    timeRef.current = t0;
    setTime(t0);
    viewer.current?.setTime(t0);
    setPlaying(autoplay && !!trace);
  }, [trace, playToken, autoplay, startAt]);

  useEffect(() => {
    if (!playing) return;
    let raf = 0;
    let last = performance.now();
    const step = (now: number) => {
      const dt = Math.min(0.1, (now - last) / 1000);
      last = now;
      let t = timeRef.current + dt * speed;
      if (t >= duration) {
        t = duration;
        setPlaying(false);
      }
      timeRef.current = t;
      viewer.current?.setTime(t);
      setTime(t);
      if (t < duration) raf = requestAnimationFrame(step);
    };
    raf = requestAnimationFrame(step);
    return () => cancelAnimationFrame(raf);
  }, [playing, speed, duration]);

  const seek = (t: number) => {
    timeRef.current = t;
    setTime(t);
    viewer.current?.setTime(t);
  };

  const toggle = () => {
    if (!trace) return;
    if (!playing && timeRef.current >= duration - 1e-3) seek(0);
    setPlaying((p) => !p);
  };

  return (
    <div className="relative flex h-full flex-col bg-ink-950">
      <div ref={host} className="relative min-h-0 flex-1 overflow-hidden" />
      {!trace && (
        <div className="pointer-events-none absolute inset-0 flex items-center justify-center">
          <div className="rounded-xl border border-ink-700 bg-ink-900/80 px-5 py-3 text-center text-sm text-ink-300 backdrop-blur">
            {placeholder ?? 'Preparando el mundo…'}
          </div>
        </div>
      )}
      <div className="flex items-center gap-3 border-t border-ink-800 bg-ink-900 px-3 py-2">
        <button
          onClick={toggle}
          disabled={!trace}
          className="flex size-8 items-center justify-center rounded-lg bg-ink-700 text-ink-100 transition hover:bg-ink-600 disabled:opacity-40"
          title={playing ? 'Pausa' : 'Reproducir'}
        >
          {playing ? (
            <svg viewBox="0 0 24 24" className="size-4" fill="currentColor"><rect x="6" y="5" width="4" height="14" rx="1" /><rect x="14" y="5" width="4" height="14" rx="1" /></svg>
          ) : (
            <svg viewBox="0 0 24 24" className="size-4" fill="currentColor"><path d="M8 5.5v13a1 1 0 0 0 1.5.86l11-6.5a1 1 0 0 0 0-1.72l-11-6.5A1 1 0 0 0 8 5.5Z" /></svg>
          )}
        </button>
        <button
          onClick={() => {
            seek(0);
            setPlaying(!!trace);
          }}
          disabled={!trace}
          className="flex size-8 items-center justify-center rounded-lg text-ink-300 transition hover:bg-ink-700 hover:text-ink-100 disabled:opacity-40"
          title="Repetir desde el principio"
        >
          <svg viewBox="0 0 24 24" className="size-4" fill="none" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" strokeLinejoin="round"><path d="M3 12a9 9 0 1 0 3-6.7" /><path d="M3 4v5h5" /></svg>
        </button>
        <input
          type="range"
          min={0}
          max={Math.max(0.01, duration)}
          step={0.01}
          value={time}
          disabled={!trace}
          onChange={(e) => {
            setPlaying(false);
            seek(Number(e.target.value));
          }}
          className="h-1 flex-1 cursor-pointer accent-[#ff6b1a]"
        />
        <span className="w-24 text-right font-mono text-xs tabular-nums text-ink-300">
          {time.toFixed(2)} / {duration.toFixed(2)} s
        </span>
        <button
          onClick={() => setSpeed((s) => SPEEDS[(SPEEDS.indexOf(s) + 1) % SPEEDS.length])}
          className="w-11 rounded-md border border-ink-700 py-1 font-mono text-xs text-ink-200 transition hover:border-ink-500"
          title="Velocidad"
        >
          {speed}x
        </button>
        <button
          onClick={() => viewer.current?.fit()}
          className="rounded-md border border-ink-700 px-2 py-1 text-xs text-ink-200 transition hover:border-ink-500"
          title="Encuadrar cámara"
        >
          Encuadrar
        </button>
      </div>
    </div>
  );
}
