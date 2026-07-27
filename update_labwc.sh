#!/bin/bash
# Sync BabyDra labwc configuration from configs/labwc to ~/.config/labwc and reconfigure

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_SOURCE="$REPO_DIR/configs/labwc"
DEST_DIR="$HOME/.config/labwc"

echo "Syncing labwc config from $CONFIG_SOURCE to $DEST_DIR..."

mkdir -p "$DEST_DIR"

# Copy all files and directories to ~/.config/labwc
cp -r "$CONFIG_SOURCE"/* "$DEST_DIR"/

# Ensure scripts have execute permission
chmod +x "$DEST_DIR"/autostart 2>/dev/null || true
chmod +x "$DEST_DIR"/switcher.sh 2>/dev/null || true

# Reconfigure labwc if running
if pgrep -x "labwc" > /dev/null; then
  labwc --reconfigure 2>/dev/null || killall -s SIGHUP labwc 2>/dev/null || true
  echo "✓ labwc reconfigured successfully!"
else
  echo "i labwc is not currently running."
fi

echo "✓ Done! Labwc configuration updated in $DEST_DIR."
