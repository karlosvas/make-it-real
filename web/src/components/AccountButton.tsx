import { useEffect, useRef, useState } from 'react';
import { loginUrl, logout, syncAccount, type MirUser } from '../lib/account';

function GithubIcon({ className = 'size-4' }: { className?: string }) {
  return (
    <svg viewBox="0 0 16 16" className={className} fill="currentColor" aria-hidden="true">
      <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0 0 16 8c0-4.42-3.58-8-8-8Z" />
    </svg>
  );
}

/** Botón de cuenta: "Entrar con GitHub" o tu avatar con menú para salir. */
export default function AccountButton({ compact = false }: { compact?: boolean }) {
  const [state, setState] = useState<{ loading: boolean; configured: boolean; user: MirUser | null }>({ loading: true, configured: true, user: null });
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    syncAccount().then((r) => setState({ loading: false, ...r }));
  }, []);

  useEffect(() => {
    if (!open) return;
    const close = (e: MouseEvent) => {
      if (!ref.current?.contains(e.target as Node)) setOpen(false);
    };
    window.addEventListener('mousedown', close);
    return () => window.removeEventListener('mousedown', close);
  }, [open]);

  if (state.loading) return <div className={compact ? 'h-7 w-7' : 'h-9 w-36'} />;

  if (!state.user) {
    return (
      <a
        href={state.configured ? loginUrl() : '/auth/github/login'}
        onClick={(e) => {
          e.currentTarget.href = state.configured ? loginUrl() : '/auth/github/login';
        }}
        className={`flex items-center gap-2 rounded-lg border border-ink-600 font-medium text-ink-100 transition hover:border-ink-400 hover:bg-ink-800 ${compact ? 'px-2.5 py-1 text-xs' : 'px-3.5 py-2 text-sm'}`}
        title="Guarda tu progreso en tu cuenta de GitHub"
      >
        <GithubIcon className={compact ? 'size-3.5' : 'size-4'} />
        Entrar con GitHub
      </a>
    );
  }

  const u = state.user;
  return (
    <div ref={ref} className="relative">
      <button
        onClick={() => setOpen((o) => !o)}
        className={`flex items-center gap-2 rounded-full border border-ink-700 bg-ink-850 pr-3 text-ink-100 transition hover:border-ink-500 ${compact ? 'py-0.5 pl-0.5 text-xs' : 'py-1 pl-1 text-sm'}`}
      >
        {u.avatar_url ? (
          <img src={`${u.avatar_url}${u.avatar_url.includes('?') ? '&' : '?'}s=64`} alt="" className={`rounded-full ${compact ? 'size-6' : 'size-7'}`} />
        ) : (
          <GithubIcon className="size-6" />
        )}
        <span className="font-medium">{u.login}</span>
      </button>
      {open && (
        <div className="absolute right-0 z-50 mt-2 w-60 overflow-hidden rounded-xl border border-ink-700 bg-ink-850 shadow-2xl">
          <div className="border-b border-ink-700 px-4 py-3">
            <div className="text-sm font-semibold text-ink-100">{u.name ?? u.login}</div>
            <div className="mt-0.5 flex items-center gap-1.5 text-xs text-emerald-300">
              <span className="size-1.5 rounded-full bg-emerald-400" /> Progreso guardado en tu cuenta
            </div>
          </div>
          <button onClick={() => logout()} className="w-full px-4 py-2.5 text-left text-sm text-ink-300 transition hover:bg-ink-800 hover:text-ink-100">
            Cerrar sesión
          </button>
        </div>
      )}
    </div>
  );
}
