#!/usr/bin/env bash
set -euo pipefail

# Load .env.test without clobbering your normal .env
if [ -f .env.test ]; then
  set -a
  source .env.test
  set +a
fi

# Sanity: ensure the template DB is reachable
psql "${DATABASE_URL}" -c "SELECT 'ok' AS ping;" >/dev/null

# Run ONLY the tauri crate tests (adjust package name if needed)
cargo test -p rust-todo-app -- --nocapture
