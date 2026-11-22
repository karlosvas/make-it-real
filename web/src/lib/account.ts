// Cuenta de GitHub y sincronización del progreso con mir-server.
// localStorage sigue siendo la caché local; el servidor es la copia buena cuando hay sesión.

export interface MirUser {
  login: string;
  name: string | null;
  avatar_url: string | null;
}

interface ServerExercise {
  solved: boolean;
  files: Record<string, string>;
  updated_at: number;
}

export const PROGRESS_EVENT = 'mir:progress';

let currentUser: MirUser | null = null;
let syncPromise: Promise<{ configured: boolean; user: MirUser | null }> | null = null;

export const getUser = () => currentUser;

const ls = {
  get: (k: string) => {
    try {
      return localStorage.getItem(k);
    } catch {
      return null;
    }
  },
  set: (k: string, v: string) => {
    try {
      localStorage.setItem(k, v);
    } catch {}
  },
  keys: (): string[] => {
    try {
      return Object.keys(localStorage);
    } catch {
      return [];
    }
  },
};

/** Lo que hay guardado en este navegador, agrupado por ejercicio. */
function localProgress() {
  const out: Record<string, { solved: boolean; files: Record<string, string> }> = {};
  for (const key of ls.keys()) {
    const draft = key.match(/^mir:draft:([^:]+):(.+)$/);
    const solved = key.match(/^mir:solved:([^:]+)$/);
    if (draft) {
      const [, id, file] = draft;
      out[id] ??= { solved: false, files: {} };
      out[id].files[file] = ls.get(key) ?? '';
    } else if (solved && ls.get(key) === '1') {
      out[solved[1]] ??= { solved: false, files: {} };
      out[solved[1]].solved = true;
    }
  }
  return out;
}

/**
 * Comprueba la sesión y, si hay usuario:
 *  1. la primera vez en este navegador, sube lo que hubiera en localStorage (sin pisar el servidor),
 *  2. descarga el progreso del servidor a localStorage.
 * Lanza el evento `mir:progress` al terminar.
 */
export function syncAccount() {
  syncPromise ??= (async () => {
    try {
      const me = await fetch('/api/me').then((r) => r.json());
      currentUser = me.user ?? null;
      if (currentUser) {
        const flag = `mir:imported:${currentUser.login}`;
        if (!ls.get(flag)) {
          await fetch('/api/progress/import', {
            method: 'POST',
            headers: { 'content-type': 'application/json' },
            body: JSON.stringify({ exercises: localProgress() }),
          });
          ls.set(flag, '1');
        }
        const data: { exercises: Record<string, ServerExercise> } = await fetch('/api/progress').then((r) => r.json());
        for (const [id, p] of Object.entries(data.exercises ?? {})) {
          if (p.solved) ls.set(`mir:solved:${id}`, '1');
          for (const [file, content] of Object.entries(p.files ?? {})) ls.set(`mir:draft:${id}:${file}`, content);
        }
      }
      window.dispatchEvent(new CustomEvent(PROGRESS_EVENT, { detail: { user: currentUser } }));
      return { configured: !!me.configured, user: currentUser };
    } catch {
      return { configured: false, user: null };
    }
  })();
  return syncPromise;
}

const pending = new Map<string, ReturnType<typeof setTimeout>>();

/** Guarda el código en el servidor (con un pequeño retardo para no mandar cada tecla). */
export function pushDraft(exerciseId: string, files: Record<string, string>, delay = 1200) {
  if (!currentUser) return;
  clearTimeout(pending.get(exerciseId));
  pending.set(
    exerciseId,
    setTimeout(() => {
      pending.delete(exerciseId);
      fetch(`/api/progress/${exerciseId}`, {
        method: 'PUT',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ files }),
        keepalive: true,
      }).catch(() => {});
    }, delay),
  );
}

export const loginUrl = () => `/auth/github/login?next=${encodeURIComponent(location.pathname + location.search)}`;

export async function logout() {
  await fetch('/auth/logout', { method: 'POST' }).catch(() => {});
  // Este navegador puede ser compartido: limpiamos la copia local.
  for (const key of ls.keys()) {
    if (key.startsWith('mir:draft:') || key.startsWith('mir:solved:') || key.startsWith('mir:imported:')) {
      try {
        localStorage.removeItem(key);
      } catch {}
    }
  }
  location.reload();
}
