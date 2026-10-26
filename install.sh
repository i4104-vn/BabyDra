#!/bin/bash
set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Ensure all remote tracking branches are fetched on clean clone
if [ -d "$REPO_ROOT/.git" ]; then
    git -C "$REPO_ROOT" config --add remote.origin.fetch "+refs/heads/*:refs/remotes/origin/*" 2>/dev/null || true
    git -C "$REPO_ROOT" fetch --all --prune 2>/dev/null || true
fi

cd "$REPO_ROOT/install"

if [ -f target/release/babydra-installer ]; then
    ./target/release/babydra-installer "$@"
else
    cargo run --release -- "$@"
fi
