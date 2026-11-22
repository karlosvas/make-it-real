export interface ExerciseFile {
  name: string;
  editable: boolean;
  content: string;
  solution?: string;
}

export interface ScenarioMeta {
  id: string;
  name: string;
}

export interface Exercise {
  id: string;
  number: number;
  title: string;
  difficulty: string;
  summary: string;
  hints: string[];
  api: string[];
  scenarios: ScenarioMeta[];
  files: ExerciseFile[];
  statementHtml: string;
}

export interface ExerciseLink {
  id: string;
  number: number;
  title: string;
}

export interface Diagnostic {
  file: string;
  line: number;
  col: number;
  severity: 'error' | 'warning';
  message: string;
}

export type ActorClass = 'actor' | 'monigote' | 'door' | 'key' | 'guard' | 'target_point';

export interface ActorMeta {
  handle: number;
  class: ActorClass;
  name: string;
  label?: string;
  walls: boolean;
  sight?: [number, number];
}

export type TraceEvent =
  | { type: 'log'; t: number; category: string; verbosity: string; text: string }
  | { type: 'screen'; t: number; text: string; color: string; duration: number }
  | { type: 'door_attempt'; t: number; door: number; who: number; allowed: boolean }
  | { type: 'pickup'; t: number; who: number; key: number }
  | { type: 'choice'; t: number; who: number; target: number }
  | { type: 'bubble'; t: number; actor: number; text: string; color: string }
  | { type: 'crash'; t: number; message: string };

export interface DebugDraw {
  t: number;
  life: number;
  persistent: boolean;
  kind: number;
  a: [number, number, number];
  b: [number, number, number];
  size: number;
  extra: number;
  color: string;
  text?: string;
}

export interface Trace {
  dt: number;
  duration: number;
  actors: ActorMeta[];
  /** Por frame: [t, (x, y, z, yaw, leafYaw, hidden, flag) × actor] */
  frames: number[][];
  events: TraceEvent[];
  draws: DebugDraw[];
}

export interface ScenarioResult {
  id: string;
  name: string;
  passed: boolean;
  message: string;
  crash?: string;
  trace: Trace;
}

export type RunResponse =
  | { stage: 'compile_error'; diagnostics: Diagnostic[]; log: string; compile_ms: number }
  | {
      stage: 'ran';
      diagnostics: Diagnostic[];
      log: string;
      compile_ms: number;
      run_ms: number;
      all_passed: boolean;
      scenarios: ScenarioResult[];
    };

export const ACTOR_STRIDE = 7;
