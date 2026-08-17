#!/bin/bash
set -e

# Run the installer TUI.
# The installer is a standalone crate that lives in `install/` on the `main`
# branch, where there is no workspace root — so build/run from this folder.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if [ -f target/release/babydra-installer ]; then
    ./target/release/babydra-installer "$@"
else
    cargo run --release -- "$@"
fi
