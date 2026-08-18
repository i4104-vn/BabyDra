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

# Copy wallpaper to standard config dir
mkdir -p "$HOME/.babydra"
cp wallpaper.png "$HOME/.babydra/wallpaper.png"

# Setup default autostart and rc.xml by copying them from configs/labwc/
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cp "$SCRIPT_DIR/configs/labwc/autostart" "$AUTOSTART_FILE"
chmod +x "$AUTOSTART_FILE"
echo "Configured labwc autostart at $AUTOSTART_FILE"

cp "$SCRIPT_DIR/configs/labwc/rc.xml" "$RC_FILE"
echo "Configured labwc rc.xml at $RC_FILE"

cp "$SCRIPT_DIR/configs/labwc/themerc-override" "$HOME/.config/labwc/themerc-override"
echo "Configured labwc themerc-override at $HOME/.config/labwc/themerc-override"
mkdir -p "$HOME/.config/labwc/themes"
cp -r "$SCRIPT_DIR/configs/labwc/themes/"* "$HOME/.config/labwc/themes/"
mkdir -p "$HOME/.config/labwc/scripts"
cp -r "$SCRIPT_DIR/configs/labwc/scripts/"* "$HOME/.config/labwc/scripts/"
chmod +x "$HOME/.config/labwc/scripts/"*
cp -r "$SCRIPT_DIR/configs/themes/BabyDra" "$HOME/.local/share/themes/"
echo "Configured labwc theme BabyDra"

# Register default image handler in ~/.local/share/applications
echo "Registering default image handler..."
mkdir -p "$HOME/.local/share/applications"
cat << 'EOF' > "$HOME/.local/share/applications/babydra-preview.desktop"
[Desktop Entry]
Type=Application
Name=BabyDra Preview
Comment=Viewer for images
Exec=/home/i4104/.local/bin/babydra-preview %f
Icon=image-x-generic
Terminal=false
Categories=Graphics;Viewer;GTK;
MimeType=image/png;image/jpeg;image/gif;image/webp;image/bmp;
NoDisplay=false
EOF

chmod +x "$HOME/.local/share/applications/babydra-preview.desktop"
update-desktop-database "$HOME/.local/share/applications" || true
xdg-mime default babydra-preview.desktop image/png image/jpeg image/gif image/webp image/bmp || true

# Register Settings application entry
echo "Registering settings manager entry..."
cat << 'EOF' > "$HOME/.local/share/applications/babydra-settings.desktop"
[Desktop Entry]
Type=Application
Name=BabyDra Settings
Comment=Configure system settings
Exec=/home/i4104/.local/bin/babydra-settings
Icon=preferences-system
Terminal=false
Categories=Settings;HardwareSettings;GTK;
NoDisplay=false
EOF
chmod +x "$HOME/.local/share/applications/babydra-settings.desktop"
update-desktop-database "$HOME/.local/share/applications" || true

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
