import { useEffect, useRef } from 'react';
import type { editor as MonacoEditor } from 'monaco-editor';
import type { Diagnostic } from '../../lib/types';
import { setupMonaco } from './monaco';

export interface EditorFile {
  name: string;
  value: string;
  readOnly: boolean;
}

interface Props {
  exerciseId: string;
  files: EditorFile[];
  active: string;
  diagnostics: Diagnostic[];
  onChange: (name: string, value: string) => void;
  onRun: () => void;
  /** Modo de edición: como VS Code (normal) o como Vim/Neovim. */
  vim?: boolean;
}

export default function CodeEditor({ exerciseId, files, active, diagnostics, onChange, onRun, vim = false }: Props) {
  const statusRef = useRef<HTMLDivElement>(null);
  const host = useRef<HTMLDivElement>(null);
  const editorRef = useRef<MonacoEditor.IStandaloneCodeEditor | null>(null);
  const models = useRef(new Map<string, MonacoEditor.ITextModel>());
  const onChangeRef = useRef(onChange);
  const onRunRef = useRef(onRun);
  onChangeRef.current = onChange;
  onRunRef.current = onRun;

  // Crear el editor una vez.
  useEffect(() => {
    const monaco = setupMonaco();
    const ed = monaco.editor.create(host.current!, {
      theme: 'mir-dark',
      language: 'cpp',
      fontFamily: '"JetBrains Mono", ui-monospace, monospace',
      fontSize: 13.5,
      lineHeight: 21,
      fontLigatures: false,
      minimap: { enabled: false },
      automaticLayout: true,
      scrollBeyondLastLine: false,
      smoothScrolling: true,
      cursorSmoothCaretAnimation: 'on',
      tabSize: 4,
      insertSpaces: false,
      renderLineHighlight: 'all',
      padding: { top: 12, bottom: 12 },
      fixedOverflowWidgets: true,
      bracketPairColorization: { enabled: true },
    });
    ed.addAction({
      id: 'mir.run',
      label: 'Ejecutar (Make It Real)',
      keybindings: [monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter],
      run: () => onRunRef.current(),
    });
    editorRef.current = ed;
    return () => {
      ed.dispose();
      for (const m of models.current.values()) m.dispose();
      models.current.clear();
    };
  }, []);

  // Sincronizar modelos con los archivos.
  useEffect(() => {
    const monaco = setupMonaco();
    for (const f of files) {
      let model = models.current.get(f.name);
      if (!model) {
        const uri = monaco.Uri.parse(`file:///${exerciseId}/${encodeURIComponent(f.name)}`);
        model = monaco.editor.getModel(uri) ?? monaco.editor.createModel(f.value, 'cpp', uri);
        model.onDidChangeContent(() => {
          const file = f.name;
          if (!f.readOnly) onChangeRef.current(file, model!.getValue());
        });
        models.current.set(f.name, model);
      } else if (model.getValue() !== f.value) {
        model.setValue(f.value);
      }
    }
    for (const [name, model] of models.current) {
      if (!files.some((f) => f.name === name)) {
        model.dispose();
        models.current.delete(name);
      }
    }
  }, [files, exerciseId]);

  // Archivo activo.
  useEffect(() => {
    const ed = editorRef.current;
    const model = models.current.get(active);
    if (!ed || !model) return;
    if (ed.getModel() !== model) ed.setModel(model);
    const file = files.find((f) => f.name === active);
    ed.updateOptions({ readOnly: !!file?.readOnly, domReadOnly: !!file?.readOnly });
  }, [active, files]);

  // Modo Vim (monaco-vim): se carga solo si se activa.
  useEffect(() => {
    const ed = editorRef.current;
    if (!vim || !ed || !statusRef.current) return;
    let disposed = false;
    let session: { dispose: () => void } | null = null;
    import('monaco-vim').then(({ initVimMode, VimMode }) => {
      if (disposed) return;
      session = initVimMode(ed, statusRef.current);
      // :w ejecuta el código (como "guardar y probar").
      const Vim = (VimMode as unknown as { Vim: { defineEx: (name: string, prefix: string, fn: () => void) => void } }).Vim;
      Vim.defineEx('write', 'w', () => onRunRef.current());
      Vim.defineEx('run', 'run', () => onRunRef.current());
      ed.focus();
    });
    return () => {
      disposed = true;
      session?.dispose();
      ed.updateOptions({ cursorStyle: 'line' });
    };
  }, [vim]);

  // Errores del compilador como marcas en el editor.
  useEffect(() => {
    const monaco = setupMonaco();
    for (const [name, model] of models.current) {
      const marks = diagnostics
        .filter((d) => d.file === name && d.line > 0)
        .map((d) => ({
          severity: d.severity === 'error' ? monaco.MarkerSeverity.Error : monaco.MarkerSeverity.Warning,
          message: d.message,
          startLineNumber: d.line,
          startColumn: d.col,
          endLineNumber: d.line,
          endColumn: model.getLineMaxColumn(Math.min(d.line, model.getLineCount())),
        }));
      monaco.editor.setModelMarkers(model, 'clang', marks);
    }
  }, [diagnostics, files]);

  return (
    <div className="flex h-full w-full flex-col">
      <div ref={host} className="min-h-0 w-full flex-1" />
      <div
        ref={statusRef}
        className={`mir-vim-status shrink-0 border-t border-ink-800 bg-ink-900 px-3 py-1 font-mono text-xs text-ink-300 ${vim ? 'flex' : 'hidden'}`}
      />
    </div>
  );
}
