import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import CodeEditor, { type EditorFile } from './editor/CodeEditor';
import ViewerPanel from './viewer/ViewerPanel';
import Split from './ui/Split';
import { useConfirm } from './ui/ConfirmDialog';
import { API_BY_ID } from '../lib/ueApi';
import { loadDraft, markSolved, isSolved, saveDraft, clearDrafts } from '../lib/progress';
import AccountButton from './AccountButton';
import { pushDraft, PROGRESS_EVENT } from '../lib/account';
import type { Diagnostic, Exercise, ExerciseLink, RunResponse, ScenarioResult, TraceEvent } from '../lib/types';

interface Props {
  exercise: Exercise;
  prev: ExerciseLink | null;
  next: ExerciseLink | null;
}

type LeftTab = 'enunciado' | 'pistas' | 'chuleta';
type BottomTab = 'resultados' | 'consola';

const difficultyColor: Record<string, string> = {
  Fácil: 'text-emerald-300 bg-emerald-400/10 border-emerald-400/30',
  Media: 'text-orange-300 bg-orange-400/10 border-orange-400/30',
  Difícil: 'text-rose-300 bg-rose-400/10 border-rose-400/30',
};

const verbosityColor: Record<string, string> = {
  Fatal: 'text-rose-400',
  Error: 'text-rose-400',
  Warning: 'text-amber-300',
  Display: 'text-ink-100',
  Log: 'text-ink-300',
};

async function runCode(exercise: string, files: Record<string, string>): Promise<RunResponse> {
  const res = await fetch('/api/run', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ exercise, files }),
  });
  const data = await res.json();
  if (!res.ok) throw new Error(data.error ?? `Error ${res.status}`);
  return data as RunResponse;
}

export default function ExerciseApp({ exercise, prev, next }: Props) {
  const editable = useMemo(() => exercise.files.filter((f) => f.editable), [exercise]);
  const [code, setCode] = useState<Record<string, string>>(() =>
    Object.fromEntries(editable.map((f) => [f.name, loadDraft(exercise.id, f.name) ?? f.content])),
  );
  const [active, setActive] = useState(editable[0]?.name ?? exercise.files[0].name);
  const [showSolution, setShowSolution] = useState(false);
  const [leftTab, setLeftTab] = useState<LeftTab>('enunciado');
  const [bottomTab, setBottomTab] = useState<BottomTab>('resultados');
  const [hintsShown, setHintsShown] = useState(0);
  const [running, setRunning] = useState(false);
  const [result, setResult] = useState<RunResponse | null>(null);
  const [preview, setPreview] = useState<ScenarioResult[] | null>(null);
  const [previewError, setPreviewError] = useState<string | null>(null);
  const urlParams = useMemo(() => new URLSearchParams(typeof location !== 'undefined' ? location.search : ''), []);
  const [selected, setSelected] = useState(() => Number(urlParams.get('s') ?? 0) || 0);
  const [startAt, setStartAt] = useState(() => Number(urlParams.get('t') ?? 0) || 0);
  const [playToken, setPlayToken] = useState(0);
  const [solved, setSolved] = useState(false);
  const [netError, setNetError] = useState<string | null>(null);
  const [dialog, ask] = useConfirm();
  const codeRef = useRef(code);
  codeRef.current = code;

  useEffect(() => setSolved(isSolved(exercise.id)), [exercise.id]);

  // Al sincronizar con la cuenta de GitHub, cargar el código guardado en el servidor
  // (salvo que ya hayas empezado a escribir).
  const dirtyRef = useRef(false);
  useEffect(() => {
    const onSync = () => {
      setSolved(isSolved(exercise.id));
      if (dirtyRef.current) return;
      setCode(Object.fromEntries(editable.map((f) => [f.name, loadDraft(exercise.id, f.name) ?? f.content])));
    };
    window.addEventListener(PROGRESS_EVENT, onSync);
    return () => window.removeEventListener(PROGRESS_EVENT, onSync);
  }, [exercise.id, editable]);

  // Al entrar: ejecutar en silencio para tener el mundo listo en el visor.
  useEffect(() => {
    let cancelled = false;
    runCode(exercise.id, codeRef.current)
      .then((r) => {
        if (cancelled) return;
        if (r.stage === 'ran') setPreview(r.scenarios);
        else setPreviewError('Tu código tiene errores: pulsa Ejecutar para verlos.');
      })
      .catch((e) => !cancelled && setPreviewError(`No hay conexión con mir-server (${e.message}). ¿Está arrancado?`));
    return () => {
      cancelled = true;
    };
  }, [exercise.id]);

  const files: EditorFile[] = useMemo(() => {
    const list: EditorFile[] = exercise.files.map((f) => ({ name: f.name, value: f.editable ? code[f.name] : f.content, readOnly: !f.editable }));
    if (showSolution) {
      for (const f of editable) if (f.solution) list.push({ name: `Solución · ${f.name}`, value: f.solution, readOnly: true });
    }
    return list;
  }, [exercise, code, showSolution, editable]);

  const onChange = useCallback(
    (name: string, value: string) => {
      setCode((c) => ({ ...c, [name]: value }));
      saveDraft(exercise.id, name, value);
      if (codeRef.current[name] !== value) {
        dirtyRef.current = true;
        pushDraft(exercise.id, { ...codeRef.current, [name]: value });
      }
    },
    [exercise.id],
  );

  const run = useCallback(async () => {
    if (running) return;
    setRunning(true);
    setNetError(null);
    try {
      const r = await runCode(exercise.id, codeRef.current);
      setResult(r);
      if (r.stage === 'ran') {
        setPreview(r.scenarios);
        const firstFail = r.scenarios.findIndex((s) => !s.passed);
        setSelected(firstFail >= 0 ? firstFail : 0);
        setStartAt(0);
        setPlayToken((t) => t + 1);
        setBottomTab('resultados');
        if (r.all_passed) {
          markSolved(exercise.id);
          setSolved(true);
        }
      } else {
        setBottomTab('consola');
        const firstErr = r.diagnostics.find((d) => d.severity === 'error' && d.file);
        if (firstErr && editable.some((f) => f.name === firstErr.file)) setActive(firstErr.file);
      }
    } catch (e) {
      setNetError((e as Error).message);
      setBottomTab('consola');
    } finally {
      setRunning(false);
    }
  }, [exercise.id, running, editable]);

  const reset = async () => {
    const choice = await ask({
      title: '¿Reiniciar el código?',
      message: 'Volverás a la plantilla original de este ejercicio y perderás lo que has escrito.',
      confirmLabel: 'Reiniciar',
      cancelLabel: 'Cancelar',
      tone: 'danger',
    });
    if (choice !== 'confirm') return;
    clearDrafts(exercise.id, editable.map((f) => f.name));
    const template = Object.fromEntries(editable.map((f) => [f.name, f.content]));
    setCode(template);
    pushDraft(exercise.id, template, 0);
    setResult(null);
  };

  const openSolution = () => {
    setShowSolution(true);
    setActive(`Solución · ${editable[0].name}`);
  };

  const revealSolution = async () => {
    if (showSolution) {
      setShowSolution(false);
      setActive(editable[0].name);
      return;
    }
    // Si ya lo has resuelto, no hay nada que proteger.
    if (solved) {
      openSolution();
      return;
    }
    const hintsLeft = exercise.hints.length - hintsShown;
    const choice = await ask({
      title: '¿Ver la solución?',
      message:
        hintsLeft > 0
          ? `Aún te quedan ${hintsLeft} ${hintsLeft === 1 ? 'pista' : 'pistas'} por ver. Desbloquearte con una pista enseña mucho más que leer la solución.`
          : 'Ya has visto todas las pistas. Si la miras, intenta después escribirla tú sin copiarla.',
      confirmLabel: 'Ver solución',
      cancelLabel: 'Seguir intentándolo',
      extraLabel: hintsLeft > 0 ? 'Ver una pista' : undefined,
    });
    if (choice === 'confirm') openSolution();
    if (choice === 'extra') {
      setHintsShown((n) => Math.min(exercise.hints.length, n + 1));
      setLeftTab('pistas');
    }
  };

  const scenarios = result?.stage === 'ran' ? result.scenarios : null;
  const shown = scenarios ?? preview;
  const current = shown?.[Math.min(selected, (shown?.length ?? 1) - 1)] ?? null;
  const diagnostics: Diagnostic[] = result?.diagnostics ?? [];
  const passedCount = scenarios?.filter((s) => s.passed).length ?? 0;
  const logs = (current?.trace.events ?? []).filter((e): e is Extract<TraceEvent, { type: 'log' }> => e.type === 'log');

  return (
    <div className="flex h-dvh flex-col bg-ink-900">
      {dialog}
      {/* Barra superior */}
      <header className="flex h-12 shrink-0 items-center gap-3 border-b border-ink-800 bg-ink-950 px-3">
        <a href="/" className="flex items-center gap-2 rounded-lg px-1.5 py-1 transition hover:bg-ink-800" title="Todos los ejercicios">
          <img src="/favicon.svg" alt="" className="size-6" />
          <span className="font-display text-sm font-bold tracking-tight text-ink-100">
            MIR<span className="text-mir-500">.</span>
          </span>
        </a>
        <div className="h-5 w-px bg-ink-700" />
        <div className="flex min-w-0 items-center gap-2">
          <span className="font-mono text-xs text-ink-400">#{String(exercise.number).padStart(2, '0')}</span>
          <h1 className="truncate font-display text-[15px] font-semibold text-ink-100">{exercise.title}</h1>
          <span className={`rounded-full border px-2 py-0.5 text-[11px] font-medium ${difficultyColor[exercise.difficulty] ?? ''}`}>{exercise.difficulty}</span>
          {solved && <span className="rounded-full border border-emerald-400/30 bg-emerald-400/10 px-2 py-0.5 text-[11px] font-medium text-emerald-300">✓ Resuelto</span>}
        </div>
        <div className="ml-auto flex items-center gap-1.5">
          <a
            href={prev ? `/ejercicios/${prev.id}/` : undefined}
            className={`rounded-lg px-2 py-1.5 text-xs text-ink-300 transition hover:bg-ink-800 hover:text-ink-100 ${prev ? '' : 'pointer-events-none opacity-30'}`}
            title={prev?.title}
          >
            ← Anterior
          </a>
          <a
            href={next ? `/ejercicios/${next.id}/` : undefined}
            className={`rounded-lg px-2 py-1.5 text-xs text-ink-300 transition hover:bg-ink-800 hover:text-ink-100 ${next ? '' : 'pointer-events-none opacity-30'}`}
            title={next?.title}
          >
            Siguiente →
          </a>
          <div className="mx-1 h-5 w-px bg-ink-700" />
          <button onClick={reset} className="rounded-lg px-2.5 py-1.5 text-xs text-ink-300 transition hover:bg-ink-800 hover:text-ink-100">
            Reiniciar
          </button>
          <button onClick={revealSolution} className="rounded-lg px-2.5 py-1.5 text-xs text-ink-300 transition hover:bg-ink-800 hover:text-ink-100">
            {showSolution ? 'Ocultar solución' : 'Ver solución'}
          </button>
          <button
            onClick={run}
            disabled={running}
            className="ml-1 flex items-center gap-2 rounded-lg bg-mir-500 px-4 py-1.5 text-sm font-semibold text-ink-950 shadow-[0_0_0_1px_rgba(0,0,0,.2),0_6px_20px_-6px_rgba(255,107,26,.6)] transition hover:bg-mir-400 disabled:cursor-wait disabled:opacity-70"
            title="Compilar y ejecutar (Ctrl+Enter)"
          >
            {running ? (
              <svg className="size-4 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3"><path d="M12 3a9 9 0 1 0 9 9" strokeLinecap="round" /></svg>
            ) : (
              <svg viewBox="0 0 24 24" className="size-4" fill="currentColor"><path d="M8 5.5v13a1 1 0 0 0 1.5.86l11-6.5a1 1 0 0 0 0-1.72l-11-6.5A1 1 0 0 0 8 5.5Z" /></svg>
            )}
            {running ? 'Compilando…' : 'Ejecutar'}
          </button>
          <div className="ml-1.5">
            <AccountButton compact />
          </div>
        </div>
      </header>

      <div className="min-h-0 flex-1">
        <Split
          direction="row"
          initial={27}
          min={16}
          max={45}
          storageKey="mir:split:left"
          first={
            <section className="flex h-full flex-col bg-ink-900">
              <nav className="flex shrink-0 gap-1 border-b border-ink-800 px-2 pt-2">
                {(['enunciado', 'pistas', 'chuleta'] as LeftTab[]).map((t) => (
                  <button
                    key={t}
                    onClick={() => setLeftTab(t)}
                    className={`rounded-t-lg px-3 py-1.5 text-xs font-medium capitalize transition ${leftTab === t ? 'bg-ink-800 text-ink-100' : 'text-ink-400 hover:text-ink-200'}`}
                  >
                    {t === 'pistas' ? `Pistas (${exercise.hints.length})` : t}
                  </button>
                ))}
              </nav>
              <div className="scroll-thin min-h-0 flex-1 overflow-y-auto p-5">
                {leftTab === 'enunciado' && (
                  <>
                    <article className="prose-mir" dangerouslySetInnerHTML={{ __html: exercise.statementHtml }} />
                    <h3 className="mt-6 mb-2 font-display text-sm font-semibold text-ink-100">Escenarios que se prueban</h3>
                    <ul className="space-y-1.5">
                      {exercise.scenarios.map((s, i) => {
                        const r = scenarios?.find((x) => x.id === s.id);
                        return (
                          <li key={s.id} className="flex items-center gap-2 text-sm text-ink-300">
                            <span className={`flex size-5 items-center justify-center rounded-full text-[10px] font-bold ${r ? (r.passed ? 'bg-emerald-400/15 text-emerald-300' : 'bg-rose-400/15 text-rose-300') : 'bg-ink-700 text-ink-400'}`}>
                              {r ? (r.passed ? '✓' : '✕') : i + 1}
                            </span>
                            {s.name}
                          </li>
                        );
                      })}
                    </ul>
                  </>
                )}
                {leftTab === 'pistas' && (
                  <div className="space-y-3">
                    {exercise.hints.slice(0, hintsShown).map((h, i) => (
                      <div key={i} className="rounded-xl border border-ink-700 bg-ink-850 p-3 text-sm leading-relaxed text-ink-200">
                        <div className="mb-1 font-mono text-[11px] text-mir-400">Pista {i + 1}</div>
                        {h}
                      </div>
                    ))}
                    {hintsShown < exercise.hints.length ? (
                      <button onClick={() => setHintsShown((n) => n + 1)} className="w-full rounded-xl border border-dashed border-ink-600 py-2.5 text-sm text-ink-300 transition hover:border-mir-500 hover:text-mir-400">
                        Mostrar pista {hintsShown + 1} de {exercise.hints.length}
                      </button>
                    ) : (
                      <p className="text-xs text-ink-400">No hay más pistas. Si te atascas, prueba con UE_LOG para ver qué valores salen.</p>
                    )}
                  </div>
                )}
                {leftTab === 'chuleta' && (
                  <div className="space-y-3">
                    <p className="text-xs text-ink-400">Lo que necesitas para este ejercicio. En el editor tienes autocompletado (Ctrl+Espacio) y documentación al pasar el ratón.</p>
                    {exercise.api.map((id) => {
                      const e = API_BY_ID.get(id);
                      if (!e) return null;
                      return (
                        <div key={id} className="rounded-xl border border-ink-700 bg-ink-850 p-3">
                          <code className="block font-mono text-[12px] leading-relaxed break-words text-sky-300">{e.signature}</code>
                          <p className="mt-1.5 text-[13px] leading-relaxed text-ink-300">{e.doc}</p>
                        </div>
                      );
                    })}
                  </div>
                )}
              </div>
            </section>
          }
          second={
            <Split
              direction="row"
              initial={50}
              min={25}
              max={75}
              storageKey="mir:split:mid"
              first={
                <section className="flex h-full flex-col bg-ink-950">
                  <nav className="scroll-thin flex shrink-0 gap-px overflow-x-auto border-b border-ink-800 bg-ink-900">
                    {files.map((f) => (
                      <button
                        key={f.name}
                        onClick={() => setActive(f.name)}
                        className={`flex shrink-0 items-center gap-1.5 border-r border-ink-800 px-3 py-2 font-mono text-xs transition ${active === f.name ? 'bg-ink-950 text-ink-100 shadow-[inset_0_2px_0_var(--color-mir-500)]' : 'text-ink-400 hover:text-ink-200'}`}
                      >
                        {f.readOnly ? (
                          <svg viewBox="0 0 24 24" className="size-3 opacity-60" fill="none" stroke="currentColor" strokeWidth="2.4"><rect x="5" y="11" width="14" height="10" rx="2" /><path d="M8 11V7a4 4 0 0 1 8 0v4" /></svg>
                        ) : (
                          <span className="size-1.5 rounded-full bg-mir-500" />
                        )}
                        {f.name}
                        {diagnostics.some((d) => d.file === f.name && d.severity === 'error') && <span className="size-1.5 rounded-full bg-rose-500" />}
                      </button>
                    ))}
                  </nav>
                  <div className="min-h-0 flex-1">
                    <CodeEditor exerciseId={exercise.id} files={files} active={active} diagnostics={diagnostics} onChange={onChange} onRun={run} />
                  </div>
                </section>
              }
              second={
                <Split
                  direction="col"
                  initial={62}
                  min={25}
                  max={85}
                  storageKey="mir:split:right"
                  first={
                    <section className="flex h-full flex-col">
                      <nav className="scroll-thin flex shrink-0 items-center gap-1 overflow-x-auto border-b border-ink-800 bg-ink-900 px-2 py-1.5">
                        {(shown ?? exercise.scenarios.map((s) => ({ ...s, passed: false }))).map((s, i) => {
                          const r = scenarios?.[i];
                          return (
                            <button
                              key={s.id}
                              onClick={() => {
                                setSelected(i);
                                setPlayToken((t) => t + 1);
                              }}
                              className={`flex shrink-0 items-center gap-1.5 rounded-md px-2.5 py-1 text-xs transition ${selected === i ? 'bg-ink-700 text-ink-100' : 'text-ink-400 hover:bg-ink-800 hover:text-ink-200'}`}
                            >
                              {r && <span className={r.passed ? 'text-emerald-400' : 'text-rose-400'}>{r.passed ? '✓' : '✕'}</span>}
                              {s.name}
                            </button>
                          );
                        })}
                      </nav>
                      <div className="min-h-0 flex-1">
                        <ViewerPanel trace={current?.trace ?? null} playToken={playToken} autoplay={!!scenarios} placeholder={previewError ?? undefined} startAt={startAt} />
                      </div>
                    </section>
                  }
                  second={
                    <section className="flex h-full flex-col bg-ink-900">
                      <nav className="flex shrink-0 items-center gap-1 border-b border-ink-800 px-2 pt-2">
                        {(['resultados', 'consola'] as BottomTab[]).map((t) => (
                          <button
                            key={t}
                            onClick={() => setBottomTab(t)}
                            className={`rounded-t-lg px-3 py-1.5 text-xs font-medium capitalize transition ${bottomTab === t ? 'bg-ink-800 text-ink-100' : 'text-ink-400 hover:text-ink-200'}`}
                          >
                            {t === 'consola' ? 'Output Log' : 'Resultados'}
                          </button>
                        ))}
                        {result && (
                          <span className="ml-auto pb-1 font-mono text-[11px] text-ink-400">
                            compilado en {result.compile_ms} ms{result.stage === 'ran' ? ` · simulado en ${result.run_ms} ms` : ''}
                          </span>
                        )}
                      </nav>
                      <div className="scroll-thin min-h-0 flex-1 overflow-y-auto">
                        {bottomTab === 'resultados' && (
                          <div className="p-3">
                            {netError && <div className="mb-2 rounded-lg border border-rose-500/30 bg-rose-500/10 p-3 text-sm text-rose-200">{netError}</div>}
                            {!result && !netError && <p className="p-2 text-sm text-ink-400">Escribe tu código y pulsa <b className="text-ink-200">Ejecutar</b> (o Ctrl+Enter). Verás al monigote probarlo en cada escenario.</p>}
                            {result?.stage === 'compile_error' && (
                              <div className="rounded-lg border border-rose-500/30 bg-rose-500/10 p-3 text-sm text-rose-200">
                                No compila: {result.diagnostics.filter((d) => d.severity === 'error').length} error(es). Míralos en el Output Log o en el editor.
                              </div>
                            )}
                            {scenarios && (
                              <>
                                <div className={`mb-3 flex items-center gap-3 rounded-xl border p-3 ${result?.stage === 'ran' && result.all_passed ? 'border-emerald-400/30 bg-emerald-400/10' : 'border-ink-700 bg-ink-850'}`}>
                                  <div className={`font-display text-2xl font-bold ${result?.stage === 'ran' && result.all_passed ? 'text-emerald-300' : 'text-ink-100'}`}>
                                    {passedCount}/{scenarios.length}
                                  </div>
                                  <div className="text-sm text-ink-300">
                                    {result?.stage === 'ran' && result.all_passed ? (
                                      <>
                                        <b className="text-emerald-300">¡Ejercicio resuelto!</b>
                                        {next && (
                                          <>
                                            {' '}Siguiente:{' '}
                                            <a className="text-mir-400 underline-offset-2 hover:underline" href={`/ejercicios/${next.id}/`}>
                                              {next.title} →
                                            </a>
                                          </>
                                        )}
                                      </>
                                    ) : (
                                      'escenarios superados'
                                    )}
                                  </div>
                                </div>
                                <ul className="space-y-1.5">
                                  {scenarios.map((s, i) => (
                                    <li key={s.id}>
                                      <button
                                        onClick={() => {
                                          setSelected(i);
                                          setPlayToken((t) => t + 1);
                                        }}
                                        className={`flex w-full items-start gap-2.5 rounded-lg border px-3 py-2 text-left transition ${selected === i ? 'border-ink-500 bg-ink-800' : 'border-transparent hover:bg-ink-850'}`}
                                      >
                                        <span className={`mt-0.5 flex size-5 shrink-0 items-center justify-center rounded-full text-[11px] font-bold ${s.passed ? 'bg-emerald-400/15 text-emerald-300' : 'bg-rose-400/15 text-rose-300'}`}>
                                          {s.passed ? '✓' : '✕'}
                                        </span>
                                        <span className="min-w-0">
                                          <span className="block text-sm font-medium text-ink-100">{s.name}</span>
                                          <span className={`block text-[13px] leading-snug ${s.crash ? 'font-mono text-rose-300' : 'text-ink-300'}`}>
                                            {s.crash ? `💥 ${s.message}` : s.message}
                                          </span>
                                        </span>
                                      </button>
                                    </li>
                                  ))}
                                </ul>
                              </>
                            )}
                          </div>
                        )}
                        {bottomTab === 'consola' && (
                          <div className="p-3 font-mono text-[12px] leading-relaxed">
                            {netError && <div className="text-rose-300">{netError}</div>}
                            {result?.diagnostics.map((d, i) => (
                              <div key={i} className={d.severity === 'error' ? 'text-rose-300' : 'text-amber-300'}>
                                {d.file ? `${d.file}:${d.line}:${d.col}: ` : ''}
                                {d.severity}: {d.message}
                              </div>
                            ))}
                            {result?.stage === 'compile_error' && result.log && (
                              <pre className="mt-2 rounded-lg border border-ink-700 bg-ink-950 p-3 whitespace-pre-wrap text-ink-300">{result.log}</pre>
                            )}
                            {current && (
                              <>
                                <div className="mb-1 text-ink-400">— Output Log · {current.name} —</div>
                                {logs.length === 0 && !current.crash && <div className="text-ink-500">(sin mensajes: usa UE_LOG(LogTemp, Warning, TEXT("…")) para depurar)</div>}
                                {logs.map((l, i) => (
                                  <div key={i} className={verbosityColor[l.verbosity] ?? 'text-ink-300'}>
                                    <span className="text-ink-500">[{l.t.toFixed(2)}s]</span> {l.category}: {l.verbosity !== 'Log' ? `${l.verbosity}: ` : ''}
                                    {l.text}
                                  </div>
                                ))}
                                {current.crash && <div className="mt-1 text-rose-400">💥 {current.crash}</div>}
                              </>
                            )}
                          </div>
                        )}
                      </div>
                    </section>
                  }
                />
              }
            />
          }
        />
      </div>
    </div>
  );
}
