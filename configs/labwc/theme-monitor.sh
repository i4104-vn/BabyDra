#!/bin/bash
# Standalone GSettings color-scheme monitor for labwc

# Get initial color scheme
VAL=$(gsettings get org.gnome.desktop.interface color-scheme 2>/dev/null | tr -d "'")

update_theme() {
  local is_dark="$1"
  if [ "$is_dark" = "prefer-dark" ] || [ -z "$is_dark" ]; then
    cat << 'EOF' > "$HOME/.config/labwc/themerc-override"
# Premium Custom Window Titlebar Theme Override for Labwc (Dark)

# General
border.width: 0

# Padding
window.titlebar.padding.width: 12
window.titlebar.padding.height: 6

# Window border colors (Gold for active windows)
window.active.border.color: #e5c197
window.inactive.border.color: #161622

# Window titlebar background
window.active.title.bg.color: #0c0c14
window.inactive.title.bg.color: #080810
window.*.title.bg: Solid

# Window titlebar text
window.active.label.text.color: #ffffff
window.inactive.label.text.color: #7c7c8c
window.label.text.justify: center

# Window button sizes and spacing
window.button.width: 14
window.button.height: 14
window.button.spacing: 8

# Window button hover overlay
window.button.hover.bg.color: #ffffff15
window.button.hover.bg.corner-radius: 4

# Active Window Buttons (Mac-like colored dots)
window.active.button.close.unpressed.image.color: #ff5f56
window.active.button.max.unpressed.image.color: #27c93f
window.active.button.iconify.unpressed.image.color: #ffbd2e

# Inactive Window Buttons (Semi-transparent / desaturated)
window.inactive.button.close.unpressed.image.color: #ff5f5666
window.inactive.button.max.unpressed.image.color: #27c93f66
window.inactive.button.iconify.unpressed.image.color: #ffbd2e66
EOF
  else
    cat << 'EOF' > "$HOME/.config/labwc/themerc-override"
# Premium Custom Window Titlebar Theme Override for Labwc (Light)

# General
border.width: 0

# Padding
window.titlebar.padding.width: 12
window.titlebar.padding.height: 6

# Window border colors (Silver/Dark Gold for active windows)
window.active.border.color: #c5a177
window.inactive.border.color: #e1e1e6

# Window titlebar background
window.active.title.bg.color: #f0f0f5
window.inactive.title.bg.color: #e1e1e6
window.*.title.bg: Solid

# Window titlebar text
window.active.label.text.color: #1a1a24
window.inactive.label.text.color: #60606f
window.label.text.justify: center

# Window button sizes and spacing
window.button.width: 14
window.button.height: 14
window.button.spacing: 8

# Window button hover overlay
window.button.hover.bg.color: #00000010
window.button.hover.bg.corner-radius: 4

# Active Window Buttons (Mac-like colored dots)
window.active.button.close.unpressed.image.color: #ff5f56
window.active.button.max.unpressed.image.color: #27c93f
window.active.button.iconify.unpressed.image.color: #ffbd2e

# Inactive Window Buttons (Semi-transparent / desaturated)
window.inactive.button.close.unpressed.image.color: #ff5f5666
window.inactive.button.max.unpressed.image.color: #27c93f66
window.inactive.button.iconify.unpressed.image.color: #ffbd2e66
EOF
  fi
  labwc --reconfigure 2>/dev/null || true
}

update_theme "$VAL"

gsettings monitor org.gnome.desktop.interface color-scheme 2>/dev/null | while read -r line; do
  VAL=$(echo "$line" | awk '{print $NF}' | tr -d "'")
  update_theme "$VAL"
done
