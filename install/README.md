# 🐉 BabyDra Step-by-Step TUI Installer

A fast, interactive Terminal User Interface (TUI) built in Rust with `ratatui` and `crossterm` for deploying the BabyDra Wayland desktop environment without recompiling from source.

## ✨ Features
- **Direct Pre-built Binary Copy**: Scans `target/release/` (or custom source folder) and deploys binaries directly to `~/.local/bin/` with `0755` permissions.
- **Centralized `/var/lib/babydra` Staging**: Bundles all binaries, wallpapers, and brand logos into `/var/lib/babydra/` and `/usr/share/babydra/` (`chmod 777`) for greeter and system services.
- **2-Panel Sidebar Wizard**: Clean 9-step wizard with status badges (`●` Completed, `►▶` Active, `○` Pending).
- **Preset Profiles**: Select between **Full Desktop (Recommended)**, **Binaries & /var/lib Only**, and **Custom Selection**.
- **System Package Integration**: Optionally verifies and installs Arch Linux pacman dependencies, AUR packages via `yay`, and configures CPU performance permissions / `i2c-dev`.
- **Desktop Dotfiles & Themes**: Syncs labwc autostart, rc.xml, GTK-3/4 settings, fontconfig, Kitty, Neovim, Fastfetch, We10X icons, and Twilight cursors.
- **Display Manager Setup**: Configures `/etc/greetd/config.toml` (cage + `/usr/bin/babydra-greeter`) and masks secondary VTs to prevent login screen flicker.
- **Real-Time Live Logs**: Streaming color-tagged logs and progress gauge.

---

## 🚀 How to Run

From the repository root:
```bash
./install/run.sh
```
Or with cargo:
```bash
cargo run -p babydra-installer --release
```
To specify a custom pre-built binary source folder:
```bash
./install/run.sh /path/to/custom/binaries
```

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `1` - `9` | Jump directly to step 1 through 9 |
| `Tab` / `n` | Next Step |
| `BackTab` / `p` | Previous Step |
| `↑` / `↓` / `j` / `k` | Navigate items in the active step |
| `Space` | Toggle checkbox for focused item |
| `a` / `A` | Select / Deselect all items in active step |
| `i` / `Enter` | Start installation (triggers confirmation dialog) |
| `s` | Change binary source directory |
| `r` | Rescan binary source directory |
| `Space` | Step 6: select variant (theme + app list source) |
| `c` | Clear log buffer |
| `g` / `G` | Jump to top / bottom of log stream |
| `?` | Show Help & Keybindings modal |
| `q` / `Ctrl+C` | Quit installer |
