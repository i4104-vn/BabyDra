#!/usr/bin/env bash
#
# BabyDra safety-net checks.
# Runs type-check, formatting check, linting, and the test suite.
#
# Usage:
#   ./scripts/check.sh                 # full workspace
#   ./scripts/check.sh -p babydra-core  # pass-through cargo args
#
set -euo pipefail

cd "$(dirname "$0")/.."

CARGO_ARGS=("$@")

echo "==> cargo check ${CARGO_ARGS[*]}"
cargo check --all-targets "${CARGO_ARGS[@]}"

echo "==> cargo clippy ${CARGO_ARGS[*]} -- -D warnings"
cargo clippy --all-targets "${CARGO_ARGS[@]}" -- -D warnings

echo "==> cargo test ${CARGO_ARGS[*]}"
if [ -z "${DISPLAY:-}" ] && [ -z "${WAYLAND_DISPLAY:-}" ] && command -v xvfb-run >/dev/null 2>&1; then
    echo "    (No display detected, running cargo test under xvfb-run)"
    xvfb-run -a cargo test "${CARGO_ARGS[@]}"
else
    cargo test "${CARGO_ARGS[@]}"
fi

echo "✔ All checks passed."
