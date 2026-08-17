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
cargo check "${CARGO_ARGS[@]}"

echo "==> cargo fmt --check"
cargo fmt --check

echo "==> cargo clippy ${CARGO_ARGS[*]} -- -D warnings"
cargo clippy "${CARGO_ARGS[@]}" -- -D warnings

echo "==> cargo test ${CARGO_ARGS[*]}"
cargo test "${CARGO_ARGS[@]}"

echo "✔ All checks passed."
