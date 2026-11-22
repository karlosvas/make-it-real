# Make It Real (MIR)

**LeetCode-style gameplay exercises for Unreal Engine.** Short challenges using Unreal's C++ API: write the code, hit **Run**, and watch in 3D how the character tries it out across several scenarios (✓/✕).

**Live:** https://make-it-real-nine.vercel.app (the app UI and exercises are in Spanish).

```
┌──────────────┬─────────────────────┬──────────────────────────┐
│  Statement   │  Door.cpp  (Monaco) │   3D viewer (three.js)   │
│  Hints       │                     │   ▶ ─────●──── 1x        │
│  API sheet   │  bool ADoor::...    ├──────────────────────────┤
│              │                     │ ✓ From the front         │
│              │                     │ ✕ From behind            │
└──────────────┴─────────────────────┴──────────────────────────┘
```

## Getting started

Requirements: `clang++` with the `wasm32` target + `wasm-ld` (the regular Linux clang already ships both), Rust and pnpm.

```bash
./scripts/dev.sh     # development: mir-server :8787 + Astro :4321  → open http://localhost:4321
./scripts/serve.sh   # everything served by mir-server             → open http://127.0.0.1:8787
```

## Deploying to Vercel

It is deployed as a container (`Dockerfile.vercel`): Rust server + clang + the website.

```bash
vercel deploy --prod --yes
```

Environment variables (Production): `MIR_GITHUB_CLIENT_ID`, `MIR_GITHUB_CLIENT_SECRET`, `MIR_PUBLIC_URL`, plus the Upstash Redis database from the Vercel Marketplace (`KV_REST_API_URL`, `KV_REST_API_TOKEN`). See `.env.example`.

## How it works

1. The website (Astro + Tailwind + a React island with Monaco and three.js) sends your C++ to `POST /api/run`.
2. **mir-server** (Rust, axum) compiles it with `clang++ --target=wasm32` against a **mini-Unreal** (`sdk/include`: `FVector`, `FRotator`, `FMath`, `TArray`, `FString`, `UE_LOG`, `AActor`, `ACharacter`, timers, `DrawDebug*`…) in ~0.2 s.
3. It runs the wasm with **wasmi** (an interpreter with "fuel": an infinite loop gets cut off instead of hanging anything) inside a **simulated world**: `CharacterMovement`-style movement, collisions with doors and walls, keys, and so on.
4. Each scenario records a **trace** (per-frame positions, events, logs, debug draws), and a Rust check decides whether your code did the right thing.
5. The website replays the trace in 3D and shows ✓/✕ with a clear message.

What you write is real Unreal C++: same names, signatures and behaviour (e.g. `FInterpTo`, `SetTimer` replacing the handle, `Array index out of bounds`…). You can paste it into an Unreal project.

## Project layout

```
sdk/include/        mini-Unreal (headers with the same paths as in UE: GameFramework/Actor.h…)
sdk/runtime/        implementation: memory, printf, actors → "ue.*" imports, timers, exports
exercises/NN-xxx/   one exercise: exercise.json, enunciado.md, templates, solution/, glue.cpp
crates/mir-server/  Rust server: compile.rs, sim/ (world, wasm host, runner), exercises/ (scenarios),
                    auth.rs + store.rs (GitHub login and progress)
web/                Astro: pages, ExerciseApp (React), CodeEditor (Monaco), Viewer3D (three.js)
```

## Tests

```bash
cargo test
```

Checks, for **every exercise**, that the solution passes all scenarios and that the template compiles but fails at least one.

---

Make It Real is not Unreal Engine and is not affiliated with Epic Games: it simulates part of its API for learning purposes.
