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
killall babydra-preview || true
killall babydra-settings || true
killall babydra-explore || true

# 5. Overwrite binaries in ~/.local/bin
echo "Installing new binaries..."
cp target/release/babydra-panel "$LOCAL_BIN/babydra-panel"
cp target/release/babydra-menu "$LOCAL_BIN/babydra-menu"
cp target/release/babydra-switcher "$LOCAL_BIN/babydra-switcher"
cp target/release/babydra-screenshot "$LOCAL_BIN/babydra-screenshot"
cp target/release/babydra-lock "$LOCAL_BIN/babydra-lock"
cp target/release/babydra-preview "$LOCAL_BIN/babydra-preview"
cp target/release/babydra-settings "$LOCAL_BIN/babydra-settings"
cp target/release/babydra-explore "$LOCAL_BIN/babydra-explore"

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
