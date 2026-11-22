#!/usr/bin/env bash
# Arranca Make It Real en modo desarrollo:
#   - mir-server (Rust) en http://127.0.0.1:8787  → compila y simula
#   - Astro dev       en http://localhost:4321    → la web (con proxy /api → 8787)
set -euo pipefail
cd "$(dirname "$0")/.."

cargo build -p mir-server
./target/debug/mir-server &
SERVER=$!
trap 'kill $SERVER 2>/dev/null' EXIT INT TERM

cd web
[ -d node_modules ] || pnpm install
pnpm dev
