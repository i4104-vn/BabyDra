#!/bin/bash
set -e

# Resolve repository root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Check Rust/Cargo toolchain availability
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo toolchain not found. Please install Rust before proceeding." >&2
    exit 1
fi

# Compile the TUI installer binary if not yet built
if [ ! -f "target/release/babydra-installer" ]; then
    echo "Compiling BabyDra TUI Installer in release mode (cargo build --release)..."
    cargo build --release -p babydra-installer
fi

# Launch the interactive TUI installer
exec ./target/release/babydra-installer "$@"
