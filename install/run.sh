#!/bin/bash
set -e

# Change to workspace root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$WORKSPACE_ROOT"

# Run the installer TUI
if [ -f "target/release/babydra-installer" ]; then
    ./target/release/babydra-installer "$@"
else
    cargo run -p babydra-installer --release -- "$@"
fi
