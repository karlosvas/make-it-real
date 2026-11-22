// Se ejecuta en build (frontmatter de Astro): lee los ejercicios de ../exercises.
import fs from 'node:fs';
import path from 'node:path';
import { marked } from 'marked';
import type { Exercise, ExerciseFile } from './types';

const ROOT = path.resolve(process.cwd(), '..', 'exercises');

interface RawMeta {
  id: string;
  number: number;
  title: string;
  difficulty: string;
  summary: string;
  hints?: string[];
  api?: string[];
  scenarios: { id: string; name: string }[];
  files: { name: string; editable: boolean }[];
}

export function loadExercises(): Exercise[] {
  if (!fs.existsSync(ROOT)) return [];
  const out: Exercise[] = [];
  for (const dir of fs.readdirSync(ROOT)) {
    const metaPath = path.join(ROOT, dir, 'exercise.json');
    if (!fs.existsSync(metaPath)) continue;
    const meta = JSON.parse(fs.readFileSync(metaPath, 'utf8')) as RawMeta;
    const files: ExerciseFile[] = meta.files.map((f) => {
      const content = fs.readFileSync(path.join(ROOT, dir, f.name), 'utf8');
      const solPath = path.join(ROOT, dir, 'solution', f.name);
      return {
        name: f.name,
        editable: f.editable,
        content,
        solution: f.editable && fs.existsSync(solPath) ? fs.readFileSync(solPath, 'utf8') : undefined,
      };
    });
    const mdPath = path.join(ROOT, dir, 'enunciado.md');
    const statementHtml = fs.existsSync(mdPath) ? (marked.parse(fs.readFileSync(mdPath, 'utf8')) as string) : '';
    out.push({
      id: meta.id,
      number: meta.number,
      title: meta.title,
      difficulty: meta.difficulty,
      summary: meta.summary,
      hints: meta.hints ?? [],
      api: meta.api ?? [],
      scenarios: meta.scenarios,
      files,
      statementHtml,
    });
  }
  return out.sort((a, b) => a.number - b.number);
}
