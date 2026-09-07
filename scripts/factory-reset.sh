#!/usr/bin/env bash
#
# BabyDra Factory Reset Script
# Restores system back to vanilla Arch Linux CLI / TTY login state.
#

set -euo pipefail

REMOVE_PACKAGES=true
AUTO_REBOOT=false
DRY_RUN=false

for arg in "$@"; do
    case "$arg" in
        --keep-packages)
            REMOVE_PACKAGES=false
            ;;
        --remove-packages)
            REMOVE_PACKAGES=true
            ;;
        --reboot)
            AUTO_REBOOT=true
            ;;
        --dry-run)
            DRY_RUN=true
            ;;
        --help|-h)
            echo "Usage: sudo $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --remove-packages   Uninstall BabyDra shell packages (default: true)"
            echo "  --keep-packages     Keep installed pacman/AUR packages"
            echo "  --reboot            Automatically reboot after reset completes"
            echo "  --dry-run           Print actions without modifying files"
            exit 0
            ;;
        *)
            echo "Unknown argument: $arg"
            exit 1
            ;;
    esac
done

# Ensure root privileges unless dry-run
if [ "$DRY_RUN" = false ] && [ "$EUID" -ne 0 ]; then
    echo "Error: This script must be run as root (or with sudo)."
    exit 1
fi

TARGET_USER="${SUDO_USER:-$(logname 2>/dev/null || echo "$USER")}"
TARGET_HOME=$(getent passwd "$TARGET_USER" 2>/dev/null | cut -d: -f6 || echo "/home/$TARGET_USER")

echo "========================================================"
echo "    Arch Linux Factory Reset — Reverting BabyDra"
echo "========================================================"
echo "Target User: $TARGET_USER"
echo "Target Home: $TARGET_HOME"
echo "Remove Packages: $REMOVE_PACKAGES"
echo "Dry Run: $DRY_RUN"
echo "========================================================"

run_cmd() {
    local desc="$1"
    shift
    echo "==> $desc"
    if [ "$DRY_RUN" = true ]; then
        echo "    [dry-run] $*"
    else
        "$@" || true
    fi
}

# 1. Stop active BabyDra and desktop compositor processes
echo ""
echo "[Step 1/6] Stopping active BabyDra and compositor processes..."
run_cmd "Stopping user processes" pkill -u "$TARGET_USER" -f "babydra-" || true
run_cmd "Stopping labwc compositor" pkill -u "$TARGET_USER" -x labwc || true
run_cmd "Stopping cage compositor" pkill -x cage || true

# 2. Restore standard Arch Linux systemd login target and getty console
echo ""
echo "[Step 2/6] Restoring Arch Linux TTY login console & disabling display manager..."
run_cmd "Stopping greetd service" systemctl stop greetd.service 2>/dev/null || true
run_cmd "Disabling greetd service" systemctl disable greetd.service 2>/dev/null || true
run_cmd "Removing greetd configuration" rm -rf /etc/greetd

for vt in 2 3 4 5 6; do
    run_cmd "Unmasking getty@tty${vt}.service" systemctl unmask "getty@tty${vt}.service" 2>/dev/null || true
done
run_cmd "Enabling getty@tty1.service" systemctl enable getty@tty1.service 2>/dev/null || true
run_cmd "Setting default target to multi-user" systemctl set-default multi-user.target 2>/dev/null || true

# 3. Clean system-wide BabyDra files
echo ""
echo "[Step 3/6] Cleaning system-wide BabyDra directories and modules..."
run_cmd "Removing /usr/bin/babydra-greeter" rm -f /usr/bin/babydra-greeter
run_cmd "Removing /usr/share/babydra" rm -rf /usr/share/babydra
run_cmd "Removing /var/lib/babydra" rm -rf /var/lib/babydra
run_cmd "Removing i2c autoload conf" rm -f /etc/modules-load.d/i2c.conf
run_cmd "Removing perf tmpfiles conf" rm -f /etc/tmpfiles.d/babydra-perf.conf

# 4. Clean user configuration, caches, and installed binaries
echo ""
echo "[Step 4/6] Cleaning user configuration, caches, and binaries for $TARGET_USER..."
if [ -d "$TARGET_HOME" ]; then
    run_cmd "Removing user binaries in ~/.local/bin" rm -f "$TARGET_HOME/.local/bin"/babydra-* "$TARGET_HOME/.local/bin/wtype"
    run_cmd "Removing ~/.babydra directory" rm -rf "$TARGET_HOME/.babydra"
    run_cmd "Removing ~/.cache/babydra directory" rm -rf "$TARGET_HOME/.cache/babydra"
    run_cmd "Removing ~/.config/labwc directory" rm -rf "$TARGET_HOME/.config/labwc"
    run_cmd "Removing BabyDra theme from ~/.local/share/themes" rm -rf "$TARGET_HOME/.local/share/themes/BabyDra"
    run_cmd "Removing BabyDra desktop entries" rm -f "$TARGET_HOME/.local/share/applications"/babydra-*.desktop
    run_cmd "Removing FileManager1 dbus service" rm -f "$TARGET_HOME/.local/share/dbus-1/services/org.freedesktop.FileManager1.service"
    
    # Remove configs installed during install.sh
    run_cmd "Removing GTK configs" rm -f "$TARGET_HOME/.config/gtk-3.0/settings.ini" "$TARGET_HOME/.config/gtk-4.0/settings.ini"
    run_cmd "Removing fontconfig conf" rm -f "$TARGET_HOME/.config/fontconfig/fonts.conf"
    run_cmd "Removing fastfetch config" rm -rf "$TARGET_HOME/.config/fastfetch"
    run_cmd "Removing kitty config" rm -rf "$TARGET_HOME/.config/kitty"
    run_cmd "Removing nvim config" rm -rf "$TARGET_HOME/.config/nvim"

    # Reset GSettings GNOME desktop interface defaults
    if [ "$DRY_RUN" = false ]; then
        su - "$TARGET_USER" -c "gsettings reset org.gnome.desktop.interface font-name 2>/dev/null || true" || true
        su - "$TARGET_USER" -c "gsettings reset org.gnome.desktop.interface document-font-name 2>/dev/null || true" || true
        su - "$TARGET_USER" -c "gsettings reset org.gnome.desktop.interface monospace-font-name 2>/dev/null || true" || true
        su - "$TARGET_USER" -c "gsettings reset org.gnome.desktop.interface icon-theme 2>/dev/null || true" || true
        su - "$TARGET_USER" -c "gsettings reset org.gnome.desktop.interface cursor-theme 2>/dev/null || true" || true
        su - "$TARGET_USER" -c "update-desktop-database \"$TARGET_HOME/.local/share/applications\" 2>/dev/null || true" || true
    fi
fi

# 5. Uninstall BabyDra shell packages if requested
echo ""
echo "[Step 5/6] Cleaning packages..."
if [ "$REMOVE_PACKAGES" = true ] && [ "$DRY_RUN" = false ]; then
    # Packages specific to the BabyDra graphical shell
    SHELL_PACKAGES="labwc greetd cage gtk4-layer-shell wlrctl ddcutil-service gammastep wlsunset kvantum-qt5"
    INSTALLED_TO_REMOVE=""
    for pkg in $SHELL_PACKAGES; do
        if pacman -Q "$pkg" &>/dev/null; then
            INSTALLED_TO_REMOVE="$INSTALLED_TO_REMOVE $pkg"
        fi
    done

    if [ -n "$INSTALLED_TO_REMOVE" ]; then
        echo "Uninstalling shell packages: $INSTALLED_TO_REMOVE"
        pacman -Rns --noconfirm $INSTALLED_TO_REMOVE 2>/dev/null || pacman -R --noconfirm $INSTALLED_TO_REMOVE 2>/dev/null || true
    else
        echo "No BabyDra-specific packages needed removal."
    fi

    echo "Cleaning orphan packages..."
    ORPHANS=$(pacman -Qtdq 2>/dev/null || true)
    if [ -n "$ORPHANS" ]; then
        pacman -Rns --noconfirm $ORPHANS 2>/dev/null || true
    fi
else
    echo "Skipping package removal as requested."
fi

# 6. Rebuild font cache & finalize
echo ""
echo "[Step 6/6] Finalizing system state..."
run_cmd "Rebuilding font cache" fc-cache -r || true

echo "========================================================"
echo "✔ Factory Reset Complete!"
echo "The system has been restored to clean Arch Linux baseline."
echo "Login prompt will be available via standard TTY console."
echo "========================================================"

if [ "$AUTO_REBOOT" = true ] && [ "$DRY_RUN" = false ]; then
    echo "Rebooting system now..."
    sleep 1
    systemctl reboot
fi
