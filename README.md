# Make It Real (MIR)

**LeetCode de gameplay para Unreal Engine.** Ejercicios cortos con la API de Unreal en C++: escribes el código, pulsas **Ejecutar** y ves en 3D cómo el monigote lo prueba en varios escenarios (✓/✕).

```
┌──────────────┬─────────────────────┬──────────────────────────┐
│  Enunciado   │  Door.cpp  (Monaco) │   Visor 3D (three.js)    │
│  Pistas      │                     │   ▶ ─────●──── 1x        │
│  Chuleta API │  bool ADoor::...    ├──────────────────────────┤
│              │                     │ ✓ Desde delante          │
│              │                     │ ✕ Desde detrás           │
└──────────────┴─────────────────────┴──────────────────────────┘
```

## Arrancar

Requisitos: `clang++` con el target `wasm32` + `wasm-ld` (el clang normal de Linux ya los trae), Rust y pnpm.

```bash
./scripts/dev.sh     # desarrollo: mir-server :8787 + Astro :4321  → abre http://localhost:4321
./scripts/serve.sh   # todo servido por mir-server              → abre http://127.0.0.1:8787
```

## Desplegar en Vercel

Se despliega como contenedor (`Dockerfile.vercel`): servidor Rust + clang + la web.

```bash
vercel deploy --prod --yes
```

Variables (entorno Production): `MIR_GITHUB_CLIENT_ID`, `MIR_GITHUB_CLIENT_SECRET`, `MIR_PUBLIC_URL` y la base de datos Upstash Redis del Marketplace (`KV_REST_API_URL`, `KV_REST_API_TOKEN`).

## Cómo funciona

1. La web (Astro + Tailwind + isla React con Monaco y three.js) manda tu C++ a `POST /api/run`.
2. **mir-server** (Rust, axum) lo compila con `clang++ --target=wasm32` contra un **mini-Unreal** (`sdk/include`: `FVector`, `FRotator`, `FMath`, `TArray`, `FString`, `UE_LOG`, `AActor`, `ACharacter`, timers, `DrawDebug*`…) en ~0,2 s.
3. Ejecuta el wasm con **wasmi** (intérprete con "combustible": un bucle infinito se corta, no cuelga nada) dentro de un **mundo simulado**: movimiento tipo `CharacterMovement`, colisiones con puertas y paredes, llaves, etc.
4. Cada escenario graba una **traza** (posiciones por frame, eventos, logs, debug draws), y una función Rust comprueba si tu código ha hecho lo correcto.
5. La web reproduce la traza en 3D y enseña ✓/✕ con un mensaje claro.

Lo que escribes es C++ de Unreal de verdad: mismos nombres, firmas y comportamiento (p. ej. `FInterpTo`, `SetTimer` reemplazando el handle, `Array index out of bounds`…). Lo puedes pegar en un proyecto de Unreal.

## Estructura

```
sdk/include/        mini-Unreal (cabeceras con las mismas rutas que en UE: GameFramework/Actor.h…)
sdk/runtime/        implementación: memoria, printf, actores→imports "ue.*", timers, exports
exercises/NN-xxx/   un ejercicio: exercise.json, enunciado.md, plantillas, solution/, glue.cpp
crates/mir-server/  servidor Rust: compile.rs, sim/ (mundo, host wasm, runner), exercises/ (escenarios)
web/                Astro: páginas, ExerciseApp (React), CodeEditor (Monaco), Viewer3D (three.js)
```

## Tests

```bash
cargo test
```

Comprueba, para **cada ejercicio**, que la solución pasa todos los escenarios y que la plantilla compila pero falla alguno.

---

Make It Real no es Unreal Engine ni está afiliado a Epic Games: simula una parte de su API para aprender.
