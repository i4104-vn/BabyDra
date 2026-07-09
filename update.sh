#!/bin/bash
set -e

echo "============================================="
echo "        BabyDra Hot Update & Reload"
echo "============================================="

# 1. Pull latest code
echo "Pulling latest code changes..."
git pull origin main || git pull || true

# 2. Rebuild in release mode
echo "Rebuilding in release mode..."
cargo build --release

# 3. Create local bin and log directories if needed
LOCAL_BIN="$HOME/.local/bin"
LOG_DIR="$HOME/.cache/babydra"
mkdir -p "$LOCAL_BIN"
mkdir -p "$LOG_DIR"

# 4. Stop running panel/menu/switcher instances
echo "Stopping active processes..."
killall babydra-panel || true
killall babydra-menu || true
killall babydra-switcher || true
killall babydra-screenshot || true
killall babydra-lock || true
killall babydra-image-preview || true

# 5. Overwrite binaries in ~/.local/bin
echo "Installing new binaries..."
cp target/release/babydra-panel "$LOCAL_BIN/babydra-panel"
cp target/release/babydra-menu "$LOCAL_BIN/babydra-menu"
cp target/release/babydra-switcher "$LOCAL_BIN/babydra-switcher"
cp target/release/babydra-screenshot "$LOCAL_BIN/babydra-screenshot"
cp target/release/babydra-lock "$LOCAL_BIN/babydra-lock"
cp target/release/babydra-image-preview "$LOCAL_BIN/babydra-image-preview"

# 6. Reload labwc settings
echo "Reloading labwc compositor..."
labwc --reconfigure || true

# 7. Start the panel and redirect stdout/stderr to log file
echo "Starting babydra-panel..."
killall fnott || true
killall xfce4-notifyd || true
~/.local/bin/babydra-panel > "$LOG_DIR/panel.log" 2>&1 &
disown

echo "============================================="
echo "Update complete! Streaming panel logs below..."
echo "Press Ctrl+C to exit log streaming."
echo "============================================="
sleep 1
tail -n 30 -f "$LOG_DIR/panel.log"
