#!/bin/bash
set -e

# Resolve the repo root (parent of scripts/) so this script works from any CWD.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_DIR"

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
cat << EOF > "$HOME/.local/share/applications/babydra-preview.desktop"
[Desktop Entry]
Type=Application
Name=BabyDra Preview
Comment=Viewer for images
Exec=$LOCAL_BIN/babydra-preview %f
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
cat << EOF > "$HOME/.local/share/applications/babydra-settings.desktop"
[Desktop Entry]
Type=Application
Name=BabyDra Settings
Comment=Configure system settings
Exec=$LOCAL_BIN/babydra-settings
Icon=preferences-system
Terminal=false
Categories=Settings;HardwareSettings;GTK;
NoDisplay=false
EOF
chmod +x "$HOME/.local/share/applications/babydra-settings.desktop"
update-desktop-database "$HOME/.local/share/applications" || true

# Register default folder handler
echo "Registering default folder handler..."
cat << EOF > "$HOME/.local/share/applications/babydra-explore.desktop"
[Desktop Entry]
Type=Application
Name=BabyDra Explore
Comment=Explore files and folders
Exec=$LOCAL_BIN/babydra-explore %u
Icon=system-file-manager
Terminal=false
Categories=System;FileTools;FileManager;GTK;
MimeType=inode/directory;
NoDisplay=false
EOF
chmod +x "$HOME/.local/share/applications/babydra-explore.desktop"
update-desktop-database "$HOME/.local/share/applications" || true
xdg-mime default babydra-explore.desktop inode/directory || true

echo "============================================="
echo "Syncing all system & application configs..."
echo "============================================="

# 6.1 Copy wallpapers and brand assets
mkdir -p "$HOME/.babydra"
cp "$REPO_DIR/wallpaper.png" "$HOME/.babydra/wallpaper.png" 2>/dev/null || true
cp "$REPO_DIR/libs/babydra-core/src/services/logo.png" "$HOME/.babydra/logo.png" 2>/dev/null || true

sudo mkdir -p /usr/share/babydra /var/lib/babydra 2>/dev/null || true
sudo chmod 777 /var/lib/babydra 2>/dev/null || true
sudo cp "$REPO_DIR/libs/babydra-core/src/services/logo.png" /usr/share/babydra/babydra-preview.png 2>/dev/null || true
sudo cp "$REPO_DIR/libs/babydra-core/src/services/logo.png" /usr/share/babydra/babydra-settings.png 2>/dev/null || true
sudo cp "$REPO_DIR/libs/babydra-core/src/services/logo.png" /usr/share/babydra/logo.png 2>/dev/null || true
sudo cp "$REPO_DIR/libs/babydra-core/src/services/logo.png" /var/lib/babydra/logo.png 2>/dev/null || true
sudo cp "$REPO_DIR/wallpaper.png" /usr/share/babydra/wallpaper.png 2>/dev/null || true
sudo cp "$REPO_DIR/wallpaper.png" /var/lib/babydra/greeter_wallpaper.png 2>/dev/null || true

# 6.2 Sync labwc configuration
mkdir -p "$DEST_DIR"
cp -r "$CONFIG_SOURCE"/* "$DEST_DIR"/
chmod +x "$DEST_DIR"/autostart 2>/dev/null || true
chmod +x "$DEST_DIR"/scripts/* 2>/dev/null || true

# 6.3 Sync GTK and fontconfig configurations
mkdir -p "$HOME/.config/gtk-3.0" "$HOME/.config/gtk-4.0" "$HOME/.config/fontconfig"
cp "$CONFIG_SOURCE/settings.ini" "$HOME/.config/gtk-3.0/settings.ini"
cp "$CONFIG_SOURCE/settings.ini" "$HOME/.config/gtk-4.0/settings.ini"
cp "$CONFIG_SOURCE/fonts.conf" "$HOME/.config/fontconfig/fonts.conf"

# 6.4 Sync Kitty terminal configuration
mkdir -p "$HOME/.config/kitty"
if [ -d "$REPO_DIR/configs/kitty" ]; then
    cp -r "$REPO_DIR/configs/kitty/"* "$HOME/.config/kitty/" 2>/dev/null || true
fi

# 6.5 Sync Neovim configuration
mkdir -p "$HOME/.config/nvim"
if [ -d "$REPO_DIR/configs/nvim" ]; then
    cp -r "$REPO_DIR/configs/nvim/"* "$HOME/.config/nvim/" 2>/dev/null || true
fi

# 6.6 Sync Fastfetch configuration
mkdir -p "$HOME/.config/fastfetch"
if [ -d "$REPO_DIR/configs/fastfetch" ]; then
    cp -r "$REPO_DIR/configs/fastfetch/"* "$HOME/.config/fastfetch/" 2>/dev/null || true
fi

# 6.7 Sync Themes, Cursors, and Icons
mkdir -p "$HOME/.local/share/themes" "$HOME/.local/share/icons"
if [ -d "$REPO_DIR/configs/themes/BabyDra" ]; then
    cp -r "$REPO_DIR/configs/themes/BabyDra" "$HOME/.local/share/themes/" 2>/dev/null || true
fi
if [ -d "$REPO_DIR/configs/themes/cursor" ]; then
    for archive in "$REPO_DIR/configs/themes/cursor/"*.tar; do
        [ -f "$archive" ] && tar -xf "$archive" -C "$HOME/.local/share/icons/" 2>/dev/null || true
    done
fi
if [ -d "$REPO_DIR/configs/themes/icons" ]; then
    for archive in "$REPO_DIR/configs/themes/icons/"*.tar; do
        [ -f "$archive" ] && tar -xf "$archive" -C "$HOME/.local/share/icons/" 2>/dev/null || true
    done
fi

# 6.8 Apply GNOME/GTK desktop interface via gsettings
gsettings set org.gnome.desktop.interface font-name 'Segoe UI Variable Static Text 13' 2>/dev/null || true
gsettings set org.gnome.desktop.interface document-font-name 'Segoe UI Variable Static Text 13' 2>/dev/null || true
gsettings set org.gnome.desktop.interface monospace-font-name 'CaskaydiaCove Nerd Font 11' 2>/dev/null || true
gsettings set org.gnome.desktop.interface icon-theme 'We10X' 2>/dev/null || true
gsettings set org.gnome.desktop.interface cursor-theme 'Twilight-cursors' 2>/dev/null || true

# 6.9 Refresh font cache
fc-cache -f 2>/dev/null || true

# 6.10 Reconfigure labwc if running
if pgrep -x "labwc" > /dev/null; then
  labwc --reconfigure 2>/dev/null || true
  echo "✓ labwc reconfigured successfully!"
else
  echo "i labwc is not currently running."
fi

echo "✓ All configurations synced successfully!"

# 7. Start the panel and redirect stdout/stderr to log file
echo "Starting babydra-panel..."
killall fnott || true
killall xfce4-notifyd || true

~/.local/bin/babydra-panel &