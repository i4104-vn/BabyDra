#!/bin/bash
set -e

echo "============================================="
echo "   BabyDra Desktop Shell Installation Script"
echo "============================================="

# 1. Install all dependencies, the Rust toolchain, and system fonts via pacman
echo "Installing Arch Linux packages..."
sudo pacman -Syu --needed --noconfirm base-devel git pkgconf gtk4 gtk4-layer-shell rust labwc meson ninja playerctl grim wl-clipboard libnotify gammastep wlsunset wireplumber pipewire-pulse pipewire-alsa ddcutil

# Ensure i2c-dev kernel module is loaded and configured to load on boot
echo "Configuring i2c-dev kernel module..."
sudo modprobe i2c-dev || true
echo "i2c-dev" | sudo tee /etc/modules-load.d/i2c.conf > /dev/null || true

# Check if yay is installed, and install it from AUR if missing
if ! command -v yay &> /dev/null; then
    echo "yay not found, installing yay-bin from AUR..."
    rm -rf /tmp/yay-bin
    git clone https://aur.archlinux.org/yay-bin.git /tmp/yay-bin
    cd /tmp/yay-bin
    makepkg -si --noconfirm
    cd -
fi

# Install AUR packages using yay
yay -S --noconfirm dolphin github-desktop fastfetch neovim awww ddcutil-service
# Core UI fonts
yay -S --noconfirm inter-font ttf-ubuntu-font-family ttf-jetbrains-mono-nerd

# Nerd Font symbols (icons in terminal and panel)
yay -S --noconfirm ttf-nerd-fonts-symbols ttf-nerd-fonts-symbols-mono

# Font Awesome icons (used by many GTK/Qt apps)
yay -S --noconfirm otf-font-awesome ttf-font-awesome

# Noto font family — covers virtually all Unicode ranges
yay -S --noconfirm noto-fonts noto-fonts-cjk noto-fonts-emoji noto-fonts-extra

# Liberation fonts (metric-compatible fallback for Arial/Times/Courier)
yay -S --noconfirm ttf-liberation

# Icon theme and Qt theming
yay -S --noconfirm papirus-icon-theme kvantum-qt5

# 2. Install wlrctl from AUR if not present (required by the window switcher)
if ! command -v wlrctl &> /dev/null; then
    echo "wlrctl not found, installing from AUR..."
    rm -rf /tmp/wlrctl
    git clone https://aur.archlinux.org/wlrctl.git /tmp/wlrctl
    cd /tmp/wlrctl
    makepkg -si --noconfirm
    cd -
fi

# 3. Check and build wtype from source (required to fix Alt modifier release states)
LOCAL_BIN="$HOME/.local/bin"
mkdir -p "$LOCAL_BIN"
if [ ! -f "$LOCAL_BIN/wtype" ]; then
    echo "wtype not found, compiling from source..."
    rm -rf /tmp/wtype
    git clone https://github.com/atx/wtype.git /tmp/wtype
    cd /tmp/wtype
    meson setup build
    ninja -C build
    cp build/wtype "$LOCAL_BIN/wtype"
    cd -
fi

# 4. Clean and rebuild the workspace in release mode
echo "Cleaning and compiling BabyDra components in release mode..."
cargo clean
cargo build --release

# 5. Stop running panel/menu/switcher/lock instances
echo "Stopping active processes..."
killall babydra-panel || true
killall babydra-menu || true
killall babydra-switcher || true
killall babydra-screenshot || true
killall babydra-lock || true
killall babydra-launcher || true
killall babydra-image-preview || true
killall babydra-preview || true
killall babydra-settings || true
killall babydra-explore || true

# 6. Install the binaries
echo "Installing binaries to $LOCAL_BIN..."
cp target/release/babydra-panel "$LOCAL_BIN/babydra-panel"
cp target/release/babydra-menu "$LOCAL_BIN/babydra-menu"
cp target/release/babydra-switcher "$LOCAL_BIN/babydra-switcher"
cp target/release/babydra-screenshot "$LOCAL_BIN/babydra-screenshot"
cp target/release/babydra-lock "$LOCAL_BIN/babydra-lock"
cp target/release/babydra-launcher "$LOCAL_BIN/babydra-launcher"
cp target/release/babydra-preview "$LOCAL_BIN/babydra-preview"
cp target/release/babydra-settings "$LOCAL_BIN/babydra-settings"
cp target/release/babydra-explore "$LOCAL_BIN/babydra-explore"

# Copy wallpaper to standard config dir
mkdir -p "$HOME/.config/babydra"
cp wallpaper.png "$HOME/.config/babydra/wallpaper.png"

# Copy custom transparent logos to system pixmaps
sudo cp crates/babydra-preview/logo.png /usr/share/pixmaps/babydra-preview.png
sudo cp crates/babydra-settings/logo.png /usr/share/pixmaps/babydra-settings.png

# 7. Copy labwc configuration files from configs/labwc/
echo "Configuring labwc compositor integrations..."
mkdir -p "$HOME/.config/labwc"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cp "$SCRIPT_DIR/configs/labwc/autostart" "$HOME/.config/labwc/autostart"
chmod +x "$HOME/.config/labwc/autostart"
cp "$SCRIPT_DIR/configs/labwc/rc.xml" "$HOME/.config/labwc/rc.xml"
cp "$SCRIPT_DIR/configs/labwc/themerc-override" "$HOME/.config/labwc/themerc-override"
mkdir -p "$HOME/.config/labwc/themes"
cp -r "$SCRIPT_DIR/configs/labwc/themes/"* "$HOME/.config/labwc/themes/"
cp "$SCRIPT_DIR/configs/labwc/switcher.sh" "$HOME/.config/labwc/switcher.sh"
chmod +x "$HOME/.config/labwc/switcher.sh"

mkdir -p "$HOME/.local/share/themes"
cp -r "$SCRIPT_DIR/configs/themes/BabyDra" "$HOME/.local/share/themes/"

mkdir -p "$HOME/.local/share/icons"
cp -r "$SCRIPT_DIR/configs/themes/cursor/aosp-cursors" "$HOME/.local/share/icons/"
cp -r "$SCRIPT_DIR/configs/themes/icons/We10X" "$HOME/.local/share/icons/"
cp -r "$SCRIPT_DIR/configs/themes/icons/We10X-blue" "$HOME/.local/share/icons/"
cp -r "$SCRIPT_DIR/configs/themes/icons/We10X-blue-dark" "$HOME/.local/share/icons/"
cp -r "$SCRIPT_DIR/configs/themes/icons/We10X-dark" "$HOME/.local/share/icons/"
cp -r "$SCRIPT_DIR/configs/themes/cursor/Twilight-cursors" "$HOME/.local/share/icons/"

# 8. Reload configuration and restart panel
echo "Reloading labwc configuration and starting panel..."
labwc --reconfigure || true
mkdir -p "$HOME/.cache/babydra"
~/.local/bin/babydra-panel > "$HOME/.cache/babydra/panel.log" 2>&1 &

# 9. Configure system-wide default fonts (Inter & JetBrains Mono)
echo "Configuring system-wide default fonts for GTK and Fontconfig..."
mkdir -p "$HOME/.config/gtk-3.0" "$HOME/.config/gtk-4.0" "$HOME/.config/fontconfig"

cp "$SCRIPT_DIR/configs/labwc/settings.ini" "$HOME/.config/gtk-3.0/settings.ini"
cp "$SCRIPT_DIR/configs/labwc/settings.ini" "$HOME/.config/gtk-4.0/settings.ini"
cp "$SCRIPT_DIR/configs/labwc/fonts.conf" "$HOME/.config/fontconfig/fonts.conf"

# 10. Configure fastfetch
echo "Configuring fastfetch..."
mkdir -p "$HOME/.config/fastfetch"
cp "$SCRIPT_DIR/configs/fastfetch/config.jsonc" "$HOME/.config/fastfetch/config.jsonc"
cp "$SCRIPT_DIR/configs/fastfetch/logo.png" "$HOME/.config/fastfetch/logo.png"

# Rebuild font cache
echo "Rebuilding font cache..."
fc-cache -fv || true

# 11. Configure default applications for image previews
echo "Registering default image handler..."
mkdir -p "$HOME/.local/share/applications"
cat << EOF > "$HOME/.local/share/applications/babydra-preview.desktop"
[Desktop Entry]
Type=Application
Name=BabyDra Preview
Comment=Viewer for images
Exec=/home/i4104/.local/bin/babydra-preview %f
Icon=babydra-preview
Terminal=false
Categories=Graphics;Viewer;GTK;
MimeType=image/png;image/jpeg;image/gif;image/webp;image/bmp;
NoDisplay=false
EOF

chmod +x "$HOME/.local/share/applications/babydra-preview.desktop"
update-desktop-database "$HOME/.local/share/applications" || true
xdg-mime default babydra-preview.desktop image/png image/jpeg image/gif image/webp image/bmp || true

# 12. Configure Settings application entry
echo "Registering settings manager entry..."
cat << EOF > "$HOME/.local/share/applications/babydra-settings.desktop"
[Desktop Entry]
Type=Application
Name=BabyDra Settings
Comment=Configure system settings
Exec=/home/i4104/.local/bin/babydra-settings
Icon=babydra-settings
Terminal=false
Categories=Settings;HardwareSettings;GTK;
NoDisplay=false
EOF
chmod +x "$HOME/.local/share/applications/babydra-settings.desktop"
update-desktop-database "$HOME/.local/share/applications" || true

# 13. Configure default applications for folder explore
echo "Registering default folder handler..."
cat << EOF > "$HOME/.local/share/applications/babydra-explore.desktop"
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

echo "============================================="
echo "Installation & Setup complete!"
echo "Binaries installed to: $LOCAL_BIN"
echo "Log file location: ~/.cache/babydra/panel.log"
echo "You can launch labwc to use the full desktop shell."
echo "============================================="
