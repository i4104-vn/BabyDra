#!/usr/bin/env bash
set -e
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cargo run --manifest-path "$REPO_ROOT/updater/Cargo.toml" "$@"
