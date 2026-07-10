#!/bin/bash
# Standalone GSettings color-scheme monitor for labwc

# Get initial color scheme
VAL=$(gsettings get org.gnome.desktop.interface color-scheme 2>/dev/null | tr -d "'")

update_theme() {
  local is_dark="$1"
  if [ "$is_dark" = "prefer-dark" ] || [ -z "$is_dark" ]; then
    cp "$HOME/.config/labwc/themes/dark" "$HOME/.config/labwc/themerc-override"
  else
    cp "$HOME/.config/labwc/themes/light" "$HOME/.config/labwc/themerc-override"
  fi
  labwc --reconfigure 2>/dev/null || true
}

update_theme "$VAL"

gsettings monitor org.gnome.desktop.interface color-scheme 2>/dev/null | while read -r line; do
  VAL=$(echo "$line" | awk '{print $NF}' | tr -d "'")
  update_theme "$VAL"
done
