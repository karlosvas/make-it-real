// Progreso y borradores en localStorage (solo en este navegador).
const safe = <T,>(fn: () => T, fallback: T): T => {
  try {
    return fn();
  } catch {
    return fallback;
  }
};

export const loadDraft = (exercise: string, file: string) => safe(() => localStorage.getItem(`mir:draft:${exercise}:${file}`), null);
export const saveDraft = (exercise: string, file: string, value: string) => safe(() => localStorage.setItem(`mir:draft:${exercise}:${file}`, value), undefined);
export const clearDrafts = (exercise: string, files: string[]) => safe(() => files.forEach((f) => localStorage.removeItem(`mir:draft:${exercise}:${f}`)), undefined);
export const isSolved = (exercise: string) => safe(() => localStorage.getItem(`mir:solved:${exercise}`) === '1', false);
export const markSolved = (exercise: string) => safe(() => localStorage.setItem(`mir:solved:${exercise}`, '1'), undefined);
