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
killall babydra-desktop || true
killall babydra-launcher || true
killall babydra-image-preview || true
killall babydra-preview || true
killall babydra-settings || true
killall fnott || true
killall xfce4-notifyd || true 

# Resolve the repo root (parent of scripts/) so this script works from any CWD.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
LOCAL_BIN="$HOME/.local/bin"

# Copy wallpaper to standard config dir if missing
mkdir -p "$HOME/.babydra"
[ -f "$HOME/.babydra/wallpaper.png" ] || cp "$REPO_ROOT/wallpaper.png" "$HOME/.babydra/wallpaper.png" 2>/dev/null || true

# Setup default autostart and rc.xml by copying them from configs/labwc/
[ -f "$AUTOSTART_FILE" ] || cp "$REPO_ROOT/configs/labwc/autostart" "$AUTOSTART_FILE"
chmod +x "$AUTOSTART_FILE" 2>/dev/null || true
echo "Configured labwc autostart at $AUTOSTART_FILE"

cp "$REPO_ROOT/configs/labwc/rc.xml" "$RC_FILE"
echo "Configured labwc rc.xml at $RC_FILE"

mkdir -p "$HOME/.config/labwc/scripts"
cp -r "$REPO_ROOT/configs/labwc/scripts/"* "$HOME/.config/labwc/scripts/"
chmod +x "$HOME/.config/labwc/scripts/"*
cp -r "$REPO_ROOT/configs/themes/BabyDra" "$HOME/.local/share/themes/"
echo "Configured labwc theme BabyDra"

# Register desktop entries from desktops/
echo "Installing desktop application entries from desktops/..."
mkdir -p "$HOME/.local/share/applications"
if [ -d "$REPO_ROOT/desktops" ]; then
    cp "$REPO_ROOT/desktops/"*.desktop "$HOME/.local/share/applications/"
    chmod +x "$HOME/.local/share/applications/"*.desktop 2>/dev/null || true
    update-desktop-database "$HOME/.local/share/applications" || true
fi

# Configure default application MIME associations
xdg-mime default babydra-preview.desktop image/png image/jpeg image/gif image/webp image/bmp || true
xdg-mime default babydra-explore.desktop inode/directory || true
xdg-mime default babydra-notepad.desktop text/plain || true


# Commented out software rendering to allow GPU hardware acceleration for 120 FPS.
# Uncomment these if running in a VM without 3D acceleration.
# export WLR_RENDERER=pixman
# export WLR_NO_HARDWARE_CURSORS=1

# Verify critical display controls are present
if ! command -v ddcutil &> /dev/null; then
    echo "Warning: 'ddcutil' is not installed. Brightness controls for external monitors will not be available."
fi
if [ ! -f "/usr/share/dbus-1/services/com.ddcutil.DdcutilService.service" ]; then
    echo "Warning: 'ddcutil-service' D-Bus service is missing. External monitor brightness via D-Bus will not function."
fi

echo "============================================="
echo "Starting labwc compositor with BabyDra..."
echo "Press Ctrl+Alt+Backspace to exit labwc."
echo "============================================="

exec labwc
