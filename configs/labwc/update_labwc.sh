#!/bin/bash
# Sync BabyDra labwc configuration to ~/.config/labwc and reconfigure

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEST_DIR="$HOME/.config/labwc"

echo "Syncing labwc config from $SCRIPT_DIR to $DEST_DIR..."

mkdir -p "$DEST_DIR"

# Copy all files and directories to ~/.config/labwc
cp -r "$SCRIPT_DIR"/* "$DEST_DIR"/

# Ensure scripts have execute permission
chmod +x "$DEST_DIR"/autostart 2>/dev/null || true
chmod +x "$DEST_DIR"/switcher.sh 2>/dev/null || true
chmod +x "$DEST_DIR"/update_labwc.sh 2>/dev/null || true

# Reconfigure labwc if running
if pgrep -x "labwc" > /dev/null; then
  labwc --reconfigure 2>/dev/null || killall -s SIGHUP labwc 2>/dev/null || true
  echo "✓ labwc reconfigured successfully!"
else
  echo "i labwc is not running currently."
fi

echo "✓ Done! Labwc configuration updated in $DEST_DIR."
