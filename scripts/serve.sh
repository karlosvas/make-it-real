#!/usr/bin/env bash
# Construye la web y lo sirve todo desde mir-server en http://127.0.0.1:8787
set -euo pipefail
cd "$(dirname "$0")/.."
(cd web && { [ -d node_modules ] || pnpm install; } && pnpm build)
cargo run --release -p mir-server
