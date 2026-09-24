#!/usr/bin/env bash
# Automatic Battery Saver Checker Daemon for BabyDra
# Checks system battery and triggers babydra-settings CLI when saver threshold is reached.

CONF_FILE="$HOME/.babydra/babydra.conf"

parse_conf() {
    [[ ! -f "$CONF_FILE" ]] && { ENABLED="true"; THRESHOLD=20; return; }
    ENABLED=$(awk -F'=' '/\[power\]/,/\[.*\]/ { if ($1 ~ /auto_saver_enabled/) { gsub(/[ "]/, "", $2); print $2 } }' "$CONF_FILE" | head -n 1)
    THRESHOLD=$(awk -F'=' '/\[power\]/,/\[.*\]/ { if ($1 ~ /saver_threshold/) { gsub(/[ "]/, "", $2); print $2 } }' "$CONF_FILE" | head -n 1)
    ENABLED=${ENABLED:-"true"}
    THRESHOLD=${THRESHOLD:-20}
}

check_battery() {
    parse_conf
    [[ "$ENABLED" != "true" ]] && return

    local bat
    bat=$(ls -d /sys/class/power_supply/BAT* 2>/dev/null | head -n 1)
    [[ -z "$bat" || ! -d "$bat" ]] && return

    local status cap
    status=$(cat "$bat/status" 2>/dev/null)
    cap=$(cat "$bat/capacity" 2>/dev/null)
    [[ -z "$cap" ]] && return

    if [[ "$status" == "Discharging" ]] && (( cap <= THRESHOLD )); then
        # Delegate profile switching, notification, and i18n management to babydra-settings app
        if command -v babydra-settings >/dev/null 2>&1; then
            babydra-settings --apply-battery-saver
        elif [[ -x "$HOME/.local/bin/babydra-settings" ]]; then
            "$HOME/.local/bin/babydra-settings" --apply-battery-saver
        fi
    fi
}

if [[ "$1" == "--once" ]]; then
    check_battery
else
    while true; do
        check_battery
        sleep 5
    done
fi
