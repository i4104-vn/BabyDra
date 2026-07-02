#!/bin/bash

# Ensure local bin is in PATH
export PATH="$HOME/.local/bin:$PATH"
# Enable hardware acceleration for GTK4 (comment out/remove Cairo CPU renderer)
# export GSK_RENDERER=cairo

# Write config files for labwc
mkdir -p "$HOME/.config/labwc"
AUTOSTART_FILE="$HOME/.config/labwc/autostart"
RC_FILE="$HOME/.config/labwc/rc.xml"

echo "Stopping any running shell processes..."
killall babydra-panel || true
killall babydra-menu || true
killall babydra-launcher || true
killall dunst || true
killall mako || true
killall fnott || true
killall xfce4-notifyd || true 

# Copy wallpaper to standard config dir
mkdir -p "$HOME/.config/babydra"
cp wallpaper.png "$HOME/.config/babydra/wallpaper.png"

# Setup default autostart and rc.xml by copying them from configs/labwc/
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cp "$SCRIPT_DIR/configs/labwc/autostart" "$AUTOSTART_FILE"
chmod +x "$AUTOSTART_FILE"
echo "Configured labwc autostart at $AUTOSTART_FILE"

cp "$SCRIPT_DIR/configs/labwc/rc.xml" "$RC_FILE"
echo "Configured labwc rc.xml at $RC_FILE"

# Commented out software rendering to allow GPU hardware acceleration for 120 FPS.
# Uncomment these if running in a VM without 3D acceleration.
# export WLR_RENDERER=pixman
# export WLR_NO_HARDWARE_CURSORS=1

echo "============================================="
echo "Starting labwc compositor with BabyDra..."
echo "Press Ctrl+Alt+Backspace to exit labwc."
echo "============================================="

exec labwc
