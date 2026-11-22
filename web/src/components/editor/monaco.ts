// Configuración única de Monaco: worker, tema estilo Unreal y autocompletado del mini-Unreal.
import * as monaco from 'monaco-editor';
import EditorWorker from 'monaco-editor/editor/editor.worker?worker';
import { UE_API } from '../../lib/ueApi';

let ready = false;

export function setupMonaco(): typeof monaco {
  if (ready) return monaco;
  ready = true;
  (self as unknown as { MonacoEnvironment: monaco.Environment }).MonacoEnvironment = {
    getWorker: () => new EditorWorker(),
  };

  monaco.editor.defineTheme('mir-dark', {
    base: 'vs-dark',
    inherit: true,
    rules: [
      { token: 'keyword', foreground: 'c792ea' },
      { token: 'type', foreground: '82aaff' },
      { token: 'identifier', foreground: 'd6deeb' },
      { token: 'number', foreground: 'f78c6c' },
      { token: 'string', foreground: 'c3e88d' },
      { token: 'comment', foreground: '6b7a99', fontStyle: 'italic' },
      { token: 'keyword.directive', foreground: 'ffcb6b' },
    ],
    colors: {
      'editor.background': '#0b0d12',
      'editor.lineHighlightBackground': '#141821',
      'editorLineNumber.foreground': '#3b4459',
      'editorLineNumber.activeForeground': '#98a1b8',
      'editorCursor.foreground': '#ff8f4d',
      'editor.selectionBackground': '#2a3550',
      'editorIndentGuide.background1': '#1a1f2b',
      'editorWidget.background': '#10131a',
      'editorSuggestWidget.background': '#10131a',
      'editorSuggestWidget.border': '#2a3142',
      'editorSuggestWidget.selectedBackground': '#1f2738',
      'editorHoverWidget.background': '#10131a',
      'editorHoverWidget.border': '#2a3142',
    },
  });

  const kindMap: Record<string, monaco.languages.CompletionItemKind> = {
    function: monaco.languages.CompletionItemKind.Function,
    method: monaco.languages.CompletionItemKind.Method,
    class: monaco.languages.CompletionItemKind.Class,
    field: monaco.languages.CompletionItemKind.Field,
    macro: monaco.languages.CompletionItemKind.Snippet,
    constant: monaco.languages.CompletionItemKind.Constant,
  };

  monaco.languages.registerCompletionItemProvider('cpp', {
    triggerCharacters: ['.', ':', '>'],
    provideCompletionItems(model, position) {
      const word = model.getWordUntilPosition(position);
      const range = new monaco.Range(position.lineNumber, word.startColumn, position.lineNumber, word.endColumn);
      const seen = new Set<string>();
      const suggestions: monaco.languages.CompletionItem[] = [];
      for (const e of UE_API) {
        const key = `${e.label}|${e.signature}`;
        if (seen.has(key)) continue;
        seen.add(key);
        suggestions.push({
          label: { label: e.label, description: e.id.includes('::') ? e.id.split('::')[0] : undefined },
          kind: kindMap[e.kind],
          detail: e.signature,
          documentation: { value: e.doc },
          insertText: e.insert ?? e.label,
          insertTextRules: e.insert ? monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet : undefined,
          range,
        });
      }
      return { suggestions };
    },
  });

  monaco.languages.registerHoverProvider('cpp', {
    provideHover(model, position) {
      const word = model.getWordAtPosition(position);
      if (!word) return null;
      const matches = UE_API.filter((e) => e.label === word.word);
      if (matches.length === 0) return null;
      return {
        range: new monaco.Range(position.lineNumber, word.startColumn, position.lineNumber, word.endColumn),
        contents: matches.slice(0, 3).flatMap((e) => [{ value: '```cpp\n' + e.signature + '\n```' }, { value: e.doc }]),
      };
    },
  });
  return monaco;
}

export type Monaco = typeof monaco;
