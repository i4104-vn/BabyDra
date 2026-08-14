#!/bin/bash
set -e

echo "============================================="
echo "        BabyDra Hot Update & Reload"
echo "============================================="

# 1. Pull latest code
echo "Pulling latest code changes..."
# git pull origin || true

# 2. Rebuild in release mode
echo "Rebuilding in release mode..."
cargo build --release

# 3. Create local bin and log directories if needed
LOCAL_BIN="$HOME/.local/bin"
LOG_DIR="$HOME/.cache/babydra"
REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_SOURCE="$REPO_DIR/configs/labwc"
DEST_DIR="$HOME/.config/labwc"

mkdir -p "$LOCAL_BIN"
mkdir -p "$LOG_DIR"

# 4. Stop running panel/menu/switcher instances
echo "Stopping active processes..."
killall babydra-panel || true
killall babydra-switcher || true
killall babydra-screenshot || true
killall babydra-lock || true
killall babydra-image-preview || true
killall babydra-preview || true
killall babydra-settings || true
killall babydra-explore || true
killall babydra-greeter || true

# 5. Overwrite binaries in ~/.local/bin and /usr/bin
echo "Installing new binaries..."
cp target/release/babydra-panel "$LOCAL_BIN/babydra-panel"
cp target/release/babydra-switcher "$LOCAL_BIN/babydra-switcher"
cp target/release/babydra-screenshot "$LOCAL_BIN/babydra-screenshot"
cp target/release/babydra-lock "$LOCAL_BIN/babydra-lock"
cp target/release/babydra-preview "$LOCAL_BIN/babydra-preview"
cp target/release/babydra-settings "$LOCAL_BIN/babydra-settings"
cp target/release/babydra-explore "$LOCAL_BIN/babydra-explore"
sudo cp target/release/babydra-greeter /usr/bin/babydra-greeter

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

# Register default folder handler
echo "Registering default folder handler..."
cat << 'EOF' > "$HOME/.local/share/applications/babydra-explore.desktop"
[Desktop Entry]
Type=Application
Name=BabyDra Explore
Comment=Explore files and folders
Exec=/home/i4104/.local/bin/babydra-explore %u
Icon=system-file-manager
Terminal=false
Categories=System;FileTools;FileManager;GTK;
MimeType=inode/directory;
NoDisplay=false
EOF
chmod +x "$HOME/.local/share/applications/babydra-explore.desktop"
update-desktop-database "$HOME/.local/share/applications" || true
xdg-mime default babydra-explore.desktop inode/directory || true

echo "Syncing labwc config from $CONFIG_SOURCE to $DEST_DIR..."

mkdir -p "$DEST_DIR"

# Copy all files and directories to ~/.config/labwc
cp -r "$CONFIG_SOURCE"/* "$DEST_DIR"/

# Sync system GTK and fontconfig configurations
mkdir -p "$HOME/.config/gtk-3.0" "$HOME/.config/gtk-4.0" "$HOME/.config/fontconfig"
cp "$CONFIG_SOURCE/settings.ini" "$HOME/.config/gtk-3.0/settings.ini"
cp "$CONFIG_SOURCE/settings.ini" "$HOME/.config/gtk-4.0/settings.ini"
cp "$CONFIG_SOURCE/fonts.conf" "$HOME/.config/fontconfig/fonts.conf"

# Apply font to GNOME/GTK desktop interface via gsettings
gsettings set org.gnome.desktop.interface font-name 'Segoe UI Variable Static Text 13' 2>/dev/null || true
gsettings set org.gnome.desktop.interface document-font-name 'Segoe UI Variable Static Text 13' 2>/dev/null || true

# Refresh font cache
fc-cache -f 2>/dev/null || true

# Ensure scripts have execute permission
chmod +x "$DEST_DIR"/autostart 2>/dev/null || true
chmod +x "$DEST_DIR"/scripts/* 2>/dev/null || true

# Reconfigure labwc if running
if pgrep -x "labwc" > /dev/null; then
  labwc --reconfigure 2>/dev/null || true
  echo "✓ labwc reconfigured successfully!"
else
  echo "i labwc is not currently running."
fi

echo "✓ Done! Labwc and GTK configuration updated in $DEST_DIR."

# 7. Start the panel and redirect stdout/stderr to log file
echo "Starting babydra-panel..."
killall fnott || true
killall xfce4-notifyd || true

~/.local/bin/babydra-panel &